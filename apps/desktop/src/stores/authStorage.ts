import type { AuthUserInfo } from './auth';

const STORAGE_KEY = 'axum-vue-admin.auth';

export interface PersistedAuthSession {
  token: string;
  userInfo: AuthUserInfo | null;
}

export function readAuthSession(): PersistedAuthSession {
  if (typeof localStorage === 'undefined') {
    return { token: '', userInfo: null };
  }

  try {
    const raw = localStorage.getItem(STORAGE_KEY);
    if (!raw) {
      return { token: '', userInfo: null };
    }

    const parsed = JSON.parse(raw) as Partial<PersistedAuthSession>;
    const token = typeof parsed.token === 'string' ? parsed.token.trim() : '';
    const userInfo = parsed.userInfo && typeof parsed.userInfo === 'object' ? (parsed.userInfo as AuthUserInfo) : null;

    if (!token) {
      return { token: '', userInfo: null };
    }

    return { token, userInfo };
  } catch {
    return { token: '', userInfo: null };
  }
}

export function writeAuthSession(session: PersistedAuthSession) {
  if (typeof localStorage === 'undefined') {
    return;
  }

  const token = session.token.trim();
  if (!token) {
    localStorage.removeItem(STORAGE_KEY);
    return;
  }

  localStorage.setItem(
    STORAGE_KEY,
    JSON.stringify({
      token,
      userInfo: session.userInfo,
    } satisfies PersistedAuthSession)
  );
}

export function clearAuthSession() {
  if (typeof localStorage === 'undefined') {
    return;
  }

  localStorage.removeItem(STORAGE_KEY);
}
