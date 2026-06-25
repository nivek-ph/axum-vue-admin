import { mount } from '@vue/test-utils'
import { UiComponents } from '@/components/ui'
import { describe, expect, it, vi } from 'vitest'

vi.mock('@/api/logs', () => ({
  fetchLoginLogs: vi.fn(),
  deleteLoginLog: vi.fn(),
  fetchOperationLogs: vi.fn().mockResolvedValue({
    list: [],
    total: 0,
    page: 1,
    pageSize: 10
  })
}))

import OperationLogView from './OperationLogView.vue'

describe('OperationLogView', () => {
  it('renders operation log heading', async () => {
    const wrapper = mount(OperationLogView, {
      global: {
        plugins: [UiComponents]
      }
    })

    await Promise.resolve()
    expect(wrapper.text()).toContain('Operation logs')
    expect(wrapper.find('.page-panel-actions').text()).not.toContain('Search')
    expect(wrapper.find('.operation-log-filter').text()).toContain('Search')
  })
})
