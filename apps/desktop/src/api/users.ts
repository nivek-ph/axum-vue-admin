import { withAuthHeaders } from './core'
import { http } from './http'

export interface UserRecord {
  ID: number
  userName: string
  nickName: string
  phone: string
  email: string
  enable: number
  authority?: {
    authorityId: number
    authorityName: string
  }
}

export interface UserListResult {
  list: UserRecord[]
  total: number
  page: number
  pageSize: number
}

export function normalizeUserListResponse(payload: any): UserListResult {
  return {
    list: payload?.data?.list || [],
    total: payload?.data?.total || 0,
    page: payload?.data?.page || 1,
    pageSize: payload?.data?.pageSize || 10
  }
}

export async function fetchUsers(page = 1, pageSize = 10) {
  const res = await http.get('/users', {
    ...withAuthHeaders(),
    params: { page, pageSize }
  })
  return normalizeUserListResponse(res)
}

export async function deleteUser(id: number) {
  return http.delete(`/users/${id}`, withAuthHeaders())
}

export async function resetUserPassword(id: number, password: string) {
  return http.post(`/users/${id}/password/reset`, { ID: id, password }, withAuthHeaders())
}
