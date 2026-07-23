import type { ReactNode } from 'react'

import { cn } from '@/lib/utils'

export function PageHeader({
  description,
  actions,
  className,
}: {
  description?: ReactNode
  actions?: ReactNode
  className?: string
}) {
  return (
    <div className={cn('mb-4 flex min-h-9 flex-wrap items-center justify-between gap-3 xl:mb-5', className)}>
      <div className="min-w-0 text-muted-foreground">{description}</div>
      {actions ? <div className="flex flex-wrap items-center gap-2">{actions}</div> : null}
    </div>
  )
}
