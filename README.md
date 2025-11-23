# <img src="https://raw.githubusercontent.com/LiveSplit/LiveSplit/master/res/Icon.svg" alt="LiveSplit" height="42" width="45" align="top"/> Auto Splitting IDE

This repository hosts the IDE for LiveSplit One's [auto splitting
runtime](https://github.com/LiveSplit/livesplit-core/tree/master/crates/livesplit-auto-splitting).

## Features

- Stepping through the auto splitter's code is possible by attaching LLDB.
- The performance of the auto splitter can be measured.
- All the log output is shown directly in the IDE.
- All the variables that the auto splitter has set are shown.
- The settings of the auto splitter can be quickly changed.

## Build Instructions

In order to build the Auto Splitting IDE you need the [Rust
compiler](https://www.rust-lang.org/).

If you are on Linux, make sure to install:

```bash
sudo apt install libwebkit2gtk-4.1-dev libxdo-dev
```

You can then build and run the project with:

```bash
cargo run
```

In order to build and run a release build, use the following command:

```bash
cargo run --release
```
