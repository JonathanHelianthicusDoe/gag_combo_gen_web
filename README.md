# gag\_combo\_gen\_web

[![AGPL v3+](https://img.shields.io/badge/license-GNU%20AGPL%20v3.0%2B-663366)](./LICENSE)
[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)

The front-end implementation of the gag combo generator found
[here](https://zz.nfshost.com/). For the inner library part, see
[gag\_combo\_gen](https://github.com/JonathanHelianthicusDoe/gag_combo_gen).

## How to deploy

```bash
git clone https://github.com/JonathanHelianthicusDoe/gag_combo_gen.git
git clone https://github.com/JonathanHelianthicusDoe/gag_combo_gen_web.git
cd gag_combo_gen_web
./build.sh # Optional; the wasm binary is committed.
           # You can pass -s as an argument to optimize for size with wasm-opt,
           # but in that case you probably also want to set opt-level="z" in
           # the Cargo.toml beforehand.
cd test
npm install
npm start &
firefox localhost:3000
```

## Legal

This software is licensed to anyone under the terms of the [GNU Affero General
Public License, version 3](https://www.gnu.org/licenses/agpl-3.0.en.html) (or a
higher version, at your option).

[![GNU AGPL v3+](https://www.gnu.org/graphics/agplv3-with-text-162x68.png "GNU AGPL v3+")](https://www.gnu.org/licenses/agpl-3.0.en.html)
