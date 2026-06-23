import { pushToast } from './toast'
import { t } from '@/i18n'
import { readonly, ref } from 'vue'

type MessageInput = string | { message?: string }
type ConfirmType = 'warning' | 'danger' | 'info'

interface ConfirmDialogState {
  id: number
  title: string
  message: string
  type: ConfirmType
}

interface PendingConfirm extends ConfirmDialogState {
  resolve: () => void
  reject: (reason?: unknown) => void
}

let confirmId = 0
const pendingConfirm = ref<PendingConfirm | null>(null)
export const confirmDialog = readonly(pendingConfirm)

function normalizeMessage(input: MessageInput): string {
  const message = typeof input === 'string' ? input : input.message || ''
  return t(message)
}

export const ElMessage = {
  success(input: MessageInput) {
    pushToast({ type: 'success', message: normalizeMessage(input) })
  },
  error(input: MessageInput) {
    pushToast({ type: 'error', message: normalizeMessage(input) })
  },
  warning(input: MessageInput) {
    pushToast({ type: 'warning', message: normalizeMessage(input) })
  },
  info(input: MessageInput) {
    pushToast({ type: 'info', message: normalizeMessage(input) })
  }
}

export const ElMessageBox = {
  confirm(message: string, title = 'Confirm', options?: { type?: ConfirmType }) {
    pendingConfirm.value?.reject(new Error('cancel'))

    return new Promise<void>((resolve, reject) => {
      pendingConfirm.value = {
        id: ++confirmId,
        title: t(title),
        message: t(message),
        type: options?.type || 'warning',
        resolve,
        reject
      }
    })
  }
}

export function confirmDialogAccept() {
  const current = pendingConfirm.value
  if (!current) return
  pendingConfirm.value = null
  current.resolve()
}

export function confirmDialogCancel() {
  const current = pendingConfirm.value
  if (!current) return
  pendingConfirm.value = null
  current.reject(new Error('cancel'))
}
