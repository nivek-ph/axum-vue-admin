import { beforeEach, describe, expect, it, vi } from 'vitest'

import { currentLocale, setLocale, t, toggleLocale } from './index'

describe('i18n', () => {
  beforeEach(() => {
    localStorage.clear()
    setLocale('zh-CN')
  })

  it('defaults to English when no locale is persisted', async () => {
    localStorage.clear()
    vi.resetModules()

    const fresh = await import('./index')

    expect(fresh.currentLocale.value).toBe('en-US')
    expect(fresh.t('Users')).toBe('Users')
  })

  it('uses English source text by default', () => {
    setLocale('en-US')

    expect(t('Users')).toBe('Users')
    expect(t('Refresh list')).toBe('Refresh list')
  })

  it('translates English source text to Chinese', () => {
    setLocale('zh-CN')

    expect(t('Users')).toBe('用户管理')
    expect(t('Refresh list')).toBe('刷新列表')
  })

  it('falls back to source text for missing translations', () => {
    setLocale('en-US')

    expect(t('Untranslated text')).toBe('Untranslated text')
  })

  it('persists and toggles locale', () => {
    setLocale('en-US')

    expect(currentLocale.value).toBe('en-US')
    expect(localStorage.getItem('axum-vue-admin.locale')).toBe('en-US')

    toggleLocale()

    expect(currentLocale.value).toBe('zh-CN')
  })
})
