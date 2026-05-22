import { describe, expect, it } from 'vitest'

import { normalizeDictionaryDetailTreeResponse, normalizeDictionaryListResponse } from './dictionaries'

describe('dictionaries api adapter', () => {
  it('normalizes dictionary list payload', () => {
    const result = normalizeDictionaryListResponse({
      data: [{ ID: 1, name: '状态', type: 'status', desc: '' }]
    })

    expect(result).toHaveLength(1)
  })

  it('normalizes dictionary detail tree payload', () => {
    const result = normalizeDictionaryDetailTreeResponse({
      data: {
        list: [{ ID: 1, label: '启用', value: 'enabled', children: [] }]
      }
    })

    expect(result).toHaveLength(1)
  })
})
