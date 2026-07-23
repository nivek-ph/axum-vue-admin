import { afterEach, describe, expect, it } from 'vitest'

import { applyTheme, bootstrapTheme, useThemeStore } from './theme'

describe('theme store', () => {
  afterEach(() => {
    window.localStorage.removeItem('ava.themeMode')
    useThemeStore.setState({ mode: 'light' })
    applyTheme('light')
  })

  it('persists mode via the dark class on the document element', () => {
    useThemeStore.getState().setMode('dark')
    expect(document.documentElement.classList.contains('dark')).toBe(true)
    expect(window.localStorage.getItem('ava.themeMode')).toBe('dark')
  })

  it('bootstraps saved mode', () => {
    window.localStorage.setItem('ava.themeMode', 'dark')
    bootstrapTheme()
    expect(useThemeStore.getState().mode).toBe('dark')
    expect(document.documentElement.classList.contains('dark')).toBe(true)
  })
})
