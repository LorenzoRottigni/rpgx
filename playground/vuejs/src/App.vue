<script setup lang="ts">
import { computed, onMounted, ref } from 'vue'
import { useRpgxwEngine } from './composables/rpgxw'
import { Layer, LayerType, Tile } from './wasm/rpgxw';

const engine = ref(useRpgxwEngine())
const updateFlag = ref(0)

console.dir(engine)

const map = engine.value.get_map()
const layers = map.layers
const squareSize = 15;

function getTileStyle(tile: Tile, layer: Layer, layerIndex: number) {
  const x = tile.pointer.x;
  const y = tile.pointer.y;
  const width = (tile.effect.group ? tile.shape.width : 1) * squareSize;
  const height = (tile.effect.group ? tile.shape.height : 1) * squareSize;

  const backgroundImage = tile.effect.texture
    ? `background-image: url(${tile.effect.texture});`
    : 'background-color: blue';

  const zIndex = layer.kind === LayerType.Default ? 999 : 5 + layerIndex;
  const pointerEvents = layer.kind === LayerType.Default ? 'auto' : 'none';

  return `
    background-color: blue;
    background-size: cover;
    position: absolute;
    left: ${x * squareSize}px;
    top: ${y * squareSize}px;
    width: ${width}px;
    height: ${height}px;
    border: solid 1px rgba(255,255,255,0.1);
    opacity: 0.7;
    z-index: ${zIndex};
    pointer-events: ${pointerEvents};
    cursor: pointer;
  `;
}


const pawnStyle = computed(() => {
  updateFlag.value;
  const x = engine.value.pawn_position.x;
  const y = engine.value.pawn_position.y;

  // background-image: url(${engine.pawn.texture});
  return `
    position: absolute;
    left: ${x * squareSize}px;
    top: ${y * squareSize - squareSize}px;
    background-color: red;
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
  engine.value.move_to(tile.pointer.x, tile.pointer.y);
}
</script>

<template>
  <div
    class="container"
    tabindex="0"
    style="position: relative;"
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
</template>

<style scoped>

</style>
