import { describe, expect, it } from 'vitest'

import { normalizeServerInfoResponse, normalizeSystemConfigResponse } from './system'

describe('system api adapter', () => {
  it('normalizes system config payload', () => {
    const result = normalizeSystemConfigResponse({
      data: {
        config: {
          system: { env: 'public', addr: '127.0.0.1:3000' }
        }
      }
    })

    expect(result.system.env).toBe('public')
  })

  it('normalizes server info payload', () => {
    const result = normalizeServerInfoResponse({
      data: {
        server: {
          os: { goos: 'macos', numCpu: 8 },
          cpu: { cores: 8, cpus: [12, 18] },
          ram: { totalMb: 8192, usedMb: 2048, usedPercent: 25 },
          disk: [{ mountPoint: '/', totalGb: 500, usedGb: 125, usedPercent: 25 }]
        }
      }
    })

    expect(result.os.goos).toBe('macos')
    expect(result.disk).toHaveLength(1)
  })
})
