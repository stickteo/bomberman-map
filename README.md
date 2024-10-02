# bomberman-map
Extract maps from Bomberman Tournament or Jetters as images.

![Background 1](bg_01.png)

# Usage
Copy jetters.gba into the main directory and install Rust.

For Windows you can install Rust using the official installer: https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe

In the same directory just run this command and BMPs will be generated.
```
cargo run -- jetters.gba
```

# Introduction
Just a fun project to extract the backgrounds from Bomberman Jetters. The format of the backgrounds should be similar (or same) for Bomberman Tournament.

For now, this project can extract the main backgrounds from Jetters. An example of the background is shown above.
(It's PNG for the sake of uploading to GitHub, but the program will produce BMPs... For now.)

Currently this project is a work-in-progress... Just a collection of code snippets but most of the basic work is done.

The main purpose is just to extract backgrounds... But more can always be done. Most importantly, this project is meant to document some of the technical details required to extract these backgrounds.
