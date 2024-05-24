.PHONY: build
build:
	@echo "Building..."
	@cargo component build --target wasm32-wasi

.PHONY: clean
clean:
	@echo "Cleaning..."
	@cargo component clean

.PHONY: update
update:
	@echo "Updating..."
	@wit-deps update

.PHONY: run
run:
	@echo "Running..."
	@wasmtime serve target/wasm32-wasi/debug/wasi_http_info.wasm
