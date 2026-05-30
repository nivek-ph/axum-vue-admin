import { mount } from '@vue/test-utils'
import { UiComponents } from '@/components/ui'
import { describe, expect, it, vi } from 'vitest'

vi.mock('@/api/logs', () => ({
  fetchLoginLogs: vi.fn().mockResolvedValue({
    list: [],
    total: 0,
    page: 1,
    pageSize: 10
  }),
  deleteLoginLog: vi.fn()
}))

import LoginLogView from './LoginLogView.vue'

describe('LoginLogView', () => {
  it('renders login log heading', async () => {
    const wrapper = mount(LoginLogView, {
      global: {
        plugins: [UiComponents]
      }
    })

    await Promise.resolve()
    expect(wrapper.text()).toContain('Login logs')
  })
})
