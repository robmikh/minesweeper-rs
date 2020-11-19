# minesweeper-rs
A port of [robmikh/Minesweeper](https://github.com/robmikh/Minesweeper) using [winrt-rs](https://github.com/microsoft/winrt-rs).

## Setup
To compile, you will first need [cargo](https://www.rust-lang.org/learn/get-started). You will then need to install and run the [cargo winrt tool](https://github.com/microsoft/winrt-rs/tree/master/crates/cargo):

```
cargo install --git https://github.com/microsoft/winrt-rs cargo-winrt
cargo winrt install
```

## Running
Running this sample requires at least Windows build 1803 (v10.0.17134.0). To compile and run (after setting up), use:

```
cargo run --release
```

![minesweeper-opt2](https://user-images.githubusercontent.com/7089228/80656536-45ac2c80-8a36-11ea-8521-ab40fc922ce1.gif)
