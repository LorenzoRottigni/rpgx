<script setup lang="ts">
import { computed, onMounted, ref } from 'vue'
import Grid from './Grid.vue'
import Pawn from './Pawn.vue'
import { Coordinates, Direction, Engine, Library, Tile } from '@rpgx/js'

const props = defineProps<{
  engine: Engine,
  library: Library
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

function manageActions(target: Coordinates) {
  const actions = props.engine.getActiveScene()?.getMap().getActionsAt(target)
  if (!actions?.length) return
  actions.forEach(a => props.library.getById(a)())
}

function onClick(tile: Tile) {
  updateFlag.value++
  const steps = props.engine.getActiveScene()?.stepsTo(tile.area.origin) || []
  for (let i = 0; i < steps.length; i++) {
    const step = steps[i]
    setTimeout(() => {
      const movedTile = props.engine.getActiveScene()?.moveTo(step)
      if (movedTile) manageActions(movedTile)
      
      updateFlag.value++
    }, i * 100)
  }
}

function onKeyDown(event: KeyboardEvent) {
  let tile
  if (event.key === 'ArrowUp' || event.key.toLowerCase() === 'w') tile = props.engine.getActiveScene()?.stepTo(new Direction("up"))
  else if (event.key === 'ArrowDown' || event.key.toLowerCase() === 's') tile = props.engine.getActiveScene()?.stepTo(new Direction("down"))
  else if (event.key === 'ArrowLeft' || event.key.toLowerCase() === 'a') tile = props.engine.getActiveScene()?.stepTo(new Direction("left"))
  else if (event.key === 'ArrowRight' || event.key.toLowerCase() === 'd') tile = props.engine.getActiveScene()?.stepTo(new Direction("right"))

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
