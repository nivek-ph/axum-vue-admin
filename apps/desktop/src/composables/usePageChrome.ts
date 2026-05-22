import { computed, type Ref } from 'vue'

export function usePageChrome<T>(items: Ref<T[]>, subjectLabel: string) {
  const total = computed(() => items.value.length)
  const summary = computed(() =>
    total.value > 0 ? `当前共 ${total.value} 条${subjectLabel}` : `当前还没有${subjectLabel}数据`
  )

  return {
    total,
    summary
  }
}
