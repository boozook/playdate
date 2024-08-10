//! Utils for compilation binaries.


/// Do not forget
/// - first positional - artifact path
/// - _this args_
/// - output path `-o`
pub const GCC_ARGS_LIB: &[&str] = &["-nostartfiles",
                                    "-mthumb",
                                    "-mcpu=cortex-m7",
                                    "-mfloat-abi=hard",
                                    "-mfpu=fpv5-sp-d16",
                                    "-D__FPU_USED=1",
                                    "-Wl,--cref,--gc-sections,--no-warn-mismatch,--emit-relocs",
                                    "-fno-exceptions",
                                    "-mword-relocations",
                                    "-fno-common",
                                    "--entry",
                                    "eventHandlerShim"];


pub const RUSTFLAGS_LIB_HOST: &[&str] = &["-Ctarget-cpu=native"];
pub const RUSTFLAGS_LIB_PLAYDATE: &[&str] = &["-Ctarget-cpu=cortex-m7",
                                              "-Ctarget-feature=-fp64",
                                              "-Clink-args=--emit-relocs",
                                              "-Crelocation-model=pic",
                                              "-Csoft-float=no",
                                              "-Clink-arg=--cref",
                                              "-Clink-arg=--gc-sections"];
/// For bin.
///
/// Do not forget
/// - `-Clink-arg=-T...link_map.ld`
/// - `-L{libs-search-paths}`
pub const RUSTFLAGS_BIN_PLAYDATE: &[&str] = &["-Ctarget-cpu=cortex-m7",
                                              "-Ctarget-feature=-fp64",
                                              "-Clink-args=--emit-relocs",
                                              "-Crelocation-model=pic",
                                              "-Csoft-float=no",
                                              "-Clink-arg=--cref",
                                              "-Clink-arg=--gc-sections",
                                              "-Clink-arg=--entry=eventHandlerShim"];

/// Bin that we giving to PDC.
pub const PDX_BIN_NAME_ELF: &str = "pdex.elf";
/// Bin that is product of PDC.
pub const PDX_BIN_NAME_BIN: &str = "pdex.bin";
/// File-stem for bin, elf, and dylib files.
pub const PDX_BIN_NAME_STEM: &str = "pdex";
/// Extension of Playdate package (dir).
pub const PDX_PKG_EXT: &str = "pdx";
/// Playdate package manifest filename.
pub const PDX_PKG_MANIFEST_FILENAME: &str = "pdxinfo";


pub const fn dylib_suffix_for_host() -> &'static str {
	if cfg!(target_os = "macos") {
		"dylib"
	} else if cfg!(unix) {
		"so"
	} else if cfg!(windows) {
		"dll"
	} else {
		panic!("platform not supported");
		#[cfg(all(not(unix), not(windows)))]
		{
			compile_error!("platform not supported")
		}
	}
}

pub const fn dylib_suffix_for_host_opt() -> Option<&'static str> {
	if cfg!(target_os = "macos") {
		Some("dylib")
	} else if cfg!(unix) {
		Some("so")
	} else if cfg!(windows) {
		Some("dll")
	} else {
		None
	}
}

pub fn dylib_suffix_for_opt(target_family: &str, target_os: &str) -> Option<&'static str> {
	match target_family {
		"unix" if target_os == "macos" => Some("dylib"),
		"unix" => Some("so"),
		"windows" => Some("dll"),
		_ => None,
	}
}

pub const fn static_lib_suffix() -> &'static str { "a" }


/// Compile-time path to the linker-script [LINK_MAP_BIN_SRC].
/// __Do note resolve, it contains file as dir.__
/// Path is relative to crate root by default, it depends on your rustc configuration.
pub const LINK_MAP_BIN_PATH: &str = concat!(file!(), "/../", "layout.x");

/// Linker-script for elf that built with rustc,
/// without arm-gcc and its std lib.
pub const LINK_MAP_BIN_SRC: &str = include_str!("layout.x");
