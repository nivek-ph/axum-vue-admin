import { LogOut } from 'lucide-react'
import { useTranslation } from 'react-i18next'
import { NavLink, Outlet, useNavigate } from 'react-router-dom'
import { toast } from 'sonner'

import { logout } from '@/api/auth'
import { LanguageSwitch } from '@/components/LanguageSwitch'
import { Button } from '@/components/ui/Button'
import { useAuthStore } from '@/stores/auth'
import { useMenuStore } from '@/stores/menu'

export function AppLayout() {
  const { t } = useTranslation()
  const navigate = useNavigate()
  const items = useMenuStore((state) => state.items)
  const user = useAuthStore((state) => state.userInfo)

  async function signOut() {
    try { await logout() } catch { toast.warning(t('Server session may still be active')) }
    useAuthStore.getState().clearSession()
    useMenuStore.getState().resetAccess()
    navigate('/login', { replace: true })
  }

  return (
    <div className="app-shell">
      <aside className="sidebar">
        <div className="sidebar-brand"><div className="brand-mark small">A</div><span>{t('Admin Console')}</span></div>
        <div className="sidebar-section">
          <div className="sidebar-caption">{t('Core')}</div>
          <nav aria-label="Main navigation">
            {items.map((item) => <NavLink key={item.key} to={item.path}><span className="nav-bullet" /><span className="nav-label">{t(item.label)}</span></NavLink>)}
          </nav>
        </div>
      </aside>
      <div className="app-main">
        <header className="topbar">
          <div className="topbar-actions">
            <LanguageSwitch />
            <NavLink className="user-chip" to="/profile"><span>{user?.nickName || user?.userName}</span></NavLink>
            <Button onClick={() => void signOut()} variant="ghost"><LogOut size={16} />{t('Sign out')}</Button>
          </div>
        </header>
        <main className="content"><Outlet /></main>
      </div>
    </div>
  )
}
