import { pushToast } from './toast'
import { t } from '@/i18n'

type MessageInput = string | { message?: string }

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
  confirm(message: string, title = 'Confirm') {
    const ok = window.confirm(`${t(title)}\n\n${t(message)}`)
    return ok ? Promise.resolve() : Promise.reject(new Error('cancel'))
  }
}
