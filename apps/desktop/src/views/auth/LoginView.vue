<template>
  <div class="auth-page">
    <section class="auth-shell">
      <div class="auth-brand">
        <div class="auth-brand-kicker">axum-vue-admin</div>
        <h1 class="auth-title">{{ $t('Admin Console') }}</h1>
        <p class="auth-subtitle">
          {{ $t('Rust + Vue admin for users, roles, menus, and API permissions.') }}
        </p>
      </div>

      <UiCard class="login-card" shadow="never">
        <div class="login-card-top">
          <div class="login-card-kicker">{{ $t('Account sign-in') }}</div>
          <LanguageSwitch />
        </div>
        <h2 class="login-card-title">{{ $t('Sign in to console') }}</h2>
        <p class="login-card-subtitle">{{ $t('Use the administrator account for this environment.') }}</p>

        <UiForm class="form" @submit.prevent="handleLogin">
          <UiFormItem>
            <UiInput v-model="form.username" placeholder="Username" />
          </UiFormItem>
          <UiFormItem>
            <UiInput v-model="form.password" type="password" placeholder="Password" showPassword />
          </UiFormItem>
          <UiFormItem>
            <UiButton type="primary" class="w-full" @click="handleLogin" :loading="loading">
              {{ $t('Sign in') }}
            </UiButton>
          </UiFormItem>
        </UiForm>

        <div class="login-note">{{ $t('Default admin: admin / 123456') }}</div>
      </UiCard>
    </section>
  </div>
</template>

<script setup lang="ts">
import { reactive, ref } from 'vue'
import { useRouter } from 'vue-router'
import { ElMessage } from '@/ui/feedback'

import LanguageSwitch from '@/components/LanguageSwitch.vue'
import { getMenu, getUserInfo, login } from '@/api/auth'
import { getApiErrorMessage } from '@/api/http'
import { t } from '@/i18n'
import { useAuthStore } from '@/stores/auth'
import { isSuperAdminAuthority, useMenuStore } from '@/stores/menu'

const router = useRouter()
const authStore = useAuthStore()
const menuStore = useMenuStore()
const loading = ref(false)
const form = reactive({
  username: 'admin',
  password: '123456',
  captcha: '',
  captchaId: ''
})

async function handleLogin() {
  if (loading.value) return
  loading.value = true
  try {
    const res = await login(form)
    if (res.code !== 'OK') {
      ElMessage.error(res.message || t('Sign in failed'))
      return
    }
    const loginToken = res.data?.token
    const loginUser = res.data?.user
    if (!loginToken || !loginUser) {
      authStore.clearToken()
      menuStore.resetAccess()
      ElMessage.error(res.message || t('Sign in failed'))
      return
    }

    authStore.setSession(loginToken, loginUser)
    const [userInfoRes, menuRes] = await Promise.all([
      getUserInfo(loginToken),
      getMenu(loginToken)
    ])
    if (menuRes.code !== 'OK') {
      authStore.clearToken()
      menuStore.resetAccess()
      ElMessage.error(menuRes.message || t('Failed to load menus'))
      return
    }

    const currentUser = userInfoRes.code === 'OK' ? userInfoRes.data?.userInfo || loginUser : loginUser
    authStore.setSession(loginToken, currentUser)
    menuStore.setAuthorizedMenus(menuRes.data?.menus || [], isSuperAdminAuthority(currentUser.authority?.authorityId))
    await router.push(menuStore.firstAuthorizedPath())
  } catch (err) {
    ElMessage.error(getApiErrorMessage(err, t('Sign in failed')))
  } finally {
    loading.value = false
  }
}
</script>

<style scoped>
.auth-page {
  min-height: 100vh;
  display: grid;
  place-items: center;
  padding: 24px;
}

.auth-shell {
  display: grid;
  grid-template-columns: minmax(0, 1.2fr) minmax(360px, 430px);
  gap: 24px;
  width: min(1180px, 100%);
  padding: 16px;
  border-radius: 24px;
  border: 1px solid var(--shell-border);
  background: #fafaf9;
  box-shadow: var(--shell-shadow);
}

.auth-brand {
  display: flex;
  flex-direction: column;
  justify-content: center;
  gap: 12px;
  min-height: 360px;
  padding: 26px;
  border-radius: 20px;
  border: 1px solid #27272a;
  background: #18181b;
  color: #fafafa;
}

.auth-brand-kicker,
.login-card-kicker {
  font-size: 11px;
  text-transform: uppercase;
  letter-spacing: 0.18em;
}

.auth-brand-kicker {
  color: rgba(244, 244, 245, 0.54);
}

.auth-title {
  margin: 0;
  font-size: 38px;
  line-height: 1.08;
  letter-spacing: -0.04em;
}

.auth-subtitle {
  margin: 0;
  max-width: 36ch;
  color: rgba(244, 244, 245, 0.72);
  font-size: 14px;
  line-height: 1.55;
}

.login-card {
  display: flex;
  flex-direction: column;
  justify-content: center;
  padding: 28px 24px !important;
  border-radius: 20px;
  border: 1px solid var(--panel-border);
  background: #ffffff !important;
}

.login-card-kicker {
  color: var(--text-muted);
}

.login-card-top {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
}

.login-card-title {
  margin: 14px 0 0;
  font-size: 32px;
  line-height: 1;
  letter-spacing: -0.04em;
  color: #18181b;
}

.login-card-subtitle {
  margin: 12px 0 0;
  color: var(--text-secondary);
  font-size: 14px;
}

.form {
  margin-top: 24px;
}

.login-note {
  margin-top: 10px;
  color: var(--text-muted);
  font-size: 12px;
}

.w-full {
  width: 100%;
}

@media (max-width: 980px) {
  .auth-shell {
    grid-template-columns: 1fr;
  }

  .auth-brand {
    min-height: auto;
  }

}
</style>
