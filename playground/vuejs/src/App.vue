<script setup lang="ts">
import { computed, onMounted, ref } from 'vue'
import { useEngine, useLibrary } from './composables/rpgx'
import { WasmLayer as Layer, WasmLayerType as LayerType, WasmTile as Tile, WasmDirection } from '@rpgx/js';

const library = useLibrary()
const engine = ref(useEngine(library))

const updateFlag = ref(0)

const activeScene = computed(() => {
  updateFlag;
  return engine.value.getActiveScene()
})

const map = activeScene.value?.map
const layers = map?.getLayers()
const squareSize = 15;

console.dir(layers)

function getTileStyle(tile: Tile, layer: Layer) {
  const x = tile.pointer.x;
  const y = tile.pointer.y;
  const width = (tile.effect.group ? tile.shape.width : 1) * squareSize;
  const height = (tile.effect.group ? tile.shape.height : 1) * squareSize;

  const backgroundImage = tile.effect.textureId
    ? `background-image: ${getTexture(tile.effect.textureId)};`
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

function getTexture(id: number) {
  const texture = library.get_by_id(id);
  if (texture) {
    return `url(${texture})`;
  }
  return '';
}

const pawnStyle = computed(() => {
  updateFlag.value;

  const x = activeScene.value?.pawn.tile.pointer.x || 0;
  const y = activeScene.value?.pawn.tile.pointer.y || 0;
  const textureId = activeScene.value?.pawn.textureId

  return `
    ${textureId ? `background-image: ${getTexture(textureId)};` : ''}
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
  const actions = map?.getActionsAt(tile.pointer)
  actions?.forEach(a => {
    const action = library.get_by_id(a);
    action();
  })
}



function onClick(tile: Tile) {
  console.log('onclick')
  updateFlag.value++
  // engine.value.move_to(tile.pointer.x, tile.pointer.y);
  const steps = activeScene.value?.stepsTo(tile.pointer);
  console.log('steps')
  console.dir(steps)
  if (!steps?.length) return
  for (let i = 0; i < steps.length; i++) {
    const step = steps[i];
    setTimeout(() => {
      const tile = activeScene.value?.moveTo(step);
      console.log('after assign: ', tile?.pointer)
      console.log(activeScene.value?.pawn.tile.pointer)
      // perche pawn ha .free()?
      if (tile) manageActions(tile)
      updateFlag.value++;
    }, i * 100);
  }
}

function onKeyDown(event: KeyboardEvent) {
  console.log('keydown', event.key);
  let tile
  if (event.key === 'ArrowUp' || event.key.toLowerCase() === 'w') {
    tile = activeScene.value?.stepTo(WasmDirection.Up);
  } else if (event.key === 'ArrowDown' || event.key.toLowerCase() === 's') {
    tile = activeScene.value?.stepTo(WasmDirection.Down);
  } else if (event.key === 'ArrowLeft' || event.key.toLowerCase() === 'a') {
    tile = activeScene.value?.stepTo(WasmDirection.Left);
  } else if (event.key === 'ArrowRight' || event.key.toLowerCase() === 'd') {
    tile = activeScene.value?.stepTo(WasmDirection.Right);
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
