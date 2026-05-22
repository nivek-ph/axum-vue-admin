import { describe, expect, it } from 'vitest'

import { normalizeLoginLogListResponse, normalizeOperationLogListResponse } from './logs'

describe('logs api adapter', () => {
  it('normalizes login log payload', () => {
    const result = normalizeLoginLogListResponse({
      data: {
        list: [{ ID: 1, username: 'admin', status: true }],
        total: 1,
        page: 1,
        pageSize: 10
      }
    })

    expect(result.list).toHaveLength(1)
    expect(result.total).toBe(1)
  })

  it('normalizes operation log payload', () => {
    const result = normalizeOperationLogListResponse({
      data: {
        list: [{ ID: 1, method: 'POST', path: '/api/auth/login', status: 200 }],
        total: 1,
        page: 1,
        pageSize: 10
      }
    })

    expect(result.list).toHaveLength(1)
    expect(result.list[0].method).toBe('POST')
  })
})
