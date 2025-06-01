<script setup lang="ts">
import { computed } from 'vue'
import { WasmTile, WasmLayer, WasmLibrary, WasmLayerType } from '@rpgx/js'

const props = defineProps<{
  tile: WasmTile,
  layer: WasmLayer,
  library: WasmLibrary
}>()

const squareSize = 15

function getTexture(key: number) {
  const texture = props.library.get_by_id(key)
  return texture ? `url(${texture})` : ''
}

const style = computed(() => {
  const { tile, layer } = props
  const x = tile.pointer.x
  const y = tile.pointer.y
  const width = (tile.effect.group ? tile.shape.width : 1) * squareSize
  const height = (tile.effect.group ? tile.shape.height : 1) * squareSize
  const zIndex = 10 + layer.z
  const pointerEvents = layer.kind === WasmLayerType.Base ? 'auto' : 'none'

  return `
    background-image: ${tile.effect.textureId ? getTexture(tile.effect.textureId) : ''};
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
  `
})
</script>

<template>
  <div :style="style"></div>
</template>
