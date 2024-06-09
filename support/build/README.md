# Playdate Package Build Utils

Contains manifest format and "build assets by metadata" utils.



- - -

## Metadata

Here is the metadata format explanation in examples

### Package Info

The following fields are used to generate the package manifest:

```toml
# Playdate Package Info
# official doc: https://sdk.play.date/#pdxinfo
[package.metadata.playdate]
bundle-id = "com.yourcompany.game"
name = "My Game"               # default is package.name
author = "Alex"                # default is package.authors
version = "0.0"                # default is package.version
description = "short about"    # default is package.description

image-path = "img/system"
launch-sound-path = "sfx/jump"

content-warning = "This game contains mild realistic, violence and bloodshed."
content-warning2 = "Really scary game."

build-number = 42 # also can be string, e.g "42"

# also extra fields are supported
# acceptable types of values: string, number, boolean
foo = "bar"
```

_Note, only `bundle-id` is required, other fields are optional._


#### Target-specific Package Info

Main [Package Info](#package-info) can be overridden with special _table_ for a `bin` or `example`.
All manifest fields are acceptable, but optional.

Two formats are supported.
First is like a cargo's targets:

```toml
[[package.metadata.playdate.example]]
target = "existing-example-name" # pointing to cargo-target name
# next is same as for main manifest fields, all are optional:
bundle-id = "com.yourcompany.game.example"
name = "My Example"
```

Second if just a table:

```toml
[package.metadata.playdate.example.existing-example-name]
bundle-id = "com.yourcompany.game.example"
content-warning = "Scary experimental stuff."
```

_Important: you should not mix these two formats in the same document._


### Assets

Instructions for Playdate Package Build System such as cargo-playdate.

Describes where assets are stored, how and where they should be in the package.

```toml
[package.metadata.playdate.assets]
```

#### Dev-Assets

Assets that for examples or tests only, inherited by main assets.

```toml
[package.metadata.playdate.dev-assets]
```
Dev assets works the same way as main assets,
and further saying `assets` means both `assets` and `dev-assets`.


There is two options how to set assets - list or table:

#### Assets List

Simplest way to declare assets is just list of paths.

- Path can contain glob-patterns like `/**/*o*e.png`
- Path can contain env-vars like `${MY_VARIABLE}`
- Path can be absolute or relative to crate root

So all matched files will be included.

```toml
[package.metadata.playdate]
assets = ["assets/**/*.wav", "assets/**/*.png"]
```

If glob in path, resulting path of file starts with matched part of path, e.g.:
- for `assets/**/*.wav` it will be `foo/some.wav`, if `assets` contains `foo` dir


#### Assets Table

This is a complex way of specifying what assets should be included.
- Left hand is a path where asset should be in the package,
- Right hand is the path where source(s) should be found.

- Both hands can contain globs.
- Both hands can contain env-var queries like `${MY_VARIABLE}`

- Left hand path is relative to building playdate-package root
- Right hand path can be absolute or relative to crate root

```toml
[package.metadata.playdate.assets]
# Next line means that all png-files in SystemAssets dir wil be included and placed in img/system directory
"img/system/" = "${PLAYDATE_SDK_PATH}/Examples/Game Template/Source/SystemAssets/*.png"
# Next line means that jump.wav will be included and placed in package as sfx/jump.wav
"sfx/jump.wav" = "${PLAYDATE_SDK_PATH}/Examples/Level 1-1/Source/sfx/jump.wav"
# Next line means that img.png will be included in root of package
"/" = "assets/img.png" # path is relative to crate root
```

Also this way supports simple include and exclude instructions:
```toml
"rel-to-crate-root/file-to-include" = true   # left hand is a local path, relative to crate-root,
"file-to-exclude" = false  # OR resulting path that where asset will be in the resulting package.
```


### Options

Package build options, instruction for Playdate Package Build System such as cargo-playdate.

```toml
[package.metadata.playdate.options]
workspace = true           # use `workspace.metadata.playdate.options` as defaults (default is `false`)
assets.dependencies = true # just set or override corresponding value from `workspace.metadata`
```

Field `workspace` works like the cargo's feature [inheriting a dependency from a workspace][cargo-inheriting-dep-ws], turning on structural inheritance of `package.metadata.playdate.options` by `workspace.metadata.playdate.options`.


Available options is `assets`, see [Assets Options](#assets-options).

_Currently there is no more options, it's just reserved for future use._

This configuration is used for primary packages only. Primary packages are the ones the user selected on the command-line, either with `-p` flags or the defaults based on the current directory and the default workspace members.
So, `options` from top-level package are applying to entire dependency tree ignoring `options` of dependencies. Thus, only the end user controls how the assets will be collected & built.

_Note: this is depends on implementation, above is how it works in the reference impl `cargo-playdate`._


[cargo-inheriting-dep-ws]: https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#inheriting-a-dependency-from-a-workspace


#### Assets Options

This is how assets will be collected for your package.

```toml
[package.metadata.playdate.options.assets]
dependencies = true    # allow to build assets for dependencies (default is `false`)
overwrite = true       # overwrite existing assets in build dir (default is `true`, alias: `override`)
method = "link"        # "copy" or "link"   (default is `link`)  -  how assets should be collected, make symlinks or copy files
follow-symlinks = true # follow symlinks    (default is `true`)
```

Field `overwrite` also allows higher dependencies to overwrite assets of deeper dependency.


- - -

This software is not sponsored or supported by Panic.
