<script setup lang="ts">
import { onMounted, ref } from 'vue'
import Grid from './Grid.vue'
import Pawn from './Pawn.vue'
import { WasmEngine, WasmLibrary, WasmTile } from '@rpgx/js'

const props = defineProps<{
  engine: WasmEngine,
  library: WasmLibrary
}>()

const updateFlag = ref(0)

function manageActions(tile: WasmTile) {
  const actions = props.engine.map.get_actions_at(tile.pointer)
  actions.forEach(a => props.library.get_by_id(a)())
}

function onClick(tile: WasmTile) {
  updateFlag.value++
  const steps = props.engine.steps_to(tile.pointer)
  for (let i = 0; i < steps.length; i++) {
    const step = steps[i]
    setTimeout(() => {
      const movedTile = props.engine.move_to(step)
      manageActions(movedTile)
      updateFlag.value++
    }, i * 100)
  }
}

function onKeyDown(event: KeyboardEvent) {
  let tile
  if (event.key === 'ArrowUp' || event.key.toLowerCase() === 'w') tile = props.engine.step_to('up')
  else if (event.key === 'ArrowDown' || event.key.toLowerCase() === 's') tile = props.engine.step_to('down')
  else if (event.key === 'ArrowLeft' || event.key.toLowerCase() === 'a') tile = props.engine.step_to('left')
  else if (event.key === 'ArrowRight' || event.key.toLowerCase() === 'd') tile = props.engine.step_to('right')

  if (tile) manageActions(tile)
  updateFlag.value++
}

const containerRef = ref<HTMLDivElement | null>(null)
onMounted(() => containerRef.value?.focus())
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
      <Grid :map="engine.map" :library="library" @tileClick="onClick" />
      <Pawn :pawn="engine.pawn" :library="library" />
    </div>
  </main>
</template>
