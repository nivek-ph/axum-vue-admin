<template>
  <header class="header">
    <div>
      <div class="header-eyebrow">Desktop Console</div>
      <div class="header-title">{{ currentTitle }}</div>
    </div>

    <div class="header-actions">
      <div class="status-chip">
        <span class="status-dot" />
        <span>Local API</span>
      </div>
      <div class="user-chip">
        <div class="user-avatar">
          {{ userInitial }}
        </div>
        <div>
          <div class="user-name">{{ authStore.userInfo?.nickName || '未登录' }}</div>
          <div class="user-subtitle">
            {{ authStore.userInfo?.authority?.authorityName || '访客' }}
          </div>
        </div>
      </div>
    </div>
  </header>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useRoute } from 'vue-router'

import { useAuthStore } from '@/stores/auth'
import { useMenuStore } from '@/stores/menu'

const authStore = useAuthStore()
const menuStore = useMenuStore()
const route = useRoute()

const currentTitle = computed(() => {
  const matched = menuStore.items.find((item) => item.path === route.path)
  return matched?.label || 'Core Admin'
})

const userInitial = computed(() => {
  const source = authStore.userInfo?.nickName || authStore.userInfo?.userName || 'A'
  return source.slice(0, 1).toUpperCase()
})
</script>

<style scoped>
.header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 18px;
  padding: 22px 24px 16px;
  border-bottom: 1px solid var(--panel-border);
}

.header-eyebrow {
  color: var(--text-muted);
  font-size: 11px;
  text-transform: uppercase;
  letter-spacing: 0.18em;
}

.header-title {
  margin-top: 8px;
  font-size: 28px;
  line-height: 1;
  letter-spacing: -0.03em;
  color: #18231d;
  font-weight: 700;
}

.header-actions {
  display: flex;
  align-items: center;
  gap: 14px;
}

.status-chip,
.user-chip {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 10px 12px;
  border-radius: 12px;
  border: 1px solid var(--panel-border);
  background: #ffffff;
}

.status-chip {
  color: #3f3f46;
  font-size: 12px;
  font-weight: 700;
}

.status-dot {
  width: 8px;
  height: 8px;
  border-radius: 999px;
  background: #22c55e;
}

.user-avatar {
  width: 34px;
  height: 34px;
  border-radius: 10px;
  display: grid;
  place-items: center;
  background: #18181b;
  color: #fafafa;
  font-weight: 700;
}

.user-name {
  color: #1c2923;
  font-size: 14px;
  font-weight: 700;
}

.user-subtitle {
  margin-top: 2px;
  color: var(--text-muted);
  font-size: 12px;
}
</style>
