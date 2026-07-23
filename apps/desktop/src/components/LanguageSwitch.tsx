import { IconLanguage } from '@tabler/icons-react'
import { useTranslation } from 'react-i18next'

import { Button } from '@/components/ui/Button'
import { toggleLocale } from '@/i18n'

export function LanguageSwitch() {
  const { i18n } = useTranslation()
  return (
    <Button
      aria-label="Switch language"
      className="text-sm"
      onClick={() => void toggleLocale()}
      size="sm"
      variant="ghost"
    >
      <IconLanguage />
      {i18n.language === 'zh-CN' ? '中' : 'EN'}
    </Button>
  )
}
