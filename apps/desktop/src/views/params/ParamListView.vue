<template>
  <div class="admin-page">
    <section class="page-hero">
      <div class="page-hero-main">
        <span class="page-hero-kicker">{{ $t('System params') }}</span>
        <h2 class="page-hero-title">{{ $t('Param management') }}</h2>
        <p class="page-hero-subtitle">{{ $t('Manage key-value settings required by the admin console.') }}</p>

        <div class="page-metrics">
          <div class="page-metric">
            <div class="page-metric-label">{{ $t('Rows on page') }}</div>
            <div class="page-metric-value">{{ params.length }}</div>
          </div>
          <div class="page-metric">
            <div class="page-metric-label">{{ $t('Total params') }}</div>
            <div class="page-metric-value">{{ total }}</div>
          </div>
          <div class="page-metric">
            <div class="page-metric-label">{{ $t('Namespaces') }}</div>
            <div class="page-metric-value">{{ namespaceCount }}</div>
          </div>
        </div>
      </div>

    </section>

    <section class="page-panel">
      <div class="page-panel-header">
        <div>
          <h3 class="page-panel-title">{{ $t('Param list') }}</h3>
          <p class="page-panel-subtitle">{{ $t('Filter by name and key.') }}</p>
        </div>
        <div class="page-panel-actions">
          <UiButton @click="loadParams" :loading="loading">{{ $t('Refresh') }}</UiButton>
          <UiButton type="primary" @click="openCreateDialog">{{ $t('New') }}</UiButton>
        </div>
      </div>

      <div class="page-panel-toolbar inline-filter">
        <UiInput v-model="filters.name" placeholder="Filter by name" clearable />
        <UiInput v-model="filters.key" placeholder="Filter by key" clearable />
        <UiButton type="primary" @click="handleSearch">{{ $t('Search') }}</UiButton>
      </div>

      <div class="surface-card">
        <UiTable :data="params" :loading="loading" style="width: 100%">
          <UiTableColumn prop="id" label="ID" width="80" />
          <UiTableColumn prop="name" label="Name" min-width="160" />
          <UiTableColumn prop="key" label="Key" min-width="180" />
          <UiTableColumn prop="value" label="Value" min-width="180" />
          <UiTableColumn prop="desc" label="Notes" min-width="180" />
          <UiTableColumn label="Actions" width="180">
            <template #default="{ row }">
              <UiButton link type="primary" @click="openEditDialog(row)">{{ $t('Edit') }}</UiButton>
              <UiButton link type="danger" @click="handleDelete(row.id)">{{ $t('Delete') }}</UiButton>
            </template>
          </UiTableColumn>
        </UiTable>
      </div>

      <div class="pagination">
        <UiPagination
          background
          layout="total, prev, pager, next"
          :total="total"
          :current-page="page"
          :page-size="pageSize"
          @current-change="handlePageChange"
        />
      </div>
    </section>

    <UiDialog v-model="dialogVisible" :title="dialogMode === 'create' ? 'New param' : 'Edit param'" width="560px">
      <UiForm labelWidth="90px" @submit.prevent="submitParam">
        <UiFormItem label="Name">
          <UiInput v-model="form.name" placeholder="Example: site name" />
        </UiFormItem>
        <UiFormItem label="Key">
          <UiInput v-model="form.key" placeholder="Example: site.name" />
        </UiFormItem>
        <UiFormItem label="Value">
          <UiInput v-model="form.value" placeholder="Example: Admin Console" />
        </UiFormItem>
        <UiFormItem label="Notes">
          <UiInput v-model="form.desc" type="textarea" :rows="3" placeholder="Param description" />
        </UiFormItem>
      </UiForm>

      <template #footer>
        <UiButton @click="dialogVisible = false">{{ $t('Cancel') }}</UiButton>
        <UiButton type="primary" :loading="submitting" @click="submitParam">{{ $t('Save') }}</UiButton>
      </template>
    </UiDialog>
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted, reactive, ref } from 'vue'
import { ElMessage, ElMessageBox } from '@/ui/feedback'

import { createParam, deleteParam, fetchParams, updateParam, type ParamRecord } from '@/api/params'
import { t } from '@/i18n'

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
  id: 0,
  name: '',
  key: '',
  value: '',
  desc: ''
})
const namespaceCount = computed(
  () => new Set(params.value.map((item) => item.key.split('.').shift()).filter(Boolean)).size
)

function resetForm() {
  form.id = 0
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
    ElMessage.error(t('Failed to load params'))
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
    ElMessage.warning(t('Please complete param information'))
    return
  }

  submitting.value = true
  try {
    const payload = {
      id: form.id,
      name: form.name.trim(),
      key: form.key.trim(),
      value: form.value.trim(),
      desc: form.desc.trim()
    }
    const response =
      dialogMode.value === 'create' ? await createParam(payload) : await updateParam(payload)

    if (response.code === 'OK') {
      ElMessage.success(t(dialogMode.value === 'create' ? 'Param created' : 'Param updated'))
      dialogVisible.value = false
      await loadParams()
      return
    }
    ElMessage.error(response.message || t('Failed to save param'))
  } catch {
    ElMessage.error(t('Failed to save param'))
  } finally {
    submitting.value = false
  }
}

async function handleDelete(id: number) {
  try {
    await ElMessageBox.confirm(t('Delete this param?'), t('Notice'), { type: 'warning' })
  } catch {
    return
  }

  try {
    const response = await deleteParam(id)
    if (response.code === 'OK') {
      ElMessage.success(t('Param deleted'))
      await loadParams()
      return
    }
    ElMessage.error(response.message || t('Failed to delete param'))
  } catch {
    ElMessage.error(t('Failed to delete param'))
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
