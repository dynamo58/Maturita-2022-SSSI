[🇨🇿 Klikněte zde pro českou verzi tohoto README](https://github.com/dynamo58/gol/blob/master/README_cs.md)

---

# About

This is a repo for all source files encompassing my [Maturita](https://en.wikipedia.org/wiki/Matura#In_the_Czech_Republic) project.

It is composed of two main parts:
1) the writings (`docs/`)
    * `docs/written/guide.tex` - describing the [Rust programming language](https://www.rust-lang.org/) and related
    * `docs/written/thesis.tex` - gist of the guide + describtion of an application I made for the matura
    * `docs/oral/presentation.tex` -  the actual presentation for the oral exam (which serves as "defense" of the work)
2) a wasm-enabled GUI implementation of [Game of Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life) (`code/`).


## Disclaimer

All of the CLI outputs, GUI labels, etc.  are in Czech.

# Setup

Run the `setup.py` script for setup (linux only). \[ This is WIP\]

# Build

Run the `build.py` script for building.

cli args:
* `-os` - targeted OS (linux, mac, win)
* `-arch` - targeted architecture (wasm, ...])

(note that the actual build tools for the targets must be installed)

For example:

```bash
./build.py -arch wasm
```

```
./build.py -os win -arch i686-pc-windows-msvc
```

[...] or you can just build them the way you want, obviously ¯\\\_(ツ)\_/¯