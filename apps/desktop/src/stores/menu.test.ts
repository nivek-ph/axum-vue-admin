import { describe, expect, it } from 'vitest'

import { buildCoreMenuItems } from './menu'

describe('menu store helpers', () => {
  it('keeps core menu coverage while applying backend labels and paths', () => {
    const items = buildCoreMenuItems([
      {
        name: 'users',
        path: 'system/users',
        meta: { title: 'Users' }
      },
      {
        name: 'dashboard',
        path: 'home',
        meta: { title: 'Workbench' }
      }
    ])

    expect(items).toContainEqual({
      key: 'dashboard',
      label: 'Workbench',
      path: '/home'
    })
    expect(items).toContainEqual({
      key: 'users',
      label: 'Users',
      path: '/system/users'
    })
    expect(items.some((item) => item.key === 'roles')).toBe(true)
  })
})
