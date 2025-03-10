default: build

test: build
	cargo test --all --tests

build:
	mkdir -p target/wasm32-unknown-unknown/optimized

	cargo rustc --manifest-path=Cargo.toml --crate-type=cdylib --target=wasm32-unknown-unknown --release 
	stellar contract optimize \
		--wasm target/wasm32-unknown-unknown/release/backstop_manager.wasm \
		--wasm-out target/wasm32-unknown-unknown/optimized/backstop_manager.wasm

	cd target/wasm32-unknown-unknown/optimized/ && \
		for i in *.wasm ; do \
			ls -l "$$i"; \
		done

fmt:
	cargo fmt --all

clean:
	cargo clean

