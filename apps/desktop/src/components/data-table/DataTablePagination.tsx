import { Button } from '@/components/ui/Button'

export function DataTablePagination({
  page,
  pageCount,
  onPageChange,
  previousLabel,
  nextLabel,
  pageLabel,
}: {
  page: number
  pageCount: number
  onPageChange: (page: number) => void
  previousLabel: string
  nextLabel: string
  pageLabel: string
}) {
  const pages = Math.max(1, pageCount)

  return (
    <div className="flex items-center justify-center gap-3 pt-1 text-sm">
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
  )
}
