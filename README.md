# gag\_combo\_gen\_web

The front-end implementation of the gag combo generator found
[here](https://zz.nfshost.com/). For the inner library part, see
[gag\_combo\_gen](https://github.com/JonathanHelianthicusDoe/gag_combo_gen).

## How to deploy

```bash
git clone https://github.com/JonathanHelianthicusDoe/gag_combo_gen.git
git clone https://github.com/JonathanHelianthicusDoe/gag_combo_gen_web.git
cd gag_combo_gen_web
make # Optional, the wasm binary is committed
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
