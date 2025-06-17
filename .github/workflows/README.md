Github workflows

- On commits on whatever branch run:
  - make build-core
  - make test-core
  - make build-wasm-driver
  - make build-js-driver
  - make build-vue-plugin
  - make build-dioxus-plugin
  - (or make build-all + make test-all)

- On commit master with a git tag specified:
  - make build-all + make test-all
  - make publish-core
  - make publish-wasm-driver
  - make publish-js-driver
  - make publish-vue-plugin
  - make publish-dioxus-plugin
  - (or make publish-all)

