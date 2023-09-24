# Playdate Package Build Utils

Contains manifest format and "build assets by metadata" utils.



- - -

## Metadata

Here is the metadata format explanation in examples

### Package Info

```toml
# Playdate Package Info
# official doc: https://sdk.play.date/#pdxinfo
[package.metadata.playdate]
name = "{name}"               # optional, default is package.name
author = "{author}"           # optional, default is package.authors
version = "{version}"         # optional, default is package.version
description = "{description}" # optional, default is package.description
bundle-id = "com.yourcompany.{bundle_id}"

image-path = "img/system"      # optional
launch-sound-path = "sfx/jump" # optional

content-warning = "This game contains mild realistic, violence and bloodshed." # optional
content-warning2 = "Really scary game."                                        # optional
```


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


#### Assets Options

There is some options where to set asset options:
- `[package.metadata.playdate.assets.options]`
- `[package.metadata.playdate.options.assets]`

Both are equal but should not be both in one crate.

```toml
[package.metadata.playdate.assets.options]
dependencies = true    # allow to build assets for dependencies (default is `true`)
overwrite = true       # overwrite existing assets in build dir (default is `true`)
method = "link"        # "copy" or "link"   (default is `link`)  -  how assets should be collected, make symlinks or copy files
follow-symlinks = true # follow symlinks    (default is `true`)
```


### Options

Package build options, instruction for Playdate Package Build System such as cargo-playdate.

```toml
[package.metadata.playdate.options]
```

Available options is `assets`, see [Assets Options](#assets-options).

Currently there is no more options, it's just reserved for future use.




- - -

This software is not sponsored or supported by Panic.
