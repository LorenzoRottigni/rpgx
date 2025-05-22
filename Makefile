# Makefile at repo root

.PHONY: clean build-rpgx build-rpgxw wasm-pack-vuejs wasm-pack-nodejs \
        dev-vuejs dev-nodejs dev-dioxus fmt fmt-js build/all wasm-pack-all dev-all lint test \

build-all: build-rpgx build-rpgxw

wasm-pack-all: wasm-pack-nodejs wasm-pack-vuejs

dev-all: dev-dioxus dev-nodejs dev-vuejs

build-rpgx:
	@echo "🔧 Building rpgx"
	$(MAKE) clean
	cargo build --release -p rpgx

build-rpgxw:
	@echo "🔧 Building rpgxw (WASM)"
	$(MAKE) clean
	cargo build --target wasm32-unknown-unknown --release -p rpgxw

wasm-pack-vuejs:
	@echo "📦 Building wasm for Vue.js"
	wasm-bindgen target/wasm32-unknown-unknown/release/rpgxw.wasm \
		--out-dir playground/vuejs/src/wasm --target bundler

wasm-pack-nodejs:
	@echo "📦 Building wasm for Node.js"
	wasm-bindgen target/wasm32-unknown-unknown/release/rpgxw.wasm \
		--out-dir playground/nodejs/wasm --target nodejs

dev-vuejs:
	@echo "🚀 Starting Vue.js dev server"
	cd playground/vuejs && npm install && npm run dev

dev-nodejs:
	@echo "🚀 Starting Node.js dev server"
	cd playground/nodejs && npm install && npx ts-node src/main.ts

dev-dioxus-web:
	@echo "🚀 Starting Dioxus dev server"
	cd playground/dioxus && cargo install --locked dioxus-cli && dx serve --platform web

dev-dioxus-desktop:
	@echo "🚀 Starting Dioxus dev server"
	cd playground/dioxus && cargo install --locked dioxus-cli && dx serve --platform desktop

clean:
	@echo "🧹 Cleaning project"
	cargo clean
	cd playground/nodejs && rm -rf node_modules dist
	cd playground/vuejs && rm -rf node_modules dist
	cd playground/dioxus && cargo clean

fmt:
	@echo "🎨 Formatting Rust code"
	cargo fmt --all

fmt-js:
	cd playground/vuejs && npx prettier --write .
	cd playground/nodejs && npx prettier --write .

lint:
	@echo "🔍 Linting Rust code"
	cargo clippy --workspace --all-targets -- -D warnings

test:
	@echo "🧪 Running all Rust tests"
	cargo test --workspace