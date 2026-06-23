import { describe, expect, it } from 'vitest'

import { queryClient } from './query'

describe('queryClient', () => {
  it('uses admin-friendly defaults for cached server state', () => {
    const defaults = queryClient.getDefaultOptions()

    expect(defaults.queries?.staleTime).toBe(30_000)
    expect(defaults.queries?.retry).toBe(1)
  })
})
