<script setup lang="ts">
const { navBar } = useNavbar()
const { themeLabel, themeIcon, rotateTheme } = useAppTheme()
const userStore = useUserStore()

const bottomItems = computed<NavbarItem[]>(() => [
  {
    name: 'テーマ: ' + themeLabel.value,
    icon: themeIcon.value,
    type: 'action',
    action: rotateTheme,
  },
])

const filterItems = (items: NavbarItem[]): NavbarItem[] => {
  const isLoggedin = userStore.isLoggedIn
  return items.filter(item => {
    if (item.hideLogin && isLoggedin) return false
    if (item.hideLogout && !isLoggedin) return false
    if (item.condition && !item.condition()) return false
    return true
  })
}
</script>

<template>
  <v-navigation-drawer v-model="navBar" app>
    <v-list dense>
      <v-list-item exact>
        <v-list-item-title>Home</v-list-item-title>
      </v-list-item>
      <v-list-item>
        <v-list-item-title>About</v-list-item-title>
      </v-list-item>
    </v-list>
    <template #append>
      <VList>
        <AppNavDrawerItem v-for="item in filterItems(bottomItems)" :key="item.name" :item="item" />
      </VList>
    </template>
  </v-navigation-drawer>
</template>
