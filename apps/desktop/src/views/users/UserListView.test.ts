import { flushPromises, mount } from '@vue/test-utils'
import { beforeEach, describe, expect, it, vi } from 'vitest'
import { QueryClient, VueQueryPlugin } from '@tanstack/vue-query'
import { UiComponents } from '@/components/ui'
import { createRouter, createMemoryHistory } from 'vue-router'

import UserListView from './UserListView.vue'

const mocks = vi.hoisted(() => ({
  fetchUsers: vi.fn().mockResolvedValue({
    list: [],
    total: 0,
    page: 1,
    pageSize: 10
  }),
  createUser: vi.fn(),
  deleteUser: vi.fn(),
  resetUserPassword: vi.fn(),
  updateUserAuthorities: vi.fn(),
  fetchAuthorities: vi.fn().mockResolvedValue([
    {
      authorityId: 888,
      authorityName: 'Super Admin',
      parentId: 0,
      defaultRouter: 'dashboard',
      children: [],
      dataAuthorityId: []
    },
    {
      authorityId: 1001,
      authorityName: 'Dev',
      parentId: 0,
      defaultRouter: 'dashboard',
      children: [],
      dataAuthorityId: []
    }
  ])
}))

vi.mock('@/api/users', () => ({
  fetchUsers: mocks.fetchUsers,
  createUser: mocks.createUser,
  deleteUser: mocks.deleteUser,
  resetUserPassword: mocks.resetUserPassword,
  updateUserAuthorities: mocks.updateUserAuthorities
}))

vi.mock('@/api/authorities', () => ({
  fetchAuthorities: mocks.fetchAuthorities
}))

function mountView() {
  const router = createRouter({
    history: createMemoryHistory(),
    routes: [{ path: '/', component: UserListView }]
  })
  const queryClient = new QueryClient({
    defaultOptions: {
      queries: {
        retry: false
      }
    }
  })
  return mount(UserListView, {
    global: {
      plugins: [UiComponents, router, [VueQueryPlugin, { queryClient }]]
    }
  })
}

describe('UserListView', () => {
  beforeEach(() => {
    mocks.fetchUsers.mockResolvedValue({
      list: [],
      total: 0,
      page: 1,
      pageSize: 10
    })
  })

  it('renders user management heading', () => {
    const wrapper = mountView()

    expect(wrapper.text()).toContain('Users')
  })

  it('opens a new-user dialog from the user page', async () => {
    const wrapper = mountView()

    await wrapper.find('[data-test="new-user-button"]').trigger('click')

    expect(wrapper.text()).toContain('New user')
    expect(wrapper.text()).toContain('Username')
    expect(wrapper.text()).toContain('Password')
    expect(wrapper.text()).toContain('Role')
    expect(wrapper.text()).toContain('Super Admin')
    expect(wrapper.find('[data-test="user-role-select"]').exists()).toBe(true)
    expect(wrapper.find('[data-test="user-role-select"] select').exists()).toBe(false)
  })

  it('opens a change-role dialog from an existing user row', async () => {
    mocks.fetchUsers.mockResolvedValue({
      list: [
        {
          ID: 4,
          userName: 'nick',
          nickName: 'nick',
          phone: '',
          email: '',
          enable: 1,
          authority: {
            authorityId: 1001,
            authorityName: 'Dev'
          }
        }
      ],
      total: 1,
      page: 1,
      pageSize: 10
    })
    const wrapper = mountView()
    await flushPromises()

    await wrapper.find('[data-test="change-user-role-button"]').trigger('click')

    expect(wrapper.text()).toContain('Change user role')
    expect(wrapper.text()).toContain('nick')
    expect(wrapper.text()).toContain('Dev')
    expect(wrapper.find('[data-test="edit-user-role-select"]').exists()).toBe(true)
  })
})
