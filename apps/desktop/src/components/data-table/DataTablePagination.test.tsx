import { cleanup, render, screen } from '@testing-library/react'
import { afterEach, describe, expect, it, vi } from 'vitest'

import { DataTablePagination } from '@/components/data-table/DataTablePagination'

afterEach(cleanup)

describe('DataTablePagination', () => {
  it('shows the total record count beside the current page', () => {
    render(
      <DataTablePagination
        nextLabel="Next"
        onPageChange={vi.fn()}
        page={2}
        pageCount={5}
        pageLabel="Page"
        previousLabel="Previous"
        totalText="42 records total"
      />,
    )

    expect(screen.getByText('42 records total')).toBeInTheDocument()
    expect(screen.getByText('Page 2 / 5')).toBeInTheDocument()
  })
})
