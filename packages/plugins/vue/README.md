# RPGX Vue Plugin

This package provides a Vue.js plugin for integrating the RPGX game engine into Vue applications. It enables reactive, component-based interfaces for grid-based RPGs using Vue 3.

## Features

- Seamless integration with RPGX core and WASM driver
- Reactive Vue components for RPG maps, pawns, and interactions
- Designed for use with RPGX-powered playgrounds and apps

## Prerequisites

- Node.js and npm
- Vue 3 project setup
- RPGX WASM driver built (see [RPGX WASM Driver README](../../drivers/wasm/README.md))

## Installation

Install dependencies and build the plugin:

```sh
npm install
npm run build
```

## Usage

Import and register the RPGX Vue plugin in your Vue application:

```js
import { createApp } from 'vue';
import App from './App.vue';
import RPGXPlugin from '@rpgx/vue';

const app = createApp(App);
app.use(RPGXPlugin);
app.mount('#app');
```

Use the provided components in your templates to render and interact with the RPGX game state.

## Development

To start the playground or develop the plugin:

```sh
npm run dev
```

## License

RPGX is licensed under the [MIT License](../../../LICENSE).