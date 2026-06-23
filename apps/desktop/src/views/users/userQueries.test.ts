import { describe, expect, it } from 'vitest'

import { userQueryKeys } from './userQueries'

describe('userQueryKeys', () => {
  it('keeps list query keys stable and filter-aware', () => {
    expect(userQueryKeys.list(1, 10)).toEqual(['users', 'list', { page: 1, pageSize: 10 }])
  })
})
