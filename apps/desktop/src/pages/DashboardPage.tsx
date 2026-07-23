import { useQueries, useQuery } from '@tanstack/react-query'
import {
  IconActivity,
  IconBookmarks,
  IconBuilding,
  IconStack2,
  IconSettings,
  IconShield,
  IconUsers,
  type Icon,
} from '@tabler/icons-react'
import { useTranslation } from 'react-i18next'
import { Link } from 'react-router-dom'
import { Area, AreaChart, Bar, BarChart, CartesianGrid, ResponsiveContainer, Tooltip, XAxis, YAxis } from 'recharts'

import { fetchAuditStats } from '@/api/audit'
import { listDepartments, type DeptRecord } from '@/api/departments'
import { fetchDictionaries } from '@/api/dictionaries'
import { fetchFiles } from '@/api/files'
import { fetchParams } from '@/api/params'
import { listRoles } from '@/api/roles'
import { fetchUsers } from '@/api/users'
import { PageHeader } from '@/components/PageHeader'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { useAuthStore } from '@/stores/auth'
import { useMenuStore } from '@/stores/menu'

const TOP_N = 10
const TOP_BAR_ROW_HEIGHT = 36

function countDepartments(nodes: DeptRecord[]): number {
  return nodes.reduce((sum, node) => sum + 1 + countDepartments(node.children ?? []), 0)
}

function actionLabel(action: string) {
  switch (action) {
    case 'auth.login':
      return 'Login'
    case 'auth.access_denied':
      return 'Access denied'
    case 'user.assign_roles':
      return 'Assign roles'
    default:
      return action
  }
}

type NamedCount = { name: string; count: number }

function padTopN(items: NamedCount[], n = TOP_N): NamedCount[] {
  if (items.length >= n) return items.slice(0, n)
  return [...items, ...Array.from({ length: n - items.length }, () => ({ name: '—', count: 0 }))]
}

const chartTooltip = {
  background: 'var(--popover)',
  border: '1px solid var(--border)',
  borderRadius: 8,
  fontSize: 12,
}
const chartTickStyle = { fill: 'var(--muted-foreground)', fontSize: 11 }

const RESOURCE_CARDS: Array<{
  key: string
  label: string
  path: string
  icon: Icon
}> = [
  { key: 'users', label: 'Users', path: '/users', icon: IconUsers },
  { key: 'roles', label: 'Roles', path: '/roles', icon: IconShield },
  { key: 'departments', label: 'Departments', path: '/departments', icon: IconBuilding },
  { key: 'files', label: 'Files', path: '/files', icon: IconStack2 },
  { key: 'params', label: 'Params', path: '/params', icon: IconSettings },
  { key: 'dictionaries', label: 'Dictionaries', path: '/dictionaries', icon: IconBookmarks },
  { key: 'audit-events', label: 'Audit events', path: '/audit-events', icon: IconActivity },
]

export function DashboardPage() {
  const { t } = useTranslation()
  const user = useAuthStore((state) => state.userInfo)
  const canAccess = useMenuStore((state) => state.canAccess)
  const menuItems = useMenuStore((state) => state.items)

  const visibleCards = RESOURCE_CARDS.filter((card) => canAccess(card.key))
  const canAudit = canAccess('audit-events')
  const quickLinks = menuItems.filter((item) => item.key !== 'dashboard' && item.key !== 'profile')

  const resourceQueries = useQueries({
    queries: [
      {
        queryKey: ['dashboard', 'users'],
        queryFn: () => fetchUsers(1, 1),
        enabled: canAccess('users'),
      },
      {
        queryKey: ['dashboard', 'roles'],
        queryFn: listRoles,
        enabled: canAccess('roles'),
      },
      {
        queryKey: ['dashboard', 'departments'],
        queryFn: listDepartments,
        enabled: canAccess('departments'),
      },
      {
        queryKey: ['dashboard', 'files'],
        queryFn: () => fetchFiles({ page: 1, pageSize: 1 }),
        enabled: canAccess('files'),
      },
      {
        queryKey: ['dashboard', 'params'],
        queryFn: () => fetchParams({ page: 1, pageSize: 1 }),
        enabled: canAccess('params'),
      },
      {
        queryKey: ['dashboard', 'dictionaries'],
        queryFn: () => fetchDictionaries(),
        enabled: canAccess('dictionaries'),
      },
    ],
  })

  const [usersQ, rolesQ, deptsQ, filesQ, paramsQ, dictsQ] = resourceQueries

  const stats = useQuery({
    queryKey: ['dashboard', 'audit-stats', 14],
    queryFn: () => fetchAuditStats(14),
    enabled: canAudit,
  })

  const totals: Record<string, number | undefined> = {
    users: usersQ.data?.total,
    roles: rolesQ.data?.length,
    departments: deptsQ.data ? countDepartments(deptsQ.data) : undefined,
    files: filesQ.data?.total,
    params: paramsQ.data?.total,
    dictionaries: dictsQ.data?.length,
    'audit-events': stats.data?.eventCount,
  }

  const loadingByKey: Record<string, boolean> = {
    users: usersQ.isLoading,
    roles: rolesQ.isLoading,
    departments: deptsQ.isLoading,
    files: filesQ.isLoading,
    params: paramsQ.isLoading,
    dictionaries: dictsQ.isLoading,
    'audit-events': stats.isLoading,
  }

  const dailyData =
    stats.data?.daily.map((row) => ({
      label: row.date.slice(5),
      logins: row.logins,
      uniqueIps: row.uniqueIps,
    })) ?? []

  const hourlyData =
    stats.data?.byHour.map((row) => ({
      label: `${String(row.hour).padStart(2, '0')}:00`,
      logins: row.logins,
    })) ?? []

  const moduleData = padTopN(
    stats.data?.topActions.map((row) => ({
      name: t(actionLabel(row.name)),
      count: row.count,
    })) ?? [],
  )

  const ipData = padTopN(
    stats.data?.topIps.map((row) => ({
      name: row.name,
      count: row.count,
    })) ?? [],
  )

  const hasModuleData = (stats.data?.topActions.length ?? 0) > 0
  const hasIpData = (stats.data?.topIps.length ?? 0) > 0
  const topChartHeight = TOP_N * TOP_BAR_ROW_HEIGHT

  return (
    <div className="space-y-5 xl:space-y-6">
      <PageHeader
        description={
          <h1 className="text-lg font-semibold text-foreground xl:text-xl">
            {t('Welcome back')}, {user?.nickName || user?.userName}.
          </h1>
        }
      />

      {visibleCards.length > 0 && (
        <div className="grid grid-cols-2 gap-3 sm:grid-cols-3 md:grid-cols-4 xl:grid-cols-4 2xl:grid-cols-7 xl:gap-4">
          {visibleCards.map((card) => {
            const Icon = card.icon
            const value = totals[card.key]
            const loading = loadingByKey[card.key]
            const valueLabel = loading ? '…' : value == null ? '—' : value.toLocaleString()
            return (
              <Link aria-label={`${t(card.label)}: ${valueLabel}`} key={card.key} to={card.path}>
                <Card className="h-full transition-colors hover:border-primary/40 hover:bg-accent/40">
                  <CardContent className="flex items-center gap-3 py-1">
                    <div className="flex size-9 shrink-0 items-center justify-center rounded-lg bg-primary/10 text-primary xl:size-10">
                      <Icon className="size-4 xl:size-5" />
                    </div>
                    <div className="min-w-0">
                      <p className="truncate text-xs text-muted-foreground xl:text-sm">{t(card.label)}</p>
                      <strong className="text-lg font-semibold xl:text-xl">{valueLabel}</strong>
                    </div>
                  </CardContent>
                </Card>
              </Link>
            )
          })}
        </div>
      )}

      {canAudit && (
        <div className="grid grid-cols-1 gap-4 lg:grid-cols-2 xl:gap-5">
          <Card>
            <CardHeader>
              <CardTitle>{t('Daily access')}</CardTitle>
            </CardHeader>
            <CardContent>
              {dailyData.length > 0 ? (
                <ResponsiveContainer className="min-h-[240px] xl:min-h-[280px]" height={240} width="100%">
                  <AreaChart data={dailyData}>
                    <defs>
                      <linearGradient id="dashboardDailyFill" x1="0" x2="0" y1="0" y2="1">
                        <stop offset="0%" stopColor="var(--chart-1)" stopOpacity={0.28} />
                        <stop offset="100%" stopColor="var(--chart-1)" stopOpacity={0.02} />
                      </linearGradient>
                    </defs>
                    <CartesianGrid stroke="var(--border)" strokeDasharray="3 6" vertical={false} />
                    <XAxis axisLine={false} dataKey="label" tick={chartTickStyle} tickLine={false} />
                    <YAxis allowDecimals={false} axisLine={false} tick={chartTickStyle} tickLine={false} width={28} />
                    <Tooltip contentStyle={chartTooltip} />
                    <Area
                      dataKey="logins"
                      fill="url(#dashboardDailyFill)"
                      isAnimationActive={false}
                      name={t('Logins')}
                      stroke="var(--chart-1)"
                      strokeWidth={2}
                      type="monotone"
                    />
                  </AreaChart>
                </ResponsiveContainer>
              ) : (
                <p className="py-8 text-center text-sm text-muted-foreground">{t('No audit events')}</p>
              )}
            </CardContent>
          </Card>
          <Card>
            <CardHeader>
              <CardTitle>{t('Access time analysis')}</CardTitle>
            </CardHeader>
            <CardContent>
              {hourlyData.length > 0 ? (
                <ResponsiveContainer className="min-h-[240px] xl:min-h-[280px]" height={240} width="100%">
                  <AreaChart data={hourlyData}>
                    <defs>
                      <linearGradient id="dashboardHourlyFill" x1="0" x2="0" y1="0" y2="1">
                        <stop offset="0%" stopColor="var(--chart-1)" stopOpacity={0.28} />
                        <stop offset="100%" stopColor="var(--chart-1)" stopOpacity={0.02} />
                      </linearGradient>
                    </defs>
                    <CartesianGrid stroke="var(--border)" strokeDasharray="3 6" vertical={false} />
                    <XAxis axisLine={false} dataKey="label" interval={2} tick={chartTickStyle} tickLine={false} />
                    <YAxis allowDecimals={false} axisLine={false} tick={chartTickStyle} tickLine={false} width={28} />
                    <Tooltip contentStyle={chartTooltip} />
                    <Area
                      dataKey="logins"
                      fill="url(#dashboardHourlyFill)"
                      isAnimationActive={false}
                      name={t('Logins')}
                      stroke="var(--chart-1)"
                      strokeWidth={2}
                      type="monotone"
                    />
                  </AreaChart>
                </ResponsiveContainer>
              ) : (
                <p className="py-8 text-center text-sm text-muted-foreground">{t('No audit events')}</p>
              )}
            </CardContent>
          </Card>
          <Card>
            <CardHeader>
              <CardTitle>{t('Popular modules (Top 10)')}</CardTitle>
            </CardHeader>
            <CardContent>
              {hasModuleData ? (
                <ResponsiveContainer height={topChartHeight} width="100%">
                  <BarChart data={moduleData} layout="vertical" margin={{ left: 8, right: 12 }}>
                    <CartesianGrid horizontal={false} stroke="var(--border)" strokeDasharray="3 6" />
                    <XAxis
                      allowDecimals={false}
                      axisLine={false}
                      tick={chartTickStyle}
                      tickLine={false}
                      type="number"
                    />
                    <YAxis
                      axisLine={false}
                      dataKey="name"
                      tick={chartTickStyle}
                      tickLine={false}
                      type="category"
                      width={72}
                    />
                    <Tooltip contentStyle={chartTooltip} />
                    <Bar
                      barSize={18}
                      dataKey="count"
                      fill="var(--chart-1)"
                      isAnimationActive={false}
                      name={t('Logins')}
                      radius={[0, 4, 4, 0]}
                    />
                  </BarChart>
                </ResponsiveContainer>
              ) : (
                <p className="py-8 text-center text-sm text-muted-foreground">{t('No audit events')}</p>
              )}
            </CardContent>
          </Card>
          <Card>
            <CardHeader>
              <CardTitle>{t('Top source IPs (Top 10)')}</CardTitle>
            </CardHeader>
            <CardContent>
              {hasIpData ? (
                <ResponsiveContainer height={topChartHeight} width="100%">
                  <BarChart data={ipData} layout="vertical" margin={{ left: 8, right: 12 }}>
                    <CartesianGrid horizontal={false} stroke="var(--border)" strokeDasharray="3 6" />
                    <XAxis
                      allowDecimals={false}
                      axisLine={false}
                      tick={chartTickStyle}
                      tickLine={false}
                      type="number"
                    />
                    <YAxis
                      axisLine={false}
                      dataKey="name"
                      tick={chartTickStyle}
                      tickLine={false}
                      type="category"
                      width={96}
                    />
                    <Tooltip contentStyle={chartTooltip} />
                    <Bar
                      barSize={18}
                      dataKey="count"
                      fill="var(--chart-1)"
                      isAnimationActive={false}
                      name={t('Logins')}
                      radius={[0, 4, 4, 0]}
                    />
                  </BarChart>
                </ResponsiveContainer>
              ) : (
                <p className="py-8 text-center text-sm text-muted-foreground">{t('No audit events')}</p>
              )}
            </CardContent>
          </Card>
        </div>
      )}

      {quickLinks.length > 0 && (
        <Card>
          <CardHeader>
            <CardTitle>{t('Quick functions')}</CardTitle>
          </CardHeader>
          <CardContent>
            <div className="grid grid-cols-2 gap-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5">
              {quickLinks.map((item) => {
                const Icon = RESOURCE_CARDS.find((card) => card.key === item.key)?.icon ?? IconActivity
                return (
                  <Link
                    className="flex items-center gap-2 rounded-lg border p-2.5 text-sm transition-colors hover:bg-muted"
                    key={item.key}
                    to={item.path}
                  >
                    <Icon size={16} />
                    <span className="truncate">{t(item.label)}</span>
                  </Link>
                )
              })}
            </div>
          </CardContent>
        </Card>
      )}
    </div>
  )
}
