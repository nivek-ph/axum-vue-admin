import axios from 'axios';
import type { AxiosError } from 'axios';

import { clearAuthSession } from '@/stores/authStorage';
import { useAuthStore } from '@/stores/auth';

const defaultApiBaseUrl = 'http://127.0.0.1:3000/api';

export const http = axios.create({
  baseURL: import.meta.env.VITE_API_BASE_URL || defaultApiBaseUrl,
  timeout: 15_000,
});

export interface ApiEnvelope {
  code: string;
  message: string;
  data?: unknown;
}

export function isApiEnvelope(value: unknown): value is ApiEnvelope {
  if (typeof value !== 'object' || value === null) return false;
  const o = value as Record<string, unknown>;
  return typeof o.code === 'string' && typeof o.message === 'string';
}

export class ApiHttpError extends Error {
  readonly status?: number;
  readonly body?: ApiEnvelope;

  constructor(message: string, opts?: { status?: number; body?: ApiEnvelope; cause?: unknown }) {
    super(message, opts?.cause !== undefined ? { cause: opts.cause } : undefined);
    this.name = 'ApiHttpError';
    this.status = opts?.status;
    this.body = opts?.body;
  }
}

/** Prefer backend `{ message }`; otherwise fallback (e.g. network / non-JSON error). */
export function getApiErrorMessage(err: unknown, fallback: string): string {
  if (err instanceof ApiHttpError) {
    const m = err.message?.trim();
    return m ? m : fallback;
  }
  if (axios.isAxiosError(err)) {
    const data = err.response?.data;
    if (isApiEnvelope(data)) {
      const m = data.message?.trim();
      if (m) return m;
    }
  }
  if (err instanceof Error) {
    const m = err.message?.trim();
    if (m) return m;
  }
  return fallback;
}

http.interceptors.response.use(
  (response) => response.data,
  (error: AxiosError) => {
    const status = error.response?.status;
    const requestUrl = error.config?.url || '';
    const isPublicAuthRequest = requestUrl.includes('/auth/login');

    if (status === 401 && !isPublicAuthRequest) {
      try {
        useAuthStore().clearToken();
      } catch {
        clearAuthSession();
      }

      if (typeof window !== 'undefined' && !window.location.hash.includes('/login')) {
        window.location.hash = '#/login';
      }
    }

    const data = error.response?.data;
    if (isApiEnvelope(data)) {
      const msg = data.message?.trim() ? data.message : '请求失败';
      return Promise.reject(new ApiHttpError(msg, { status, body: data, cause: error }));
    }
    return Promise.reject(error);
  }
);
