import { createPinia, setActivePinia } from 'pinia';
import { describe, expect, it } from 'vitest';
import { createAppRouter } from './index';
import { useAuthStore } from '@/stores/auth';
import { useMenuStore } from '@/stores/menu';

describe('router', () => {
  it('includes core admin routes', () => {
    const router = createAppRouter();
    const routeNames = router.getRoutes().map((route) => route.name);

    expect(routeNames).toContain('login');
    expect(routeNames).toContain('dashboard');
    expect(routeNames).toContain('users');
    expect(routeNames).toContain('roles');
    expect(routeNames).toContain('departments');
    expect(routeNames).toContain('permissions');
    expect(routeNames).toContain('api-permissions');
    expect(routeNames).not.toContain('forbidden');
  });

  it('redirects unauthenticated users to login', async () => {
    setActivePinia(createPinia());
    const router = createAppRouter();
    const authStore = useAuthStore();
    authStore.clearToken();

    await router.push('/users');

    expect(router.currentRoute.value.name).toBe('login');
  });

  it('redirects authenticated users away from routes missing from their current menu', async () => {
    setActivePinia(createPinia());
    const router = createAppRouter();
    const authStore = useAuthStore();
    const menuStore = useMenuStore();
    authStore.setSession('token-123', {
      ID: 1,
      userName: 'operator',
      nickName: 'Operator',
      authority: {
        authorityId: 999,
        authorityName: 'Operator',
        defaultRouter: 'dashboard',
      },
    });
    menuStore.setAuthorizedMenus([{ name: 'dashboard', path: 'dashboard', meta: { title: 'Dashboard' } }]);

    await router.push('/roles');

    expect(router.currentRoute.value.name).toBe('dashboard');
  });
});
