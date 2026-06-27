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
  createPermission: vi.fn(),
  updatePermission: vi.fn(),
  deletePermission: vi.fn()
}))

import PermissionListView from './PermissionListView.vue'

describe('PermissionListView', () => {
  it('renders permission resources', async () => {
    const wrapper = mount(PermissionListView, {
      global: {
        plugins: [UiComponents]
      }
    })

    await flushPromises()

    expect(wrapper.text()).toContain('Permissions')
    expect(wrapper.text()).toContain('system:user:list')
    expect(wrapper.text()).toContain('List Users')
  })
})
