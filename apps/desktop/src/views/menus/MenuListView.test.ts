import { mount } from '@vue/test-utils'
import { ElementCompat } from '@/ui/elementCompat'
import { describe, expect, it, vi } from 'vitest'

vi.mock('@/api/authorities', () => ({
  fetchAuthorities: vi.fn().mockResolvedValue([])
}))

vi.mock('@/api/menus', () => ({
  fetchMenuList: vi.fn().mockResolvedValue([]),
  createMenu: vi.fn(),
  updateMenu: vi.fn(),
  deleteMenu: vi.fn(),
  fetchMenuRoles: vi.fn().mockResolvedValue({
    authorityIds: [],
    defaultRouterAuthorityIds: []
  }),
  setMenuRoles: vi.fn()
}))

import MenuListView from './MenuListView.vue'

describe('MenuListView', () => {
  it('renders menu management actions', async () => {
    const wrapper = mount(MenuListView, {
      global: {
        plugins: [ElementCompat]
      }
    })

    await Promise.resolve()
    expect(wrapper.text()).toContain('菜单管理')
    expect(wrapper.text()).toContain('新增菜单')
  })
})
