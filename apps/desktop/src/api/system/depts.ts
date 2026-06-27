import { withAuthHeaders, type ApiResponse } from '../core'
import { http } from '../http'

export interface DeptRecord {
  id: number
  parent_id?: number | null
  name: string
  code: string
  sort: number
  status: string
  children?: DeptRecord[]
}

export interface DeptPayload {
  parent_id?: number | null
  name: string
  code: string
  sort?: number
  status?: string
}

export function normalizeDeptTree(payload: ApiResponse<{ list?: DeptRecord[] }>) {
  return Array.isArray(payload?.data?.list) ? payload.data.list : []
}

export async function listDepts() {
  const response = await http.get('/depts', withAuthHeaders())
  return normalizeDeptTree(response)
}

export function createDept(payload: DeptPayload) {
  return http.post('/depts', payload, withAuthHeaders())
}

export function updateDept(id: number, payload: DeptPayload) {
  return http.put(`/depts/${id}`, payload, withAuthHeaders())
}

export function deleteDept(id: number) {
  return http.delete(`/depts/${id}`, withAuthHeaders())
}
