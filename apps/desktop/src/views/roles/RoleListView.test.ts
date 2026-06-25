import { mount } from '@vue/test-utils';
import { createPinia, setActivePinia } from 'pinia';
import { UiComponents } from '@/components/ui';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import { useAuthStore } from '@/stores/auth';
import { setLocale } from '@/i18n';

const mocks = vi.hoisted(() => ({
  setAuthorityMenus: vi.fn().mockResolvedValue({ code: 'OK' }),
}));

vi.mock('@/api/authorities', () => ({
  fetchAuthorities: vi.fn().mockResolvedValue([
    {
      authorityId: 1,
      authorityName: 'Developer',
      parentId: 0,
      defaultRouter: 'dashboard',
      children: [],
      dataAuthorityId: [],
    },
    {
      authorityId: 888,
      authorityName: 'Super Admin',
      parentId: 0,
      defaultRouter: 'dashboard',
      children: [],
      dataAuthorityId: [],
    },
  ]),
  createAuthority: vi.fn(),
  updateAuthority: vi.fn(),
  deleteAuthority: vi.fn(),
  fetchAuthorityUsers: vi.fn().mockResolvedValue([2]),
  setRoleUsers: vi.fn(),
}));

vi.mock('@/api/menus', () => ({
  fetchPermissionMenuList: vi.fn().mockResolvedValue([
    {
      ID: 1,
      parentId: 0,
      path: 'system',
      name: 'system',
      hidden: false,
      component: 'view/system/index.vue',
      sort: 1,
      meta: { title: 'System' },
      parameters: [],
      menuBtn: [],
      children: [
        {
          ID: 2,
          parentId: 1,
          path: 'users',
          name: 'users',
          hidden: false,
          component: 'view/users/index.vue',
          sort: 2,
          meta: { title: 'Users' },
          parameters: [],
          menuBtn: [],
          menuType: 'page',
          children: [
            {
              ID: 20,
              parentId: 2,
              path: '',
              name: 'users:list',
              hidden: true,
              component: '',
              sort: 1,
              meta: { title: 'List' },
              parameters: [],
              menuBtn: [],
              menuType: 'action',
              permission: 'system:user:list',
              method: 'GET',
              apiPath: '/api/users',
              children: [],
            },
            {
              ID: 21,
              parentId: 2,
              path: '',
              name: 'users:create',
              hidden: true,
              component: '',
              sort: 2,
              meta: { title: 'Create' },
              parameters: [],
              menuBtn: [],
              menuType: 'action',
              permission: 'system:user:create',
              method: 'POST',
              apiPath: '/api/users',
              children: [],
            },
          ],
        },
      ],
    },
  ]),
  fetchAuthorityMenus: vi.fn().mockResolvedValue([1, 2]),
  fetchPermissionMenuRoleMatrix: vi.fn().mockResolvedValue({
    1: [1],
    2: [1],
    20: [1],
  }),
  setAuthorityMenus: mocks.setAuthorityMenus,
}));

vi.mock('@/api/users', () => ({
  fetchUsers: vi.fn().mockResolvedValue({
    list: [
      { ID: 1, userName: 'admin', nickName: 'admin', phone: '', email: '', enable: 1 },
      { ID: 2, userName: 'nick', nickName: 'nick', phone: '', email: '', enable: 1 },
    ],
    total: 2,
    page: 1,
    pageSize: 10,
  }),
}));

import RoleListView from './RoleListView.vue';

async function flushWorkbench() {
  await Promise.resolve();
  await Promise.resolve();
  await Promise.resolve();
  await Promise.resolve();
}

function mountWithAuthority(authorityId = 888) {
  const pinia = createPinia();
  setActivePinia(pinia);
  const authStore = useAuthStore();
  authStore.setSession('token-123', {
    ID: authorityId === 888 ? 1 : 2,
    userName: authorityId === 888 ? 'admin' : 'nick',
    nickName: authorityId === 888 ? 'admin' : 'nick',
    authority: {
      authorityId,
      authorityName: authorityId === 888 ? 'Super Admin' : 'Developer',
      defaultRouter: 'dashboard',
    },
    permissions:
      authorityId === 888
        ? []
        : ['system:role:list', 'system:role:permission-tree', 'system:role:permission-matrix'],
  });

  return mount(RoleListView, {
    global: {
      plugins: [pinia, UiComponents],
    },
  });
}

describe('RoleListView', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    window.localStorage.clear();
    setLocale('en-US');
  });

  it('renders a role permission workbench', async () => {
    const wrapper = mountWithAuthority();

    await flushWorkbench();
    const header = wrapper.find('.role-workspace-header');
    expect(wrapper.text()).toContain('Roles');
    expect(wrapper.text()).toContain('New');
    expect(header.text()).not.toContain('Total 2 roles');
    expect(wrapper.text()).toContain('Function permissions');
    expect(wrapper.text()).toContain('Role users');
    expect(wrapper.text()).toContain('Developer');
    expect(wrapper.text()).toContain('Super Admin');
    expect(wrapper.text()).toContain('Users');
    expect(wrapper.text()).toContain('Page access');
    expect(wrapper.text()).toContain('List');
    expect(wrapper.text()).toContain('Create');
  });

  it('translates role workbench chrome and backend menu labels to Chinese', async () => {
    setLocale('zh-CN');
    const wrapper = mountWithAuthority();

    await flushWorkbench();

    expect(wrapper.text()).toContain('功能权限');
    expect(wrapper.text()).toContain('角色用户');
    expect(wrapper.text()).toContain('用户管理');
    expect(wrapper.text()).toContain('页面访问');
    expect(wrapper.text()).toContain('列表');
    expect(wrapper.text()).toContain('新增');
    expect(wrapper.text()).not.toContain('Function permissions');
    expect(wrapper.text()).not.toContain('Page access');
  });

  it('edits page and action permissions for the selected role', async () => {
    const wrapper = mountWithAuthority();

    await flushWorkbench();
    await wrapper.find('[data-test="role-list"] button:first-child').trigger('click');
    await flushWorkbench();

    const developerMenuCheckbox = wrapper.find('[data-test="menu-permission-2-1"]');
    expect((developerMenuCheckbox.element as HTMLInputElement).checked).toBe(true);
    await developerMenuCheckbox.setValue(false);

    await wrapper.find('[data-test="save-function-permissions"]').trigger('click');
    await flushWorkbench();

    expect(mocks.setAuthorityMenus).toHaveBeenCalled();
    const [authorityId, menuIds] = mocks.setAuthorityMenus.mock.calls.at(-1)!;
    expect(authorityId).toBe(1);
    expect(menuIds).not.toContain(2);
    expect(menuIds).not.toContain(20);
  });

  it('disables permission editing for the super admin role', async () => {
    const wrapper = mountWithAuthority();

    await flushWorkbench();
    await wrapper.find('[data-test="role-list"] button:last-child').trigger('click');
    await flushWorkbench();

    expect(wrapper.find('[data-test="save-function-permissions"]').exists()).toBe(false);

    const superAdminMenuCheckbox = wrapper.find('[data-test="menu-permission-2-888"]');
    expect((superAdminMenuCheckbox.element as HTMLInputElement).disabled).toBe(true);

    const superAdminActionCheckbox = wrapper.find('[data-test="action-permission-system:user:list-888"]');
    expect((superAdminActionCheckbox.element as HTMLInputElement).disabled).toBe(true);
  });

  it('hides permission save controls without role permission', async () => {
    const wrapper = mountWithAuthority(1);

    await flushWorkbench();

    expect(wrapper.find('[data-test="save-function-permissions"]').exists()).toBe(false);
    const menuCheckbox = wrapper.find('[data-test="menu-permission-2-1"]');
    expect((menuCheckbox.element as HTMLInputElement).disabled).toBe(true);

    const actionCheckbox = wrapper.find('[data-test="action-permission-system:user:list-1"]');
    expect((actionCheckbox.element as HTMLInputElement).disabled).toBe(true);
  });
});
