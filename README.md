# minesweeper-rs
A port of [robmikh/Minesweeper](https://github.com/robmikh/Minesweeper) using the
[`windows-composition`](https://github.com/microsoft/windows-rs/tree/fa5059c3e7462ce66982ff7b9c523fae55b9d189/crates/libs/composition)
and [`windows-window`](https://github.com/microsoft/windows-rs/tree/fa5059c3e7462ce66982ff7b9c523fae55b9d189/crates/libs/window)
crates from [windows-rs PR #4879](https://github.com/microsoft/windows-rs/pull/4879).

## Running
Running this sample requires at least Windows build 1803 (v10.0.17134.0). To compile and run (after setting up), use:

```
cargo run --release
```

![minesweeper-opt2](https://user-images.githubusercontent.com/7089228/80656536-45ac2c80-8a36-11ea-8521-ab40fc922ce1.gif)
