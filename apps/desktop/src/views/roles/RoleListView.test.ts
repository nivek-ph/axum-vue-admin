import { mount } from '@vue/test-utils'
import { ElementCompat } from '@/ui/elementCompat'
import { describe, expect, it, vi } from 'vitest'

vi.mock('@/api/authorities', () => ({
  fetchAuthorities: vi.fn().mockResolvedValue([]),
  createAuthority: vi.fn(),
  updateAuthority: vi.fn(),
  deleteAuthority: vi.fn(),
  fetchAuthorityUsers: vi.fn().mockResolvedValue([]),
  setRoleUsers: vi.fn()
}))

vi.mock('@/api/users', () => ({
  fetchUsers: vi.fn().mockResolvedValue({
    list: [],
    total: 0,
    page: 1,
    pageSize: 10
  })
}))

import RoleListView from './RoleListView.vue'

describe('RoleListView', () => {
  it('renders role management actions', async () => {
    const wrapper = mount(RoleListView, {
      global: {
        plugins: [ElementCompat]
      }
    })

    await Promise.resolve()
    expect(wrapper.text()).toContain('角色管理')
    expect(wrapper.text()).toContain('新增角色')
  })
})
