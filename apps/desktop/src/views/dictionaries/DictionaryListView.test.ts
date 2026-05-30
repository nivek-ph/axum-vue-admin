import { mount } from '@vue/test-utils'
import { UiComponents } from '@/components/ui'
import { describe, expect, it, vi } from 'vitest'

vi.mock('@/api/dictionaries', () => ({
  fetchDictionaries: vi.fn().mockResolvedValue([]),
  fetchDictionaryDetails: vi.fn().mockResolvedValue([]),
  createDictionary: vi.fn(),
  updateDictionary: vi.fn(),
  deleteDictionary: vi.fn()
}))

import DictionaryListView from './DictionaryListView.vue'

describe('DictionaryListView', () => {
  it('renders dictionaries heading', async () => {
    const wrapper = mount(DictionaryListView, {
      global: {
        plugins: [UiComponents]
      }
    })

    await Promise.resolve()
    expect(wrapper.text()).toContain('Dictionary management')
  })
})
