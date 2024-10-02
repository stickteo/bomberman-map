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

# Technical Decisions

## Using Rust (and thoughts)
In comparison to using C, a lot of "basic" features are simply included within the Rust core library. I didn't want to build everything up from the ground, like a vector (dynamic array), within C that may or may not break later on...

In theory C could have all of these features within the standard library... Though seemingly Rust is what C++ should have been. (Rust makes it easy to allocate and free memory since it's done automatically with a borrow checker...)

Fighting with the Rust compiler can be annoying at times... However, things don't break and time isn't wasted on memory bugs... Overall a nice experience with certain bells and whistles included.

Rust does have a functional style. You could do a lot of the "basic" things sorting, deduplicating, or filtering things... Operations like these can be non-trivially long to write within C. This of course can make Rust code very dense and perhaps intimidating to read. In some sense though Rust is a higher-level language and you don't have to spend a lot of time trying to parse for loops within C...

In a lot of ways, Rust crystalizes a lot of patterns into specific functions... A major example would be Rust's Iterator. In C it's a pointer to an array of a certain type... Going to the next element means adding by that type's size. Rust would abstract away the need to add by that specific size to iterate to the next element... For another example, the deduplicating in Rust requires sorting first. This makes sense and makes Rust code transparent. (While a lot of scripting languages are not very transparent and certain libraries have its own conventions...)

## Image Format
Currently images are output as BMP files. BMP is chosen for its simplicity and for the ability to use a palette.
