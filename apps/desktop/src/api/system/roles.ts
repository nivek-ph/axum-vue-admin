import { withAuthHeaders, type ApiResponse } from '../core'
import { http } from '../http'

export interface RoleResource {
  id: number
  code: string
  name: string
  status: string
  sort: number
  data_scope: string
  is_system: boolean
}

export interface RolePayload {
  code: string
  name: string
  status?: string
  sort?: number
  data_scope?: string
}

export interface RolePermissionPayload {
  permissionIds: number[]
}

export interface RoleDeptPayload {
  deptIds: number[]
}

export function normalizeRoleList(payload: ApiResponse<{ list?: RoleResource[] }>) {
  return Array.isArray(payload?.data?.list) ? payload.data.list : []
}

export function normalizeRoleIds(payload: ApiResponse<{ permissionIds?: number[]; deptIds?: number[] }>, key: 'permissionIds' | 'deptIds') {
  const values = payload?.data?.[key]
  return Array.isArray(values) ? values : []
}

function sortedIds(ids: number[]) {
  return [...ids].filter((id) => Number.isFinite(id)).sort((left, right) => left - right)
}

export function buildRolePermissionPayload(permissionIds: number[]): RolePermissionPayload {
  return { permissionIds: sortedIds(permissionIds) }
}

export function buildRoleDeptPayload(deptIds: number[]): RoleDeptPayload {
  return { deptIds: sortedIds(deptIds) }
}

export async function listRoles() {
  const response = await http.get('/roles', withAuthHeaders())
  return normalizeRoleList(response)
}

export function createRole(payload: RolePayload) {
  return http.post('/roles', payload, withAuthHeaders())
}

export function updateRole(id: number, payload: RolePayload) {
  return http.put(`/roles/${id}`, payload, withAuthHeaders())
}

export function deleteRole(id: number) {
  return http.delete(`/roles/${id}`, withAuthHeaders())
}

export async function getRolePermissionIds(id: number) {
  const response = await http.get(`/roles/${id}/permissions`, withAuthHeaders())
  return normalizeRoleIds(response, 'permissionIds')
}

export function setRolePermissionIds(id: number, permissionIds: number[]) {
  return http.put(`/roles/${id}/permissions`, buildRolePermissionPayload(permissionIds), withAuthHeaders())
}

export async function getRoleDeptIds(id: number) {
  const response = await http.get(`/roles/${id}/depts`, withAuthHeaders())
  return normalizeRoleIds(response, 'deptIds')
}

export function setRoleDeptIds(id: number, deptIds: number[]) {
  return http.put(`/roles/${id}/depts`, buildRoleDeptPayload(deptIds), withAuthHeaders())
}
