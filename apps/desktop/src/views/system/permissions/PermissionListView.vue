<template>
  <div class="admin-page">
    <section class="page-panel">
      <div class="page-panel-header">
        <div>
          <h3 class="page-panel-title">{{ $t('Permissions') }}</h3>
          <p class="page-panel-subtitle">{{ $t('Manage unified permission resources.') }}</p>
        </div>
        <div class="page-panel-actions">
          <UiInput v-model="keyword" placeholder="Search code/resource" />
          <UiButton @click="loadPermissions" :loading="loading">{{ $t('Refresh') }}</UiButton>
          <UiButton type="primary" @click="openCreateDialog">{{ $t('New') }}</UiButton>
        </div>
      </div>

      <div class="surface-card">
        <UiTable :data="filteredPermissions" :loading="loading" style="width: 100%">
          <UiTableColumn prop="id" label="ID" width="80" />
          <UiTableColumn prop="module_key" label="Module" width="120" />
          <UiTableColumn prop="resource" label="Resource" width="140" />
          <UiTableColumn prop="action" label="Action" width="130" />
          <UiTableColumn prop="code" label="Code" min-width="220" />
          <UiTableColumn prop="name" label="Name" min-width="160" />
          <UiTableColumn prop="type" label="Type" width="100" />
          <UiTableColumn prop="status" label="Status" width="110" />
          <UiTableColumn label="Actions" width="160">
            <template #default="{ row }">
              <UiButton link type="primary" @click="openEditDialog(row)">{{ $t('Edit') }}</UiButton>
              <UiButton link type="danger" @click="removePermission(row)">{{ $t('Delete') }}</UiButton>
            </template>
          </UiTableColumn>
        </UiTable>
      </div>
    </section>

    <UiDialog v-model="dialogVisible" :title="dialogMode === 'create' ? 'New permission' : 'Edit permission'" width="560px">
      <UiForm class="permission-form" @submit.prevent="submitPermission">
        <UiFormItem label="Module">
          <UiInput v-model="form.module_key" placeholder="system" />
        </UiFormItem>
        <UiFormItem label="Resource">
          <UiInput v-model="form.resource" placeholder="user" />
        </UiFormItem>
        <UiFormItem label="Action">
          <UiInput v-model="form.action" placeholder="create" />
        </UiFormItem>
        <UiFormItem label="Code">
          <UiInput v-model="form.code" placeholder="system:user:create" />
        </UiFormItem>
        <UiFormItem label="Name">
          <UiInput v-model="form.name" placeholder="Create user" />
        </UiFormItem>
        <UiFormItem label="Type">
          <UiSelect v-model="form.type">
            <UiOption value="page" label="page" />
            <UiOption value="action" label="action" />
            <UiOption value="api" label="api" />
            <UiOption value="data" label="data" />
          </UiSelect>
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
        <UiButton type="primary" :loading="submitting" @click="submitPermission">{{ $t('Save') }}</UiButton>
      </template>
    </UiDialog>
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted, reactive, ref } from 'vue'
import { ElMessage, ElMessageBox } from '@/ui/feedback'
import {
  createPermission,
  deletePermission,
  listPermissions,
  updatePermission,
  type PermissionPayload,
  type PermissionResource
} from '@/api/system/permissions'
import { t } from '@/i18n'

type DialogMode = 'create' | 'edit'

const permissions = ref<PermissionResource[]>([])
const keyword = ref('')
const loading = ref(false)
const submitting = ref(false)
const dialogVisible = ref(false)
const dialogMode = ref<DialogMode>('create')
const editingId = ref<number | null>(null)
const form = reactive<PermissionPayload>({
  module_key: 'system',
  resource: '',
  action: '',
  code: '',
  name: '',
  type: 'action',
  status: 'enabled'
})

const filteredPermissions = computed(() => {
  const q = keyword.value.trim().toLowerCase()
  if (!q) return permissions.value
  return permissions.value.filter((item) =>
    [item.module_key, item.resource, item.action, item.code, item.name]
      .some((value) => value.toLowerCase().includes(q))
  )
})

async function loadPermissions() {
  loading.value = true
  try {
    permissions.value = await listPermissions()
  } catch {
    ElMessage.error(t('Failed to load permissions'))
  } finally {
    loading.value = false
  }
}

function resetForm() {
  form.module_key = 'system'
  form.resource = ''
  form.action = ''
  form.code = ''
  form.name = ''
  form.type = 'action'
  form.status = 'enabled'
}

function openCreateDialog() {
  dialogMode.value = 'create'
  editingId.value = null
  resetForm()
  dialogVisible.value = true
}

function openEditDialog(permission: PermissionResource) {
  dialogMode.value = 'edit'
  editingId.value = permission.id
  form.module_key = permission.module_key
  form.resource = permission.resource
  form.action = permission.action
  form.code = permission.code
  form.name = permission.name
  form.type = permission.type
  form.status = permission.status
  dialogVisible.value = true
}

async function submitPermission() {
  if (!form.module_key.trim() || !form.resource.trim() || !form.action.trim() || !form.code.trim() || !form.name.trim()) {
    ElMessage.warning(t('Please complete permission information'))
    return
  }

  submitting.value = true
  try {
    const response = dialogMode.value === 'create'
      ? await createPermission(form)
      : await updatePermission(editingId.value as number, form)
    if (response.code === 'OK') {
      ElMessage.success(t('Saved'))
      dialogVisible.value = false
      await loadPermissions()
      return
    }
    ElMessage.error(response.message || t('Failed to save permission'))
  } catch {
    ElMessage.error(t('Failed to save permission'))
  } finally {
    submitting.value = false
  }
}

async function removePermission(permission: PermissionResource) {
  try {
    await ElMessageBox.confirm(t('Delete permission "{name}"?', { name: permission.name }), t('Notice'), { type: 'warning' })
  } catch {
    return
  }

  try {
    const response = await deletePermission(permission.id)
    if (response.code === 'OK') {
      ElMessage.success(t('Deleted'))
      await loadPermissions()
      return
    }
    ElMessage.error(response.message || t('Delete failed'))
  } catch {
    ElMessage.error(t('Delete failed'))
  }
}

onMounted(loadPermissions)
</script>

<style scoped>
.permission-form {
  display: grid;
  gap: 14px;
}
</style>
