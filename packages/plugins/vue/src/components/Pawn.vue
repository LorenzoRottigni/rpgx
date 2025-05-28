<script setup lang="ts">
import { computed } from 'vue'
import { WasmPawn, ResourceLibrary } from '@rpgx/js'

const props = defineProps<{
  pawn: WasmPawn,
  library: ResourceLibrary
}>()

const squareSize = 15

function getTexture(key: number) {
  const texture = props.library.get_texture_by_id(key)
  return texture ? `url(${texture})` : ''
}

const style = computed(() => {
  const x = props.pawn.tile.pointer.x
  const y = props.pawn.tile.pointer.y
  return `
    ${props.pawn.texture_id ? `background-image: ${getTexture(props.pawn.texture_id)};` : ''}
    position: absolute;
    left: ${x * squareSize}px;
    top: ${y * squareSize - squareSize}px;
    background-size: cover;
    background-position: center center;
    z-index: 100;
    width: ${squareSize}px;
    height: ${squareSize * 2}px;
    transition: all 0.1s;
  `
})
</script>

<template>
  <div class="pawn" :style="style"></div>
</template>
