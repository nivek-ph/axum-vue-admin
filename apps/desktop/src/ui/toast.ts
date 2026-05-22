import { readonly, ref } from 'vue'

export type ToastType = 'success' | 'error' | 'warning' | 'info'

export interface ToastItem {
  id: number
  type: ToastType
  message: string
}

const toasts = ref<ToastItem[]>([])
let toastId = 0

export const toastItems = readonly(toasts)

export function pushToast(input: { type: ToastType; message: string }) {
  const message = input.message.trim()
  if (!message) return

  const item = {
    id: ++toastId,
    type: input.type,
    message
  }

  toasts.value = [...toasts.value, item]
  window.setTimeout(() => removeToast(item.id), 3200)
}

export function removeToast(id: number) {
  toasts.value = toasts.value.filter((item) => item.id !== id)
}
