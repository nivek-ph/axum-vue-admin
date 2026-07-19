import { zodResolver } from '@hookform/resolvers/zod'
import { useEffect, useState } from 'react'
import { useForm } from 'react-hook-form'
import { useTranslation } from 'react-i18next'
import { useNavigate } from 'react-router-dom'
import { toast } from 'sonner'
import { z } from 'zod'

import { fetchCaptcha, getCurrentMenu, getUserInfo, login, type CaptchaData } from '@/api/auth'
import { getApiErrorMessage } from '@/api/http'
import { Button } from '@/components/ui/Button'
import { isSuperAdmin, useAuthStore } from '@/stores/auth'
import { useMenuStore } from '@/stores/menu'

const schema = z.object({
  username: z.string().trim().min(1),
  password: z.string().min(1),
  captcha: z.string(),
})

type FormValues = z.infer<typeof schema>

export function LoginPage() {
  const { t } = useTranslation()
  const navigate = useNavigate()
  const [captcha, setCaptcha] = useState<CaptchaData | null>(null)
  const [submitting, setSubmitting] = useState(false)
  const setSession = useAuthStore((state) => state.setSession)
  const setUserAndPermissions = useAuthStore((state) => state.setUserAndPermissions)
  const setAuthorizedMenus = useMenuStore((state) => state.setAuthorizedMenus)
  const { register, handleSubmit } = useForm<FormValues>({ resolver: zodResolver(schema), defaultValues: { username: '', password: '', captcha: '' } })

  async function loadCaptcha() {
    try {
      const response = await fetchCaptcha()
      if (response.code === 'OK') setCaptcha(response.data ?? null)
      else toast.error(response.message)
    } catch (error) {
      toast.error(getApiErrorMessage(error, t('Sign in failed')))
    }
  }

  useEffect(() => { void loadCaptcha() }, [])

  async function submit(values: FormValues) {
    setSubmitting(true)
    try {
      const response = await login({ ...values, captchaId: captcha?.captchaId ?? '' })
      if (response.code !== 'OK' || !response.data) throw new Error(response.message || t('Sign in failed'))
      setSession({ accessToken: response.data.accessToken, refreshToken: response.data.refreshToken, userInfo: response.data.user })
      const [userResponse, menuResponse] = await Promise.all([
        getUserInfo(response.data.accessToken),
        getCurrentMenu(response.data.accessToken),
      ])
      const user = userResponse.data?.userInfo ?? response.data.user
      if (userResponse.code !== 'OK' || menuResponse.code !== 'OK') throw new Error(t('Sign in failed'))
      setUserAndPermissions(user, menuResponse.data?.permissions ?? [])
      setAuthorizedMenus(menuResponse.data?.menus ?? [], isSuperAdmin(user))
      const routeName = user.homeRoute?.replace(/^\/+/, '') || 'dashboard'
      const menu = useMenuStore.getState()
      navigate(menu.canAccess(routeName) ? `/${routeName}` : menu.firstAuthorizedPath(), { replace: true })
    } catch (error) {
      useAuthStore.getState().clearSession()
      useMenuStore.getState().resetAccess()
      toast.error(getApiErrorMessage(error, t('Sign in failed')))
      await loadCaptcha()
    } finally {
      setSubmitting(false)
    }
  }

  return (
    <main className="login-page">
      <section className="login-brand">
        <div className="brand-mark">A</div>
        <p className="eyebrow">AXUM · REACT</p>
        <h1>{t('Admin Console')}</h1>
        <p>{t('Secure identity, access, configuration, and audit operations in one focused workspace.')}</p>
      </section>
      <section className="login-card">
        <p className="eyebrow">{t('Welcome back')}</p>
        <h2>{t('Sign in')}</h2>
        <form onSubmit={handleSubmit(submit)}>
          <label>{t('Username')}<input autoComplete="username" {...register('username')} /></label>
          <label>{t('Password')}<input autoComplete="current-password" type="password" {...register('password')} /></label>
          {captcha?.openCaptcha && (
            <div className="captcha-row">
              <label>{t('Captcha')}<input {...register('captcha')} /></label>
              <button aria-label="Reload captcha" className="captcha-image" onClick={() => void loadCaptcha()} type="button">
                <img alt="Captcha" src={captcha.picPath} />
              </button>
            </div>
          )}
          <Button disabled={submitting} type="submit" variant="primary">{submitting ? '…' : t('Sign in')}</Button>
        </form>
      </section>
    </main>
  )
}
