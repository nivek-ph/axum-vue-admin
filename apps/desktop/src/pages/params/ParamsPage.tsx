import { getCoreRowModel, useReactTable, type ColumnDef } from '@tanstack/react-table'
import { IconPencil, IconPlus, IconSearch, IconTrash } from '@tabler/icons-react'
import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query'
import { useMemo, useState } from 'react'
import { useTranslation } from 'react-i18next'
import { toast } from 'sonner'

import { createParam, deleteParam, fetchParams, updateParam, type ParamRecord } from '@/api/params'
import { DataTable } from '@/components/data-table/DataTable'
import { DataTablePagination } from '@/components/data-table/DataTablePagination'
import { PageHeader } from '@/components/PageHeader'
import { Button } from '@/components/ui/Button'
import { Card, CardContent } from '@/components/ui/card'
import { useConfirm } from '@/components/ConfirmProvider'
import { Dialog, DialogContent, DialogFooter, DialogHeader, DialogTitle } from '@/components/ui/dialog'
import { Input } from '@/components/ui/input'
import { Label } from '@/components/ui/label'

const emptyForm: ParamRecord = { id: 0, name: '', key: '', value: '', desc: '' }
const PAGE_SIZE = 10

export function ParamsPage() {
  const { t } = useTranslation()
  const queryClient = useQueryClient()
  const confirmAction = useConfirm()
  const [page, setPage] = useState(1)
  const [draftFilters, setDraftFilters] = useState({ name: '', key: '' })
  const [filters, setFilters] = useState(draftFilters)
  const [open, setOpen] = useState(false)
  const [form, setForm] = useState<ParamRecord>(emptyForm)
  const query = useQuery({
    queryKey: ['params', page, filters],
    queryFn: () => fetchParams({ page, pageSize: PAGE_SIZE, ...filters }),
  })
  const invalidate = () => queryClient.invalidateQueries({ queryKey: ['params'] })
  const saveMutation = useMutation({
    mutationFn: () =>
      form.id ? updateParam(form) : createParam({ name: form.name, key: form.key, value: form.value, desc: form.desc }),
    onSuccess: async () => {
      toast.success(t('Param saved'))
      setOpen(false)
      await invalidate()
    },
    onError: () => toast.error(t('Failed to save param')),
  })
  const deleteMutation = useMutation({
    mutationFn: deleteParam,
    onSuccess: async () => {
      toast.success(t('Param deleted'))
      await invalidate()
    },
  })
  const pageCount = Math.max(1, Math.ceil((query.data?.total ?? 0) / PAGE_SIZE))

  const columns = useMemo<ColumnDef<ParamRecord>[]>(
    () => [
      {
        accessorKey: 'name',
        header: t('Name'),
        cell: ({ row }) => row.original.name,
      },
      {
        accessorKey: 'key',
        header: t('Key'),
        cell: ({ row }) => <code className="rounded bg-muted px-1.5 py-0.5 text-xs">{row.original.key}</code>,
      },
      {
        accessorKey: 'value',
        header: t('Value'),
        cell: ({ row }) => row.original.value,
      },
      {
        accessorKey: 'desc',
        header: t('Description'),
        cell: ({ row }) => row.original.desc || '—',
      },
      {
        id: 'actions',
        header: t('Actions'),
        enableHiding: false,
        cell: ({ row }) => {
          const item = row.original
          return (
            <div className="flex flex-wrap gap-1">
              <Button
                onClick={() => {
                  setForm(item)
                  setOpen(true)
                }}
                variant="ghost"
              >
                <IconPencil size={14} />
                {t('Edit')}
              </Button>
              <Button
                onClick={() =>
                  void confirmAction(t('Delete param "{{name}}"?', { name: item.name })).then((yes) => {
                    if (yes) deleteMutation.mutate(item.id)
                  })
                }
                variant="ghost"
              >
                <IconTrash size={14} />
                {t('Delete')}
              </Button>
            </div>
          )
        },
      },
    ],
    [confirmAction, deleteMutation, t],
  )

  const table = useReactTable({
    data: query.data?.list ?? [],
    columns,
    pageCount,
    manualPagination: true,
    getCoreRowModel: getCoreRowModel(),
    state: {
      pagination: { pageIndex: page - 1, pageSize: PAGE_SIZE },
    },
  })

  function save() {
    if (!form.name.trim() || !form.key.trim()) {
      toast.error(t('Name and key are required'))
      return
    }
    saveMutation.mutate()
  }

  return (
    <div className="flex flex-col gap-3">
      <PageHeader
        description={
          <h1 className="text-base font-semibold text-foreground">{t('Manage runtime key-value configuration.')}</h1>
        }
        actions={
          <Button
            onClick={() => {
              setForm(emptyForm)
              setOpen(true)
            }}
          >
            <IconPlus size={16} />
            {t('New param')}
          </Button>
        }
      />
      <Card>
        <CardContent className="flex flex-col gap-3">
          <div className="flex flex-wrap items-center gap-2">
            <Input
              aria-label="Filter by name"
              className="w-40"
              onChange={(event) => setDraftFilters((current) => ({ ...current, name: event.target.value }))}
              placeholder={t('Name')}
              value={draftFilters.name}
            />
            <Input
              aria-label="Filter by key"
              className="w-40"
              onChange={(event) => setDraftFilters((current) => ({ ...current, key: event.target.value }))}
              placeholder={t('Key')}
              value={draftFilters.key}
            />
            <Button
              onClick={() => {
                setPage(1)
                setFilters(draftFilters)
              }}
              variant="outline"
            >
              <IconSearch size={16} />
              {t('Search')}
            </Button>
          </div>
          <DataTable
            cellClassName="py-1.5"
            emptyLabel={t('No params')}
            errorLabel={t('Failed to load data')}
            isError={query.isError}
            isLoading={query.isLoading}
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
            totalText={t('Record total', { count: query.data?.total ?? 0 })}
          />
        </CardContent>
      </Card>
      <Dialog onOpenChange={setOpen} open={open}>
        <DialogContent className="sm:max-w-lg">
          <DialogHeader>
            <DialogTitle>{t(form.id ? 'Edit param' : 'New param')}</DialogTitle>
          </DialogHeader>
          <div className="grid grid-cols-2 gap-3">
            <div className="flex flex-col gap-1.5">
              <Label htmlFor="param-name">{t('Name')}</Label>
              <Input
                id="param-name"
                onChange={(event) => setForm((current) => ({ ...current, name: event.target.value }))}
                value={form.name}
              />
            </div>
            <div className="flex flex-col gap-1.5">
              <Label htmlFor="param-key">{t('Key')}</Label>
              <Input
                id="param-key"
                onChange={(event) => setForm((current) => ({ ...current, key: event.target.value }))}
                value={form.key}
              />
            </div>
            <div className="col-span-2 flex flex-col gap-1.5">
              <Label htmlFor="param-value">{t('Value')}</Label>
              <Input
                id="param-value"
                onChange={(event) => setForm((current) => ({ ...current, value: event.target.value }))}
                value={form.value}
              />
            </div>
            <div className="col-span-2 flex flex-col gap-1.5">
              <Label htmlFor="param-desc">{t('Description')}</Label>
              <Input
                id="param-desc"
                onChange={(event) => setForm((current) => ({ ...current, desc: event.target.value }))}
                value={form.desc}
              />
            </div>
          </div>
          <DialogFooter>
            <Button onClick={() => setOpen(false)} variant="outline">
              {t('Cancel')}
            </Button>
            <Button disabled={saveMutation.isPending} onClick={save}>
              {t('Save')}
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>
    </div>
  )
}
