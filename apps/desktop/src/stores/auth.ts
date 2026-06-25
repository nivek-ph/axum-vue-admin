import { computed, ref } from 'vue';
import { defineStore } from 'pinia';

import { clearAuthSession, readAuthSession, writeAuthSession } from './authStorage';

export interface AuthUserInfo {
  ID: number;
  userName: string;
  nickName: string;
  headerImg?: string;
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
    if (userInfo.value?.authority?.authorityId === SUPER_ADMIN_AUTHORITY_ID) return true;
    return permissionSet.value.has(permission);
  }

  return {
    token,
    userInfo,
    isAuthenticated,
    permissionSet,
    can,
    setToken,
    setSession,
    clearToken,
  };
});
