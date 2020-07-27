# Building the `*-uwp-windows-msvc` toolchain
Clone the [rust](https://github.com/rust-lang/rust) repo. You'll need to setup a `config.toml` file by copying the existing `config.toml.example` file and editing it:

```
# In addition to all host triples, other triples to produce the standard library
# for. Each host triple will be used to produce a copy of the standard library
# for each target triple.
#
# Defaults to just the build triple
target = ["x86_64-uwp-windows-msvc"]
```

Then follow the instructions in the [README](https://github.com/rust-lang/rust/blob/master/README.md#msvc) to build the toolchain. Afterwards you can link your newly built toolchain to rustup for easier use.

```
python x.py build
rustup toolchain link local build\x86_64-pc-windows-msvc\stage2
```

There's one last step. You'll need put a copy of `rustfmt.exe` into the `bin` directory for your toolchain. For example, I placed mine in `build\x86_64-pc-windows-msvc\stage2\bin`.

Once you have everything in place, you'll be able to build using you're new toolchain:

```
cargo +local build --target x86_64-uwp-windows-msvc
(powershell.exe) Add-AppxPackage -Register AppxManifest.xml
```

## Troubleshooting

In the event you're getting errors when compiling llvm, make sure you use a **non-preview** version of Visual Studio 2019. I believe there's a fix, but it wasn't present in the snapshot I built from (stable - [5c1f21c3b82](https://github.com/rust-lang/rust/tree/5c1f21c3b82297671ad3ae1e8c942d2ca92e84f2)). 
