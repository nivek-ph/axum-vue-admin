import { mount } from '@vue/test-utils'
import { describe, expect, it, vi } from 'vitest'
import { QueryClient, VueQueryPlugin } from '@tanstack/vue-query'
import { UiComponents } from '@/components/ui'
import { createRouter, createMemoryHistory } from 'vue-router'

import UserListView from './UserListView.vue'

vi.mock('@/api/users', () => ({
  fetchUsers: vi.fn().mockResolvedValue({
    list: [],
    total: 0,
    page: 1,
    pageSize: 10
  }),
  createUser: vi.fn(),
  deleteUser: vi.fn(),
  resetUserPassword: vi.fn()
}))

vi.mock('@/api/authorities', () => ({
  fetchAuthorities: vi.fn().mockResolvedValue([
    {
      authorityId: 888,
      authorityName: 'Super Admin',
      parentId: 0,
      defaultRouter: 'dashboard',
      children: [],
      dataAuthorityId: []
    }
  ])
}))

describe('UserListView', () => {
  it('renders user management heading', () => {
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
    const wrapper = mount(UserListView, {
      global: {
        plugins: [UiComponents, router, [VueQueryPlugin, { queryClient }]]
      }
    })

    expect(wrapper.text()).toContain('Users')
  })

  it('opens a new-user dialog from the user page', async () => {
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
    const wrapper = mount(UserListView, {
      global: {
        plugins: [UiComponents, router, [VueQueryPlugin, { queryClient }]]
      }
    })

    await wrapper.find('[data-test="new-user-button"]').trigger('click')

    expect(wrapper.text()).toContain('New user')
    expect(wrapper.text()).toContain('Username')
    expect(wrapper.text()).toContain('Password')
    expect(wrapper.text()).toContain('Role')
    expect(wrapper.text()).toContain('Super Admin')
    expect(wrapper.find('[data-test="user-role-select"]').exists()).toBe(true)
    expect(wrapper.find('[data-test="user-role-select"] select').exists()).toBe(false)
  })
})
