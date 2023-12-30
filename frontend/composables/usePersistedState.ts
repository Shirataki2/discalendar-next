export const usePersistedState = <T>(identifier: string, defaultOptions: T) => {
  const store = useCookie<T | undefined>(identifier)
  const persistedObject = useState<T>(identifier, (): T => {
    const item = store.value
    if (item === null || item === undefined) {
      return defaultOptions
    }
    return item ?? defaultOptions
  })

  watch(
    persistedObject,
    object => {
      store.value = object
    },
    { deep: true },
  )

  return persistedObject
}
