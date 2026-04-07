---
layout: default
title: Install
nav_order: 2
has_children: true
---

## From binaries

- Fetch the [latest release](https://github.com/vascocosta/gluonscript/releases) for your platform from GitHub.

- Extract the archive into a `.gluonscript` folder at the root of your `HOME` folder.

- Add the location of the `.gluonscript\bin` to your `PATH`.

---

## From source

- Fetch the source code from GitHub by cloning the repo with:

```
git clone https://github.com/vascocosta/gluonscript.git
```

- Compile the source code (you need the `Rust toolchain`):

```
cd gluonscript
cargo build -r
```

---

## Notes

As you can see there is no installer, the interpreter is self-contained and can be executed from anywhere. However, it is strongly recommended that you add the location of the binary to your `PATH` variable so that you can run `gluonscript` from anywhere on the command line.

### Running a script

- Edit `my_script.gs` on your preferred IDE.

- Run the script from the command line:

```
gluonscript my_script.gs
```