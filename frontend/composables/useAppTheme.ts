import { useTheme } from 'vuetify'

export const useAppTheme = () => {
  type Theme = 'light' | 'dark' | 'system'
  const theme = usePersistedState<Theme>('theme', 'system')
  const vuetifyTheme = useTheme()

  const setTheme = (_theme: Theme) => {
    theme.value = _theme
    const osDarkMode = window.matchMedia('(prefers-color-scheme: dark)').matches
    const isDark = theme.value === 'dark' || (theme.value === 'system' && osDarkMode)
    vuetifyTheme.global.name.value = isDark ? 'customDarkTheme' : 'customLightTheme'
  }

  const themeLabel = computed(() => {
    switch (theme.value) {
      case 'light':
        return 'ライト'
      case 'dark':
        return 'ダーク'
      case 'system':
        return 'システム'
    }
  })

  const themeIcon = computed(() => {
    switch (theme.value) {
      case 'light':
        return 'mdi-weather-sunny'
      case 'dark':
        return 'mdi-weather-night'
      case 'system':
        return 'mdi-laptop'
    }
  })

  const rotateTheme = () => {
    switch (theme.value) {
      case 'light':
        setTheme('dark')
        break
      case 'dark':
        setTheme('system')
        break
      case 'system':
        setTheme('light')
        break
    }
  }

  onMounted(() => {
    let isDark = false
    window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', e => {
      if (theme.value === 'system') {
        isDark = e.matches
        vuetifyTheme.global.name.value = isDark ? 'customDarkTheme' : 'customLightTheme'
      }
    })
    window.matchMedia('(prefers-color-scheme: light)').addEventListener('change', e => {
      if (theme.value === 'system') {
        isDark = !e.matches
        vuetifyTheme.global.name.value = isDark ? 'customDarkTheme' : 'customLightTheme'
      }
    })
    isDark =
      theme.value === 'dark' || (theme.value === 'system' && window.matchMedia('(prefers-color-scheme: dark)').matches)
    vuetifyTheme.global.name.value = isDark ? 'customDarkTheme' : 'customLightTheme'
  })

  return {
    theme,
    setTheme,
    themeLabel,
    themeIcon,
    rotateTheme,
  }
}
