<script lang="ts" setup>
const userStore = useUserStore()
await userStore.logout()
onMounted(() => {
  window.location.href = '/'
})
</script>

<template>
  <div>Processing</div>
</template>

<style scoped></style>
