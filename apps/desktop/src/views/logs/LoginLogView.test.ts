import { mount } from '@vue/test-utils'
import { ElementCompat } from '@/ui/elementCompat'
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
        plugins: [ElementCompat]
      }
    })

    await Promise.resolve()
    expect(wrapper.text()).toContain('登录日志')
  })
})
