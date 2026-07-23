import { getCoreRowModel, useReactTable, type ColumnDef } from '@tanstack/react-table'
import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query'
import { IconPencil, IconPlus, IconSearch, IconTrash } from '@tabler/icons-react'
import { useEffect, useMemo, useState } from 'react'
import { useTranslation } from 'react-i18next'
import { toast } from 'sonner'

import {
  createDictionary,
  createDictionaryDetail,
  deleteDictionary,
  deleteDictionaryDetail,
  fetchDictionaries,
  fetchDictionaryDetails,
  updateDictionary,
  updateDictionaryDetail,
  type DictionaryDetailPayload,
  type DictionaryDetailRecord,
  type DictionaryPayload,
  type DictionaryRecord,
} from '@/api/dictionaries'
import { DataTable } from '@/components/data-table/DataTable'
import { PageHeader } from '@/components/PageHeader'
import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/ui/Button'
import { Card, CardContent } from '@/components/ui/card'
import { useConfirm } from '@/components/ConfirmProvider'
import { Dialog, DialogContent, DialogFooter, DialogHeader, DialogTitle } from '@/components/ui/dialog'
import { Input } from '@/components/ui/input'
import { Label } from '@/components/ui/label'
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select'
import { cn } from '@/lib/utils'

type FlatDetail = DictionaryDetailRecord & { displayLevel: number }
const emptyDictionary: DictionaryPayload = { name: '', type: '', status: true, desc: '', parentId: null }
const emptyDetail: DictionaryDetailPayload = {
  label: '',
  value: '',
  extend: '',
  status: true,
  sort: 0,
  sysDictionaryId: 0,
  parentId: null,
}
function flatten(items: DictionaryDetailRecord[], level = 0): FlatDetail[] {
  return items.flatMap((item) => [{ ...item, displayLevel: level }, ...flatten(item.children ?? [], level + 1)])
}

export function DictionariesPage() {
  const { t } = useTranslation()
  const queryClient = useQueryClient()
  const confirmAction = useConfirm()
  const [search, setSearch] = useState('')
  const [filter, setFilter] = useState('')
  const [selectedId, setSelectedId] = useState<number | null>(null)
  const [dictionaryOpen, setDictionaryOpen] = useState(false)
  const [dictionaryForm, setDictionaryForm] = useState<DictionaryPayload>(emptyDictionary)
  const [detailOpen, setDetailOpen] = useState(false)
  const [detailForm, setDetailForm] = useState<DictionaryDetailPayload>(emptyDetail)
  const dictionaries = useQuery({ queryKey: ['dictionaries', filter], queryFn: () => fetchDictionaries(filter) })
  const details = useQuery({
    queryKey: ['dictionary-details', selectedId],
    queryFn: () => fetchDictionaryDetails(selectedId as number),
    enabled: selectedId !== null,
  })

  useEffect(() => {
    if (!selectedId && dictionaries.data?.length) setSelectedId(dictionaries.data[0].id)
    if (selectedId && dictionaries.data && !dictionaries.data.some((item) => item.id === selectedId))
      setSelectedId(dictionaries.data[0]?.id ?? null)
  }, [dictionaries.data, selectedId])

  const selected = dictionaries.data?.find((item) => item.id === selectedId) ?? null
  const invalidateDictionaries = () => queryClient.invalidateQueries({ queryKey: ['dictionaries'] })
  const invalidateDetails = () => queryClient.invalidateQueries({ queryKey: ['dictionary-details', selectedId] })
  const dictionaryMutation = useMutation({
    mutationFn: () =>
      dictionaryForm.id
        ? updateDictionary(dictionaryForm as DictionaryPayload & { id: number })
        : createDictionary(dictionaryForm),
    onSuccess: async () => {
      toast.success(t('Dictionary saved'))
      setDictionaryOpen(false)
      await invalidateDictionaries()
    },
    onError: () => toast.error(t('Failed to save dictionary')),
  })
  const detailMutation = useMutation({
    mutationFn: () =>
      detailForm.id
        ? updateDictionaryDetail(detailForm as DictionaryDetailPayload & { id: number })
        : createDictionaryDetail(detailForm),
    onSuccess: async () => {
      toast.success(t('Dictionary detail saved'))
      setDetailOpen(false)
      await invalidateDetails()
    },
    onError: () => toast.error(t('Failed to save dictionary detail')),
  })
  const deleteDictionaryMutation = useMutation({
    mutationFn: deleteDictionary,
    onSuccess: async () => {
      toast.success(t('Dictionary deleted'))
      setSelectedId(null)
      await invalidateDictionaries()
    },
  })
  const deleteDetailMutation = useMutation({
    mutationFn: ({ dictionaryId, id }: { dictionaryId: number; id: number }) =>
      deleteDictionaryDetail(dictionaryId, id),
    onSuccess: async () => {
      toast.success(t('Dictionary detail deleted'))
      await invalidateDetails()
    },
  })

  const flatDetails = useMemo(() => flatten(details.data ?? []), [details.data])

  function editDictionary(item?: DictionaryRecord) {
    setDictionaryForm(item ? { ...item } : { ...emptyDictionary })
    setDictionaryOpen(true)
  }
  function editDetail(item?: DictionaryDetailRecord, parentId: number | null = null) {
    if (!selectedId) return
    setDetailForm(
      item
        ? {
            id: item.id,
            label: item.label,
            value: item.value,
            extend: item.extend,
            status: item.status,
            sort: item.sort,
            sysDictionaryId: item.sysDictionaryId,
            parentId: item.parentId ?? null,
          }
        : { ...emptyDetail, sysDictionaryId: selectedId, parentId },
    )
    setDetailOpen(true)
  }
  function saveDictionary() {
    if (!dictionaryForm.name.trim() || !dictionaryForm.type.trim()) {
      toast.error(t('Name and type are required'))
      return
    }
    dictionaryMutation.mutate()
  }
  function saveDetail() {
    if (!detailForm.label.trim() || !detailForm.value.trim()) {
      toast.error(t('Label and value are required'))
      return
    }
    detailMutation.mutate()
  }

  const columns: ColumnDef<FlatDetail>[] = [
    {
      accessorKey: 'label',
      header: t('Label'),
      cell: ({ row }) => {
        const item = row.original
        return (
          <div className="flex flex-col" style={{ paddingLeft: 14 + item.displayLevel * 20 }}>
            <strong className="font-medium">{item.label}</strong>
            <span className="text-xs text-muted-foreground">{item.path}</span>
          </div>
        )
      },
    },
    {
      accessorKey: 'value',
      header: t('Value'),
      cell: ({ row }) => <code className="rounded bg-muted px-1.5 py-0.5 text-xs">{row.original.value}</code>,
    },
    {
      accessorKey: 'sort',
      header: t('Sort'),
      cell: ({ row }) => row.original.sort,
    },
    {
      accessorKey: 'status',
      header: t('Status'),
      cell: ({ row }) => (
        <Badge variant={row.original.status ? 'default' : 'outline'}>
          {t(row.original.status ? 'Enabled' : 'Disabled')}
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
            <Button onClick={() => editDetail(undefined, item.id)} variant="ghost">
              <IconPlus size={14} />
              {t('Add child')}
            </Button>
            <Button onClick={() => editDetail(item)} variant="ghost">
              <IconPencil size={14} />
              {t('Edit')}
            </Button>
            <Button
              onClick={() =>
                void confirmAction(t('Delete dictionary detail "{{name}}"?', { name: item.label })).then((yes) => {
                  if (yes && selectedId) deleteDetailMutation.mutate({ dictionaryId: selectedId, id: item.id })
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
  ]

  const table = useReactTable({
    data: flatDetails,
    columns,
    getCoreRowModel: getCoreRowModel(),
  })

  return (
    <div className="flex flex-col gap-3">
      <PageHeader
        description={
          <h1 className="text-base font-semibold text-foreground">
            {t('Manage dictionaries and their hierarchical detail values.')}
          </h1>
        }
        actions={
          <Button onClick={() => editDictionary()}>
            <IconPlus size={16} />
            {t('New dictionary')}
          </Button>
        }
      />
      <section className="flex flex-1 gap-3">
        <Card className="w-56 shrink-0">
          <CardContent className="flex flex-col gap-2">
            <div className="flex items-center gap-1.5">
              <Input
                aria-label="Search dictionaries"
                onChange={(event) => setSearch(event.target.value)}
                placeholder={t('Dictionary name')}
                value={search}
              />
              <Button aria-label={t('Search')} onClick={() => setFilter(search)} size="icon" variant="outline">
                <IconSearch size={15} />
              </Button>
            </div>
            <div className="flex flex-col gap-0.5">
              {dictionaries.isLoading ? (
                <p className="px-2 py-3 text-sm text-muted-foreground">{t('Loading…')}</p>
              ) : dictionaries.isError ? (
                <p className="px-2 py-3 text-sm text-destructive">{t('Failed to load dictionaries')}</p>
              ) : dictionaries.data?.length ? (
                dictionaries.data.map((item) => (
                  <button
                    className={cn(
                      'flex flex-col rounded-md px-2 py-1.5 text-left text-sm transition-colors hover:bg-muted',
                      selectedId === item.id && 'bg-accent text-accent-foreground',
                    )}
                    key={item.id}
                    onClick={() => setSelectedId(item.id)}
                    type="button"
                  >
                    <strong className="font-medium">{item.name}</strong>
                    <span className="text-xs text-muted-foreground">{item.type}</span>
                  </button>
                ))
              ) : (
                <p className="px-2 py-3 text-sm text-muted-foreground">{t('No dictionaries')}</p>
              )}
            </div>
          </CardContent>
        </Card>
        <Card className="min-w-0 flex-1">
          <CardContent className="flex flex-col gap-3">
            <div className="flex flex-wrap items-start justify-between gap-2">
              <div>
                <h2 className="text-base font-semibold text-foreground">{selected?.name ?? t('Dictionaries')}</h2>
                {selected && (
                  <p className="text-sm text-muted-foreground">
                    <code className="rounded bg-muted px-1.5 py-0.5 text-xs">{selected.type}</code> ·{' '}
                    {t(selected.status ? 'Enabled' : 'Disabled')}
                  </p>
                )}
              </div>
              {selected && (
                <div className="flex flex-wrap items-center gap-2">
                  <Button onClick={() => editDictionary(selected)} variant="outline">
                    {t('Edit dictionary')}
                  </Button>
                  <Button
                    onClick={() =>
                      void confirmAction(t('Delete dictionary "{{name}}"?', { name: selected.name })).then((yes) => {
                        if (yes) deleteDictionaryMutation.mutate(selected.id)
                      })
                    }
                    variant="outline"
                  >
                    {t('Delete')}
                  </Button>
                  <Button onClick={() => editDetail()}>
                    <IconPlus size={15} />
                    {t('New root detail')}
                  </Button>
                </div>
              )}
            </div>
            <DataTable
              cellClassName="py-1.5"
              emptyLabel={selectedId ? t('No dictionary details') : t('Select a dictionary')}
              errorLabel={t('Failed to load data')}
              isError={dictionaries.isError || (Boolean(selectedId) && details.isError)}
              isLoading={dictionaries.isLoading || (Boolean(selectedId) && details.isLoading)}
              loadingLabel={t('Loading…')}
              summary={t('Record total', { count: flatDetails.length })}
              table={table}
            />
          </CardContent>
        </Card>
      </section>
      <Dialog onOpenChange={setDictionaryOpen} open={dictionaryOpen}>
        <DialogContent className="sm:max-w-lg">
          <DialogHeader>
            <DialogTitle>{t(dictionaryForm.id ? 'Edit dictionary' : 'New dictionary')}</DialogTitle>
          </DialogHeader>
          <div className="grid grid-cols-2 gap-3">
            <div className="flex flex-col gap-1.5">
              <Label htmlFor="dictionary-name">{t('Name')}</Label>
              <Input
                id="dictionary-name"
                onChange={(event) => setDictionaryForm((current) => ({ ...current, name: event.target.value }))}
                value={dictionaryForm.name}
              />
            </div>
            <div className="flex flex-col gap-1.5">
              <Label htmlFor="dictionary-type">{t('Type')}</Label>
              <Input
                id="dictionary-type"
                onChange={(event) => setDictionaryForm((current) => ({ ...current, type: event.target.value }))}
                value={dictionaryForm.type}
              />
            </div>
            <div className="flex flex-col gap-1.5">
              <Label htmlFor="dictionary-desc">{t('Description')}</Label>
              <Input
                id="dictionary-desc"
                onChange={(event) => setDictionaryForm((current) => ({ ...current, desc: event.target.value }))}
                value={dictionaryForm.desc}
              />
            </div>
            <div className="flex flex-col gap-1.5">
              <Label htmlFor="dictionary-status">{t('Status')}</Label>
              <Select
                onValueChange={(value) => setDictionaryForm((current) => ({ ...current, status: value === 'enabled' }))}
                value={dictionaryForm.status ? 'enabled' : 'disabled'}
              >
                <SelectTrigger aria-label={t('Status')} className="w-full" id="dictionary-status">
                  <SelectValue />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="enabled">{t('Enabled')}</SelectItem>
                  <SelectItem value="disabled">{t('Disabled')}</SelectItem>
                </SelectContent>
              </Select>
            </div>
          </div>
          <DialogFooter>
            <Button onClick={() => setDictionaryOpen(false)} variant="outline">
              {t('Cancel')}
            </Button>
            <Button disabled={dictionaryMutation.isPending} onClick={saveDictionary}>
              {t('Save')}
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>
      <Dialog onOpenChange={setDetailOpen} open={detailOpen}>
        <DialogContent className="sm:max-w-lg">
          <DialogHeader>
            <DialogTitle>{t(detailForm.id ? 'Edit dictionary detail' : 'New dictionary detail')}</DialogTitle>
          </DialogHeader>
          <div className="grid grid-cols-2 gap-3">
            <div className="flex flex-col gap-1.5">
              <Label htmlFor="detail-label">{t('Label')}</Label>
              <Input
                id="detail-label"
                onChange={(event) => setDetailForm((current) => ({ ...current, label: event.target.value }))}
                value={detailForm.label}
              />
            </div>
            <div className="flex flex-col gap-1.5">
              <Label htmlFor="detail-value">{t('Value')}</Label>
              <Input
                id="detail-value"
                onChange={(event) => setDetailForm((current) => ({ ...current, value: event.target.value }))}
                value={detailForm.value}
              />
            </div>
            <div className="flex flex-col gap-1.5">
              <Label htmlFor="detail-extend">{t('Extend')}</Label>
              <Input
                id="detail-extend"
                onChange={(event) => setDetailForm((current) => ({ ...current, extend: event.target.value }))}
                value={detailForm.extend}
              />
            </div>
            <div className="flex flex-col gap-1.5">
              <Label htmlFor="detail-sort">{t('Sort')}</Label>
              <Input
                id="detail-sort"
                onChange={(event) => setDetailForm((current) => ({ ...current, sort: Number(event.target.value) }))}
                type="number"
                value={detailForm.sort}
              />
            </div>
            <div className="flex flex-col gap-1.5">
              <Label htmlFor="detail-status">{t('Status')}</Label>
              <Select
                onValueChange={(value) => setDetailForm((current) => ({ ...current, status: value === 'enabled' }))}
                value={detailForm.status ? 'enabled' : 'disabled'}
              >
                <SelectTrigger aria-label={t('Status')} className="w-full" id="detail-status">
                  <SelectValue />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="enabled">{t('Enabled')}</SelectItem>
                  <SelectItem value="disabled">{t('Disabled')}</SelectItem>
                </SelectContent>
              </Select>
            </div>
          </div>
          <DialogFooter>
            <Button onClick={() => setDetailOpen(false)} variant="outline">
              {t('Cancel')}
            </Button>
            <Button disabled={detailMutation.isPending} onClick={saveDetail}>
              {t('Save')}
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>
    </div>
  )
}
