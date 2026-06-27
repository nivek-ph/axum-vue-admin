<template>
  <div class="admin-page">
    <section class="page-panel">
      <div class="page-panel-header">
        <div>
          <h3 class="page-panel-title">{{ $t('Departments') }}</h3>
          <p class="page-panel-subtitle">{{ $t('Manage department hierarchy and status.') }}</p>
        </div>
        <div class="page-panel-actions">
          <UiButton @click="loadDepts" :loading="loading">{{ $t('Refresh') }}</UiButton>
          <UiButton type="primary" @click="openCreateDialog()">{{ $t('New') }}</UiButton>
        </div>
      </div>

      <div class="surface-card">
        <UiTable :data="depts" row-key="id" default-expand-all :loading="loading" style="width: 100%">
          <UiTableColumn prop="id" label="ID" width="80" />
          <UiTableColumn prop="name" label="Name" min-width="180" />
          <UiTableColumn prop="code" label="Code" min-width="180" />
          <UiTableColumn prop="sort" label="Sort" width="90" />
          <UiTableColumn label="Status" width="110">
            <template #default="{ row }">
              <UiTag :type="row.status === 'enabled' ? 'success' : 'info'">{{ row.status }}</UiTag>
            </template>
          </UiTableColumn>
          <UiTableColumn label="Actions" width="220">
            <template #default="{ row }">
              <UiButton link @click="openCreateDialog(row.id)">{{ $t('Add child') }}</UiButton>
              <UiButton link type="primary" @click="openEditDialog(row)">{{ $t('Edit') }}</UiButton>
              <UiButton link type="danger" @click="removeDept(row)">{{ $t('Delete') }}</UiButton>
            </template>
          </UiTableColumn>
        </UiTable>
      </div>
    </section>

    <UiDialog v-model="dialogVisible" :title="dialogMode === 'create' ? 'New department' : 'Edit department'" width="520px">
      <UiForm class="dept-form" @submit.prevent="submitDept">
        <UiFormItem label="Parent ID">
          <UiInputNumber v-model="form.parent_id" :min="0" :precision="0" class="w-full" />
        </UiFormItem>
        <UiFormItem label="Name">
          <UiInput v-model="form.name" placeholder="Operations" />
        </UiFormItem>
        <UiFormItem label="Code">
          <UiInput v-model="form.code" placeholder="operations" />
        </UiFormItem>
        <UiFormItem label="Sort">
          <UiInputNumber v-model="form.sort" :min="0" :precision="0" class="w-full" />
        </UiFormItem>
        <UiFormItem label="Status">
          <UiSelect v-model="form.status">
            <UiOption value="enabled" label="enabled" />
            <UiOption value="disabled" label="disabled" />
          </UiSelect>
        </UiFormItem>
      </UiForm>
      <template #footer>
        <UiButton @click="dialogVisible = false">{{ $t('Cancel') }}</UiButton>
        <UiButton type="primary" :loading="submitting" @click="submitDept">{{ $t('Save') }}</UiButton>
      </template>
    </UiDialog>
  </div>
</template>

<script setup lang="ts">
import { onMounted, reactive, ref } from 'vue'
import { ElMessage, ElMessageBox } from '@/ui/feedback'
import { createDept, deleteDept, listDepts, updateDept, type DeptRecord } from '@/api/system/depts'
import { t } from '@/i18n'

type DialogMode = 'create' | 'edit'

const depts = ref<DeptRecord[]>([])
const loading = ref(false)
const submitting = ref(false)
const dialogVisible = ref(false)
const dialogMode = ref<DialogMode>('create')
const editingId = ref<number | null>(null)
const form = reactive({
  parent_id: null as number | null,
  name: '',
  code: '',
  sort: 0,
  status: 'enabled'
})

async function loadDepts() {
  loading.value = true
  try {
    depts.value = await listDepts()
  } catch {
    ElMessage.error(t('Failed to load departments'))
  } finally {
    loading.value = false
  }
}

function resetForm(parentId: number | null = null) {
  form.parent_id = parentId
  form.name = ''
  form.code = ''
  form.sort = 0
  form.status = 'enabled'
}

function openCreateDialog(parentId: number | null = null) {
  dialogMode.value = 'create'
  editingId.value = null
  resetForm(parentId)
  dialogVisible.value = true
}

function openEditDialog(dept: DeptRecord) {
  dialogMode.value = 'edit'
  editingId.value = dept.id
  form.parent_id = dept.parent_id ?? null
  form.name = dept.name
  form.code = dept.code
  form.sort = dept.sort
  form.status = dept.status
  dialogVisible.value = true
}

async function submitDept() {
  if (!form.name.trim() || !form.code.trim()) {
    ElMessage.warning(t('Please complete department information'))
    return
  }

  submitting.value = true
  try {
    const payload = { ...form, parent_id: form.parent_id || null }
    const response = dialogMode.value === 'create'
      ? await createDept(payload)
      : await updateDept(editingId.value as number, payload)
    if (response.code === 'OK') {
      ElMessage.success(t('Saved'))
      dialogVisible.value = false
      await loadDepts()
      return
    }
    ElMessage.error(response.message || t('Failed to save department'))
  } catch {
    ElMessage.error(t('Failed to save department'))
  } finally {
    submitting.value = false
  }
}

async function removeDept(dept: DeptRecord) {
  try {
    await ElMessageBox.confirm(t('Delete department "{name}"?', { name: dept.name }), t('Notice'), { type: 'warning' })
  } catch {
    return
  }

  try {
    const response = await deleteDept(dept.id)
    if (response.code === 'OK') {
      ElMessage.success(t('Deleted'))
      await loadDepts()
      return
    }
    ElMessage.error(response.message || t('Delete failed'))
  } catch {
    ElMessage.error(t('Delete failed'))
  }
}

onMounted(loadDepts)
</script>

<style scoped>
.dept-form {
  display: grid;
  gap: 14px;
}

.w-full {
  width: 100%;
}
</style>
