import { useTranslation } from 'react-i18next'

import { useAuthStore } from '@/stores/auth'

export function ProfilePage() {
  const { t } = useTranslation()
  const user = useAuthStore((state) => state.userInfo)
  return (
    <div className="page-stack">
      <header className="page-header"><div><p className="eyebrow">{t('Current account')}</p><h1>{t('Profile')}</h1></div></header>
      <section className="profile-card">
        <div className="avatar">{(user?.nickName || user?.userName || 'A').slice(0, 1)}</div>
        <div><h2>{user?.nickName || t('Not signed in')}</h2><p>{user?.userName}</p><p>{user?.roles?.map((role) => role.name).join(' / ')}</p></div>
      </section>
    </div>
  )
}
