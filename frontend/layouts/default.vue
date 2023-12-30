<script setup lang="ts">
const userStore = useUserStore()
onMounted(async () => {
  await userStore.fetchUser()
})
</script>

<template>
  <VApp>
    <AppHeader />
    <AppNavDrawer />
    <VMain>
      <slot />
    </VMain>
  </VApp>
</template>
