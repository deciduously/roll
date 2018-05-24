# roll
[![Build Status](https://travis-ci.org/deciduously/roll.svg?branch=master)](https://travis-ci.org/deciduously/roll)

CLI interface to roll dice.

## Usage

Invoke with no arguments for an interactive mode which will evaluate commands in sequence separated by spaces and prompt for further input until a SIGINT.

Invoke with argument `serve` to run the web server - `cargo run -- serve` or `roll serve`if installed to your $PATH.

Alternatively invoke with your command as space-separated arguments to run a single operation.

Command formats:

* A list of valid rolls in XdX format, separated by spaces - `1d2 2d4 3d8`
* A list of string identifier to lookup in the item table - `blello`, `blello, ian`
* A multiplier followed by either a list of valid rolls or a string identifier - `3 1d7`, `3 1d8 2d9`, `3 blello`, `3 blello ian`

Web server endpoints:

* `localhost:8080/roll/:cmd` where `:cmd` is any of the above, but separated by slashes instead of spaces: `/roll/1d6`, `/roll/2d8/3d9`, `/roll/3/10d20`, `/roll/9/blello/ian`

## Dependencies

Stable rust 1.25.0+

## Crates

* [gotham](https://gotham.rs)
* [lazy_static](https://github.com/rust-lang-nursery/lazy-static.rs)
* [rand](https://github.com/rust-lang-nursery/rand)
* [regex](https://github.com/rust-lang/regex)
* [serde/serde_derive](https://serde.rs)
* [serde_yaml](https://github.com/dtolnay/serde-yaml)
