import { http } from './http'
import { withAuthHeaders, type ApiResponse } from './core'

export interface DictionaryRecord {
  ID: number
  name: string
  type: string
  status?: boolean
  desc: string
  parentID?: number | null
}

export interface DictionaryDetailRecord {
  ID: number
  label: string
  value: string
  extend: string
  status?: boolean
  sort: number
  sysDictionaryID: number
  parentID?: number | null
  level: number
  path: string
  children: DictionaryDetailRecord[]
}

export interface DictionaryDetailPayload {
  ID: number
  label: string
  value: string
  extend: string
  status?: boolean
  sort: number
  sysDictionaryID: number
  parentID?: number | null
}

export function normalizeDictionaryListResponse(payload: ApiResponse<DictionaryRecord[]>) {
  return Array.isArray(payload?.data) ? payload.data : []
}

export function normalizeDictionaryDetailTreeResponse(
  payload: ApiResponse<{ list: DictionaryDetailRecord[] }>
) {
  return payload?.data?.list || []
}

export async function fetchDictionaries(name = '') {
  const response = await http.get('/dictionaries', {
    ...withAuthHeaders(),
    params: { name: name || undefined }
  })
  return normalizeDictionaryListResponse(response)
}

export async function fetchDictionaryDetails(sysDictionaryID: number) {
  const response = await http.get(`/dictionaries/${sysDictionaryID}/details/tree`, withAuthHeaders())
  return normalizeDictionaryDetailTreeResponse(response)
}

export async function createDictionary(payload: DictionaryRecord) {
  return http.post('/dictionaries', payload, withAuthHeaders())
}

export async function updateDictionary(payload: DictionaryRecord) {
  return http.put(`/dictionaries/${payload.ID}`, payload, withAuthHeaders())
}

export async function deleteDictionary(id: number) {
  return http.delete(`/dictionaries/${id}`, withAuthHeaders())
}

export async function createDictionaryDetail(payload: DictionaryDetailPayload) {
  return http.post(
    '/dictionary-details',
    {
      ...payload,
      level: 0,
      path: '',
      children: []
    },
    withAuthHeaders()
  )
}

export async function updateDictionaryDetail(payload: DictionaryDetailPayload) {
  return http.put(
    `/dictionary-details/${payload.ID}`,
    {
      ...payload,
      level: 0,
      path: '',
      children: []
    },
    withAuthHeaders()
  )
}

export async function deleteDictionaryDetail(id: number) {
  return http.delete(`/dictionary-details/${id}`, withAuthHeaders())
}
