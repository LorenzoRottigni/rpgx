<script setup lang="ts">
import Tile from './Tile.vue'
import { Map, Library } from '@rpgx/js'

defineProps<{
  map: Map,
  library: Library
}>()

defineEmits<{
  (e: 'tileClick', tile: any): void
}>()
</script>

<template>
  <div>
    <div v-for="(layer, i) in map.layers" :key="'layer-' + i">
      <Tile
        v-for="(tile, j) in layer.render()"
        :key="`layer-${i}-${j}`"
        :tile="tile"
        :layer="layer"
        :library="library"
        @click="() => $emit('tileClick', tile)"
      />
    </div>
  </div>
</template>
