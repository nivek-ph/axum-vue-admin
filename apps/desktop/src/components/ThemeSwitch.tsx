import { IconMoon, IconSun } from '@tabler/icons-react'
import { useTranslation } from 'react-i18next'

import { Button } from '@/components/ui/Button'
import { useThemeStore } from '@/stores/theme'

export function ThemeSwitch() {
  const { t } = useTranslation()
  const mode = useThemeStore((state) => state.mode)
  const toggleMode = useThemeStore((state) => state.toggleMode)

  return (
    <Button
      aria-label={mode === 'light' ? t('Dark mode') : t('Light mode')}
      onClick={toggleMode}
      size="icon-sm"
      variant="ghost"
    >
      {mode === 'light' ? <IconMoon /> : <IconSun />}
    </Button>
  )
}
