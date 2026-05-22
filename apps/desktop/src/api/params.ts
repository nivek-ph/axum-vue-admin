import { http } from './http'
import { withAuthHeaders, type ApiResponse } from './core'

export interface ParamRecord {
  ID: number
  name: string
  key: string
  value: string
  desc: string
}

export interface ParamFilters {
  page?: number
  pageSize?: number
  name?: string
  key?: string
}

export interface ParamListResult {
  list: ParamRecord[]
  total: number
  page: number
  pageSize: number
}

export function normalizeParamListResponse(
  payload: ApiResponse<ParamListResult>
): ParamListResult {
  return {
    list: payload?.data?.list || [],
    total: payload?.data?.total || 0,
    page: payload?.data?.page || 1,
    pageSize: payload?.data?.pageSize || 10
  }
}

export async function fetchParams(filters: ParamFilters = {}) {
  const response = await http.get('/params', {
    ...withAuthHeaders(),
    params: {
      page: filters.page || 1,
      pageSize: filters.pageSize || 10,
      name: filters.name || undefined,
      key: filters.key || undefined
    }
  })
  return normalizeParamListResponse(response)
}

export async function createParam(payload: ParamRecord) {
  return http.post('/params', payload, withAuthHeaders())
}

export async function updateParam(payload: ParamRecord) {
  return http.put(`/params/${payload.ID}`, payload, withAuthHeaders())
}

export async function deleteParam(id: number) {
  return http.delete(`/params/${id}`, withAuthHeaders())
}
