.PHONY: opt size

opt:
	cargo +nightly build --target wasm32-unknown-unknown --release && \
	cp target/wasm32-unknown-unknown/release/gag_combo_gen_web.wasm wasm/gag_combo_gen_web.wasm && \
	chmod -x wasm/gag_combo_gen_web.wasm && \
	wasm-opt -O4 --strip-debug -o wasm/gag_combo_gen_web.wasm wasm/gag_combo_gen_web.wasm

size:
	cargo +nightly build --target wasm32-unknown-unknown --release && \
	cp target/wasm32-unknown-unknown/release/gag_combo_gen_web.wasm wasm/gag_combo_gen_web.wasm && \
	chmod -x wasm/gag_combo_gen_web.wasm && \
	wasm-opt -Oz --strip-debug -o wasm/gag_combo_gen_web.wasm wasm/gag_combo_gen_web.wasm
