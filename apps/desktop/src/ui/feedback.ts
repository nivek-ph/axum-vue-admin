import { pushToast } from './toast'

type MessageInput = string | { message?: string }

function normalizeMessage(input: MessageInput): string {
  return typeof input === 'string' ? input : input.message || ''
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
  confirm(message: string, title = '确认') {
    const ok = window.confirm(`${title}\n\n${message}`)
    return ok ? Promise.resolve() : Promise.reject(new Error('cancel'))
  }
}
