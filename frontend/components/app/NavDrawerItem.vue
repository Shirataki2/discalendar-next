<script setup lang="ts">
import type { NavbarItem } from '@/utils/types'

interface Props {
  item: NavbarItem
}
defineProps<Props>()
</script>

<template>
  <VListItem
    :to="item.type === 'route' && !item.external ? item.path : undefined"
    :href="item.type === 'route' && item.external ? item.path : undefined"
    :target="item.type === 'route' && item.external ? '_blank' : undefined"
    :title="item.type === 'route' && item.external ? `${item.name} ↗` : item.name"
    color="primary"
    :prepend-icon="item.icon"
    :class="item.sub ? 'ml-3' : ''"
    exact
    @click="item.type === 'action' ? item.action() : undefined"
  />
</template>
