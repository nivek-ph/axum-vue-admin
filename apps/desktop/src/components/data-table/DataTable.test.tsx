import { getCoreRowModel, useReactTable, type ColumnDef } from '@tanstack/react-table'
import { cleanup, render, screen } from '@testing-library/react'
import { afterEach, describe, expect, it } from 'vitest'

import { DataTable } from '@/components/data-table/DataTable'

type Row = {
  name: string
}

const columns: ColumnDef<Row>[] = [{ accessorKey: 'name', header: 'Name' }]

afterEach(cleanup)

function Example({
  data = [],
  expandedContent = false,
  isError = false,
  isLoading = false,
}: {
  data?: Row[]
  expandedContent?: boolean
  isError?: boolean
  isLoading?: boolean
}) {
  const table = useReactTable({
    columns,
    data,
    getCoreRowModel: getCoreRowModel(),
  })

  return (
    <DataTable
      emptyLabel="No records"
      errorLabel="Records failed to load"
      isError={isError}
      isLoading={isLoading}
      loadingLabel="Loading records"
      renderExpandedContent={expandedContent ? (row) => `${row.original.name} details` : undefined}
      table={table}
    />
  )
}

describe('DataTable', () => {
  it('renders a loading state before rows are available', () => {
    render(<Example isLoading />)

    expect(screen.getByText('Loading records')).toBeInTheDocument()
    expect(screen.queryByText('No records')).not.toBeInTheDocument()
  })

  it('renders an error state instead of an empty state', () => {
    render(<Example isError />)

    expect(screen.getByText('Records failed to load')).toBeInTheDocument()
    expect(screen.queryByText('No records')).not.toBeInTheDocument()
  })

  it('renders an empty state when loading succeeds without rows', () => {
    render(<Example />)

    expect(screen.getByText('No records')).toBeInTheDocument()
  })

  it('renders table data when rows are available', () => {
    render(<Example data={[{ name: 'Ada' }]} />)

    expect(screen.getByRole('columnheader', { name: 'Name' })).toBeInTheDocument()
    expect(screen.getByRole('cell', { name: 'Ada' })).toBeInTheDocument()
  })

  it('renders expanded content beneath a data row', () => {
    render(<Example data={[{ name: 'Ada' }]} expandedContent />)

    expect(screen.getByText('Ada details')).toBeInTheDocument()
  })
})
