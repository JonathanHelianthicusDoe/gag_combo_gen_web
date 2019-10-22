#!/usr/bin/env bash

set -ex

cargo update --aggressive
cargo +nightly build --target wasm32-unknown-unknown --release

cp ./target/wasm32-unknown-unknown/release/gag_combo_gen_web.wasm ./wasm/gag_combo_gen_web.wasm
chmod a-x ./wasm/gag_combo_gen_web.wasm

if [[ "$1" =~ ^-?s(ize)?$ ]]; then
    wasm-opt -Oz --strip-debug -o ./wasm/gag_combo_gen_web.wasm ./wasm/gag_combo_gen_web.wasm
else
    wasm-opt -O4 --strip-debug -o ./wasm/gag_combo_gen_web.wasm ./wasm/gag_combo_gen_web.wasm
fi

npx eslint -c ./js/.eslintrc.json --ignore-path ./js/.eslintignore ./js
npx eslint -c ./test/.eslintrc.json --ignore-path ./test/.eslintignore ./test
