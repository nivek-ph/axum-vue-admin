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
}

export const useAuthStore = defineStore('auth', () => {
  const persisted = readAuthSession();
  const token = ref(persisted.token);
  const userInfo = ref<AuthUserInfo | null>(persisted.userInfo);

  const isAuthenticated = computed(() => token.value.length > 0);

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

  return {
    token,
    userInfo,
    isAuthenticated,
    setToken,
    setSession,
    clearToken,
  };
});
