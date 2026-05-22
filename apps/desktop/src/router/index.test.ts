import { createPinia, setActivePinia } from 'pinia'
import { describe, expect, it } from 'vitest'
import { createAppRouter } from './index'
import { useAuthStore } from '@/stores/auth'

describe('router', () => {
  it('includes core admin routes', () => {
    const router = createAppRouter()
    const routeNames = router.getRoutes().map((route) => route.name)

    expect(routeNames).toContain('login')
    expect(routeNames).toContain('dashboard')
    expect(routeNames).toContain('users')
    expect(routeNames).toContain('roles')
  })

  it('redirects unauthenticated users to login', async () => {
    setActivePinia(createPinia())
    const router = createAppRouter()
    const authStore = useAuthStore()
    authStore.clearToken()

    await router.push('/users')

    expect(router.currentRoute.value.name).toBe('login')
  })
})
