import { mount } from '@vue/test-utils'
import { ElementCompat } from '@/ui/elementCompat'
import { describe, expect, it, vi } from 'vitest'

vi.mock('@/api/files', () => ({
  fetchFiles: vi.fn().mockResolvedValue({
    list: [],
    total: 0,
    page: 1,
    pageSize: 10
  }),
  fetchCategories: vi.fn().mockResolvedValue([]),
  saveCategory: vi.fn(),
  deleteCategory: vi.fn(),
  importFileUrl: vi.fn(),
  renameFile: vi.fn(),
  deleteFile: vi.fn()
}))

import FileLibraryView from './FileLibraryView.vue'

describe('FileLibraryView', () => {
  it('renders file library heading', async () => {
    const wrapper = mount(FileLibraryView, {
      global: {
        plugins: [ElementCompat]
      }
    })

    await Promise.resolve()
    expect(wrapper.text()).toContain('文件管理')
  })
})
