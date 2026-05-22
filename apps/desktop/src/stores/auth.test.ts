import { createPinia, setActivePinia } from 'pinia'
import { afterEach, describe, expect, it } from 'vitest'
import { useAuthStore } from './auth'
import { clearAuthSession, readAuthSession } from './authStorage'

describe('auth store', () => {
  afterEach(() => {
    clearAuthSession()
  })

  it('starts logged out', () => {
    setActivePinia(createPinia())
    const store = useAuthStore()

    expect(store.token).toBe('')
    expect(store.isAuthenticated).toBe(false)
  })

  it('stores token and user info after login state is set', () => {
    setActivePinia(createPinia())
    const store = useAuthStore()

    store.setSession('token-123', {
      ID: 1,
      userName: 'admin',
      nickName: '系统管理员'
    })

    expect(store.token).toBe('token-123')
    expect(store.isAuthenticated).toBe(true)
    expect(store.userInfo?.userName).toBe('admin')
    expect(readAuthSession().token).toBe('token-123')
  })

  it('clears persisted session on logout', () => {
    setActivePinia(createPinia())
    const store = useAuthStore()

    store.setSession('token-123', {
      ID: 1,
      userName: 'admin',
      nickName: '系统管理员'
    })
    store.clearToken()

    expect(store.isAuthenticated).toBe(false)
    expect(readAuthSession().token).toBe('')
  })
})
