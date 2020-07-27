# minesweeper-rs
A port of [robmikh/Minesweeper](https://github.com/robmikh/Minesweeper) using [winrt-rs](https://github.com/microsoft/winrt-rs).

## Getting started
Running this sample requires at least Windows build 1803 (v10.0.17134.0). To compile and run, use [cargo](https://www.rust-lang.org/learn/get-started):

### Desktop
Running Minesweeper as a normal desktop application can be done as follows:

```
cargo run --release
```

### UWP

Running Minesweeper as a UWP application can be done by building for a `*-uwp-windows-msvc` target and then registering the app. Because the `*-uwp-windows-msvc` targets are currently tier 3, you'll need to compile a toolchain yourself. More information can be found [here](UWP.md). 

```
cargo +local build --target x86_64-uwp-windows-msvc
(powershell.exe) Add-AppxPackage -Register AppxManifest.xml
```
*NOTE: AppManifest.xml currently assumes the `x86_64-uwp-windows-msvc` target but can be updated.*

Then launch minesweeper-rs from the Start Menu.

![minesweeper-opt2](https://user-images.githubusercontent.com/7089228/80656536-45ac2c80-8a36-11ea-8521-ab40fc922ce1.gif)
