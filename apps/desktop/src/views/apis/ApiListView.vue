<template>
  <div class="admin-page">
    <section class="page-hero">
      <div class="page-hero-main">
        <span class="page-hero-kicker">API Registry</span>
        <h2 class="page-hero-title">API 管理</h2>
        <p class="page-hero-subtitle">围绕核心 admin 接口做注册、检索和角色授权，不再把非核心平台接口拉回桌面端。</p>

        <div class="page-metrics">
          <div class="page-metric">
            <div class="page-metric-label">当前页条目</div>
            <div class="page-metric-value">{{ apis.length }}</div>
          </div>
          <div class="page-metric">
            <div class="page-metric-label">接口总数</div>
            <div class="page-metric-value">{{ total }}</div>
          </div>
          <div class="page-metric">
            <div class="page-metric-label">接口分组</div>
            <div class="page-metric-value">{{ apiGroups.length }}</div>
          </div>
        </div>
      </div>

      <aside class="page-hero-side">
        <div>
          <div class="page-note-label">模块摘要</div>
          <div class="page-note-value">{{ summary }}</div>
        </div>
        <div class="page-hero-actions">
          <el-button @click="loadApis" :loading="loading">刷新接口</el-button>
          <el-button type="primary" @click="openCreateDialog">新增 API</el-button>
        </div>
      </aside>
    </section>

    <section class="page-panel">
      <div class="page-panel-header">
        <div>
          <h3 class="page-panel-title">接口目录</h3>
          <p class="page-panel-subtitle">按路径、描述与分组筛选，列表保持紧凑可读。</p>
        </div>
      </div>

      <div class="page-panel-toolbar inline-filter">
        <el-input v-model="filters.path" placeholder="按路径过滤" clearable />
        <el-input v-model="filters.description" placeholder="按描述过滤" clearable />
        <el-select v-model="filters.apiGroup" clearable placeholder="分组">
          <el-option v-for="group in apiGroups" :key="group" :label="group" :value="group" />
        </el-select>
        <el-select v-model="filters.method" clearable placeholder="方法">
          <el-option v-for="item in methodOptions" :key="item" :label="item" :value="item" />
        </el-select>
        <el-button type="primary" @click="handleSearch">查询</el-button>
      </div>

      <div class="surface-card">
        <el-table :data="apis" v-loading="loading" style="width: 100%">
          <el-table-column prop="ID" label="ID" width="80" />
          <el-table-column prop="method" label="方法" width="100">
            <template #default="{ row }">
              <el-tag :type="methodTagType(row.method)">{{ row.method }}</el-tag>
            </template>
          </el-table-column>
          <el-table-column prop="path" label="路径" min-width="260" />
          <el-table-column prop="description" label="描述" min-width="180" />
          <el-table-column prop="apiGroup" label="分组" min-width="120" />
          <el-table-column label="操作" width="260">
            <template #default="{ row }">
              <el-button link type="primary" @click="openEditDialog(row)">编辑</el-button>
              <el-button link @click="openRoleDialog(row)">分配角色</el-button>
              <el-button link type="danger" @click="handleDelete(row)">删除</el-button>
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

    <el-dialog
      v-model="dialogVisible"
      :title="dialogMode === 'create' ? '新增 API' : '编辑 API'"
      width="560px"
    >
      <el-form label-width="100px" @submit.prevent="submitApi">
        <el-form-item label="请求方法">
          <el-select v-model="form.method" class="w-full">
            <el-option v-for="item in methodOptions" :key="item" :label="item" :value="item" />
          </el-select>
        </el-form-item>
        <el-form-item label="接口路径">
          <el-input v-model="form.path" placeholder="/api/users" />
        </el-form-item>
        <el-form-item label="接口描述">
          <el-input v-model="form.description" placeholder="获取用户列表" />
        </el-form-item>
        <el-form-item label="接口分组">
          <el-input v-model="form.apiGroup" placeholder="user" />
        </el-form-item>
      </el-form>

      <template #footer>
        <el-button @click="dialogVisible = false">取消</el-button>
        <el-button type="primary" :loading="submitting" @click="submitApi">保存</el-button>
      </template>
    </el-dialog>

    <el-dialog v-model="roleDialogVisible" title="分配角色" width="520px">
      <div class="dialog-summary">
        {{ selectedApi?.method }} {{ selectedApi?.path }}
      </div>
      <el-select
        v-model="selectedAuthorityIds"
        multiple
        filterable
        class="w-full"
        placeholder="选择可访问角色"
      >
        <el-option
          v-for="authority in authorityOptions"
          :key="authority.authorityId"
          :label="`${authority.authorityName} (${authority.authorityId})`"
          :value="authority.authorityId"
        />
      </el-select>

      <template #footer>
        <el-button @click="roleDialogVisible = false">取消</el-button>
        <el-button type="primary" :loading="roleSubmitting" @click="submitApiRoles">
          保存角色
        </el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted, reactive, ref } from 'vue'
import { ElMessage, ElMessageBox } from '@/ui/feedback'

import { fetchAuthorities, type AuthorityRecord } from '@/api/authorities'
import {
  createApi,
  deleteApi,
  fetchApiGroups,
  fetchApiRoles,
  fetchApis,
  setApiRoles,
  updateApi,
  type ApiRecord
} from '@/api/apis'
import { usePageChrome } from '@/composables/usePageChrome'

type DialogMode = 'create' | 'edit'

const methodOptions = ['GET', 'POST', 'PUT', 'DELETE']
const authorities = ref<AuthorityRecord[]>([])
const apiGroups = ref<string[]>([])
const apis = ref<ApiRecord[]>([])
const loading = ref(false)
const dialogVisible = ref(false)
const dialogMode = ref<DialogMode>('create')
const submitting = ref(false)
const roleDialogVisible = ref(false)
const roleSubmitting = ref(false)
const selectedApi = ref<ApiRecord | null>(null)
const selectedAuthorityIds = ref<number[]>([])
const total = ref(0)
const page = ref(1)
const pageSize = ref(10)
const filters = reactive({
  path: '',
  description: '',
  apiGroup: '',
  method: ''
})
const form = reactive<ApiRecord>({
  ID: 0,
  path: '',
  description: '',
  apiGroup: '',
  method: 'GET'
})

const authorityOptions = computed(() => flattenAuthorities(authorities.value))
const { summary } = usePageChrome(apis, '条接口')

function flattenAuthorities(list: AuthorityRecord[]): AuthorityRecord[] {
  return list.flatMap((item) => [item, ...flattenAuthorities(item.children || [])])
}

function resetForm() {
  form.ID = 0
  form.path = ''
  form.description = ''
  form.apiGroup = ''
  form.method = 'GET'
}

function methodTagType(method: string) {
  if (method === 'GET') return 'success'
  if (method === 'POST') return 'primary'
  if (method === 'PUT') return 'warning'
  return 'danger'
}

async function loadApis() {
  loading.value = true
  try {
    const result = await fetchApis({
      page: page.value,
      pageSize: pageSize.value,
      path: filters.path,
      description: filters.description,
      apiGroup: filters.apiGroup,
      method: filters.method
    })
    apis.value = result.list
    total.value = result.total
  } catch {
    ElMessage.error('获取 API 列表失败')
  } finally {
    loading.value = false
  }
}

async function loadBaseData() {
  try {
    const [groups, authorityList] = await Promise.all([fetchApiGroups(), fetchAuthorities()])
    apiGroups.value = groups
    authorities.value = authorityList
  } catch {
    ElMessage.error('获取基础数据失败')
  }
}

function handleSearch() {
  page.value = 1
  loadApis()
}

function handlePageChange(nextPage: number) {
  page.value = nextPage
  loadApis()
}

function openCreateDialog() {
  dialogMode.value = 'create'
  resetForm()
  dialogVisible.value = true
}

function openEditDialog(api: ApiRecord) {
  dialogMode.value = 'edit'
  form.ID = api.ID
  form.path = api.path
  form.description = api.description
  form.apiGroup = api.apiGroup
  form.method = api.method
  dialogVisible.value = true
}

async function submitApi() {
  if (!form.path.trim() || !form.description.trim() || !form.apiGroup.trim()) {
    ElMessage.warning('请填写完整 API 信息')
    return
  }

  submitting.value = true
  try {
    const payload = {
      ID: form.ID,
      path: form.path.trim(),
      description: form.description.trim(),
      apiGroup: form.apiGroup.trim(),
      method: form.method
    }
    const response =
      dialogMode.value === 'create' ? await createApi(payload) : await updateApi(payload)

    if (response.code === 'OK') {
      ElMessage.success(dialogMode.value === 'create' ? 'API 已创建' : 'API 已更新')
      dialogVisible.value = false
      await loadBaseData()
      await loadApis()
      return
    }

    ElMessage.error(response.message || '保存 API 失败')
  } catch {
    ElMessage.error('保存 API 失败')
  } finally {
    submitting.value = false
  }
}

async function openRoleDialog(api: ApiRecord) {
  selectedApi.value = api
  roleDialogVisible.value = true
  try {
    const selection = await fetchApiRoles(api.path, api.method)
    selectedAuthorityIds.value = selection.authorityIds
  } catch {
    ElMessage.error('获取 API 角色失败')
  }
}

async function submitApiRoles() {
  if (!selectedApi.value) return

  roleSubmitting.value = true
  try {
    const response = await setApiRoles(
      selectedApi.value.path,
      selectedApi.value.method,
      selectedAuthorityIds.value
    )
    if (response.code === 'OK') {
      ElMessage.success('API 角色已更新')
      roleDialogVisible.value = false
      return
    }

    ElMessage.error(response.message || '保存 API 角色失败')
  } catch {
    ElMessage.error('保存 API 角色失败')
  } finally {
    roleSubmitting.value = false
  }
}

async function handleDelete(api: ApiRecord) {
  try {
    await ElMessageBox.confirm(`确定删除接口“${api.method} ${api.path}”吗？`, '提示', {
      type: 'warning'
    })
  } catch {
    return
  }

  try {
    const response = await deleteApi(api.ID)
    if (response.code === 'OK') {
      ElMessage.success('API 已删除')
      await loadBaseData()
      await loadApis()
      return
    }

    ElMessage.error(response.message || '删除 API 失败')
  } catch {
    ElMessage.error('删除 API 失败')
  }
}

onMounted(async () => {
  await loadBaseData()
  await loadApis()
})
</script>

<style scoped>
.pagination {
  display: flex;
  justify-content: flex-end;
  margin-top: 16px;
}

.dialog-summary {
  margin-bottom: 12px;
  color: #334155;
  font-weight: 600;
}

.w-full {
  width: 100%;
}
</style>
