<script setup lang="ts">
import { computed, onMounted, ref } from 'vue'
import Grid from './Grid.vue'
import Pawn from './Pawn.vue'
import { WasmCoordinates, WasmDirection, WasmEngine, WasmLibrary, WasmTile } from '@rpgx/js'

const props = defineProps<{
  engine: WasmEngine,
  library: WasmLibrary
}>()

const updateFlag = ref(0)

const scene = computed(() => {
  updateFlag.value;
  return props.engine.getActiveScene() || null
})

const map = computed(() => {
  updateFlag.value;
  return scene.value?.getMap() || null
})

const pawn = computed(() => {
  updateFlag.value;
  return scene.value?.getPawn()
})

function manageActions(target: WasmCoordinates) {
  const actions = props.engine.getActiveScene()?.getMap().getActionsAt(target)
  if (!actions?.length) return
  actions.forEach(a => props.library.get_by_id(a)())
}

function onClick(tile: WasmTile) {
  updateFlag.value++
  const steps = props.engine.getActiveScene()?.steps_to(tile.area.origin) || []
  for (let i = 0; i < steps.length; i++) {
    const step = steps[i]
    setTimeout(() => {
      const movedTile = props.engine.getActiveScene()?.move_to(step)
      if (movedTile) manageActions(movedTile)
      
      updateFlag.value++
    }, i * 100)
  }
}

function onKeyDown(event: KeyboardEvent) {
  let tile
  if (event.key === 'ArrowUp' || event.key.toLowerCase() === 'w') tile = props.engine.getActiveScene()?.step_to(new WasmDirection("up"))
  else if (event.key === 'ArrowDown' || event.key.toLowerCase() === 's') tile = props.engine.getActiveScene()?.step_to(new WasmDirection("down"))
  else if (event.key === 'ArrowLeft' || event.key.toLowerCase() === 'a') tile = props.engine.getActiveScene()?.step_to(new WasmDirection("left"))
  else if (event.key === 'ArrowRight' || event.key.toLowerCase() === 'd') tile = props.engine.getActiveScene()?.step_to(new WasmDirection("right"))

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
      <Grid v-if="map" :map="map" :library="library" @tileClick="onClick" />
      <Pawn v-if="pawn" :pawn="pawn" :library="library" />
    </div>
  </main>
</template>
