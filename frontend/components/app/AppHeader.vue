<script lang="ts" setup>
const { toggleNavbar } = useNavbar()
const userStore = useUserStore()

const router = useRouter()

const onLogoClick = (): void => {
  router.push('/')
}
</script>

<template>
  <v-app-bar app density="compact">
    <v-app-bar-nav-icon @click="toggleNavbar"></v-app-bar-nav-icon>
    <v-toolbar-title class="pointer" @click="onLogoClick">Application</v-toolbar-title>
    <v-spacer></v-spacer>
    <template #append>
      <v-btn v-if="userStore.isLoggedIn" variant="text" to="/logout">
        Logout
      </v-btn>
      <v-btn v-else variant="text" to="/login">
        Login
      </v-btn>
    </template>
    <nuxt-loading-indicator />
  </v-app-bar>
</template>
