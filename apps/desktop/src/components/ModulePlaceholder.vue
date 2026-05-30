<template>
  <div class="admin-page">
    <section class="page-hero">
      <div class="page-hero-main">
        <span class="page-hero-kicker">{{ $t(kicker) }}</span>
        <h2 class="page-hero-title">{{ $t(title) }}</h2>
        <p class="page-hero-subtitle">{{ $t(description) }}</p>

        <div class="page-metrics">
          <div class="page-metric">
            <div class="page-metric-label">{{ $t('Current status') }}</div>
            <div class="page-metric-value">{{ $t(statusLabel) }}</div>
          </div>
          <div class="page-metric">
            <div class="page-metric-label">{{ $t('Focus') }}</div>
            <div class="page-metric-value">{{ $t(focusLabel) }}</div>
          </div>
          <div class="page-metric">
            <div class="page-metric-label">{{ $t('Next step') }}</div>
            <div class="page-metric-value">{{ $t(nextLabel) }}</div>
          </div>
        </div>
      </div>

      <aside class="page-hero-side">
        <div>
          <div class="page-note-label">{{ $t('Delivery note') }}</div>
          <div class="page-note-value">{{ $t(note) }}</div>
        </div>
        <div class="page-hero-actions">
          <UiButton @click="router.back()">{{ $t('Back') }}</UiButton>
          <UiButton type="primary" @click="router.push(primaryActionPath)">
            {{ $t(primaryActionLabel) }}
          </UiButton>
        </div>
      </aside>
    </section>

    <section class="page-panel">
      <div class="page-panel-header">
        <div>
          <h3 class="page-panel-title">{{ $t('Module plan') }}</h3>
          <p class="page-panel-subtitle">{{ $t('This page keeps the entry point while core data flows are filled in.') }}</p>
        </div>
      </div>

      <div class="placeholder-grid">
        <article class="placeholder-card">
          <div class="placeholder-card-label">{{ $t('Kept here') }}</div>
          <ul class="placeholder-list">
            <li v-for="item in keeps" :key="item">{{ $t(item) }}</li>
          </ul>
        </article>

        <article class="placeholder-card">
          <div class="placeholder-card-label">{{ $t('Not included') }}</div>
          <ul class="placeholder-list">
            <li v-for="item in skips" :key="item">{{ $t(item) }}</li>
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
    statusLabel: 'Planned',
    focusLabel: 'Core capability',
    nextLabel: 'Connect backend API',
    primaryActionLabel: 'Back to dashboard',
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
