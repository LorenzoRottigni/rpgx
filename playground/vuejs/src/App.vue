<script setup lang="ts">
import { computed, onMounted, ref } from 'vue'
import { useEngine, useLibrary } from './composables/rpgxw'
import { WasmLayer as Layer, WasmLayerType as LayerType, WasmTile as Tile } from './wasm/rpgxw';

const library = useLibrary()
const engine = ref(useEngine(library))

const updateFlag = ref(0)

const map = engine.value.map()
const layers = map.layers
const squareSize = 15;

function getTileStyle(tile: Tile, layer: Layer) {
  const x = tile.pointer.x;
  const y = tile.pointer.y;
  const width = (tile.effect.group ? tile.shape.width : 1) * squareSize;
  const height = (tile.effect.group ? tile.shape.height : 1) * squareSize;

  const backgroundImage = tile.effect.texture_id
    ? `background-image: ${getTexture(tile.effect.texture_id)};`
    : ''
  const zIndex = 10 + layer.z;
  const pointerEvents = layer.kind === LayerType.Base ? 'auto' : 'none';

  return `
    ${backgroundImage}
    background-size: cover;
    position: absolute;
    left: ${x * squareSize}px;
    top: ${y * squareSize}px;
    width: ${width}px;
    height: ${height}px;
    border: solid 1px rgba(255,255,255,0.1);
    z-index: ${zIndex};
    pointer-events: ${pointerEvents};
    cursor: pointer;
  `;
}

function getTexture(key: number) {
  const texture = library.get_texture_by_id(key);
  if (texture) {
    return `url(${texture})`;
  }
  return '';
}

const pawnStyle = computed(() => {
  updateFlag.value;
  const x = engine.value.pawn.tile.pointer.x;
  const y = engine.value.pawn.tile.pointer.y;

  return `
    ${engine.value.pawn.texture_id ? `background-image: ${getTexture(engine.value.pawn.texture_id)};` : ''}
    position: absolute;
    left: ${x * squareSize}px;
    top: ${y * squareSize - squareSize}px;
    background-size: cover;
    background-position: center center;
    z-index: 100;
    width: ${squareSize}px;
    height: ${squareSize * 2}px;
    transition: all 0.1s;
  `;
});

function manageActions(tile: Tile) {
  const actions = map.get_actions_at(tile.pointer)
  actions.forEach(a => {
    // const action = library.get_action_by_id(a);
    library.call_action_by_id(a)
  })
}

function onClick(tile: Tile) {
  console.log('onclick')
  updateFlag.value++
  // engine.value.move_to(tile.pointer.x, tile.pointer.y);
  const steps = engine.value.steps_to(tile.pointer);
  for (let i = 0; i < steps.length; i++) {
    const step = steps[i];
    setTimeout(() => {
      const tile = engine.value.move_to(step);
      manageActions(tile)
      updateFlag.value++;
    }, i * 100);
  }
}

function onKeyDown(event: KeyboardEvent) {
  console.log('keydown', event.key);
  let tile
  if (event.key === 'ArrowUp' || event.key.toLowerCase() === 'w') {
    tile = engine.value.step_to("up");
  } else if (event.key === 'ArrowDown' || event.key.toLowerCase() === 's') {
    tile = engine.value.step_to("down");
  } else if (event.key === 'ArrowLeft' || event.key.toLowerCase() === 'a') {
    tile = engine.value.step_to("left");
  } else if (event.key === 'ArrowRight' || event.key.toLowerCase() === 'd') {
    tile = engine.value.step_to("right");
  }
  if (tile) {
    manageActions(tile)
  }
  updateFlag.value++;
}

const containerRef = ref<HTMLDivElement | null>(null);

onMounted(() => {
  containerRef.value?.focus();
});
</script>

<template>
  <main>
    <div
      ref="containerRef"
      class="container"
      tabindex="0"
      style="position: relative;"
      @keydown="onKeyDown"
    >
      <div
        v-for="(layer, layerIndex) in layers"
        :key="'layer-' + layerIndex"
      >
        <div
          v-for="(tile, tileIndex) in layer.tiles"
          :key="`layer-${layerIndex}-${tileIndex}`"
          :class="layer.kind === LayerType.Base ? 'base-layer-tile' : 'layer-tile'"
          :style="getTileStyle(tile, layer)"
          @click="onClick(tile)"
        ></div>
      </div>

      <div
        class="pawn"
        :style="pawnStyle"
      ></div>
    </div>
  </main>
</template>

<style>
* {
  box-sizing: border-box;
}

body {
  padding: 0;
  background-color: black;
}
main {
  padding: 20px;
}
</style>
