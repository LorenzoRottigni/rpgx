.PHONY: dev-vue dev-node dev-dioxus-web dev-dioxus-desktop clean test-core test-wasm-driver \
        build-core build-wasm-driver build-js-driver build-vue build-vue-plugin \
        build-dioxus-web build-dioxus-desktop

# === Variables ===
WASM_TARGET = wasm32-unknown-unknown

WASM_OUT_VUE = playground/vuejs/src/wasm
WASM_OUT_NODE = playground/nodejs/wasm
WASM_OUT_DRIVER_JS = packages/drivers/js/pkg

WASM_BIN = target/$(WASM_TARGET)/release/rpgx_wasm.wasm

# === Development tasks ===

dev-vue:
	@$(MAKE) build-js-driver
	@$(MAKE) build-vue-plugin
	@echo "🚀 Starting Vue.js playground..."
	cd playground/vuejs && npm install && npm run dev

dev-node:
	@$(MAKE) build-js-driver
	@echo "🚀 Starting NodeJS playground..."
	cd playground/nodejs && npx ts-node index.ts

dev-dioxus-web:
	@echo "🚀 Starting Dioxus Web Application..."
	cd playground/dioxus && cargo install --locked dioxus-cli && dx serve --platform web --features web

dev-dioxus-desktop:
	@echo "🚀 Starting Dioxus Desktop Application..."
	cd playground/dioxus && cargo install --locked dioxus-cli && dx serve --platform desktop --features desktop

# === Clean task ===

clean:
	@echo "🧹 Cleaning project..."
	cargo clean
	rm -rf playground/vuejs/node_modules playground/vuejs/dist
	cd playground/dioxus && cargo clean

# === Test tasks ===

test-core:
	@echo "🧪 Running tests for RPGX core..."
	cargo test -p rpgx

test-wasm-driver:
	@echo "🧪 Running tests for RPGX wasm driver..."
	cargo test -p rpgx-wasm

# === Build tasks ===

build-core:
	@echo "📦 Building release for Rust crate..."
	cargo build --release -p rpgx

build-wasm-driver:
	@echo "🔧 Building RPGX wasm driver..."
	cargo build --target $(WASM_TARGET) --release -p rpgx-wasm

build-js-driver:
	@echo "🔧 Generating RPGX WASM driver and NodeJS loader..."
	cargo build --target $(WASM_TARGET) --release -p rpgx-wasm
	wasm-bindgen $(WASM_BIN) --out-dir $(WASM_OUT_DRIVER_JS) --target bundler

build-vue:
	@echo "🚀 Building Vue.js playground..."
	$(MAKE) build-js-driver
	$(MAKE) build-vue-plugin
	cd playground/vuejs && npm install && npm run build

build-vue-plugin:
	@echo "🚀 Building Vue.js driver plugin..."
	$(MAKE) build-js-driver
	cd packages/plugins/vue && npm install && npm run build

build-dioxus-web:
	@echo "🚀 Building Dioxus Web Application..."
	cd playground/dioxus && cargo install --locked dioxus-cli && dx build --platform web

build-dioxus-desktop:
	@echo "🚀 Building Dioxus Desktop Application..."
	cd playground/dioxus && cargo install --locked dioxus-cli && dx build --platform desktop
