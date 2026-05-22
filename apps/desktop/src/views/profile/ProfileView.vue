<template>
  <div class="admin-page">
    <section class="page-hero">
      <div class="page-hero-main">
        <span class="page-hero-kicker">Operator Profile</span>
        <h2 class="page-hero-title">个人中心</h2>
        <p class="page-hero-subtitle">这页保留当前登录者的身份概览，作为桌面端控制台的个人入口。</p>

        <div class="page-metrics">
          <div class="page-metric">
            <div class="page-metric-label">昵称</div>
            <div class="page-metric-value">{{ authStore.userInfo?.nickName || '未登录' }}</div>
          </div>
          <div class="page-metric">
            <div class="page-metric-label">用户名</div>
            <div class="page-metric-value">{{ authStore.userInfo?.userName || '-' }}</div>
          </div>
          <div class="page-metric">
            <div class="page-metric-label">当前角色</div>
            <div class="page-metric-value">
              {{ authStore.userInfo?.authority?.authorityName || '访客' }}
            </div>
          </div>
        </div>
      </div>

      <aside class="page-hero-side">
        <div>
          <div class="page-note-label">当前摘要</div>
          <div class="page-note-value">个人中心先承担身份展示，后续再补自助编辑资料与设置。</div>
        </div>
        <div class="page-hero-actions">
          <el-button @click="router.push('/dashboard')">返回仪表盘</el-button>
          <el-button type="primary" @click="router.push('/users')">查看用户列表</el-button>
        </div>
      </aside>
    </section>

    <section class="page-panel">
      <div class="page-panel-header">
        <div>
          <h3 class="page-panel-title">身份卡片</h3>
          <p class="page-panel-subtitle">当前先保留简洁的身份信息，不引入冗余的社交化个人页布局。</p>
        </div>
      </div>

      <div class="profile-card">
        <div class="profile-avatar">
          {{ (authStore.userInfo?.nickName || authStore.userInfo?.userName || 'A').slice(0, 1) }}
        </div>
        <div class="profile-content">
          <div class="profile-name">{{ authStore.userInfo?.nickName || '未登录' }}</div>
          <div class="profile-role">{{ authStore.userInfo?.authority?.authorityName || '访客' }}</div>
          <div class="profile-meta">
            用户 ID：{{ authStore.userInfo?.ID || '-' }} / 默认路由：{{
              authStore.userInfo?.authority?.defaultRouter || 'dashboard'
            }}
          </div>
        </div>
      </div>
    </section>
  </div>
</template>

<script setup lang="ts">
import { useRouter } from 'vue-router'

import { useAuthStore } from '@/stores/auth'

const router = useRouter()
const authStore = useAuthStore()
</script>

<style scoped>
.profile-card {
  display: flex;
  align-items: center;
  gap: 18px;
  padding: 24px;
  border-radius: 24px;
  background: rgba(255, 255, 255, 0.62);
  border: 1px solid rgba(92, 102, 89, 0.08);
}

.profile-avatar {
  width: 72px;
  height: 72px;
  border-radius: 24px;
  display: grid;
  place-items: center;
  background: linear-gradient(145deg, #355c49 0%, #d2b180 100%);
  color: #fffdf8;
  font-size: 28px;
  font-weight: 700;
}

.profile-name {
  color: #1c2923;
  font-size: 24px;
  font-weight: 700;
}

.profile-role {
  margin-top: 6px;
  color: var(--text-secondary);
  font-size: 14px;
}

.profile-meta {
  margin-top: 10px;
  color: var(--text-muted);
  font-size: 13px;
}
</style>
