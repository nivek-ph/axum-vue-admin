import { cn } from '@/lib/utils'

interface BrandMarkProps {
  className?: string
  size?: 'default' | 'small' | 'large'
}

export function BrandMark({ className, size = 'default' }: BrandMarkProps) {
  return (
    <div
      aria-hidden="true"
      className={cn(
        'inline-flex shrink-0 items-center justify-center text-primary',
        size === 'small' && 'size-7',
        size === 'default' && 'size-10',
        size === 'large' && 'size-14 xl:size-16',
        className,
      )}
    >
      <svg className="size-full" fill="none" viewBox="0 0 32 32">
        <rect fill="currentColor" height="22" opacity="0.28" rx="4" width="10" x="4" y="5" />
        <rect fill="currentColor" height="16" opacity="0.55" rx="4" width="10" x="11" y="8" />
        <rect fill="currentColor" height="22" rx="4" width="10" x="18" y="5" />
      </svg>
    </div>
  )
}
