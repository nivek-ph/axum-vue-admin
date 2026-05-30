import { mount } from '@vue/test-utils'
import { UiComponents } from '@/components/ui'
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
        plugins: [UiComponents]
      }
    })

    await Promise.resolve()
    expect(wrapper.text()).toContain('Menus')
    expect(wrapper.text()).toContain('New menu')
  })
})
