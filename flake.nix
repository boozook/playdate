{
  # nix flake lock --update-input cargo-playdate-src
  inputs = {
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    flake-utils.url = "github:numtide/flake-utils";

    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };
  };

  outputs = {
    self,
    crane,
    flake-utils,
    nixpkgs,
    rust-overlay,
  }:
    let
      supportedSystems = [
        flake-utils.lib.system.aarch64-darwin
        flake-utils.lib.system.x86_64-darwin
      ];
    in
      flake-utils.lib.eachSystem supportedSystems (
        system: let
          parameters = {
            inherit system;
            overlays = [
              (import rust-overlay)
            ];
          };

          pkgs = import nixpkgs parameters;

          playdateSdk = { version, hash }: (
            pkgs.stdenv.mkDerivation {
              name = "playdate-sdk-${version}";
              src = pkgs.fetchzip {
                inherit hash;
                url = "https://download.panic.com/playdate_sdk/PlaydateSDK-${version}.zip";
                stripRoot = false;
              };
              postUnpack = ''
                /usr/sbin/pkgutil --expand-full $src/PlaydateSDK.pkg source/PlaydateSDK
              '';
              installPhase = ''
                cp -R ./PlaydateSDK/PlaydateSDK.pkg/Payload/PlaydateSDK $out
                # runHook postInstall
              '';
              # postInstall = ''
              #   # Manually apply the patch since it's more intuitive to create
              #   # patches from the installed SDK.
              #   patch -d $out -p1 --input=${./nix/patches/playdate-sdk.patch}
              # '';
            }
          );

          # _rustToolchain = pkgs.rust-bin.stable."1.75.0".default;
          # _rustToolchain = pkgs.rust-bin.nightly."2023-12-28".default;
          # _rustToolchain = pkgs.rust-bin.nightly."2024-04-12".default;
          # _rustToolchain = pkgs.rust-bin.fromRustupToolchainFile cargoPlaydateRustToolchainPath;
          _rustToolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;

          rustToolchain = (_rustToolchain.override({
            # For device
            targets = [
              "thumbv7em-none-eabihf"
            ];
            extensions = [
              "cargo"
              "clippy"
              "rust-analyzer"
              "rust-src"
              "rust-std"
              "rustfmt"
            ];
          }));

          # NB: we don't need to overlay our custom toolchain for the *entire*
          # pkgs (which would require rebuidling anything else which uses rust).
          # Instead, we just want to update the scope that crane will use by
          # appending our specific toolchain there.
          craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;
          # craneLib = crane.lib."${system}";
        in
          rec {
            packages.cargo-playdate = craneLib.buildPackage {
              inherit (craneLib.crateNameFromCargoToml {
                cargoToml = ./cargo/Cargo.toml;
              }) pname version;
              src = ./.;
              nativeBuildInputs = [
                pkgs.cmake
                pkgs.libiconv

                # https://nixos.wiki/wiki/Rust#Building_the_openssl-sys_crate
                pkgs.openssl
                pkgs.pkg-config
              ] ++ pkgs.lib.optionals pkgs.stdenv.isDarwin (with pkgs.darwin.apple_sdk.frameworks; [
                CoreFoundation
                CoreServices
                IOKit
                Security
                SystemConfiguration
              ]);
              # init_args_before_cmd test fails, possibly due to CWD
              doCheck = false;
            };

            # packages.cmake = pkgs.cmake;

            packages.playdate-sdk = playdateSdk {
              version = "2.4.2";
              # hash = "";
              hash = "sha256-gDrNhJJjYeQEWpcgeHqNobg7n4qrIRURCDn+TUk30M0=";
            };

            devShell = pkgs.mkShell {
              ARM_GCC_PATH = "${pkgs.gcc-arm-embedded}/bin/arm-none-eabi-gcc";
              # CARGO_PLAYDATE_LOG = "trace"; https://docs.rs/env_logger/latest/env_logger/#enabling-logging
              # CC = "${pkgs.gcc-arm-embedded}/bin/arm-none-eabi-gcc";
              # CRATE_CC_NO_DEFAULTS = true;
              PLAYDATE_SDK_PATH = packages.playdate-sdk;
              #  If we have multiple Playdate devices connected
              #  at the same time, we should specify the serial number
              #  of a specific device.
              #
              #  If there is only one device, cargo-playdate will find the
              #  device itself and use a different interface (USB bulk) if
              #  possible, and only use the serial port as a fallback.
              #
              # See https://github.com/boozook/playdate/issues/314#issuecomment-2063353703
              # PLAYDATE_SERIAL_DEVICE = "/dev/cu.usbmodemPDU1_Y0503601";
              nativeBuildInputs = [
                packages.cargo-playdate
                # pkgs.gcc-arm-embedded # for arm-none-eabi-gcc
                rustToolchain
              ];
            };
          }
      );
}
