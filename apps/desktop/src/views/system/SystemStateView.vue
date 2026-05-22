<template>
  <div class="admin-page">
    <section class="page-hero">
      <div class="page-hero-main">
        <span class="page-hero-kicker">Runtime Status</span>
        <h2 class="page-hero-title">系统状态</h2>
        <p class="page-hero-subtitle">聚合当前服务地址、系统环境和基础资源信息，用于桌面端本地运维视角。</p>

        <div class="page-metrics">
          <div class="page-metric">
            <div class="page-metric-label">运行环境</div>
            <div class="page-metric-value">{{ config.system.env || '-' }}</div>
          </div>
          <div class="page-metric">
            <div class="page-metric-label">CPU 核心</div>
            <div class="page-metric-value">{{ server.os.numCpu || 0 }}</div>
          </div>
          <div class="page-metric">
            <div class="page-metric-label">内存占用</div>
            <div class="page-metric-value">{{ server.ram.usedPercent || 0 }}%</div>
          </div>
        </div>
      </div>

      <aside class="page-hero-side">
        <div>
          <div class="page-note-label">当前摘要</div>
          <div class="page-note-value">{{ summary }}</div>
        </div>
        <div class="page-hero-actions">
          <el-button @click="loadState" :loading="loading">刷新状态</el-button>
          <el-button type="primary" @click="router.push('/system-config')">查看配置</el-button>
        </div>
      </aside>
    </section>

    <section class="dashboard-grid">
      <article class="page-panel">
        <div class="page-panel-header">
          <div>
            <h3 class="page-panel-title">运行配置</h3>
            <p class="page-panel-subtitle">当前从后端读取的系统配置摘要。</p>
          </div>
        </div>

        <div class="surface-card state-list">
          <div class="state-row">
            <span>服务地址</span>
            <strong>{{ config.system.addr || '-' }}</strong>
          </div>
          <div class="state-row">
            <span>数据库类型</span>
            <strong>{{ config.system['db-type'] || '-' }}</strong>
          </div>
          <div class="state-row">
            <span>文件目录</span>
            <strong>{{ config.local.storePath || '-' }}</strong>
          </div>
          <div class="state-row">
            <span>验证码</span>
            <strong>{{ config.captcha.openCaptcha ? '开启' : '关闭' }}</strong>
          </div>
        </div>
      </article>

      <article class="page-panel">
        <div class="page-panel-header">
          <div>
            <h3 class="page-panel-title">系统资源</h3>
            <p class="page-panel-subtitle">基础运行状态，适合作为本地桌面后台的健康视图。</p>
          </div>
        </div>

        <div class="surface-card state-list">
          <div class="state-row">
            <span>操作系统</span>
            <strong>{{ server.os.goos || '-' }}</strong>
          </div>
          <div class="state-row">
            <span>编译器</span>
            <strong>{{ server.os.compiler || '-' }}</strong>
          </div>
          <div class="state-row">
            <span>CPU 负载样本</span>
            <strong>{{ cpuPreview }}</strong>
          </div>
          <div class="state-row">
            <span>磁盘用量</span>
            <strong>{{ diskPreview }}</strong>
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

import {
  fetchServerInfo,
  fetchSystemConfig,
  type ServerInfo,
  type SystemConfig
} from '@/api/system'

const router = useRouter()
const loading = ref(false)
const config = reactive<SystemConfig>({
  system: { env: '', addr: '', 'db-type': '' },
  captcha: { openCaptcha: 0, openCaptchaTimeOut: 0 },
  local: { storePath: '' }
})
const server = reactive<ServerInfo>({
  os: { goos: '', numCpu: 0, compiler: '', goVersion: '', numGoroutine: 0 },
  cpu: { cores: 0, cpus: [] },
  ram: { totalMb: 0, usedMb: 0, usedPercent: 0 },
  disk: []
})

const summary = computed(() => {
  const addr = config.system.addr || '未配置'
  return `当前服务地址 ${addr}，内存使用 ${server.ram.usedPercent || 0}%`
})
const cpuPreview = computed(() => server.cpu.cpus.slice(0, 4).join(' / ') || '-')
const diskPreview = computed(() => {
  const disk = server.disk[0]
  if (!disk) return '-'
  return `${disk.mountPoint} · ${disk.usedGb}/${disk.totalGb} GB`
})

async function loadState() {
  loading.value = true
  try {
    const [nextConfig, nextServer] = await Promise.all([fetchSystemConfig(), fetchServerInfo()])
    Object.assign(config, nextConfig)
    Object.assign(server, nextServer)
  } catch {
    ElMessage.error('获取系统状态失败')
  } finally {
    loading.value = false
  }
}

onMounted(() => {
  loadState()
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
