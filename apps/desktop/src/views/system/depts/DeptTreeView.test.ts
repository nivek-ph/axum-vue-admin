import { flushPromises, mount } from '@vue/test-utils'
import { describe, expect, it, vi } from 'vitest'
import { UiComponents } from '@/components/ui'

vi.mock('@/api/system/depts', () => ({
  listDepts: vi.fn().mockResolvedValue([
    {
      id: 1,
      parent_id: null,
      name: 'Head Office',
      code: 'head_office',
      sort: 0,
      status: 'enabled',
      children: []
    }
  ]),
  createDept: vi.fn(),
  updateDept: vi.fn(),
  deleteDept: vi.fn()
}))

import DeptTreeView from './DeptTreeView.vue'

describe('DeptTreeView', () => {
  it('renders the department tree workspace', async () => {
    const wrapper = mount(DeptTreeView, {
      global: {
        plugins: [UiComponents]
      }
    })

    await flushPromises()

    expect(wrapper.text()).toContain('Departments')
    expect(wrapper.text()).toContain('Head Office')
    expect(wrapper.text()).toContain('head_office')
  })
})
