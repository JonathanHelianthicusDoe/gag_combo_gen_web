#!/usr/bin/env bash

set -ex

cargo update --aggressive
cargo +nightly build --target wasm32-unknown-unknown --release

cp ./target/wasm32-unknown-unknown/release/gag_combo_gen_web.wasm \
    ./wasm/gag_combo_gen_web.wasm
chmod a-x ./wasm/gag_combo_gen_web.wasm

if [[ "$1" =~ ^-?s(ize)?$ ]]; then
    wasm-opt -Oz --strip-debug -o ./wasm/gag_combo_gen_web.wasm \
        ./wasm/gag_combo_gen_web.wasm
else
    wasm-opt -O4 --strip-debug -o ./wasm/gag_combo_gen_web.wasm \
        ./wasm/gag_combo_gen_web.wasm
fi

npx tsc --build ./js/tsconfig.json

npx eslint -c ./js/.eslintrc.json --ext .ts ./js
npx eslint -c ./test/.eslintrc.json --ignore-path ./test/.eslintignore ./test

npx prettier --write ./js/main.ts ./js/main.js
