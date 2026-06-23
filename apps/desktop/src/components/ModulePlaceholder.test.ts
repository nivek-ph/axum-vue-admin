import { mount } from '@vue/test-utils'
import { createPinia, setActivePinia } from 'pinia'
import { createMemoryHistory, createRouter } from 'vue-router'
import { describe, expect, it } from 'vitest'

import { UiComponents } from '@/components/ui'
import { useMenuStore } from '@/stores/menu'
import ModulePlaceholder from './ModulePlaceholder.vue'

describe('ModulePlaceholder', () => {
  it('hides the primary action when the current role cannot access its route', () => {
    setActivePinia(createPinia())
    const menuStore = useMenuStore()
    menuStore.setAuthorizedMenus([{ name: 'profile', path: 'profile', meta: { title: 'Profile' } }])
    const router = createRouter({
      history: createMemoryHistory(),
      routes: [{ path: '/', component: ModulePlaceholder }]
    })

    const wrapper = mount(ModulePlaceholder, {
      props: {
        kicker: 'Core',
        title: 'Dashboard',
        description: 'Description',
        note: 'Note',
        keeps: [],
        skips: []
      },
      global: {
        plugins: [UiComponents, router]
      }
    })

    expect(wrapper.text()).not.toContain('Back to dashboard')
  })
})
