import { createContext, useContext, useRef, useState, type ReactNode } from 'react'
import { useTranslation } from 'react-i18next'

import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
} from '@/components/ui/alert-dialog'
import { buttonVariants } from '@/components/ui/Button'
import { cn } from '@/lib/utils'

type Confirm = (message: string) => Promise<boolean>

const ConfirmContext = createContext<Confirm | null>(null)

function highlightQuotedNames(message: string): ReactNode {
  const nodes: ReactNode[] = []
  const pattern = /([“"])([^“”"]+)([”"])/g
  let lastIndex = 0
  let match: RegExpExecArray | null
  let key = 0

  while ((match = pattern.exec(message))) {
    if (match.index > lastIndex) {
      nodes.push(message.slice(lastIndex, match.index))
    }
    nodes.push(
      <span className="font-semibold text-foreground" key={key}>
        {match[2]}
      </span>,
    )
    key += 1
    lastIndex = match.index + match[0].length
  }

  if (lastIndex < message.length) {
    nodes.push(message.slice(lastIndex))
  }

  return nodes.length > 0 ? nodes : message
}

export function ConfirmProvider({ children }: { children: ReactNode }) {
  const { t } = useTranslation()
  const [message, setMessage] = useState('')
  const [open, setOpen] = useState(false)
  const resolveRef = useRef<((confirmed: boolean) => void) | null>(null)

  function finish(confirmed: boolean) {
    resolveRef.current?.(confirmed)
    resolveRef.current = null
    setOpen(false)
  }

  const confirm: Confirm = (nextMessage) => {
    resolveRef.current?.(false)
    setMessage(nextMessage)
    setOpen(true)
    return new Promise((resolve) => {
      resolveRef.current = resolve
    })
  }

  return (
    <ConfirmContext.Provider value={confirm}>
      {children}
      <AlertDialog
        open={open}
        onOpenChange={(nextOpen) => {
          if (!nextOpen) finish(false)
        }}
      >
        <AlertDialogContent className="sm:max-w-md">
          <AlertDialogHeader>
            <AlertDialogTitle>{t('Confirm action')}</AlertDialogTitle>
            <AlertDialogDescription>{highlightQuotedNames(message)}</AlertDialogDescription>
          </AlertDialogHeader>
          <AlertDialogFooter>
            <AlertDialogCancel onClick={() => finish(false)}>{t('Cancel')}</AlertDialogCancel>
            <AlertDialogAction className={cn(buttonVariants({ variant: 'destructive' }))} onClick={() => finish(true)}>
              {t('Confirm')}
            </AlertDialogAction>
          </AlertDialogFooter>
        </AlertDialogContent>
      </AlertDialog>
    </ConfirmContext.Provider>
  )
}

export function useConfirm() {
  const confirm = useContext(ConfirmContext)
  if (!confirm) throw new Error('useConfirm must be used within ConfirmProvider')
  return confirm
}
