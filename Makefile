.PHONY: \
	dev-vue-playground dev-node-playground dev-dioxus-web-playground dev-dioxus-desktop-playground \
	clean-core clean-vue-playground clean-dioxus-playground clean-all \
	test-core test-wasm-driver test-all \
	build-core build-wasm-driver build-js-driver build-vue-playground build-vue-plugin \
	build-dioxus-web-playground build-dioxus-desktop-playground build-dioxus-plugin build-all \
	publish-core publish-wasm-driver publish-js-driver publish-dioxus-plugin publish-vue-plugin publish-all \
	install-dioxus-cli check fmt help

# === Variables ===
WASM_TARGET = wasm32-unknown-unknown

WASM_OUT_VUE = playground/vuejs/src/wasm
WASM_OUT_NODE = playground/nodejs/wasm
WASM_OUT_DRIVER_JS = packages/drivers/js/pkg

WASM_BIN = target/$(WASM_TARGET)/release/rpgx_wasm.wasm

# === Help ===
help:
	@echo "Available commands:"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-35s\033[0m %s\n", $$1, $$2}'

# === Install ===
install-dioxus-cli: ## Install Dioxus CLI
	cargo install --locked dioxus-cli

# === Development ===
dev-vue-playground: ## Run VueJS playground
	@$(MAKE) build-js-driver
	@$(MAKE) build-vue-plugin
	@echo "🚀 Starting Vue.js playground..."
	cd playground/vuejs && npm install && npm run dev

dev-node-playground: ## Run NodeJS playground
	@$(MAKE) build-js-driver
	@echo "🚀 Starting NodeJS playground..."
	cd playground/nodejs && npx ts-node index.ts

dev-dioxus-web-playground: install-dioxus-cli ## Run Dioxus web app
	@echo "🚀 Starting Dioxus Web Application..."
	cd playground/dioxus && dx serve --platform web --features web

dev-dioxus-desktop-playground: install-dioxus-cli ## Run Dioxus desktop app
	@echo "🚀 Starting Dioxus Desktop Application..."
	cd playground/dioxus && dx serve --platform desktop --features desktop

# === Clean ===
clean-core:
	@echo "🧹 Cleaning RPGX core..."
	cargo clean -p rpgx

clean-vue-playground:
	@echo "🧹 Cleaning VueJS playground..."
	rm -rf playground/vuejs/node_modules playground/vuejs/dist

clean-dioxus-playground:
	@echo "🧹 Cleaning Dioxus playground..."
	cargo clean -p rpgx-spaceship

clean-all: clean-core clean-dioxus-playground clean-vue-playground

# === Test ===
test-core:
	@echo "🧪 Running tests for RPGX core..."
	cargo test -p rpgx

test-wasm-driver:
	@echo "🧪 Running tests for RPGX wasm driver..."
	cargo test -p rpgx-wasm

test-all: test-core test-wasm-driver ## Run all tests

# === Build ===
build-core:
	@echo "📦 Building RPGX core..."
	cargo build --release -p rpgx

build-wasm-driver:
	@echo "🔧 Building RPGX wasm driver..."
	cargo build --target $(WASM_TARGET) --release -p rpgx-wasm

build-js-driver:
	@echo "🔧 Generating RPGX WASM driver and NodeJS loader..."
	cargo build --target $(WASM_TARGET) --release -p rpgx-wasm
	wasm-bindgen $(WASM_BIN) --out-dir $(WASM_OUT_DRIVER_JS) --target bundler

build-dioxus-plugin:
	@echo "🔧 Building Dioxus plugin..."
	cargo build --release -p rpgx-dioxus

build-vue-plugin:
	@echo "🔧 Building Vue.js driver plugin..."
	@$(MAKE) build-js-driver
	cd packages/plugins/vue && npm install && npm run build

build-vue-playground:
	@echo "🚀 Building Vue.js playground..."
	@$(MAKE) build-js-driver
	@$(MAKE) build-vue-plugin
	cd playground/vuejs && npm install && npm run build

build-dioxus-web-playground: install-dioxus-cli
	@echo "🚀 Building Dioxus Web Application..."
	cd playground/dioxus && dx build --platform web --features web

build-dioxus-desktop-playground: install-dioxus-cli
	@echo "🚀 Building Dioxus Desktop Application..."
	cd playground/dioxus && dx build --platform desktop --features desktop

build-all: build-core build-wasm-driver build-js-driver build-dioxus-plugin build-vue-plugin build-vue-playground build-dioxus-web-playground build-dioxus-desktop-playground ## Build all targets

# === Linting & Formatting ===
check: ## Run cargo check and clippy
	cargo check --all
	cargo clippy --all -- -D warnings

fmt: ## Format code
	cargo fmt --all

# === Publish ===
publish-core:
	@echo "⚠️ Use 'publish-all' to release the entire workspace instead of individual crates"

publish-wasm-driver:
	@echo "⚠️ Use 'publish-all' to release the entire workspace instead of individual crates"

publish-js-driver:
	cd packages/drivers/js && npm publish --access public

publish-dioxus-plugin:
	@echo "⚠️ Use 'publish-all' to release the entire workspace instead of individual crates"

publish-vue-plugin:
	cd packages/plugins/vue && npm publish --access public

publish-all:
	@echo "🚀 Releasing entire Rust workspace (excluding rpgx-spaceship)..."
	cargo release patch --execute --workspace --exclude rpgx-spaceship
	@echo "🚀 Publishing JS packages..."
	cd packages/drivers/js && npm publish --access public
	cd packages/plugins/vue && npm publish --access public
