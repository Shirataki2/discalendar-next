<script setup lang="ts">
interface Props {
  size?: number
  color?: string
  duration?: number
}

const props = withDefaults(defineProps<Props>(), {
  size: 100,
  color: '#88D477',
  duration: 800,
})

const rectStyle = computed(() => ({
  backgroundColor: props.color,
  animationDuration: props.duration + 'ms',
  margin: '0 2px',
}))

const wrapperStyle = computed(() => ({
  width: props.size + 'px',
  height: props.size + 'px',
}))
</script>

<template>
  <div style="text-align: center">
    <div :style="wrapperStyle" class="spinner spinner--stretch">
      <div :style="rectStyle" class="rect rect-1" />
      <div :style="rectStyle" class="rect rect-2" />
      <div :style="rectStyle" class="rect rect-3" />
      <div :style="rectStyle" class="rect rect-4" />
      <div :style="rectStyle" class="rect rect-5" />
    </div>
  </div>
</template>

<style lang="scss">
.spinner {
  display: inline-block;
  text-align: center;
  font-size: 10px;
  * {
    box-sizing: border-box;
    line-height: 0;
  }
}

.rect {
  animation: sk-stretchdelay 1.2s ease-in-out infinite;
}
.spinner > div {
  height: 100%;
  width: 10%;
  display: inline-block;
}
.spinner .rect-1 {
  animation-delay: -1.2s;
}
.spinner .rect-2 {
  animation-delay: -1.1s;
}
.spinner .rect-3 {
  animation-delay: -1s;
}
.spinner .rect-4 {
  animation-delay: -0.9s;
}
.spinner .rect-5 {
  animation-delay: -0.8s;
}
@keyframes sk-stretchdelay {
  0%,
  40%,
  100% {
    transform: scaleY(0.3);
  }
  20% {
    transform: scaleY(1);
  }
}
</style>
