import { afterEach, describe, expect, it } from 'vitest';

import { clearAuthSession, readAuthSession, writeAuthSession } from './authStorage';

describe('authStorage', () => {
  afterEach(() => {
    clearAuthSession();
  });

  it('persists and restores token with user info', () => {
    writeAuthSession({
      token: 'token-123',
      userInfo: {
        ID: 1,
        userName: 'admin',
        nickName: '系统管理员',
      },
    });

    expect(readAuthSession()).toEqual({
      token: 'token-123',
      userInfo: {
        ID: 1,
        userName: 'admin',
        nickName: '系统管理员',
      },
    });
  });

  it('clears invalid or empty sessions', () => {
    localStorage.setItem('axum-vue-admin.auth', '{not-json');
    expect(readAuthSession()).toEqual({ token: '', userInfo: null });

    writeAuthSession({ token: '   ', userInfo: null });
    expect(localStorage.getItem('axum-vue-admin.auth')).toBeNull();
  });
});
