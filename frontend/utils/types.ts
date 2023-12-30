// import * as djs from 'discord.js'

export type NavbarRoute = {
  name: string
  icon: string
  path: string
  type: 'route'
  sub?: boolean
  external?: boolean
  hideLogin?: boolean
  hideLogout?: boolean
  condition?: () => boolean
}

export type NavbarAction = {
  name: string
  icon: string
  action: () => void
  type: 'action'
  sub?: boolean
  hideLogin?: boolean
  hideLogout?: boolean
  condition?: () => boolean
}

export type NavbarItem = NavbarRoute | NavbarAction

// Api types

export type GuildInfo = {
  id: string
  icon?: string
  name: string
  owner: boolean
  permissions: string
}

export type InviteGuildResponse = {
  invited: GuildInfo[]
  invitable: GuildInfo[]
  not_invitable: GuildInfo[]
}
