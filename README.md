## Rust-MagickWand: MagickWand bindings for Rust.

## Getting started
Main crate file: lib/magick_wand.rs
### Exposed modules:
* ```wand```: You should probably just look through the library to see what it
exposes; it's not very big and is subject to change.

## Running tests
Due to an existing bug in the Rust compiler (version 0.6),
one can only load an external module with custom linker arguments
once, globally. This makes it impossible to run all the tests at a time,
as the MagickWand OOP wrapper makes use of the exposed C API, and the external
C api is tested as well, so it'd get loaded twice. Go take a look at
```lib/test_magick_wand.rs``` for a rundown of how to completely test the
entire library.
