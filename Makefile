.PHONY: dev-vue dev-node dev-dioxus-web dev-dioxus-desktop clean test build wasm-build \
        build-vue build-dioxus-web build-dioxus-desktop

WASM_TARGET=wasm32-unknown-unknown
WASM_OUT_VUE=playground/vuejs/src/wasm
WASM_OUT_NODE=playground/nodejs/wasm
WASM_BIN=target/$(WASM_TARGET)/release/rpgxw.wasm

define wasm_bundle
	@echo "🔧 Generating RPGXW WASM bundle..."
	cargo build --target $(WASM_TARGET) --release -p rpgxw
	@echo "🔧 Generating RPGXW WASM NodeJS loader..."
	wasm-bindgen $(WASM_BIN) --out-dir $(1) --target bundler
endef

dev-vue:
	$(call wasm_bundle,$(WASM_OUT_VUE))
	@echo "🚀 Starting Vue.js playground..."
	cd playground/vuejs && npm install && npm run dev

dev-node:
	$(call wasm_bundle,$(WASM_OUT_NODE))
	@echo "🚀 Starting NodeJS playground..."
	cd playground/nodejs && npx ts-node index.ts

dev-dioxus-web:
	@echo "🚀 Starting Dioxus Web Application..."
	cd playground/dioxus && cargo install --locked dioxus-cli && dx serve --platform web --features web

dev-dioxus-desktop:
	@echo "🚀 Starting Dioxus Desktop Application..."
	cd playground/dioxus && cargo install --locked dioxus-cli && dx serve --platform desktop --features desktop

clean:
	@echo "🧹 Cleaning project..."
	cargo clean
	rm -rf playground/vuejs/node_modules playground/vuejs/dist
	cd playground/dioxus && cargo clean

test-core:
	@echo "🧪 Running tests for RPGX..."
	cargo test -p rpgx

test-wasm:
	@echo "🧪 Running tests for RPGXW..."
	cargo test -p rpgxw

build-core:
	@echo "📦 Building release for Rust crate..."
	cargo build --release -p rpgx

build-wasm:
	@echo "🔧 Building RPGXW..."
	cargo build --target $(WASM_TARGET) --release -p rpgxw

build-vue:
	$(call wasm_bundle,$(WASM_OUT_VUE))
	@echo "🚀 Building Vue.js playground..."
	cd playground/vuejs && npm install && npm run build

build-dioxus-web:
	@echo "🚀 Building Dioxus Web Application..."
	cd playground/dioxus && cargo install --locked dioxus-cli && dx build --platform web

build-dioxus-desktop:
	@echo "🚀 Building Dioxus Desktop Application..."
	cd playground/dioxus && cargo install --locked dioxus-cli && dx build --platform desktop
