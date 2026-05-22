<template>
  <div class="admin-page">
    <section class="page-hero">
      <div class="page-hero-main">
        <span class="page-hero-kicker">{{ kicker }}</span>
        <h2 class="page-hero-title">{{ title }}</h2>
        <p class="page-hero-subtitle">{{ description }}</p>

        <div class="page-metrics">
          <div class="page-metric">
            <div class="page-metric-label">当前状态</div>
            <div class="page-metric-value">{{ statusLabel }}</div>
          </div>
          <div class="page-metric">
            <div class="page-metric-label">定位</div>
            <div class="page-metric-value">{{ focusLabel }}</div>
          </div>
          <div class="page-metric">
            <div class="page-metric-label">后续动作</div>
            <div class="page-metric-value">{{ nextLabel }}</div>
          </div>
        </div>
      </div>

      <aside class="page-hero-side">
        <div>
          <div class="page-note-label">交付说明</div>
          <div class="page-note-value">{{ note }}</div>
        </div>
        <div class="page-hero-actions">
          <el-button @click="router.back()">返回上一页</el-button>
          <el-button type="primary" @click="router.push(primaryActionPath)">
            {{ primaryActionLabel }}
          </el-button>
        </div>
      </aside>
    </section>

    <section class="page-panel">
      <div class="page-panel-header">
        <div>
          <h3 class="page-panel-title">模块规划</h3>
          <p class="page-panel-subtitle">视觉层已收口，数据交互后续按核心 admin 范围继续补齐。</p>
        </div>
      </div>

      <div class="placeholder-grid">
        <article class="placeholder-card">
          <div class="placeholder-card-label">本页保留</div>
          <ul class="placeholder-list">
            <li v-for="item in keeps" :key="item">{{ item }}</li>
          </ul>
        </article>

        <article class="placeholder-card">
          <div class="placeholder-card-label">本页不再做</div>
          <ul class="placeholder-list">
            <li v-for="item in skips" :key="item">{{ item }}</li>
          </ul>
        </article>
      </div>
    </section>
  </div>
</template>

<script setup lang="ts">
import { useRouter } from 'vue-router'

withDefaults(
  defineProps<{
    kicker: string
    title: string
    description: string
    note: string
    keeps: string[]
    skips: string[]
    statusLabel?: string
    focusLabel?: string
    nextLabel?: string
    primaryActionLabel?: string
    primaryActionPath?: string
  }>(),
  {
    statusLabel: '规划中',
    focusLabel: '核心能力',
    nextLabel: '接后端接口',
    primaryActionLabel: '返回仪表盘',
    primaryActionPath: '/dashboard'
  }
)

const router = useRouter()
</script>

<style scoped>
.placeholder-grid {
  display: grid;
  grid-template-columns: repeat(2, minmax(0, 1fr));
  gap: 16px;
}

.placeholder-card {
  padding: 20px;
  border-radius: 22px;
  border: 1px solid rgba(92, 102, 89, 0.08);
  background: rgba(255, 255, 255, 0.62);
}

.placeholder-card-label {
  color: #23312b;
  font-size: 14px;
  font-weight: 700;
}

.placeholder-list {
  margin: 12px 0 0;
  padding-left: 18px;
  color: var(--text-secondary);
}

.placeholder-list li + li {
  margin-top: 8px;
}

@media (max-width: 960px) {
  .placeholder-grid {
    grid-template-columns: 1fr;
  }
}
</style>
