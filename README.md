# gag\_combo\_gen\_web

[![AGPL v3+](https://img.shields.io/badge/license-GNU%20AGPL%20v3%2B-663366)](./LICENSE)
[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)

The front-end implementation of the gag combo generator found
[here](https://zz.nfshost.com/gag_combo_gen_web/index.html). For the inner
library part, see
[gag\_combo\_gen](https://github.com/JonathanHelianthicusDoe/gag_combo_gen).

## Build dependencies

* `cargo` and the latest stable Rust toolchain
  ([https://rustup.rs/](https://rustup.rs/))
* `wasm32-unknown-unknown` target support
  (`rustup target add wasm32-unknown-unknown`)
* `npm` ([https://www.npmjs.com/](https://www.npmjs.com/))
* `wasm-opt` ([https://github.com/WebAssembly/binaryen][binaryen])

## How to deploy

```bash
git clone https://github.com/JonathanHelianthicusDoe/gag_combo_gen.git
git clone https://github.com/JonathanHelianthicusDoe/gag_combo_gen_web.git
cd gag_combo_gen_web
npm install
./build.sh # You can pass -s as an argument to optimize for size with wasm-opt,
           # but in that case you probably also want to set opt-level="z" in
           # the Cargo.toml beforehand.

# For testing
cd test
npm install
npm start &
firefox localhost:3000
```

## Legal

This software is licensed to anyone under the terms of the [GNU Affero General
Public License, version 3](https://www.gnu.org/licenses/agpl-3.0.en.html) (or
any higher version of the same license, at your option).

[![GNU AGPL v3+](https://www.gnu.org/graphics/agplv3-with-text-162x68.png
"GNU AGPL v3+")](https://www.gnu.org/licenses/agpl-3.0.en.html)

[binaryen]: https://github.com/WebAssembly/binaryen
