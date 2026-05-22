<template>
  <div class="admin-page">
    <section class="page-hero">
      <div class="page-hero-main">
        <span class="page-hero-kicker">System Params</span>
        <h2 class="page-hero-title">参数管理</h2>
        <p class="page-hero-subtitle">维护核心后台运行所需的系统参数，只保留键值配置的核心场景。</p>

        <div class="page-metrics">
          <div class="page-metric">
            <div class="page-metric-label">当前页条目</div>
            <div class="page-metric-value">{{ params.length }}</div>
          </div>
          <div class="page-metric">
            <div class="page-metric-label">总参数数</div>
            <div class="page-metric-value">{{ total }}</div>
          </div>
          <div class="page-metric">
            <div class="page-metric-label">命名空间</div>
            <div class="page-metric-value">{{ namespaceCount }}</div>
          </div>
        </div>
      </div>

      <aside class="page-hero-side">
        <div>
          <div class="page-note-label">模块摘要</div>
          <div class="page-note-value">{{ summary }}</div>
        </div>
        <div class="page-hero-actions">
          <el-button @click="loadParams" :loading="loading">刷新参数</el-button>
          <el-button type="primary" @click="openCreateDialog">新增参数</el-button>
        </div>
      </aside>
    </section>

    <section class="page-panel">
      <div class="page-panel-header">
        <div>
          <h3 class="page-panel-title">参数列表</h3>
          <p class="page-panel-subtitle">支持按名称与键过滤，保留最常用的 CRUD 流程。</p>
        </div>
      </div>

      <div class="page-panel-toolbar inline-filter">
        <el-input v-model="filters.name" placeholder="按名称过滤" clearable />
        <el-input v-model="filters.key" placeholder="按键过滤" clearable />
        <el-button type="primary" @click="handleSearch">查询</el-button>
      </div>

      <div class="surface-card">
        <el-table :data="params" v-loading="loading" style="width: 100%">
          <el-table-column prop="ID" label="ID" width="80" />
          <el-table-column prop="name" label="名称" min-width="160" />
          <el-table-column prop="key" label="键" min-width="180" />
          <el-table-column prop="value" label="值" min-width="180" />
          <el-table-column prop="desc" label="说明" min-width="180" />
          <el-table-column label="操作" width="180">
            <template #default="{ row }">
              <el-button link type="primary" @click="openEditDialog(row)">编辑</el-button>
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

    <el-dialog v-model="dialogVisible" :title="dialogMode === 'create' ? '新增参数' : '编辑参数'" width="560px">
      <el-form label-width="90px" @submit.prevent="submitParam">
        <el-form-item label="名称">
          <el-input v-model="form.name" placeholder="例如：站点名称" />
        </el-form-item>
        <el-form-item label="键">
          <el-input v-model="form.key" placeholder="例如：site.name" />
        </el-form-item>
        <el-form-item label="值">
          <el-input v-model="form.value" placeholder="例如：Core Admin" />
        </el-form-item>
        <el-form-item label="说明">
          <el-input v-model="form.desc" type="textarea" :rows="3" placeholder="参数说明" />
        </el-form-item>
      </el-form>

      <template #footer>
        <el-button @click="dialogVisible = false">取消</el-button>
        <el-button type="primary" :loading="submitting" @click="submitParam">保存</el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted, reactive, ref } from 'vue'
import { ElMessage, ElMessageBox } from '@/ui/feedback'

import { usePageChrome } from '@/composables/usePageChrome'
import { createParam, deleteParam, fetchParams, updateParam, type ParamRecord } from '@/api/params'

type DialogMode = 'create' | 'edit'

const params = ref<ParamRecord[]>([])
const loading = ref(false)
const submitting = ref(false)
const dialogVisible = ref(false)
const dialogMode = ref<DialogMode>('create')
const total = ref(0)
const page = ref(1)
const pageSize = ref(10)
const filters = reactive({
  name: '',
  key: ''
})
const form = reactive<ParamRecord>({
  ID: 0,
  name: '',
  key: '',
  value: '',
  desc: ''
})
const { summary } = usePageChrome(params, '条参数')
const namespaceCount = computed(
  () => new Set(params.value.map((item) => item.key.split('.').shift()).filter(Boolean)).size
)

function resetForm() {
  form.ID = 0
  form.name = ''
  form.key = ''
  form.value = ''
  form.desc = ''
}

async function loadParams() {
  loading.value = true
  try {
    const result = await fetchParams({
      page: page.value,
      pageSize: pageSize.value,
      name: filters.name,
      key: filters.key
    })
    params.value = result.list
    total.value = result.total
  } catch {
    ElMessage.error('获取参数列表失败')
  } finally {
    loading.value = false
  }
}

function handleSearch() {
  page.value = 1
  loadParams()
}

function handlePageChange(nextPage: number) {
  page.value = nextPage
  loadParams()
}

function openCreateDialog() {
  dialogMode.value = 'create'
  resetForm()
  dialogVisible.value = true
}

function openEditDialog(item: ParamRecord) {
  dialogMode.value = 'edit'
  Object.assign(form, item)
  dialogVisible.value = true
}

async function submitParam() {
  if (!form.name.trim() || !form.key.trim()) {
    ElMessage.warning('请填写完整参数信息')
    return
  }

  submitting.value = true
  try {
    const payload = {
      ID: form.ID,
      name: form.name.trim(),
      key: form.key.trim(),
      value: form.value.trim(),
      desc: form.desc.trim()
    }
    const response =
      dialogMode.value === 'create' ? await createParam(payload) : await updateParam(payload)

    if (response.code === 'OK') {
      ElMessage.success(dialogMode.value === 'create' ? '参数已创建' : '参数已更新')
      dialogVisible.value = false
      await loadParams()
      return
    }
    ElMessage.error(response.message || '保存参数失败')
  } catch {
    ElMessage.error('保存参数失败')
  } finally {
    submitting.value = false
  }
}

async function handleDelete(id: number) {
  try {
    await ElMessageBox.confirm('确定删除该参数吗？', '提示', { type: 'warning' })
  } catch {
    return
  }

  try {
    const response = await deleteParam(id)
    if (response.code === 'OK') {
      ElMessage.success('参数已删除')
      await loadParams()
      return
    }
    ElMessage.error(response.message || '删除参数失败')
  } catch {
    ElMessage.error('删除参数失败')
  }
}

onMounted(() => {
  loadParams()
})
</script>

<style scoped>
.pagination {
  display: flex;
  justify-content: flex-end;
  margin-top: 16px;
}
</style>
