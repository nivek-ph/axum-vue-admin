import { config } from '@vue/test-utils'
import { beforeAll, beforeEach } from 'vitest'

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

let setTestLocale: ((locale: 'zh-CN' | 'en-US') => void) | undefined

beforeAll(async () => {
  const { I18nPlugin, setLocale } = await import('@/i18n')
  setTestLocale = setLocale
  config.global.plugins = [...(config.global.plugins || []), I18nPlugin]
})

beforeEach(() => {
  storage.clear()
  setTestLocale?.('en-US')
})
