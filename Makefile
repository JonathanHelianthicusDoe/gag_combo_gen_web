.PHONY: opt size

opt:
	cargo +nightly build --target wasm32-unknown-unknown --release && \
	wasm-gc target/wasm32-unknown-unknown/release/gag_combo_gen_web.wasm \
		-o wasm/gag_combo_gen_web.wasm && \
	wasm-opt -O3 wasm/gag_combo_gen_web.wasm

size:
	cargo +nightly build --target wasm32-unknown-unknown --release && \
	wasm-gc target/wasm32-unknown-unknown/release/gag_combo_gen_web.wasm \
		-o wasm/gag_combo_gen_web.wasm && \
	wasm-opt -Os wasm/gag_combo_gen_web.wasm
