import { getCoreRowModel, useReactTable, type ColumnDef } from '@tanstack/react-table'
import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query'
import { IconKey, IconPlus, IconRefresh, IconSearch, IconShield, IconTrash } from '@tabler/icons-react'
import { useMemo, useState } from 'react'
import { useTranslation } from 'react-i18next'
import { toast } from 'sonner'

import { listRoles } from '@/api/roles'
import {
  assignUserRoles,
  createUser,
  deleteUser,
  fetchUsers,
  resetUserPassword,
  type CreateUserForm,
  type UserRecord,
} from '@/api/users'
import { DataTable } from '@/components/data-table/DataTable'
import { DataTablePagination } from '@/components/data-table/DataTablePagination'
import { PageHeader } from '@/components/PageHeader'
import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/ui/Button'
import { Card, CardContent } from '@/components/ui/card'
import { Checkbox } from '@/components/ui/checkbox'
import { useConfirm } from '@/components/ConfirmProvider'
import { Dialog, DialogContent, DialogFooter, DialogHeader, DialogTitle } from '@/components/ui/dialog'
import { Input } from '@/components/ui/input'
import { Label } from '@/components/ui/label'
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select'
import { useAuthStore } from '@/stores/auth'

const emptyForm: CreateUserForm = {
  userName: '',
  nickName: '',
  password: '123456',
  phone: '',
  email: '',
  enable: 1,
  roleIds: [],
}
const PAGE_SIZE = 10

export function UsersPage() {
  const { t } = useTranslation()
  const queryClient = useQueryClient()
  const can = useAuthStore((state) => state.can)
  const confirmAction = useConfirm()
  const [page, setPage] = useState(1)
  const [draftKeyword, setDraftKeyword] = useState('')
  const [keyword, setKeyword] = useState('')
  const [createOpen, setCreateOpen] = useState(false)
  const [roleUser, setRoleUser] = useState<UserRecord | null>(null)
  const [form, setForm] = useState<CreateUserForm>(emptyForm)
  const [selectedRoles, setSelectedRoles] = useState<number[]>([])
  const users = useQuery({
    queryKey: ['users', page, PAGE_SIZE, keyword],
    queryFn: () => fetchUsers({ page, pageSize: PAGE_SIZE, keyword }),
  })
  const roles = useQuery({ queryKey: ['roles'], queryFn: listRoles })
  const invalidate = () => queryClient.invalidateQueries({ queryKey: ['users'] })
  const createMutation = useMutation({
    mutationFn: createUser,
    onSuccess: async (response) => {
      if (response.code !== 'OK') throw new Error(response.message)
      toast.success(t('User created'))
      setCreateOpen(false)
      setForm(emptyForm)
      await invalidate()
    },
    onError: (error) => toast.error(error.message || t('Create failed')),
  })
  const roleMutation = useMutation({
    mutationFn: ({ id, roleIds }: { id: number; roleIds: number[] }) => assignUserRoles(id, roleIds),
    onSuccess: async () => {
      toast.success(t('User role updated'))
      setRoleUser(null)
      await invalidate()
    },
  })
  const deleteMutation = useMutation({
    mutationFn: deleteUser,
    onSuccess: async () => {
      toast.success(t('User deleted'))
      await invalidate()
    },
  })
  const resetMutation = useMutation({
    mutationFn: (id: number) => resetUserPassword(id),
    onSuccess: () => toast.success(t('Password reset to 123456')),
  })
  const pageCount = Math.max(1, Math.ceil((users.data?.total ?? 0) / PAGE_SIZE))

  const columns = useMemo<ColumnDef<UserRecord>[]>(
    () => [
      {
        accessorKey: 'id',
        header: 'ID',
        cell: ({ row }) => row.original.id,
      },
      {
        id: 'user',
        header: t('User'),
        cell: ({ row }) => {
          const item = row.original
          return (
            <div className="flex flex-col">
              <strong className="font-medium">{item.nickName}</strong>
              <span className="text-xs text-muted-foreground">
                {item.userName}
                <br />
                {item.email}
              </span>
            </div>
          )
        },
      },
      {
        accessorKey: 'deptName',
        header: t('Department'),
        cell: ({ row }) => row.original.deptName || '—',
      },
      {
        id: 'roles',
        header: t('Roles'),
        cell: ({ row }) => (
          <div className="flex flex-wrap gap-1">
            {row.original.roles?.map((role) => (
              <Badge key={role.id} variant="secondary">
                {role.name}
              </Badge>
            ))}
          </div>
        ),
      },
      {
        accessorKey: 'enable',
        header: t('Status'),
        cell: ({ row }) => (
          <Badge variant={row.original.enable === 1 ? 'default' : 'outline'}>
            {t(row.original.enable === 1 ? 'Enabled' : 'Disabled')}
          </Badge>
        ),
      },
      {
        id: 'actions',
        header: t('Actions'),
        enableHiding: false,
        cell: ({ row }) => {
          const item = row.original
          return (
            <div className="flex flex-wrap gap-1">
              {can('system:user:assign-roles') && (
                <Button
                  onClick={() => {
                    setRoleUser(item)
                    setSelectedRoles(item.roleIds?.length ? item.roleIds : (item.roles?.map((role) => role.id) ?? []))
                  }}
                  variant="ghost"
                >
                  <IconShield size={14} />
                  {t('Change role')}
                </Button>
              )}
              {can('system:user:reset-password') && (
                <Button onClick={() => resetMutation.mutate(item.id)} variant="ghost">
                  <IconKey size={14} />
                  {t('Reset password')}
                </Button>
              )}
              {can('system:user:delete') && (
                <Button
                  onClick={() =>
                    void confirmAction(t('Delete user "{{name}}"?', { name: item.userName })).then((confirmed) => {
                      if (confirmed) deleteMutation.mutate(item.id)
                    })
                  }
                  variant="ghost"
                >
                  <IconTrash size={14} />
                  {t('Delete')}
                </Button>
              )}
            </div>
          )
        },
      },
    ],
    [can, confirmAction, deleteMutation, resetMutation, t],
  )

  const table = useReactTable({
    data: users.data?.list ?? [],
    columns,
    pageCount,
    manualPagination: true,
    getCoreRowModel: getCoreRowModel(),
    state: {
      pagination: { pageIndex: page - 1, pageSize: PAGE_SIZE },
    },
  })

  function openCreate() {
    const firstRole = roles.data?.[0]?.id
    setForm({ ...emptyForm, roleIds: firstRole ? [firstRole] : [] })
    setCreateOpen(true)
  }

  function searchUsers() {
    setKeyword(draftKeyword.trim())
    setPage(1)
  }

  function resetSearch() {
    setDraftKeyword('')
    setKeyword('')
    setPage(1)
  }

  function update<K extends keyof CreateUserForm>(key: K, value: CreateUserForm[K]) {
    setForm((current) => ({ ...current, [key]: value }))
  }

  function toggleFormRole(roleId: number) {
    setForm((current) => ({
      ...current,
      roleIds: current.roleIds.includes(roleId)
        ? current.roleIds.filter((id) => id !== roleId)
        : [...current.roleIds, roleId],
    }))
  }

  function submitCreate() {
    if (!form.userName.trim() || !form.nickName.trim() || !form.password || form.roleIds.length === 0) {
      toast.error(t('Username, nickname, password, and role are required'))
      return
    }
    createMutation.mutate(form)
  }

  return (
    <div className="flex flex-col gap-3">
      <PageHeader
        description={
          <h1 className="text-base font-semibold text-foreground">
            {t('Manage operator accounts, roles, and account recovery.')}
          </h1>
        }
        actions={
          <>
            <Button onClick={() => void users.refetch()} variant="outline">
              <IconRefresh size={16} />
              {t('Refresh')}
            </Button>
            {can('system:user:create') && (
              <Button onClick={openCreate}>
                <IconPlus size={16} />
                {t('New user')}
              </Button>
            )}
          </>
        }
      />
      <Card>
        <CardContent className="flex flex-col gap-3">
          <form
            className="flex flex-wrap items-center gap-2"
            onSubmit={(event) => {
              event.preventDefault()
              searchUsers()
            }}
          >
            <Input
              aria-label={t('Search users')}
              className="w-64"
              onChange={(event) => setDraftKeyword(event.target.value)}
              placeholder={t('Search users')}
              value={draftKeyword}
            />
            <Button type="submit">
              <IconSearch size={16} />
              {t('Search')}
            </Button>
            <Button onClick={resetSearch} type="button" variant="outline">
              {t('Reset')}
            </Button>
          </form>
          <DataTable
            cellClassName="py-1.5"
            emptyLabel={t('No users')}
            errorLabel={t('Failed to load data')}
            isError={users.isError}
            isLoading={users.isLoading}
            loadingLabel={t('Loading…')}
            table={table}
          />
          <DataTablePagination
            nextLabel={t('Next')}
            onPageChange={setPage}
            page={page}
            pageCount={pageCount}
            pageLabel={t('Page')}
            previousLabel={t('Previous')}
            totalText={t('Record total', { count: users.data?.total ?? 0 })}
          />
        </CardContent>
      </Card>

      <Dialog onOpenChange={setCreateOpen} open={createOpen}>
        <DialogContent className="sm:max-w-lg">
          <DialogHeader>
            <DialogTitle>{t('New user')}</DialogTitle>
          </DialogHeader>
          <div className="grid grid-cols-2 gap-3">
            <div className="flex flex-col gap-1.5">
              <Label htmlFor="user-username">{t('Username')}</Label>
              <Input
                id="user-username"
                onChange={(event) => update('userName', event.target.value)}
                value={form.userName}
              />
            </div>
            <div className="flex flex-col gap-1.5">
              <Label htmlFor="user-nickname">{t('Nickname')}</Label>
              <Input
                id="user-nickname"
                onChange={(event) => update('nickName', event.target.value)}
                value={form.nickName}
              />
            </div>
            <div className="flex flex-col gap-1.5">
              <Label htmlFor="user-password">{t('Password')}</Label>
              <Input
                id="user-password"
                onChange={(event) => update('password', event.target.value)}
                type="password"
                value={form.password}
              />
            </div>
            <div className="flex flex-col gap-1.5">
              <Label htmlFor="user-status">{t('Status')}</Label>
              <Select
                onValueChange={(value) => {
                  if (value != null) update('enable', Number(value))
                }}
                value={String(form.enable)}
              >
                <SelectTrigger aria-label={t('Status')} className="w-full" id="user-status">
                  <SelectValue />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="1">{t('Enabled')}</SelectItem>
                  <SelectItem value="0">{t('Disabled')}</SelectItem>
                </SelectContent>
              </Select>
            </div>
            <div className="flex flex-col gap-1.5">
              <Label htmlFor="user-phone">{t('Phone')}</Label>
              <Input id="user-phone" onChange={(event) => update('phone', event.target.value)} value={form.phone} />
            </div>
            <div className="flex flex-col gap-1.5">
              <Label htmlFor="user-email">{t('Email')}</Label>
              <Input id="user-email" onChange={(event) => update('email', event.target.value)} value={form.email} />
            </div>
            <div className="col-span-2 flex flex-col gap-1.5">
              <Label>{t('Role')}</Label>
              <div className="flex flex-wrap gap-3 rounded-lg border border-input p-2.5">
                {roles.data?.map((role) => (
                  <div className="flex items-center gap-1.5 text-sm" key={role.id}>
                    <Checkbox
                      aria-label={role.name}
                      checked={form.roleIds.includes(role.id)}
                      onCheckedChange={() => toggleFormRole(role.id)}
                    />
                    <span>{role.name}</span>
                  </div>
                ))}
              </div>
            </div>
          </div>
          <DialogFooter>
            <Button onClick={() => setCreateOpen(false)} variant="outline">
              {t('Cancel')}
            </Button>
            <Button disabled={createMutation.isPending} onClick={submitCreate}>
              {t('Create user')}
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>

      <Dialog onOpenChange={(open) => !open && setRoleUser(null)} open={Boolean(roleUser)}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>{t('Change user role')}</DialogTitle>
          </DialogHeader>
          <p className="text-sm text-muted-foreground">
            {roleUser?.nickName} · {roleUser?.userName}
          </p>
          <div className="flex flex-col gap-2">
            {roles.data?.map((role) => (
              <div className="flex items-center gap-2 text-sm" key={role.id}>
                <Checkbox
                  aria-label={role.name}
                  checked={selectedRoles.includes(role.id)}
                  onCheckedChange={() =>
                    setSelectedRoles((current) =>
                      current.includes(role.id) ? current.filter((id) => id !== role.id) : [...current, role.id],
                    )
                  }
                />
                <span>{role.name}</span>
              </div>
            ))}
          </div>
          <DialogFooter>
            <Button onClick={() => setRoleUser(null)} variant="outline">
              {t('Cancel')}
            </Button>
            <Button
              disabled={selectedRoles.length === 0}
              onClick={() => roleUser && roleMutation.mutate({ id: roleUser.id, roleIds: selectedRoles })}
            >
              {t('Save roles')}
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>
    </div>
  )
}
