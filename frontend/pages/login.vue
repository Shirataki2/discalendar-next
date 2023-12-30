<script lang="ts" setup>
onMounted(() => {
  const apiUrl = 'http://localhost:5000'
  const authUrl = `${apiUrl}/auth/login`
  window.location.href = authUrl
})
</script>

<template>
  <div>Processing</div>
</template>

<style scoped></style>
