.PHONY: dev-vue dev-node dev-dioxus-web dev-dioxus-desktop clean test build wasm-build \
        build-vue build-dioxus-web build-dioxus-desktop build-vue-plugin

WASM_TARGET=wasm32-unknown-unknown
WASM_OUT_VUE=playground/vuejs/src/wasm
WASM_OUT_NODE=playground/nodejs/wasm
WASM_OUT_DRIVER_JS=packages/drivers/js/pkg
WASM_BIN=target/$(WASM_TARGET)/release/rpgx_wasm.wasm

dev-vue:
	$(MAKE) build-js-driver
	@echo "🚀 Starting Vue.js playground..."
	cd playground/vuejs && npm install && npm run dev

dev-node:
	$(MAKE) build-js-driver
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
	@echo "🧪 Running tests for RPGX wasm driver..."
	cargo test -p rpgx-wasm

build-core:
	@echo "📦 Building release for Rust crate..."
	cargo build --release -p rpgx

build-wasm:
	@echo "🔧 Building RPGX wasm driver..."
	cargo build --target $(WASM_TARGET) --release -p rpgx-wasm

build-vue:
	$(call wasm_bundle,$(WASM_OUT_VUE))
	@echo "🚀 Building Vue.js playground..."
	cd playground/vuejs && npm install && npm run build

build-js-driver:
	@echo "🔧 Generating RPGX WASM driver..."
	cargo build --target $(WASM_TARGET) --release -p rpgx-wasm
	@echo "🔧 Generating RPGX WASM NodeJS loader..."
	wasm-bindgen $(WASM_BIN) --out-dir $(WASM_OUT_DRIVER_JS) --target bundler

build-vue-plugin:
	$(MAKE) build-js-driver
	cd packages/plugins/vue && npm install && npm run build

build-dioxus-web:
	@echo "🚀 Building Dioxus Web Application..."
	cd playground/dioxus && cargo install --locked dioxus-cli && dx build --platform web

build-dioxus-desktop:
	@echo "🚀 Building Dioxus Desktop Application..."
	cd playground/dioxus && cargo install --locked dioxus-cli && dx build --platform desktop
