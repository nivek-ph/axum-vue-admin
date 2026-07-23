import { Button } from '@/components/ui/Button'
import type { ReactNode } from 'react'

export function DataTablePagination({
  page,
  pageCount,
  onPageChange,
  previousLabel,
  nextLabel,
  pageLabel,
  totalText,
}: {
  page: number
  pageCount: number
  onPageChange: (page: number) => void
  previousLabel: string
  nextLabel: string
  pageLabel: string
  totalText: ReactNode
}) {
  const pages = Math.max(1, pageCount)

  return (
    <div className="flex flex-wrap items-center justify-end gap-3 pt-1 text-sm">
      <span className="text-muted-foreground">{totalText}</span>
      <div className="flex items-center gap-3">
        <Button disabled={page <= 1} onClick={() => onPageChange(page - 1)} size="sm" variant="outline">
          {previousLabel}
        </Button>
        <span className="text-muted-foreground">
          {pageLabel} {page} / {pages}
        </span>
        <Button disabled={page >= pages} onClick={() => onPageChange(page + 1)} size="sm" variant="outline">
          {nextLabel}
        </Button>
      </div>
    </div>
  )
}
