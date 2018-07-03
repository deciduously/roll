# roll
[![Build Status](https://travis-ci.org/deciduously/roll.svg?branch=master)](https://travis-ci.org/deciduously/roll)

Web and CLI interface to roll dice.

## Usage

* `yarn dev` and point browser to `localhost:3449` for figwheel and watching the Rust backend.  If you don't issue `:cljs/quit` at the REPL or you'll have to kill figwheel some other way.
* `yarn prod` and point browser to `localhost:8080` for the production build.
* `yarn demo` for a dev server with no hot-reloading - point your browser to `localhost:8080`.

For EMACS/CIDER, run `yarn emacs` and then from Emacs invoke `M-x cider-jack-in-clojurescript`.  You'll need to invoke `(start-figwheel!)`.  Further instructions can be found [here](https://github.com/bhauman/lein-figwheel/wiki/Using-the-Figwheel-REPL-within-NRepl).

The backend component also has two other CLI modes:

* Invoke with no arguments for a REPL which will evaluate commands in sequence separated by spaces and prompt for further input until a SIGINT.
* Invoke with your command as space-separated arguments to run a single operation.

You can run the webserver directly via `roll serve`

Command formats:

* A list of valid rolls in XdX format, separated by spaces - `1d2 2d4 3d8`
* A list of string identifier to lookup in the item table - `blello`, `blello, ian`
* A multiplier followed by either a list of valid rolls or a string identifier - `3 1d7`, `3 1d8 2d9`, `3 blello`, `3 blello ian`

The multiplier is a little buggy on the web side - it only rolls out the first trailing arg.  Stay tuned.

Web server endpoints:

* `GET /roll/:cmd` where `:cmd` is any of the above, but separated by slashes instead of spaces: `/roll/1d6`, `/roll/2d8/3d9`, `/roll/3/10d20`, `/roll/9/blello/ian`
* `GET /items` returns all the items stored in `db.sqlite`
* `POST /item` with an `application/json` request body like `'{"name": "blaster","damage": "1d8"}'` to insert that item into the DB

## Dependencies

* Stable rust 1.25.0+
* Java JRE 1.8+
* yarn

## Libraries
### Rust Crates

* [actix/actix_web](https://actix.rs)
* [lazy_static](https://github.com/rust-lang-nursery/lazy-static.rs)
* [rand](https://github.com/rust-lang-nursery/rand)
* [regex](https://github.com/rust-lang/regex)
* [serde/serde_derive](https://serde.rs)

### ClojureScript Libraries

* [reagent](https://reagent-project.github.io)
* [re-frame](https://github.com/Day8/re-frame)
* [re-frame-http-fx](https://github.com/Day8/re-frame-http-fx)
