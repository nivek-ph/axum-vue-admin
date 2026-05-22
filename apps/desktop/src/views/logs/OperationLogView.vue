<template>
  <div class="admin-page">
    <section class="page-hero">
      <div class="page-hero-main">
        <span class="page-hero-kicker">Operation Trail</span>
        <h2 class="page-hero-title">操作日志</h2>
        <p class="page-hero-subtitle">追踪后台主要操作的请求路径、方法、状态码和执行用户，服务核心治理排查。</p>

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
            <div class="page-metric-label">异常状态</div>
            <div class="page-metric-value">{{ errorCount }}</div>
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
          <h3 class="page-panel-title">操作审计</h3>
          <p class="page-panel-subtitle">聚焦 method / path / status 三个核心维度，不做过载式审计界面。</p>
        </div>
      </div>

      <div class="page-panel-toolbar inline-filter">
        <el-select v-model="filters.method" clearable placeholder="方法">
          <el-option v-for="item in methodOptions" :key="item" :label="item" :value="item" />
        </el-select>
        <el-input v-model="filters.path" placeholder="按路径过滤" clearable />
        <el-select v-model="statusModel" clearable placeholder="状态码">
          <el-option label="200" :value="200" />
          <el-option label="401" :value="401" />
          <el-option label="500" :value="500" />
        </el-select>
        <el-button type="primary" @click="handleSearch">查询</el-button>
      </div>

      <div class="surface-card">
        <el-table :data="logs" v-loading="loading" style="width: 100%" @selection-change="handleSelectionChange">
          <el-table-column type="selection" width="44" />
          <el-table-column prop="ID" label="ID" width="80" />
          <el-table-column prop="method" label="方法" width="100">
            <template #default="{ row }">
              <el-tag :type="row.status >= 400 ? 'danger' : row.method === 'GET' ? 'success' : 'primary'">
                {{ row.method }}
              </el-tag>
            </template>
          </el-table-column>
          <el-table-column prop="path" label="路径" min-width="240" />
          <el-table-column prop="status" label="状态" width="100" />
          <el-table-column label="用户" min-width="140">
            <template #default="{ row }">
              {{ row.user?.nickName || row.user?.userName || '-' }}
            </template>
          </el-table-column>
          <el-table-column prop="CreatedAt" label="时间" min-width="180" />
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
import { deleteOperationLogs, fetchOperationLogs, type OperationLogRecord } from '@/api/logs'

const methodOptions = ['GET', 'POST', 'PUT', 'DELETE']
const logs = ref<OperationLogRecord[]>([])
const loading = ref(false)
const total = ref(0)
const page = ref(1)
const pageSize = ref(10)
const filters = reactive({
  method: '',
  path: ''
})
const statusModel = ref<number | undefined>()
const selectedIds = ref<number[]>([])
const { summary } = usePageChrome(logs, '条操作记录')
const errorCount = computed(() => logs.value.filter((item) => item.status >= 400).length)

async function loadLogs() {
  loading.value = true
  try {
    const result = await fetchOperationLogs({
      page: page.value,
      pageSize: pageSize.value,
      method: filters.method,
      path: filters.path,
      status: statusModel.value
    })
    logs.value = result.list
    total.value = result.total
  } catch {
    ElMessage.error('获取操作日志失败')
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

function handleSelectionChange(rows: OperationLogRecord[]) {
  selectedIds.value = rows.map((row) => row.ID)
}

async function handleBatchDelete() {
  if (selectedIds.value.length === 0) return

  try {
    await ElMessageBox.confirm(`确定批量删除 ${selectedIds.value.length} 条操作日志吗？`, '提示', {
      type: 'warning'
    })
  } catch {
    return
  }

  try {
    const response = await deleteOperationLogs(selectedIds.value)
    if (response.code === 'OK') {
      ElMessage.success('操作日志已批量删除')
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
