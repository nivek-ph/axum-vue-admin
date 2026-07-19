import type { AxiosAdapter } from 'axios';
import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { afterEach, beforeEach, describe, expect, it } from 'vitest';

import { http } from '@/api/http';
import { Application } from '@/app/Application';
import { useAuthStore } from '@/stores/auth';
import { useMenuStore } from '@/stores/menu';

describe('Audit workflow', () => {
  const originalAdapter = http.defaults.adapter;
  beforeEach(() => {
    useAuthStore.getState().clearSession();
    useMenuStore.getState().resetAccess();
  });
  afterEach(() => {
    http.defaults.adapter = originalAdapter;
  });

  it('opens the selected event and preserves its timestamp and structured changes', async () => {
    const user = userEvent.setup();
    const currentUser = { id: 1, userName: 'admin', nickName: 'Admin', roles: [{ id: 1, code: 'super_admin', name: 'Super Admin' }] };
    useAuthStore.getState().setSession({ accessToken: 'token', refreshToken: 'refresh', userInfo: currentUser });
    const event = { id: 9, actorId: 1, actorLabel: 'admin', action: 'user.update', resourceType: 'user', resourceId: '42', result: 'succeeded', sourceIp: '127.0.0.1', userAgent: 'browser', changes: [{ field: 'email', before: 'a@example.com', after: 'b@example.com' }], createdAt: '2026-07-18T01:00:00Z' };
    http.defaults.adapter = (async (config) => {
      let data: unknown;
      if (config.url === '/users/me') data = { code: 'OK', message: 'ok', data: { userInfo: currentUser } };
      else if (config.url === '/menus/current') data = { code: 'OK', message: 'ok', data: { menus: [{ name: 'audit-events', path: 'audit-events' }], permissions: [] } };
      else if (config.url === '/audit/events') data = { code: 'OK', message: 'ok', data: { list: [event], total: 1, page: 1, pageSize: 10 } };
      else if (config.url === '/audit/events/9') data = { code: 'OK', message: 'ok', data: event };
      else throw new Error(`Unexpected request: ${config.method} ${config.url}`);
      return { data, status: 200, statusText: 'OK', headers: {}, config };
    }) as AxiosAdapter;
    window.history.replaceState({}, '', '/audit-events');
    render(<Application />);

    await user.click(await screen.findByRole('button', { name: 'View detail' }));
    expect(await screen.findByRole('heading', { name: 'Audit event detail' })).toBeInTheDocument();
    expect(await screen.findAllByText('2026-07-18T01:00:00Z')).toHaveLength(2);
    expect(screen.getByText(/a@example\.com/)).toBeInTheDocument();
    expect(screen.getByText(/b@example\.com/)).toBeInTheDocument();
  });
});
