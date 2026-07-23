import { flexRender, type Row, type Table as TanStackTable } from '@tanstack/react-table'
import { Fragment, type ReactNode } from 'react'

import { DataTableEmptyState } from '@/components/data-table/DataTableEmptyState'
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from '@/components/ui/table'

export function DataTable<TData>({
  cellClassName,
  emptyLabel,
  errorLabel,
  isError = false,
  isLoading = false,
  loadingLabel,
  renderExpandedContent,
  rowClassName,
  table,
  tableClassName,
}: {
  cellClassName?: string
  emptyLabel: ReactNode
  errorLabel: ReactNode
  isError?: boolean
  isLoading?: boolean
  loadingLabel: ReactNode
  renderExpandedContent?: (row: Row<TData>) => ReactNode
  rowClassName?: (row: Row<TData>) => string | undefined
  table: TanStackTable<TData>
  tableClassName?: string
}) {
  const rows = table.getRowModel().rows
  const colSpan = Math.max(1, table.getVisibleLeafColumns().length)

  return (
    <Table className={tableClassName}>
      <TableHeader>
        {table.getHeaderGroups().map((headerGroup) => (
          <TableRow key={headerGroup.id}>
            {headerGroup.headers.map((header) => (
              <TableHead key={header.id}>
                {header.isPlaceholder ? null : flexRender(header.column.columnDef.header, header.getContext())}
              </TableHead>
            ))}
          </TableRow>
        ))}
      </TableHeader>
      <TableBody>
        {isLoading ? (
          <DataTableEmptyState colSpan={colSpan}>{loadingLabel}</DataTableEmptyState>
        ) : isError ? (
          <DataTableEmptyState className="text-destructive" colSpan={colSpan}>
            {errorLabel}
          </DataTableEmptyState>
        ) : rows.length === 0 ? (
          <DataTableEmptyState colSpan={colSpan}>{emptyLabel}</DataTableEmptyState>
        ) : (
          rows.map((row) => {
            const expandedContent = renderExpandedContent?.(row)
            return (
              <Fragment key={row.id}>
                <TableRow className={rowClassName?.(row)}>
                  {row.getVisibleCells().map((cell) => (
                    <TableCell className={cellClassName} key={cell.id}>
                      {flexRender(cell.column.columnDef.cell, cell.getContext())}
                    </TableCell>
                  ))}
                </TableRow>
                {expandedContent ? (
                  <TableRow className="bg-muted/20 hover:bg-muted/20">
                    <TableCell className="p-0 whitespace-normal" colSpan={colSpan}>
                      {expandedContent}
                    </TableCell>
                  </TableRow>
                ) : null}
              </Fragment>
            )
          })
        )}
      </TableBody>
    </Table>
  )
}
