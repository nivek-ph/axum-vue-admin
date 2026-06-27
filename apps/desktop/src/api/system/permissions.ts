import { withAuthHeaders, type ApiResponse } from '../core'
import { http } from '../http'

export interface PermissionResource {
  id: number
  module_key: string
  resource: string
  action: string
  code: string
  name: string
  type: 'page' | 'action' | 'api' | 'data'
  status: string
}

export interface PermissionPayload {
  module_key: string
  resource: string
  action: string
  code: string
  name: string
  type?: 'page' | 'action' | 'api' | 'data'
  status?: string
}

export interface PermissionApiBinding {
  method: string
  path_pattern: string
}

export function normalizePermissionList(payload: ApiResponse<{ list?: PermissionResource[] }>) {
  return Array.isArray(payload?.data?.list) ? payload.data.list : []
}

export function normalizePermissionApis(payload: ApiResponse<{ apis?: PermissionApiBinding[] }>) {
  return Array.isArray(payload?.data?.apis) ? payload.data.apis : []
}

export async function listPermissions() {
  const response = await http.get('/permissions', withAuthHeaders())
  return normalizePermissionList(response)
}

export function createPermission(payload: PermissionPayload) {
  return http.post('/permissions', payload, withAuthHeaders())
}

export function updatePermission(id: number, payload: PermissionPayload) {
  return http.put(`/permissions/${id}`, payload, withAuthHeaders())
}

export function deletePermission(id: number) {
  return http.delete(`/permissions/${id}`, withAuthHeaders())
}

export async function listPermissionApis(id: number) {
  const response = await http.get(`/permissions/${id}/apis`, withAuthHeaders())
  return normalizePermissionApis(response)
}

export function setPermissionApis(id: number, apis: PermissionApiBinding[]) {
  return http.put(`/permissions/${id}/apis`, { apis }, withAuthHeaders())
}
