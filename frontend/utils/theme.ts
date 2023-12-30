import type { ThemeDefinition } from 'vuetify'
import colors from './palette'

export const customLightTheme: ThemeDefinition = {
  dark: false,
  colors: {
    background: colors.grey.lighten4,
    surface: colors.shades.white,
    primary: colors.lightBlue.accent3,
    secondary: colors.pink.accent3,
    success: colors.teal.accent4,
    warning: colors.orange.accent4,
    error: colors.red.accent4,
    info: colors.blue.accent4,
    'on-background': colors.shades.black,
    'on-surface': colors.grey.darken4,
    'on-primary': colors.grey.lighten5,
    'on-secondary': colors.grey.lighten5,
    'on-success': colors.grey.lighten5,
    'on-warning': colors.grey.lighten5,
    'on-error': colors.grey.lighten5,
    'on-info': colors.grey.lighten5,

    header: colors.shades.white,
  },
}

export const customDarkTheme: ThemeDefinition = {
  dark: true,
  colors: {
    background: colors.grey.darken4,
    surface: colors.grey.darken4,
    primary: colors.lightBlue.base,
    secondary: colors.deepOrange.accent2,
    success: colors.green.darken1,
    warning: colors.amber.darken3,
    error: colors.red.darken1,
    info: colors.cyan.accent2,
    'on-background': colors.shades.white,
    'on-surface': colors.grey.lighten4,
    'on-primary': colors.grey.lighten4,
    'on-secondary': colors.grey.lighten4,
    'on-success': colors.shades.white,
    'on-warning': colors.shades.white,
    'on-error': colors.red.lighten5,
    'on-info': colors.shades.white,

    header: colors.blue.darken4,
  },
}

export default {
  defaultTheme: 'customDarkTheme',
  themes: {
    customLightTheme,
    customDarkTheme,
  },
}
