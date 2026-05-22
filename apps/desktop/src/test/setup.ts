import { beforeEach } from 'vitest'

const storage = new Map<string, string>()

const localStorageMock = {
  getItem(key: string) {
    return storage.has(key) ? storage.get(key)! : null
  },
  setItem(key: string, value: string) {
    storage.set(key, value)
  },
  removeItem(key: string) {
    storage.delete(key)
  },
  clear() {
    storage.clear()
  }
}

Object.defineProperty(globalThis, 'localStorage', {
  configurable: true,
  value: localStorageMock
})

beforeEach(() => {
  storage.clear()
})
