import { bearerAuthorization } from './core'
import { http } from './http'

export interface LoginPayload {
  username: string
  password: string
  captcha: string
  captchaId: string
}

export function login(payload: LoginPayload) {
  return http.post('/auth/login', payload)
}

export function getUserInfo(token: string) {
  return http.get('/users/me', {
    headers: {
      Authorization: bearerAuthorization(token)
    }
  })
}

export function getMenu(token: string) {
  return http.get('/menus/current', {
    headers: {
      Authorization: bearerAuthorization(token)
    }
  })
}
