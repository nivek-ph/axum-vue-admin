<template>
  <div class="auth-page">
    <section class="auth-shell">
      <div class="auth-brand">
        <div class="auth-brand-kicker">axum-vue-admin</div>
        <h1 class="auth-title">管理后台</h1>
        <p class="auth-subtitle">
          Rust + Vue，专注用户、角色、菜单与 API 权限等核心后台能力。
        </p>
      </div>

      <el-card class="login-card" shadow="never">
        <div class="login-card-kicker">Sign In</div>
        <h2 class="login-card-title">登录控制台</h2>
        <p class="login-card-subtitle">使用当前环境里的管理员账户进入核心后台。</p>

        <el-form class="form" @submit.prevent="handleLogin">
          <el-form-item>
            <el-input v-model="form.username" placeholder="用户名" />
          </el-form-item>
          <el-form-item>
            <el-input v-model="form.password" type="password" placeholder="密码" show-password />
          </el-form-item>
          <el-form-item>
            <el-button type="primary" class="w-full" @click="handleLogin" :loading="loading">
              登录并进入后台
            </el-button>
          </el-form-item>
        </el-form>

        <div class="login-note">默认管理员：admin / 123456</div>
      </el-card>
    </section>
  </div>
</template>

<script setup lang="ts">
import { reactive, ref } from 'vue'
import { useRouter } from 'vue-router'
import { ElMessage } from '@/ui/feedback'

import { getMenu, getUserInfo, login } from '@/api/auth'
import { getApiErrorMessage } from '@/api/http'
import { useAuthStore } from '@/stores/auth'
import { buildCoreMenuItems, useMenuStore } from '@/stores/menu'

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
      ElMessage.error(res.message || '登录失败')
      return
    }
    authStore.setSession(res.data.token, res.data.user)
    const [userInfoRes, menuRes] = await Promise.all([
      getUserInfo(res.data.token),
      getMenu(res.data.token)
    ])
    if (userInfoRes.code === 'OK') {
      authStore.setSession(res.data.token, userInfoRes.data.userInfo)
    }
    if (menuRes.code === 'OK') {
      menuStore.setItems(buildCoreMenuItems(menuRes.data.menus || []))
    }
    await router.push('/dashboard')
  } catch (err) {
    ElMessage.error(getApiErrorMessage(err, '登录失败'))
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
