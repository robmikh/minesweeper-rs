# minesweeper-rs
A port of [robmikh/Minesweeper](https://github.com/robmikh/Minesweeper) using [winrt-rs](https://github.com/microsoft/winrt-rs).

## Running
Running this sample requires at least Windows build 1803 (v10.0.17134.0). To compile and run, use [cargo](https://www.rust-lang.org/learn/get-started):

### Desktop
Running the sample as a normal desktop application can be done as follows:
```
cargo run --release
```

### UWP
Running the sample as a UWP application can be done by building for a `*-uwp-windows-msvc` target and then registering the app. NOTE: AppManifest.xml currently assumes `x86_64-uwp-windows-msvc` but can be updated.
```
cargo +local build --target x86_64-uwp-windows-msvc
Add-AppxPackage -Register AppxManifest.xml
```

Then launch Minesweeper-rs from the Start Menu.

![minesweeper-opt2](https://user-images.githubusercontent.com/7089228/80656536-45ac2c80-8a36-11ea-8521-ab40fc922ce1.gif)


#### Building the `*-uwp-windows-msvc` toolchain
Clone the [rust](https://github.com/rust-lang/rust) repo. You'll need to setup a `config.toml` file by copying the existing `config.toml.example` file and editing it:

```
# In addition to all host triples, other triples to produce the standard library
# for. Each host triple will be used to produce a copy of the standard library
# for each target triple.
#
# Defaults to just the build triple
target = ["x86_64-uwp-windows-msvc"]
```

Then follow the instructions on the README to build and then add the toolchain:

```
python x.py build
rustup toolchain link local build\x86_64-pc-windows-msvc\stage2
```

You'll also need to copy an existing copy of `rustfmt.ext` into `build\x86_64-pc-windows-msvc\stage2\bin`. If you hit compilation issues, make sure you're using Visual Studio 2019 (not preview!).