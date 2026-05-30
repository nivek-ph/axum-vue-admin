import { mount } from '@vue/test-utils'
import { UiComponents } from '@/components/ui'
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
        plugins: [UiComponents]
      }
    })

    await Promise.resolve()
    expect(wrapper.text()).toContain('Param management')
  })
})
