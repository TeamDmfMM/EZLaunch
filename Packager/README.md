# Building the packager
_(also known as the Creator)_

## Requirements

### Windows

 - 64bit
 - rust (cargo and rustc)
 - ubuntu for windows

#### Ubuntu on Windows requirements

 - gcc
 - deps for conrod

### Linux/Mac

 - rust (cargo and rustc)
 - working gcc
 - deps for conrod

## Setting up dev env

### Windows

---

**Run all commands inside the Ubuntu on Windows `bash` shell**

---

You will need to run `cargo update` from the root of this folder
in order to get all the required packages.

Next, do a test build with `cargo build --target x86_64-pc-windows-gnu`. Once this finishes, make sure
to copy the `packager_gui.exe.manifest` file to the output directory. The gui should now run.

### Linux/Mac

Set up the project as any other rust project.

## Creating a build

### Windows

---

**Run all commands inside the Ubuntu on Windows `bash` shell**

---

In order to build the packager binaries, use the commands below

#### `packager_gui.exe`

```
$ cargo rustc --target x86_64-pc-windows-gnu --release --bin packager_gui -- -C link_args="-Wl,--subsystem,windows"
```

#### `packager.exe`

```
$ cargo build --target x86_64-pc-windows-gnu --release --bin packager
```

### Linux

To create the binaries, just run `cargo build`

### Mac

Same as linux.