import type { ReactNode } from 'react'

import { TableCell, TableRow } from '@/components/ui/table'
import { cn } from '@/lib/utils'

export function DataTableEmptyState({
  className,
  colSpan,
  children,
}: {
  className?: string
  colSpan: number
  children: ReactNode
}) {
  return (
    <TableRow>
      <TableCell className={cn('py-8 text-center text-muted-foreground', className)} colSpan={colSpan}>
        {children}
      </TableCell>
    </TableRow>
  )
}
