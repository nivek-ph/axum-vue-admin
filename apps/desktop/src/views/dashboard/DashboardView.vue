<template>
  <div class="admin-page">
    <section class="page-hero">
      <div class="page-hero-main">
        <span class="page-hero-kicker">Overview</span>
        <h2 class="page-hero-title">核心控制台</h2>
        <p class="page-hero-subtitle">
          桌面端只保留核心 admin 能力。你现在看到的是一套收敛后的后台壳，角色、菜单、API 和用户管理是当前主轴。
        </p>

        <div class="page-metrics">
          <div class="page-metric">
            <div class="page-metric-label">可用入口</div>
            <div class="page-metric-value">{{ menuStore.items.length }}</div>
          </div>
          <div class="page-metric">
            <div class="page-metric-label">当前身份</div>
            <div class="page-metric-value">{{ currentRole }}</div>
          </div>
        </div>
      </div>

      <aside class="page-hero-side">
        <div>
          <div class="page-note-label">当前摘要</div>
          <div class="page-note-value">壳层、登录、用户、角色、菜单、API 已统一到同一套视觉语言。</div>
        </div>
        <div class="page-hero-actions">
          <el-button @click="router.push('/users')">用户管理</el-button>
          <el-button type="primary" @click="router.push('/roles')">角色管理</el-button>
        </div>
      </aside>
    </section>

    <section class="dashboard-grid">
      <article class="page-panel">
        <div class="page-panel-header">
          <div>
            <h3 class="page-panel-title">快捷入口</h3>
            <p class="page-panel-subtitle">优先处理当前已经接好的核心页面。</p>
          </div>
        </div>

        <div class="shortcut-grid">
          <button
            v-for="item in quickLinks"
            :key="item.path"
            class="shortcut-card"
            type="button"
            @click="router.push(item.path)"
          >
            <div class="shortcut-title">{{ item.title }}</div>
            <div class="shortcut-subtitle">{{ item.description }}</div>
          </button>
        </div>
      </article>

    </section>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useRouter } from 'vue-router'

import { useAuthStore } from '@/stores/auth'
import { useMenuStore } from '@/stores/menu'

const router = useRouter()
const authStore = useAuthStore()
const menuStore = useMenuStore()

const currentRole = computed(() => authStore.userInfo?.authority?.authorityName || '访客')

const quickLinks = [
  { path: '/users', title: '用户管理', description: '查看用户状态、重置密码、清理账号。' },
  { path: '/roles', title: '角色管理', description: '维护角色树、默认路由和成员归属。' },
  { path: '/menus', title: '菜单管理', description: '整理桌面端菜单结构和角色可见范围。' },
  { path: '/apis', title: 'API 管理', description: '登记核心接口并分配角色访问权限。' }
]
</script>

<style scoped>
.dashboard-grid {
  display: grid;
  grid-template-columns: repeat(2, minmax(0, 1fr));
  gap: 18px;
}

.shortcut-grid {
  display: grid;
  grid-template-columns: repeat(2, minmax(0, 1fr));
  gap: 14px;
}

.shortcut-card {
  padding: 18px;
  border: 1px solid rgba(92, 102, 89, 0.08);
  border-radius: 22px;
  background: rgba(255, 255, 255, 0.62);
  text-align: left;
  cursor: pointer;
  transition: transform 160ms ease, box-shadow 160ms ease, border-color 160ms ease;
}

.shortcut-card:hover {
  transform: translateY(-2px);
  border-color: rgba(55, 95, 75, 0.18);
  box-shadow: 0 18px 32px rgba(67, 75, 68, 0.08);
}

.shortcut-title {
  color: #1c2923;
  font-size: 16px;
  font-weight: 700;
}

.shortcut-subtitle {
  margin-top: 8px;
  color: var(--text-secondary);
  font-size: 13px;
}

@media (max-width: 960px) {
  .dashboard-grid,
  .shortcut-grid {
    grid-template-columns: 1fr;
  }
}
</style>
