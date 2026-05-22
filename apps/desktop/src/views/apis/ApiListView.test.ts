import { mount } from '@vue/test-utils'
import { ElementCompat } from '@/ui/elementCompat'
import { describe, expect, it, vi } from 'vitest'

vi.mock('@/api/apis', () => ({
  fetchApiGroups: vi.fn().mockResolvedValue([]),
  fetchApis: vi.fn().mockResolvedValue({
    list: [],
    total: 0,
    page: 1,
    pageSize: 10
  }),
  createApi: vi.fn(),
  updateApi: vi.fn(),
  deleteApi: vi.fn(),
  fetchApiRoles: vi.fn().mockResolvedValue({
    authorityIds: []
  }),
  setApiRoles: vi.fn()
}))

vi.mock('@/api/authorities', () => ({
  fetchAuthorities: vi.fn().mockResolvedValue([])
}))

import ApiListView from './ApiListView.vue'

describe('ApiListView', () => {
  it('renders api management actions', async () => {
    const wrapper = mount(ApiListView, {
      global: {
        plugins: [ElementCompat]
      }
    })

    await Promise.resolve()
    expect(wrapper.text()).toContain('API 管理')
    expect(wrapper.text()).toContain('新增 API')
  })
})
