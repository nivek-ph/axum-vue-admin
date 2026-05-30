import { mount } from '@vue/test-utils'
import { UiComponents } from '@/components/ui'
import { describe, expect, it, vi } from 'vitest'
import { createMemoryHistory, createRouter } from 'vue-router'

vi.mock('@/api/system', () => ({
  fetchServerInfo: vi.fn(),
  fetchSystemConfig: vi.fn().mockResolvedValue({
    system: { env: 'public', addr: '127.0.0.1:3000', 'db-type': 'pgsql' },
    captcha: { openCaptcha: 0, openCaptchaTimeOut: 0 },
    local: { storePath: './uploads' }
  })
}))

import SystemConfigView from './SystemConfigView.vue'

describe('SystemConfigView', () => {
  it('renders system config heading', async () => {
    const router = createRouter({
      history: createMemoryHistory(),
      routes: [{ path: '/', component: SystemConfigView }]
    })

    const wrapper = mount(SystemConfigView, {
      global: {
        plugins: [UiComponents, router]
      }
    })

    await Promise.resolve()
    expect(wrapper.text()).toContain('System config')
  })
})
