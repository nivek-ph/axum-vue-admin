import { computed, type Ref } from 'vue'
import { t } from '@/i18n'

export function usePageChrome<T>(items: Ref<T[]>, subjectLabel: string) {
  const total = computed(() => items.value.length)
  const translatedSubject = computed(() => t(subjectLabel))
  const nounLabel = computed(() => translatedSubject.value.replace(/^\p{Script=Han}/u, ''))
  const summary = computed(() =>
    total.value > 0
      ? t('Current total: {count} {subject}', { count: total.value, subject: translatedSubject.value })
      : t('No {subject} data', { subject: nounLabel.value })
  )

  return {
    total,
    summary
  }
}
