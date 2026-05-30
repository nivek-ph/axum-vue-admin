import { ref } from 'vue'
import { beforeEach, describe, expect, it } from 'vitest'

import { usePageChrome } from './usePageChrome'
import { setLocale } from '@/i18n'

describe('usePageChrome', () => {
  beforeEach(() => {
    setLocale('zh-CN')
  })

  it('formats count summaries without duplicating classifiers', () => {
    const users = ref([{}, {}, {}])

    const { total, summary } = usePageChrome(users, 'users')

    expect(total.value).toBe(3)
    expect(summary.value).toBe('当前共 3 位用户')
  })

  it('formats empty summaries with the noun only', () => {
    const apis = ref([])

    const { summary } = usePageChrome(apis, 'APIs')

    expect(summary.value).toBe('暂无接口数据')
  })

  it('formats summaries in English when locale changes', () => {
    const users = ref([{}])
    const { summary } = usePageChrome(users, 'users')

    setLocale('en-US')

    expect(summary.value).toBe('Current total: 1 users')
  })
})
