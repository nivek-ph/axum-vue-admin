<template>
  <aside class="sidebar">
    <div class="brand-card">
      <div class="brand-mark">A</div>
      <div>
        <div class="brand-eyebrow">Core Control</div>
        <div class="brand-title">Axum Vue Admin</div>
      </div>
    </div>

    <div class="sidebar-section">
      <div class="sidebar-caption">核心管理</div>
      <nav class="nav">
        <RouterLink
          v-for="item in menuStore.items"
          :key="item.key"
          :to="item.path"
          class="nav-link"
          active-class="is-active"
        >
          <span class="nav-bullet" />
          <span class="nav-label">{{ item.label }}</span>
        </RouterLink>
      </nav>
    </div>
  </aside>
</template>

<script setup lang="ts">
import { useMenuStore } from '@/stores/menu'

const menuStore = useMenuStore()
</script>

<style scoped>
.sidebar {
  display: flex;
  flex-direction: column;
  gap: 24px;
  padding: 24px 18px 18px;
  background: var(--sidebar-bg);
  border-right: 1px solid var(--sidebar-border);
}

.brand-card {
  display: flex;
  align-items: center;
  gap: 14px;
  padding: 16px 14px;
  border-radius: 16px;
  background: rgba(255, 255, 255, 0.03);
  border: 1px solid rgba(255, 255, 255, 0.05);
}

.brand-mark {
  width: 38px;
  height: 38px;
  border-radius: 12px;
  display: grid;
  place-items: center;
  background: #fafafa;
  color: #18181b;
  font-weight: 800;
  font-size: 18px;
}

.brand-eyebrow {
  color: rgba(244, 244, 245, 0.42);
  font-size: 10px;
  letter-spacing: 0.18em;
  text-transform: uppercase;
}

.brand-title {
  margin-top: 4px;
  color: var(--sidebar-strong);
  font-size: 17px;
  font-weight: 700;
  letter-spacing: -0.02em;
}

.sidebar-section {
  flex: 1;
  min-height: 0;
}

.sidebar-caption {
  padding: 0 12px;
  color: rgba(255, 250, 241, 0.4);
  font-size: 11px;
  text-transform: uppercase;
  letter-spacing: 0.18em;
}

.nav {
  display: flex;
  flex-direction: column;
  gap: 8px;
  margin-top: 14px;
}

.nav-link {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 11px 12px;
  border-radius: 12px;
  color: var(--sidebar-text);
  text-decoration: none;
  transition: background-color 160ms ease, color 160ms ease;
}

.nav-link:hover {
  background: rgba(255, 255, 255, 0.04);
  color: var(--sidebar-strong);
}

.nav-link.is-active {
  background: rgba(255, 255, 255, 0.08);
  color: var(--sidebar-strong);
}

.nav-bullet {
  width: 7px;
  height: 7px;
  border-radius: 999px;
  background: rgba(255, 255, 255, 0.22);
  transition: background-color 160ms ease;
}

.nav-link.is-active .nav-bullet {
  background: #fafafa;
}

.nav-label {
  font-size: 14px;
  font-weight: 600;
}

</style>
