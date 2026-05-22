import { mount } from '@vue/test-utils'
import { ElementCompat } from '@/ui/elementCompat'
import { describe, expect, it, vi } from 'vitest'

vi.mock('@/api/params', () => ({
  fetchParams: vi.fn().mockResolvedValue({
    list: [],
    total: 0,
    page: 1,
    pageSize: 10
  }),
  createParam: vi.fn(),
  updateParam: vi.fn(),
  deleteParam: vi.fn()
}))

import ParamListView from './ParamListView.vue'

describe('ParamListView', () => {
  it('renders params heading', async () => {
    const wrapper = mount(ParamListView, {
      global: {
        plugins: [ElementCompat]
      }
    })

    await Promise.resolve()
    expect(wrapper.text()).toContain('参数管理')
  })
})
