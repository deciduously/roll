# roll
[![Build Status](https://travis-ci.org/deciduously/roll.svg?branch=master)](https://travis-ci.org/deciduously/roll)

CLI interface to roll dice.

## Usage

Invoke with no arguments for an interactive mode which will evaluate commands in sequence separated by spaces and prompt for further input until a SIGINT.  Eventually I'll write a less sloppy thing.  Maybe.

Alternatively invoke with your command as space-separated arguments.

Command formats:

* A list of valid rolls in XdX format, separated by spaces - `1d2 2d4 3d8`
* A single string identifier to lookup in the item table - `blello` - for now just one at a time
* A multiplier followed by either a list of valid rolls or a string identifier - `3 1d7`, `3 1d8 2d9`, `3 blello`

## Dependencies

Stable rust 1.25.0+

## Crates

* lazy_static
* rand
* regex
* serde
* serde_yaml
* serde_derive
