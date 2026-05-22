<template>
  <div class="admin-page">
    <section class="page-hero">
      <div class="page-hero-main">
        <span class="page-hero-kicker">Audit Trail</span>
        <h2 class="page-hero-title">登录日志</h2>
        <p class="page-hero-subtitle">查看后台登录成功与失败记录，快速定位登录异常和账号使用情况。</p>

        <div class="page-metrics">
          <div class="page-metric">
            <div class="page-metric-label">当前页条目</div>
            <div class="page-metric-value">{{ logs.length }}</div>
          </div>
          <div class="page-metric">
            <div class="page-metric-label">总日志数</div>
            <div class="page-metric-value">{{ total }}</div>
          </div>
          <div class="page-metric">
            <div class="page-metric-label">失败记录</div>
            <div class="page-metric-value">{{ failedCount }}</div>
          </div>
        </div>
      </div>

      <aside class="page-hero-side">
        <div>
          <div class="page-note-label">模块摘要</div>
          <div class="page-note-value">{{ summary }}</div>
        </div>
        <div class="page-hero-actions">
          <el-button @click="loadLogs" :loading="loading">刷新日志</el-button>
          <el-button :disabled="selectedIds.length === 0" @click="handleBatchDelete">
            批量删除
          </el-button>
          <el-button type="primary" @click="page = 1; loadLogs()">重新检索</el-button>
        </div>
      </aside>
    </section>

    <section class="page-panel">
      <div class="page-panel-header">
        <div>
          <h3 class="page-panel-title">登录审计</h3>
          <p class="page-panel-subtitle">只保留核心筛选字段，避免审计页信息噪音过高。</p>
        </div>
      </div>

      <div class="page-panel-toolbar inline-filter">
        <el-input v-model="filters.username" placeholder="按用户名过滤" clearable />
        <el-select v-model="statusModel" clearable placeholder="登录状态">
          <el-option label="成功" value="success" />
          <el-option label="失败" value="failed" />
        </el-select>
        <el-button type="primary" @click="handleSearch">查询</el-button>
      </div>

      <div class="surface-card">
        <el-table :data="logs" v-loading="loading" style="width: 100%" @selection-change="handleSelectionChange">
          <el-table-column type="selection" width="44" />
          <el-table-column prop="ID" label="ID" width="80" />
          <el-table-column prop="username" label="用户名" min-width="140" />
          <el-table-column prop="ip" label="IP" min-width="140" />
          <el-table-column label="状态" width="100">
            <template #default="{ row }">
              <el-tag :type="row.status ? 'success' : 'danger'">
                {{ row.status ? '成功' : '失败' }}
              </el-tag>
            </template>
          </el-table-column>
          <el-table-column prop="errorMessage" label="错误信息" min-width="180" />
          <el-table-column prop="CreatedAt" label="时间" min-width="180" />
          <el-table-column label="操作" width="120">
            <template #default="{ row }">
              <el-button link type="danger" @click="handleDelete(row.ID)">删除</el-button>
            </template>
          </el-table-column>
        </el-table>
      </div>

      <div class="pagination">
        <el-pagination
          background
          layout="total, prev, pager, next"
          :total="total"
          :current-page="page"
          :page-size="pageSize"
          @current-change="handlePageChange"
        />
      </div>
    </section>
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted, reactive, ref } from 'vue'
import { ElMessage, ElMessageBox } from '@/ui/feedback'

import { usePageChrome } from '@/composables/usePageChrome'
import { deleteLoginLog, deleteLoginLogs, fetchLoginLogs, type LoginLogRecord } from '@/api/logs'

const logs = ref<LoginLogRecord[]>([])
const loading = ref(false)
const total = ref(0)
const page = ref(1)
const pageSize = ref(10)
const filters = reactive({
  username: ''
})
const statusModel = ref<'success' | 'failed' | undefined>()
const selectedIds = ref<number[]>([])
const { summary } = usePageChrome(logs, '条登录记录')
const failedCount = computed(() => logs.value.filter((item) => !item.status).length)

async function loadLogs() {
  loading.value = true
  try {
    const result = await fetchLoginLogs({
      page: page.value,
      pageSize: pageSize.value,
      username: filters.username,
      status:
        statusModel.value === undefined
          ? undefined
          : statusModel.value === 'success'
    })
    logs.value = result.list
    total.value = result.total
  } catch {
    ElMessage.error('获取登录日志失败')
  } finally {
    loading.value = false
  }
}

function handleSearch() {
  page.value = 1
  loadLogs()
}

function handlePageChange(nextPage: number) {
  page.value = nextPage
  loadLogs()
}

function handleSelectionChange(rows: LoginLogRecord[]) {
  selectedIds.value = rows.map((row) => row.ID)
}

async function handleDelete(id: number) {
  try {
    await ElMessageBox.confirm('确定删除这条登录日志吗？', '提示', { type: 'warning' })
  } catch {
    return
  }

  try {
    const response = await deleteLoginLog(id)
    if (response.code === 'OK') {
      ElMessage.success('登录日志已删除')
      await loadLogs()
      return
    }
    ElMessage.error(response.message || '删除失败')
  } catch {
    ElMessage.error('删除失败')
  }
}

async function handleBatchDelete() {
  if (selectedIds.value.length === 0) return

  try {
    await ElMessageBox.confirm(`确定批量删除 ${selectedIds.value.length} 条登录日志吗？`, '提示', {
      type: 'warning'
    })
  } catch {
    return
  }

  try {
    const response = await deleteLoginLogs(selectedIds.value)
    if (response.code === 'OK') {
      ElMessage.success('登录日志已批量删除')
      selectedIds.value = []
      await loadLogs()
      return
    }
    ElMessage.error(response.message || '批量删除失败')
  } catch {
    ElMessage.error('批量删除失败')
  }
}

onMounted(() => {
  loadLogs()
})
</script>

<style scoped>
.pagination {
  display: flex;
  justify-content: flex-end;
  margin-top: 16px;
}
</style>
