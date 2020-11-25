# Building for `*-uwp-windows-msvc` targets

## Required tools
First, you'll need to install the nightly toolchain:

```
rustup toolchain install nightly
rustup component add rust-src
```

I'm using version `1.50.0-nightly (1c389ffef 2020-11-24)`. If you already have a nightly toolchain installed and you're seeing an error about `SetThreadStackGuarantee`, update your nightly toolchain.

## Building Minesweeper
From the appropriate VS command prompt (e.g. "x64 Native Tools Command Prompt for VS 2019" when building for x86_64), run cargo but target a uwp target:

```
cargo +nightly build -Z build-std=std,panic_abort --target x86_64-uwp-windows-msvc
```

After that, you should be able to register your application:

```
(powershell.exe) Add-AppxPackage -Register AppxManifest.xml
```

Special thanks to [bdbai](https://github.com/bdbai) for the [firstuwp-rs](https://github.com/bdbai/firstuwp-rs) project. Without that, I wouldn't have known about the [build-std](https://doc.rust-lang.org/cargo/reference/unstable.html#build-std) cargo feature.