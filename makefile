
.PHONY: wasm
wasm:
	wasm-pack build --target web

.PHONY: bin
bin:
	cargo build --bin zeblang

