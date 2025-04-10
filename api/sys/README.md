# Modular low-level Playdate API

Low-level bindings to Playdate API with optional adapted official documentation and optional lang-items.

## Main concepts in a thesis statement

1. We must panic when something wrong and we cannot recover from it. To give an example, when we hit null-ptr in api, thi is fatal.
2. We must return error when API returns error.

That looks pretty hardcore. That's why there is four+ possible ways to get access to API endpoints, among them returning `Option` or `Result`.

_I've experimented enough with wrapping the entire API with results at every step, then painstakingly optimized their unfolding. And in the end I came to the conclusion that in this exact project the number of errors/results should be minimized._


## What's inside

- cffi bindings
- pre-generated cffi bindings
- minimal required parts such as lang-items
- simple entry point
- additional utils like `println` macro


## Prerequisites

<abbr title="Minimal Nightly Rust Version">MNRV</abbr> is [`1.81.0` from 2024-06-30][rust-toolchain.toml]

1. Rust __nightly__ toolchain (rustup is optional)
1. [Playdate SDK][sdk]
   - Ensure that env var `PLAYDATE_SDK_PATH` points to the SDK root
1. Follow the [official documentation][doc-prerequisites]
   - Ensure that `arm-none-eabi-gcc` or `gcc-arm-none-eabi` in your `PATH`

[sdk]: https://play.date/dev/#cardSDK
[doc-prerequisites]: https://sdk.play.date/Inside%20Playdate%20with%20C.html#_prerequisites
[rust-toolchain.toml]: https://github.com/boozook/playdate/blob/main/rust-toolchain.toml#L2


## Usage

<details><summary>Minimal example of the application</summary>

1. Setup library with crate-type
2. Add playdate-sys dependency

Cargo.toml:
```toml
[lib]
name = "example"
path = "src/lib.rs"
crate-type = [
	"dylib",     # for simulator
	"staticlib", # for hardware
]

[dependencies.pd]
package = "playdate-sys"
git = "this/repo/path.git"
```

3. Next is just minimal required initialization code and additionally code that prints all received events

src/lib.rs:
```rust
#![no_std]
use core::ffi::*;

#[macro_use]
extern crate alloc;

#[macro_use]
extern crate pd;
use pd::ffi::*;


#[no_mangle]
// Note: `_arg` is a key-code in simulator, otherwise it's just zero.
pub extern "C" fn eventHandlerShim(api: *const PlaydateAPI, event: PDSystemEvent, _arg: u32) -> c_int {
	match event {
		PDSystemEvent::kEventInit => unsafe {
			// register the API entry point
			pd::API = api;
			// get `setUpdateCallback` fn
			let f = (*(*api).system).setUpdateCallback.unwrap();
			// register update callback
			f(Some(on_update), core::ptr::null_mut());

			// `println` uses `API` internally, that set above
			println!("Init, Hello world!");
		},

		PDSystemEvent::kEventLock => println!("Lock"),
		PDSystemEvent::kEventUnlock => println!("Unlock"),
		PDSystemEvent::kEventPause => println!("Pause"),
		PDSystemEvent::kEventResume => println!("Resume"),
		PDSystemEvent::kEventLowPower => println!("LowPower"),
		PDSystemEvent::kEventTerminate => println!("Terminate"),
		PDSystemEvent::kEventInitLua => println!("InitLua"),
		// simulator only, keyboard events:
		PDSystemEvent::kEventKeyPressed => println!("KeyPressed"),
		PDSystemEvent::kEventKeyReleased => println!("KeyReleased"),
	}

	0 // zero means "OK, no error, continue please"
}

unsafe extern "C" fn on_update(_: *mut c_void) -> i32 { 1 /* `1` means "OK, continue updates" */ }
```

1. Also add the following config needed for proper build configuration

.cargo/config.toml:
```toml
[target.thumbv7em-none-eabihf]
rustflags = [
	"-Ctarget-cpu=cortex-m7",
	"-Ctarget-feature=-fp64",
	"-Clink-args=--emit-relocs",
	"-Crelocation-model=pic",
	"-Csoft-float=no",
	"-Clink-arg=--cref",
	"-Clink-arg=--gc-sections",
	"-Clink-arg=--entry=eventHandlerShim"
]

# Also I recommend to allow unstable options here:
[unstable]
unstable-options = true
```

5. Now build it
```shell
cargo build --lib --release --target=thumbv7em-none-eabihf -Zbuild-std=core,alloc -Zunstable-options
# Note: on windows use gcc-arm-none-eabi instead
arm-none-eabi-gcc ./target/thumbv7em-none-eabihf/release/libexample.a \
			-nostartfiles -mthumb -mcpu=cortex-m7 -mfloat-abi=hard -mfpu=fpv5-sp-d16 -D__FPU_USED=1 \
			-Wl,--cref,--gc-sections,--no-warn-mismatch,--emit-relocs -mword-relocations \
			-fno-common -fno-exceptions \
			-T$PLAYDATE_SDK_PATH/C_API/buildsupport/link_map.ld \
			-o ./target/thumbv7em-none-eabihf/release/example.elf \
			--entry eventHandlerShim
# Then prepare package with manifest and assets, place into it example.elf and call
# `$PLAYDATE_SDK_PATH/bin/pdc` with path of prepared package.
```

6. Then prepare package with manifest and assets, place into it example.elf and then
   call `$PLAYDATE_SDK_PATH/bin/pdc` with path of prepared package.
7. Install and run on device.


⚠️ Note that [cargo-playdate][cargo-playdate-crates] can do it all for you easily. Also it can build executable binaries.

</details>

- - -

See [examples](https://github.com/boozook/playdate/tree/main/api/sys/examples).


### Configure profile

Little recommendation is to add this to your Cargo.toml
```toml
[profile.dev]
panic = "abort"

[profile.release]
panic = "abort"
opt-level = "s"         # optimize for binary size (or use `3`, play with it)
overflow-checks = false # runtime integer overflow checks. (optionally, as you wish)
lto = "fat"
incremental = false
codegen-units = 1

debug = 0
strip = "symbols"        # or debuginfo
debug-assertions = false
```

This is just recommendation because this is entirely optional including `panic = "abort"` for example (unwinding there is challenging but not impossible).


## Configuration

There is some main kinds of features that you can control:
- `bindgen-...` controls binding generator and extra codegen features
- `bindings-...` to enable `derive`s for types and some codegen like documentation
- lang-items and other hand-crafted things.


### Control included parts

This crate contains some minimal required "parts" to build your application.

You can disable the features to prevent them from being and so you'll can use other allocator or panic-handler for example.

- `allocator`: global allocator
- `panic-handler`: global panic handler
- `eh-personality`: eh_personality for simulator-targets, dummy, empty, no-op

Non-default features:
- `entry-point`: simple minimal proxy entry point that caching API endpoint when app init.


### Control bindings generation

By default if we have pregenerated bindings for your configuration (target, profile, derives, etc...) we use it instead of build new.

⚠️ Env var `PLAYDATE_SDK_PATH` have to points to the PlayDate SDK directory as [described in official documentation][doc-env].

To prevent this behavior to use only _pre-built_ bindings set env var `IGNORE_EXISTING_PLAYDATE_SDK=1`.


[doc-env]: https://sdk.play.date/Inside%20Playdate%20with%20C.html#_set_playdate_sdk_path_environment_variable


There's `bindgen` used to generate bindings, and some features are re-exported:

- use `bindgen-runtime` to on/off bindgen uses runtime linking (dlopen) with libclang.
  Same for `bindgen-static`.
  [More in bindgen docs][bindgen-clang].
- use features like `bindings-derive-{name}` to ask bindgen to derive `{name}` to all entities if it possible
  - <details><summary>full list of derive- features</summary>

    - bindings-derive-default
    - bindings-derive-eq
    - bindings-derive-copy
    - bindings-derive-debug
    - bindings-derive-hash
    - bindings-derive-ord
    - bindings-derive-partialeq
    - bindings-derive-partialord

</details>

[bindgen-clang]: https://rust-lang.github.io/rust-bindgen/requirements.html?highlight=LIB#installing-clang


## Development

This crate should remain as low-level as possible.

It is possible to add extra things there only if that is:
- really small things
- absolutely necessary
- implementations of third-party traits (from other crates including core) for api types
- feature-gated, in case the thing has dependencies or isn't so small.


### Extension Development

You can add functionality that based on this package. Just create a new your oun package on-top this and re-export all features.

I must repeat myself, I apologize for intruding.

⚠️ If you want to create a library that based on this library, please __share following features of this package__
to allow everyone to properly configure entire dependency tree:
- `bindgen-runtime`
- `bindgen-static`

_This makes kind of paramount importance if a user is using several of these extensions and it can be a mess if they dictate different configurations for this package._

- `cargo new name-of-your-extension`
- `cargo add playdate-sys`
- copy & paste all features mentioned above into your Cargo.toml like this:
  ```toml
  [features]
  default = ["playdate-sys/default"]
  bindgen-runtime = ["playdate-sys/bindgen-runtime"]
  bindgen-static = ["playdate-sys/bindgen-static"]
  bindings-derive-debug = ["playdate-sys/bindings-derive-debug"]
  ```

For example see existing crates in the [repo/api][repo-api].

[repo-api]: https://github.com/boozook/playdate/tree/main/api



### How to generate _prebuilt_ bindings?

This is full example how to generate bindings with doc-comments,
but documentation is optional, so you can omit it - just remove all about docs:
- `bindings-documentation` feature
- execution `playdate-docs-parser`
- execution `rustfmt`

```bash
mk dir -p ./api/sys/gen

export PD_BUILD_PREBUILT=1

# all features excluding static- or runtime- linking bingen with libclang:
DERIVES_ALL=bindings-derive-default,bindings-derive-eq,bindings-derive-copy,bindings-derive-debug,bindings-derive-hash,bindings-derive-ord,bindings-derive-partialeq,bindings-derive-partialord
DERIVES_DEF=bindings-derive-debug

FEATURES_DEF=--features=bindings-documentation,$DERIVES_DEF
FEATURES_ALL=--features=bindings-documentation,$DERIVES_ALL

cargo build -p=playdate-sys $FEATURES_DEF
cargo build -p=playdate-sys $FEATURES_DEF --target=thumbv7em-none-eabihf

cargo build -p=playdate-sys $FEATURES_ALL
cargo build -p=playdate-sys $FEATURES_ALL --target=thumbv7em-none-eabihf

# optionally format bindings:
rustfmt ./api/sys/gen/*.rs
```

__Important note:__
To trigger changing _prebuilt_ bindings, you need set env var: `PD_BUILD_PREBUILT`, that allows changes outside of `OUT_DIR`.


[cargo-playdate-crates]: https://crates.io/crates/cargo-playdate
[crate-docs-example]: https://docs.rs/playdate-sys/0.1.3/playdate_sys/ffi/struct.playdate_sys.html


### Testing

Three available options:
1. normal tests
2. tests with mock
3. `miri` tests

In normal tests `mock` crate is available but not replaces bindings.
_Replacing bindings is needed because mock is not support all features and functionality of PlaydateOs._
```bash
cargo test -p=playdate-sys --features=lang-items --lib
```

Tests with mock uses __host-system allocator__ by `alloc` crate and optional std for printing _(`println` & `eprintln`)_.
```bash
# test with mock which uses allocator from alloc
RUSTFLAGS='--cfg mockrt="alloc"' cargo test -p=playdate-sys --features=lang-items --lib
# test with mock which uses allocator from alloc and std for print
RUSTFLAGS='--cfg mockrt="std"' cargo test -p=playdate-sys --features=lang-items --lib
```

Tests with miri also uses mock which supports miri and automatically uses transparent allocator which calls `miri`'s one.
```bash
# run miri evaluation for tests, with mock which supports miri
cargo miri test -p=playdate-sys --features=lang-items --lib

# run miri evaluation with mock as above
cargo miri run -p=playdate-sys --example=hello-world-bin --features=lang-items
```

### Testing dependent crates

Recomended solution for unit- and integration tests.

1. Move `playdate-sys`'s features which enables global handlers and/or lang-items to your crate's default features.

```toml
[features]
default = [
	"sys/lang-items", # feature-set with global panic handler and global allocator
	"entry-point", # simple minimal proxy entry point, just registers api endpoint
	# ...
]


[dependencies.sys]
package = "playdate-sys"
version = "*"
features = [ "alloc", "allocator-api" ] # any features which does not enables global handlers, lang-items.
```

And test:
```rust
#[cfg(test)]
mod tests {
	#[test]
	fn without_mock() {
		assert!(sys::api_opt!(system).is_none());
	}
}
```

So now you can run tests simply disable default features for your crate:

```bash
cargo test -p=your --lib --no-default-features
```

Same for miri:

```bash
cargo miri test -p=your --lib --no-default-features
```

And test:
```rust
#[cfg(test)]
mod tests {
	#[test]
	#[cfg(miri)]
	fn with_miri_mock() {
		// run executor with minimal set of events
		sys::mock::executor::minimal(); //  <-----------------------------------↴
		// now endpoint is `some` because already inited statically in previous ☝🏻 step
		assert!(sys::api_opt!(system).is_some());
	}
}
```

Also in some future it could be possible to use mock for normal tests, like this:
```bash
RUSTFLAGS='--cfg mockrt' cargo test ...
RUSTFLAGS='--cfg mockrt="std"' cargo test ...
```



- - -

⚠️ Prior to the version `1.0` API is unstable and can be changed without deprecation period.


This software is not sponsored or supported by Panic.
