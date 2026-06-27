import { flushPromises, mount } from '@vue/test-utils'
import { describe, expect, it, vi } from 'vitest'
import { UiComponents } from '@/components/ui'

vi.mock('@/api/system/permissions', () => ({
  listPermissions: vi.fn().mockResolvedValue([
    {
      id: 1,
      module_key: 'system',
      resource: 'user',
      action: 'list',
      code: 'system:user:list',
      name: 'List Users',
      type: 'action',
      status: 'enabled'
    }
  ]),
  listPermissionApis: vi.fn().mockResolvedValue([
    { method: 'GET', path_pattern: '/api/users' }
  ]),
  setPermissionApis: vi.fn()
}))

import ApiPermissionListView from './ApiPermissionListView.vue'

describe('ApiPermissionListView', () => {
  it('renders API bindings for the selected permission', async () => {
    const wrapper = mount(ApiPermissionListView, {
      global: {
        plugins: [UiComponents]
      }
    })

    await flushPromises()

    expect(wrapper.text()).toContain('API permission bindings')
    expect(wrapper.text()).toContain('List Users')
    const inputValues = wrapper.findAll('input').map((input) => (input.element as HTMLInputElement).value)
    expect(inputValues).toContain('/api/users')
  })
})
