<script lang="ts" setup>
const selectedDate = ref([new Date()])

const events = computed(() => {
  return [
    {
      title: 'Event 1',
      start: new Date('2023-12-25 10:00'),
      end: new Date('2023-12-25 13:00'),
      color: 'red'
    },
    {
      title: 'Event 2 lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.',
      start: new Date('2023-12-28 12:00'),
      end: new Date('2023-12-28 13:00'),
      color: 'red'
    },
    {
      title: 'Event 3\nMultiline',
      start: new Date('2023-12-30 14:00'),
      end: new Date('2023-12-30 15:00'),
      color: 'red'
    }
  ]
})
</script>

<template>
  <v-card class="ma-3 pa-2">
    <v-calendar
      id="calendar"
      ref="calendar"
      v-model="selectedDate"
      view-mode="week"
      :events="events"
    />
  </v-card>
</template>

