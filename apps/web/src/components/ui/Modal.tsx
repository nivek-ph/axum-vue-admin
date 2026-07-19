import * as Dialog from '@radix-ui/react-dialog'
import { X } from 'lucide-react'
import type { ReactNode } from 'react'

export function Modal({ open, onOpenChange, title, children, footer }: { open: boolean; onOpenChange: (open: boolean) => void; title: string; children: ReactNode; footer?: ReactNode }) {
  return (
    <Dialog.Root open={open} onOpenChange={onOpenChange}>
      <Dialog.Portal>
        <Dialog.Overlay className="modal-overlay" />
        <Dialog.Content className="modal-content">
          <div className="modal-header"><Dialog.Title>{title}</Dialog.Title><Dialog.Close aria-label="Close"><X size={18} /></Dialog.Close></div>
          <div className="modal-body">{children}</div>
          {footer && <div className="modal-footer">{footer}</div>}
        </Dialog.Content>
      </Dialog.Portal>
    </Dialog.Root>
  )
}
