import { http } from './http';
import { withAuthHeaders, type ApiResponse } from './core';

export interface AuthorityRecord {
  authorityId: number;
  authorityName: string;
  code?: string;
  status?: string;
  sort?: number;
  dataScope?: string;
  isSystem?: boolean;
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
  code?: string;
  status?: string;
  sort?: number;
  dataScope?: string;
  parentId: number;
  defaultRouter?: string;
}

export function normalizeAuthorityListResponse(payload: ApiResponse<AuthorityRecord[]>) {
  if (Array.isArray(payload?.data)) return payload.data;

  const list = (payload?.data as any)?.list;
  if (Array.isArray(list)) {
    return list.map((role: any) => ({
      authorityId: role.authorityId ?? role.id,
      authorityName: role.authorityName ?? role.name,
      code: role.code,
      status: role.status,
      sort: role.sort,
      dataScope: role.data_scope ?? role.dataScope,
      isSystem: role.is_system ?? role.isSystem,
      parentId: role.parentId ?? 0,
      defaultRouter: role.defaultRouter ?? 'dashboard',
      children: role.children ?? [],
      dataAuthorityId: role.dataAuthorityId ?? [],
    }));
  }

  return [];
}

export async function fetchAuthorities() {
  const response = await http.get('/roles', withAuthHeaders());
  return normalizeAuthorityListResponse(response);
}

export async function createAuthority(payload: AuthorityPayload) {
  return http.post(
    '/roles',
    {
      code: payload.code || `role_${payload.authorityId}`,
      name: payload.authorityName,
      status: 'enabled',
      sort: payload.authorityId,
      data_scope: 'all',
    },
    withAuthHeaders()
  );
}

export async function updateAuthority(payload: AuthorityPayload) {
  return http.put(
    `/roles/${payload.authorityId}`,
    {
      code: payload.code || `role_${payload.authorityId}`,
      name: payload.authorityName,
      status: payload.status || 'enabled',
      sort: payload.sort ?? payload.authorityId,
      data_scope: payload.dataScope || 'all',
    },
    withAuthHeaders()
  );
}

export async function deleteAuthority(authorityId: number) {
  return http.delete(`/roles/${authorityId}`, withAuthHeaders());
}

export async function fetchAuthorityUsers(authorityId: number) {
  const response = await http.get(`/roles/${authorityId}/users`, withAuthHeaders());
  return Array.isArray(response?.data) ? response.data : [];
}

export function buildRoleUsersPayload(userIds: number[]) {
  return {
    userIds: [...new Set(userIds.filter((id) => Number.isFinite(id)))].sort((left, right) => left - right),
  };
}

export async function setRoleUsers(authorityId: number, userIds: number[]) {
  return http.put(`/roles/${authorityId}/users`, buildRoleUsersPayload(userIds), withAuthHeaders());
}
