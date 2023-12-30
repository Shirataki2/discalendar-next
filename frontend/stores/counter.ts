export const useCounterStore = defineStore('counter', () => {
  const count = ref(0)
  const increment = () => count.value++
  return {
    count,
    increment,
  }
}, {
  persist: true,
})
