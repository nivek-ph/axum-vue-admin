import { getUserInfo } from '@/api/auth'

import { useAuthStore } from './auth'

export async function bootstrapAuthSession() {
  const authStore = useAuthStore()
  if (!authStore.isAuthenticated) {
    return
  }

  try {
    const response = await getUserInfo(authStore.token)
    if (response.code !== 'OK') {
      authStore.clearToken()
      return
    }

    authStore.setSession(authStore.token, response.data.userInfo)
  } catch {
    authStore.clearToken()
  }
}
