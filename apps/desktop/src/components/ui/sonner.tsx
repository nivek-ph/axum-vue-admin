import { IconCircleCheck, IconInfoCircle, IconLoader2, IconCircleX, IconAlertTriangle } from '@tabler/icons-react'
import { Toaster as Sonner, type ToasterProps } from 'sonner'

import { useThemeStore } from '@/stores/theme'

function Toaster({ ...props }: ToasterProps) {
  const mode = useThemeStore((state) => state.mode)

  return (
    <Sonner
      theme={mode}
      className="toaster group"
      icons={{
        success: <IconCircleCheck className="size-4" />,
        info: <IconInfoCircle className="size-4" />,
        warning: <IconAlertTriangle className="size-4" />,
        error: <IconCircleX className="size-4" />,
        loading: <IconLoader2 className="size-4 animate-spin" />,
      }}
      style={
        {
          '--normal-bg': 'var(--popover)',
          '--normal-text': 'var(--popover-foreground)',
          '--normal-border': 'var(--border)',
          '--border-radius': 'var(--radius)',
        } as React.CSSProperties
      }
      toastOptions={{
        classNames: {
          toast: 'cn-toast',
        },
      }}
      {...props}
    />
  )
}

export { Toaster }
