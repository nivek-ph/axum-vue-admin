import type { AxiosAdapter } from 'axios'
import { QueryClientProvider } from '@tanstack/react-query'
import { render, screen, waitFor } from '@testing-library/react'
import { MemoryRouter } from 'react-router-dom'
import { afterEach, beforeEach, describe, expect, it } from 'vitest'

import { http } from '@/api/http'
import { createQueryClient } from '@/lib/query'
import { DashboardPage } from '@/pages/DashboardPage'
import { useAuthStore } from '@/stores/auth'
import { useMenuStore } from '@/stores/menu'

describe('Dashboard page', () => {
  const originalAdapter = http.defaults.adapter

  beforeEach(() => {
    useAuthStore.getState().clearSession()
    useMenuStore.getState().resetAccess()
  })

  afterEach(() => {
    http.defaults.adapter = originalAdapter
  })

  it('loads resource totals and audit visit stats from the API', async () => {
    useAuthStore.getState().setSession({
      accessToken: 'token',
      refreshToken: 'refresh',
      userInfo: {
        id: 1,
        userName: 'admin',
        nickName: 'Admin',
        roles: [{ id: 1, code: 'super_admin', name: 'Super Admin' }],
      },
    })
    useMenuStore.getState().setAuthorizedMenus([], true)
    http.defaults.adapter = (async (config) => {
      const url = config.url ?? ''
      let data: unknown
      if (url === '/users') data = { code: 'OK', message: 'ok', data: { list: [], total: 12, page: 1, pageSize: 1 } }
      else if (url === '/roles') data = { code: 'OK', message: 'ok', data: { list: [{ id: 1 }, { id: 2 }] } }
      else if (url === '/depts')
        data = { code: 'OK', message: 'ok', data: { list: [{ id: 1, children: [{ id: 2 }] }] } }
      else if (url === '/files')
        data = { code: 'OK', message: 'ok', data: { list: [], total: 4, page: 1, pageSize: 1 } }
      else if (url === '/params')
        data = { code: 'OK', message: 'ok', data: { list: [], total: 6, page: 1, pageSize: 1 } }
      else if (url === '/dictionaries') data = { code: 'OK', message: 'ok', data: [{ id: 1 }, { id: 2 }, { id: 3 }] }
      else if (url === '/audit/events/stats')
        data = {
          code: 'OK',
          message: 'ok',
          data: {
            days: 14,
            loginCount: 5,
            uniqueIps: 3,
            eventCount: 9,
            daily: [{ date: '2026-07-22', logins: 2, uniqueIps: 1 }],
            byHour: Array.from({ length: 24 }, (_, hour) => ({ hour, logins: hour === 10 ? 2 : 0 })),
            topActions: [{ name: 'auth.login', count: 5 }],
            topIps: [{ name: '127.0.0.1', count: 4 }],
          },
        }
      else throw new Error(`Unexpected request: ${config.method} ${url}`)
      return { data, status: 200, statusText: 'OK', headers: {}, config }
    }) as AxiosAdapter

    render(
      <QueryClientProvider client={createQueryClient()}>
        <MemoryRouter>
          <DashboardPage />
        </MemoryRouter>
      </QueryClientProvider>,
    )

    expect(screen.getByRole('heading', { name: 'Welcome back, Admin.' })).toBeInTheDocument()
    await waitFor(() => {
      expect(screen.getByRole('link', { name: 'Users: 12' })).toHaveAttribute('href', '/users')
    })
    expect(screen.getByRole('link', { name: 'Roles: 2' })).toHaveAttribute('href', '/roles')
    expect(await screen.findByText('Daily access')).toBeInTheDocument()
    expect(screen.getByText('Access time analysis')).toBeInTheDocument()
    expect(screen.getByText('Popular modules (Top 10)')).toBeInTheDocument()
    expect(screen.getByText('Top source IPs (Top 10)')).toBeInTheDocument()
    expect(screen.queryByText('Recent audit events')).not.toBeInTheDocument()
  })
})
