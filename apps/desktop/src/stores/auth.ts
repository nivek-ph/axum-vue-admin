import { computed, ref } from 'vue';
import { defineStore } from 'pinia';

import { clearAuthSession, readAuthSession, writeAuthSession } from './authStorage';

export interface AuthUserInfo {
  ID: number;
  userName: string;
  nickName: string;
  headerImg?: string;
  deptId?: number | null;
  deptName?: string;
  roles?: Array<{
    id: number;
    code: string;
    name: string;
  }>;
  roleIds?: number[];
  authority?: {
    authorityId: number;
    authorityName: string;
    defaultRouter: string;
  };
  permissions?: string[];
}

import { SUPER_ADMIN_AUTHORITY_ID } from '@/constants/auth';

export const useAuthStore = defineStore('auth', () => {
  const persisted = readAuthSession();
  const token = ref(persisted.token);
  const userInfo = ref<AuthUserInfo | null>(persisted.userInfo);

  const isAuthenticated = computed(() => token.value.length > 0);
  const permissionSet = computed(() => new Set(userInfo.value?.permissions || []));
  const roles = computed(() => userInfo.value?.roles || []);
  const permissions = computed(() => userInfo.value?.permissions || []);
  const isSuperAdmin = computed(
    () =>
      userInfo.value?.authority?.authorityId === SUPER_ADMIN_AUTHORITY_ID ||
      roles.value.some((role) => role.code === 'super_admin')
  );

  function persistSession() {
    writeAuthSession({
      token: token.value,
      userInfo: userInfo.value,
    });
  }

  function setToken(value: string) {
    token.value = value;
    persistSession();
  }

  function setSession(nextToken: string, nextUserInfo: AuthUserInfo) {
    token.value = nextToken;
    userInfo.value = nextUserInfo;
    persistSession();
  }

  function clearToken() {
    token.value = '';
    userInfo.value = null;
    clearAuthSession();
  }

  function can(permission: string) {
    if (isSuperAdmin.value) return true;
    return permissionSet.value.has(permission);
  }

  return {
    token,
    userInfo,
    isAuthenticated,
    roles,
    permissions,
    permissionSet,
    isSuperAdmin,
    can,
    setToken,
    setSession,
    clearToken,
  };
});
