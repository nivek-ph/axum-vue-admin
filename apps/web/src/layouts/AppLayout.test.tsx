import { cleanup, render, screen } from '@testing-library/react'
import { afterEach, beforeEach, describe, expect, it } from 'vitest'
import { MemoryRouter, Route, Routes } from 'react-router-dom'

import i18n from '@/i18n'
import { useAuthStore } from '@/stores/auth'
import { useMenuStore } from '@/stores/menu'

import { AppLayout } from './AppLayout'

describe('AppLayout sidebar', () => {
  beforeEach(async () => {
    useAuthStore.getState().setSession({
      accessToken: 'token',
      refreshToken: 'refresh',
      userInfo: { id: 1, userName: 'admin', nickName: 'Admin', roles: [{ id: 1, code: 'super_admin', name: 'Super Admin' }] },
    })
    useMenuStore.getState().resetAccess()
    await i18n.changeLanguage('zh-CN')
  })

  afterEach(async () => {
    cleanup()
    useAuthStore.getState().clearSession()
    await i18n.changeLanguage('en-US')
  })

  it('always shows the complete Chinese navigation without a collapse control', () => {
    render(
      <MemoryRouter initialEntries={['/dashboard']}>
        <Routes>
          <Route element={<AppLayout />}>
            <Route path="/dashboard" element={<div>Dashboard content</div>} />
          </Route>
        </Routes>
      </MemoryRouter>,
    )

    expect(screen.getByText('核心管理')).toBeVisible()
    expect(screen.getByRole('link', { name: '用户管理' })).toBeVisible()
    expect(screen.getByRole('link', { name: '角色管理' })).toBeVisible()
    expect(screen.queryByRole('button', { name: 'Toggle sidebar' })).not.toBeInTheDocument()
  })
})
