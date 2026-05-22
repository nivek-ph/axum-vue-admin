import { describe, expect, it } from 'vitest'

import { normalizeCategoryListResponse, normalizeFileListResponse } from './files'

describe('files api adapter', () => {
  it('normalizes file list payload', () => {
    const result = normalizeFileListResponse({
      data: {
        list: [{ ID: 1, name: 'logo.png', url: '/uploads/logo.png', tag: 'png', classId: 0 }],
        total: 1,
        page: 1,
        pageSize: 10
      }
    })

    expect(result.list).toHaveLength(1)
  })

  it('normalizes category tree payload', () => {
    const result = normalizeCategoryListResponse({
      data: [{ ID: 1, name: '图片', pid: 0, children: [] }]
    })

    expect(result).toHaveLength(1)
  })
})
