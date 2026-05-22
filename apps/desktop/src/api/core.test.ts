import { describe, expect, it } from 'vitest'

import { bearerAuthorization } from './core'

describe('bearerAuthorization', () => {
  it('prefixes non-empty tokens with Bearer', () => {
    expect(bearerAuthorization('abc.def.ghi')).toBe('Bearer abc.def.ghi')
  })

  it('returns empty string for blank tokens', () => {
    expect(bearerAuthorization('')).toBe('')
    expect(bearerAuthorization('   ')).toBe('')
  })
})
