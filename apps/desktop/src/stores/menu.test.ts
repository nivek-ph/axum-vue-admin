import { describe, expect, it } from 'vitest'

import { buildCoreMenuItems } from './menu'

describe('menu store helpers', () => {
  it('keeps core menu coverage while applying backend labels and paths', () => {
    const items = buildCoreMenuItems([
      {
        name: 'users',
        path: 'system/users',
        meta: { title: '用户管理' }
      },
      {
        name: 'dashboard',
        path: 'home',
        meta: { title: '工作台' }
      }
    ])

    expect(items).toContainEqual({
      key: 'dashboard',
      label: '工作台',
      path: '/home'
    })
    expect(items).toContainEqual({
      key: 'users',
      label: '用户管理',
      path: '/system/users'
    })
    expect(items.some((item) => item.key === 'roles')).toBe(true)
  })
})
