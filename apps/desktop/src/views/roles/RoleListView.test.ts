import { mount } from '@vue/test-utils'
import { UiComponents } from '@/components/ui'
import { describe, expect, it, vi } from 'vitest'

vi.mock('@/api/authorities', () => ({
  fetchAuthorities: vi.fn().mockResolvedValue([
    {
      authorityId: 1,
      authorityName: '开发',
      parentId: 0,
      defaultRouter: 'dashboard',
      children: [],
      dataAuthorityId: []
    }
  ]),
  createAuthority: vi.fn(),
  updateAuthority: vi.fn(),
  deleteAuthority: vi.fn(),
  fetchAuthorityUsers: vi.fn().mockResolvedValue([2]),
  setRoleUsers: vi.fn()
}))

vi.mock('@/api/users', () => ({
  fetchUsers: vi.fn().mockResolvedValue({
    list: [
      { ID: 1, userName: 'admin', nickName: 'admin', phone: '', email: '', enable: 1 },
      { ID: 2, userName: 'nick', nickName: 'nick', phone: '', email: '', enable: 1 }
    ],
    total: 2,
    page: 1,
    pageSize: 10
  })
}))

import RoleListView from './RoleListView.vue'

describe('RoleListView', () => {
  it('renders role management actions', async () => {
    const wrapper = mount(RoleListView, {
      global: {
        plugins: [UiComponents]
      }
    })

    await Promise.resolve()
    expect(wrapper.text()).toContain('Roles')
    expect(wrapper.text()).toContain('New role')
  })

  it('renders a scannable member assignment panel', async () => {
    const wrapper = mount(RoleListView, {
      global: {
        plugins: [UiComponents]
      }
    })

    await Promise.resolve()
    await Promise.resolve()
    await wrapper.find('[data-test="assign-members-button"]').trigger('click')
    await Promise.resolve()
    await Promise.resolve()

    expect(wrapper.find('input[placeholder="Search users"]').exists()).toBe(true)
    expect(wrapper.text()).toContain('Selected members')
    expect(wrapper.text()).toContain('1 / 2')
    expect(wrapper.text()).toContain('admin')
    expect(wrapper.text()).toContain('nick')
  })
})
