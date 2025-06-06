<script setup lang="ts">
import { computed } from 'vue'
import { WasmPawn, WasmLibrary } from '@rpgx/js'

const props = defineProps<{
  pawn: WasmPawn,
  library: WasmLibrary
}>()

const squareSize = 15

function getTexture(id: number) {
  const texture = props.library.get_by_id(id)
  return texture ? `url(${texture})` : ''
}

const style = computed(() => {
  const x = props.pawn.pointer.x
  const y = props.pawn.pointer.y
  return `
    ${props.pawn.textureId ? `background-image: ${getTexture(props.pawn.textureId)};` : ''}
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
