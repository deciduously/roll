# roll
[![Build Status](https://travis-ci.org/deciduously/bfrs.svg?branch=master)](https://travis-ci.org/deciduously/bfrs)

CLI interface to roll dice.

## Usage

Supply rolls either as args like `roll 1d6` or `roll 2d10 7d83 1d4` or invoke with no arguments for an interactive mode which will evaluate any rolls antered separated by spaces and prompt for further input until a SIGINT.  Eventually I'll write a less sloppy thing.  Maybe.

Input options:

* A list of valid rolls in XdX format, separated by spaces
* A string identifier to lookup in the item table
* A multiplier followed by either a list of valid rolls or a string identifier

## Dependencies

Stable rust 1.25.0+
