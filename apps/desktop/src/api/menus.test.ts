import { describe, expect, it } from 'vitest'

import { normalizeMenuListResponse, normalizeMenuRoleSelection } from './menus'

describe('menu api adapter', () => {
  it('normalizes backend menu tree payload', () => {
    const result = normalizeMenuListResponse({
      data: [
        {
          ID: 1,
          name: 'dashboard',
          meta: { title: '仪表盘' },
          children: [{ ID: 2, name: 'users', meta: { title: '用户' } }]
        }
      ]
    })

    expect(result).toHaveLength(1)
    expect(result[0].children).toHaveLength(1)
  })

  it('normalizes assigned role ids', () => {
    const result = normalizeMenuRoleSelection({
      data: {
        authorityIds: [888],
        defaultRouterAuthorityIds: [888]
      }
    })

    expect(result.authorityIds).toEqual([888])
    expect(result.defaultRouterAuthorityIds).toEqual([888])
  })
})
