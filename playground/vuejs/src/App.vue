<script setup lang="ts">
import { computed, onMounted, ref } from 'vue'
import { useLayer, useMap, useMasks, useRpgxwEngine } from './composables/rpgxw'
import { Layer, LayerType, Tile } from './wasm/rpgxw';

const engine = ref(useRpgxwEngine())
const updateFlag = ref(0)

console.dir(engine)

const map = engine.value.get_map()
const layers = map.layers
const squareSize = 15;

console.dir(layers)
console.dir(useMasks())
console.dir(useLayer())
console.dir(useMap())

function getTileStyle(tile: Tile, layer: Layer, layerIndex: number) {
  const x = tile.pointer.x;
  const y = tile.pointer.y;
  const width = (tile.effect.group ? tile.shape.width : 1) * squareSize;
  const height = (tile.effect.group ? tile.shape.height : 1) * squareSize;

  const backgroundImage = tile.effect.texture
    ? `background-image: url(${tile.effect.texture});`
    : ''
  const zIndex = layer.kind === LayerType.Default ? 999 : 5 + layerIndex;
  const pointerEvents = layer.kind === LayerType.Default ? 'auto' : 'none';

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


const pawnStyle = computed(() => {
  updateFlag.value;
  const x = engine.value.pawn.tile.pointer.x;
  const y = engine.value.pawn.tile.pointer.y;

  return `
    ${engine.value.pawn.texture ? `background-image: url(${engine.value.pawn.texture});` : ''}
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

function onClick(tile: Tile) {
  console.log('onclick')
  updateFlag.value++
  // engine.value.move_to(tile.pointer.x, tile.pointer.y);
  const steps = engine.value.steps_to(tile.pointer.x, tile.pointer.y);
  for (let i = 0; i < steps.length; i++) {
    const step = steps[i];
    setTimeout(() => {
      engine.value.move_to(step.x, step.y);
      updateFlag.value++;
    }, i * 100);
  }
}

function onKeyDown(event: KeyboardEvent) {
  console.log('keydown', event.key);
  if (event.key === 'ArrowUp' || event.key.toLowerCase() === 'w') {
    engine.value.step_to("up");
  } else if (event.key === 'ArrowDown' || event.key.toLowerCase() === 's') {
    engine.value.step_to("down");
  } else if (event.key === 'ArrowLeft' || event.key.toLowerCase() === 'a') {
    engine.value.step_to("left");
  } else if (event.key === 'ArrowRight' || event.key.toLowerCase() === 'd') {
    engine.value.step_to("right");
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
          :class="layer.kind === LayerType.Default ? 'base-layer-tile' : 'layer-tile'"
          :style="getTileStyle(tile, layer, layerIndex)"
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
