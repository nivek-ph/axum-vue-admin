<template>
  <div class="admin-page">
    <section class="page-hero">
      <div class="page-hero-main">
        <span class="page-hero-kicker">System Settings</span>
        <h2 class="page-hero-title">系统配置</h2>
        <p class="page-hero-subtitle">以只读配置为主，优先呈现影响后台运行的关键参数，而不是把旧平台所有设置页都搬回来。</p>

        <div class="page-metrics">
          <div class="page-metric">
            <div class="page-metric-label">运行环境</div>
            <div class="page-metric-value">{{ config.system.env || '-' }}</div>
          </div>
          <div class="page-metric">
            <div class="page-metric-label">服务地址</div>
            <div class="page-metric-value">{{ addrLabel }}</div>
          </div>
          <div class="page-metric">
            <div class="page-metric-label">验证码</div>
            <div class="page-metric-value">{{ config.captcha.openCaptcha ? '开启' : '关闭' }}</div>
          </div>
        </div>
      </div>

      <aside class="page-hero-side">
        <div>
          <div class="page-note-label">当前摘要</div>
          <div class="page-note-value">配置页目前保持轻量，只读展示核心运行参数。</div>
        </div>
        <div class="page-hero-actions">
          <el-button @click="loadConfig" :loading="loading">刷新配置</el-button>
          <el-button type="primary" @click="router.push('/system-state')">查看状态</el-button>
        </div>
      </aside>
    </section>

    <section class="dashboard-grid">
      <article class="page-panel">
        <div class="page-panel-header">
          <div>
            <h3 class="page-panel-title">系统参数</h3>
            <p class="page-panel-subtitle">当前环境对桌面端最重要的运行配置。</p>
          </div>
        </div>

        <div class="surface-card state-list">
          <div class="state-row">
            <span>环境</span>
            <strong>{{ config.system.env || '-' }}</strong>
          </div>
          <div class="state-row">
            <span>服务地址</span>
            <strong>{{ config.system.addr || '-' }}</strong>
          </div>
          <div class="state-row">
            <span>数据库</span>
            <strong>{{ config.system['db-type'] || '-' }}</strong>
          </div>
          <div class="state-row">
            <span>多点登录</span>
            <strong>{{ config.system['use-multipoint'] ? '开启' : '关闭' }}</strong>
          </div>
        </div>
      </article>

      <article class="page-panel">
        <div class="page-panel-header">
          <div>
            <h3 class="page-panel-title">本地配置</h3>
            <p class="page-panel-subtitle">与桌面端本地运行直接相关的补充项。</p>
          </div>
        </div>

        <div class="surface-card state-list">
          <div class="state-row">
            <span>存储目录</span>
            <strong>{{ config.local.storePath || '-' }}</strong>
          </div>
          <div class="state-row">
            <span>验证码开关</span>
            <strong>{{ config.captcha.openCaptcha ? '开启' : '关闭' }}</strong>
          </div>
          <div class="state-row">
            <span>验证码超时</span>
            <strong>{{ config.captcha.openCaptchaTimeOut || 0 }}</strong>
          </div>
          <div class="state-row">
            <span>严格鉴权</span>
            <strong>{{ config.system['use-strict-auth'] ? '开启' : '关闭' }}</strong>
          </div>
        </div>
      </article>
    </section>
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted, reactive, ref } from 'vue'
import { useRouter } from 'vue-router'
import { ElMessage } from '@/ui/feedback'

import { fetchSystemConfig, type SystemConfig } from '@/api/system'

const router = useRouter()
const loading = ref(false)
const config = reactive<SystemConfig>({
  system: { env: '', addr: '', 'db-type': '' },
  captcha: { openCaptcha: 0, openCaptchaTimeOut: 0 },
  local: { storePath: '' }
})

const addrLabel = computed(() => {
  if (!config.system.addr) return '-'
  return config.system.addr.replace('127.0.0.1:', ':')
})

async function loadConfig() {
  loading.value = true
  try {
    const nextConfig = await fetchSystemConfig()
    Object.assign(config, nextConfig)
  } catch {
    ElMessage.error('获取系统配置失败')
  } finally {
    loading.value = false
  }
}

onMounted(() => {
  loadConfig()
})
</script>

<style scoped>
.dashboard-grid {
  display: grid;
  grid-template-columns: repeat(2, minmax(0, 1fr));
  gap: 18px;
}

.state-list {
  padding: 6px 18px;
}

.state-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 16px;
  padding: 18px 0;
}

.state-row + .state-row {
  border-top: 1px solid var(--panel-border);
}

.state-row span {
  color: var(--text-secondary);
}

.state-row strong {
  color: var(--text-primary);
}

@media (max-width: 960px) {
  .dashboard-grid {
    grid-template-columns: 1fr;
  }
}
</style>
