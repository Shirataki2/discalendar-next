<script setup lang="ts">
const counter = useCounterStore()
</script>

<template>
  <div>
    <h1>Nuxt Routing set up successfully!</h1>
    <span>{{ counter.count }}</span>
    <VBtn color="primary" icon variant="text" @click="counter.increment">+</VBtn>
  </div>
</template>
