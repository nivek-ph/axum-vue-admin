import {
  IconActivity,
  IconBookmarks,
  IconBuilding,
  IconLayoutDashboard,
  IconLayoutSidebar,
  IconLayoutSidebarLeftCollapse,
  IconLogout,
  IconSettings,
  IconShield,
  IconShieldCheck,
  IconStack2,
  IconUser,
  IconUsers,
  type Icon,
} from '@tabler/icons-react'
import { useState } from 'react'
import { useTranslation } from 'react-i18next'
import { NavLink, Outlet, useNavigate } from 'react-router-dom'
import { toast } from 'sonner'

import { logout } from '@/api/auth'
import { BrandMark } from '@/components/BrandMark'
import { LanguageSwitch } from '@/components/LanguageSwitch'
import { ThemeSwitch } from '@/components/ThemeSwitch'
import { Button, buttonVariants } from '@/components/ui/Button'
import { ScrollArea } from '@/components/ui/scroll-area'
import { Separator } from '@/components/ui/separator'
import { Tooltip, TooltipContent, TooltipProvider, TooltipTrigger } from '@/components/ui/tooltip'
import { cn } from '@/lib/utils'
import { useAuthStore } from '@/stores/auth'
import { useMenuStore } from '@/stores/menu'

const SIDEBAR_COLLAPSED_KEY = 'ava.sidebarCollapsed'

const MENU_ICONS: Record<string, Icon> = {
  dashboard: IconLayoutDashboard,
  users: IconUsers,
  roles: IconShield,
  departments: IconBuilding,
  menus: IconShieldCheck,
  params: IconSettings,
  dictionaries: IconBookmarks,
  files: IconStack2,
  'audit-events': IconActivity,
  profile: IconUser,
}

export function AppLayout() {
  const { t } = useTranslation()
  const navigate = useNavigate()
  const items = useMenuStore((state) => state.items)
  const user = useAuthStore((state) => state.userInfo)
  const [collapsed, setCollapsed] = useState(() => window.localStorage.getItem(SIDEBAR_COLLAPSED_KEY) === '1')

  function toggleSidebar() {
    setCollapsed((current) => {
      const next = !current
      window.localStorage.setItem(SIDEBAR_COLLAPSED_KEY, next ? '1' : '0')
      return next
    })
  }

  async function signOut() {
    try {
      await logout()
    } catch {
      toast.warning(t('Server session may still be active'))
    }
    useAuthStore.getState().clearSession()
    useMenuStore.getState().resetAccess()
    navigate('/login', { replace: true })
  }

  return (
    <TooltipProvider delay={200}>
      <div className="flex min-h-svh flex-col bg-background">
        <header className="sticky top-0 z-30 flex h-12 items-center justify-between gap-3 border-b border-border bg-background/95 px-4 backdrop-blur xl:h-14 xl:px-5">
          <div className="flex items-center gap-2.5">
            <BrandMark size="small" />
            <strong className="text-sm font-semibold tracking-tight xl:text-base">{t('Admin Console')}</strong>
          </div>
          <div className="flex items-center gap-1.5">
            <ThemeSwitch />
            <LanguageSwitch />
            <NavLink className={cn(buttonVariants({ size: 'sm', variant: 'ghost' }), 'text-sm')} to="/profile">
              {user?.nickName || user?.userName}
            </NavLink>
            <Button className="text-sm" onClick={() => void signOut()} size="sm" variant="ghost">
              <IconLogout data-icon="inline-start" />
              {t('Sign out')}
            </Button>
          </div>
        </header>

        <div className="flex min-h-0 flex-1">
          <aside
            className={cn(
              'flex shrink-0 flex-col border-r border-sidebar-border bg-sidebar text-sidebar-foreground transition-[width] duration-150',
              collapsed ? 'w-14' : 'w-44',
            )}
          >
            <ScrollArea className="flex-1 px-2.5 py-3">
              <nav aria-label="Main navigation" className="flex flex-col gap-1">
                {items.map((item) => {
                  const Icon = MENU_ICONS[item.key] ?? IconLayoutDashboard
                  const label = t(item.label)
                  const link = (
                    <NavLink
                      aria-label={label}
                      className={({ isActive }) =>
                        cn(
                          'flex items-center gap-2.5 rounded-lg px-2.5 py-2 text-sm text-sidebar-foreground transition-colors hover:bg-sidebar-accent hover:text-sidebar-accent-foreground',
                          collapsed && 'justify-center px-0',
                          isActive && 'bg-sidebar-accent font-medium text-sidebar-accent-foreground',
                        )
                      }
                      to={item.path}
                    >
                      <Icon className="size-4 shrink-0" />
                      {!collapsed ? <span className="truncate">{label}</span> : null}
                    </NavLink>
                  )

                  if (!collapsed) return <div key={item.key}>{link}</div>
                  return (
                    <Tooltip key={item.key}>
                      <TooltipTrigger render={link} />
                      <TooltipContent side="right">{label}</TooltipContent>
                    </Tooltip>
                  )
                })}
              </nav>
            </ScrollArea>

            <Separator />
            <div className="p-2.5">
              <Button
                aria-expanded={!collapsed}
                aria-label={collapsed ? t('Expand') : t('Collapse')}
                className={cn('w-full', collapsed && 'px-0')}
                onClick={toggleSidebar}
                size="sm"
                variant="ghost"
              >
                {collapsed ? <IconLayoutSidebar /> : <IconLayoutSidebarLeftCollapse />}
                {!collapsed ? <span>{collapsed ? t('Expand') : t('Collapse')}</span> : null}
              </Button>
            </div>
          </aside>

          <main className="min-w-0 flex-1 overflow-auto p-4 md:p-5 xl:p-6">
            <Outlet />
          </main>
        </div>
      </div>
    </TooltipProvider>
  )
}
