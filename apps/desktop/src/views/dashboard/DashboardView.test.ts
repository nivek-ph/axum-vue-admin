import { mount } from '@vue/test-utils'
import { createPinia, setActivePinia } from 'pinia'
import { createMemoryHistory, createRouter } from 'vue-router'
import { describe, expect, it } from 'vitest'

import { UiComponents } from '@/components/ui'
import { useAuthStore } from '@/stores/auth'
import { useMenuStore } from '@/stores/menu'
import DashboardView from './DashboardView.vue'

describe('DashboardView', () => {
  it('hides shortcut entries that are not in the current role menu', () => {
    setActivePinia(createPinia())
    const authStore = useAuthStore()
    const menuStore = useMenuStore()
    authStore.setSession('token-123', {
      ID: 1,
      userName: 'operator',
      nickName: 'Operator',
      authority: {
        authorityId: 999,
        authorityName: 'Operator',
        defaultRouter: 'dashboard'
      }
    })
    menuStore.setAuthorizedMenus([{ name: 'dashboard', path: 'dashboard', meta: { title: 'Dashboard' } }])
    const router = createRouter({
      history: createMemoryHistory(),
      routes: [{ path: '/', component: DashboardView }]
    })

    const wrapper = mount(DashboardView, {
      global: {
        plugins: [UiComponents, router]
      }
    })

    expect(wrapper.findAll('.shortcut-card')).toHaveLength(0)
    expect(wrapper.findAll('.page-hero-actions button')).toHaveLength(0)
  })
})
