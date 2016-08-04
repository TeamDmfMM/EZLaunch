# Getting a dev env ready

## What you need

You will need:
 - rust
 - iup

### Rust
Just get the newest version of rust and cargo

### IUP
todo: give instructions

## Using cargo

Run `cargo build` to build the gui and the the actual packager.
After this is done (on windows), make sure to copy all the DLLs in build_libs to target/debug

On other systems, just make sure you have the libraries accessible.

Then, just run the generated executables.

If you need to make a windowed version, use `cargo --bin packager_gui -- -mwindows` on gcc.

# Building a release

Similar to above, but make sure you use a windowed version for packager_gui, and build packager separately.