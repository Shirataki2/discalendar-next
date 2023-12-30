import { ClientUser } from 'discord.js'

export const useUserStore = defineStore('user', () => {
  const user = ref<ClientUser | null>(null)
  const isLoggedIn = computed(() => user.value !== null)

  const fetchUser = async () => {
    try {
      const fetchedUser = await $fetch<ClientUser>('/api/v1/users/me')
      user.value = fetchedUser
    } catch (_) {
      user.value = null
    }
  }

  const logout = async () => {
    await $fetch('/api/v1/auth/logout')
    user.value = null
  }

  const avatarUrl = computed(() => {
    if (!user.value) return null
    return `https://cdn.discordapp.com/avatars/${user.value.id}/${user.value.avatar}.png`
  })

  return {
    user,
    isLoggedIn,
    fetchUser,
    logout,
    avatarUrl,
  }
}, {
  persist: true,
})
