import { mount } from '@vue/test-utils'
import { describe, expect, it } from 'vitest'
import { ElementCompat } from '@/ui/elementCompat'
import { createRouter, createMemoryHistory } from 'vue-router'

import UserListView from './UserListView.vue'

describe('UserListView', () => {
  it('renders user management heading', () => {
    const router = createRouter({
      history: createMemoryHistory(),
      routes: [{ path: '/', component: UserListView }]
    })
    const wrapper = mount(UserListView, {
      global: {
        plugins: [ElementCompat, router]
      }
    })

    expect(wrapper.text()).toContain('用户管理')
  })
})
