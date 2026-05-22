import { http } from './http';
import { withAuthHeaders, type ApiResponse } from './core';

export interface AuthorityRecord {
  authorityId: number;
  authorityName: string;
  parentId: number;
  defaultRouter: string;
  children: AuthorityRecord[];
  dataAuthorityId: Array<{
    authorityId: number;
    authorityName: string;
  }>;
}

export interface AuthorityPayload {
  authorityId: number;
  authorityName: string;
  parentId: number;
  defaultRouter?: string;
}

export function normalizeAuthorityListResponse(payload: ApiResponse<AuthorityRecord[]>) {
  return Array.isArray(payload?.data) ? payload.data : [];
}

export async function fetchAuthorities() {
  const response = await http.get('/roles', withAuthHeaders());
  return normalizeAuthorityListResponse(response);
}

export async function createAuthority(payload: AuthorityPayload) {
  return http.post(
    '/roles',
    {
      authorityId: payload.authorityId,
      authorityName: payload.authorityName,
      parentId: payload.parentId,
    },
    withAuthHeaders()
  );
}

export async function updateAuthority(payload: AuthorityPayload) {
  return http.put(`/roles/${payload.authorityId}`, payload, withAuthHeaders());
}

export async function deleteAuthority(authorityId: number) {
  return http.delete(`/roles/${authorityId}`, withAuthHeaders());
}

export async function fetchAuthorityUsers(authorityId: number) {
  const response = await http.get(`/roles/${authorityId}/users`, withAuthHeaders());
  return Array.isArray(response?.data) ? response.data : [];
}

export async function setRoleUsers(authorityId: number, userIds: number[]) {
  return http.put(`/roles/${authorityId}/users`, { authorityId, userIds }, withAuthHeaders());
}
