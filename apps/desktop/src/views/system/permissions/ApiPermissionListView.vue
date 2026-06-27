<template>
  <div class="admin-page">
    <section class="page-panel">
      <div class="page-panel-header">
        <div>
          <h3 class="page-panel-title">{{ $t('API permission bindings') }}</h3>
          <p class="page-panel-subtitle">{{ $t('Bind HTTP methods and path patterns to permission resources.') }}</p>
        </div>
        <div class="page-panel-actions">
          <UiButton @click="loadPermissions" :loading="loading">{{ $t('Refresh') }}</UiButton>
          <UiButton type="primary" :disabled="!selectedPermissionId" @click="addBinding">{{ $t('New') }}</UiButton>
          <UiButton type="primary" :disabled="!selectedPermissionId" :loading="saving" @click="saveBindings">{{ $t('Save') }}</UiButton>
        </div>
      </div>

      <div class="api-binding-layout">
        <aside class="api-binding-sidebar">
          <UiInput v-model="keyword" placeholder="Search permission" />
          <button
            v-for="permission in filteredPermissions"
            :key="permission.id"
            :class="['api-binding-item', selectedPermissionId === permission.id && 'is-active']"
            type="button"
            @click="selectPermission(permission.id)"
          >
            <span>{{ permission.name }}</span>
            <small>{{ permission.code }}</small>
          </button>
        </aside>

        <section class="surface-card">
          <UiTable :data="bindings" :loading="bindingLoading" style="width: 100%">
            <UiTableColumn label="Method" width="160">
              <template #default="{ row }">
                <UiInput v-model="row.method" placeholder="GET" />
              </template>
            </UiTableColumn>
            <UiTableColumn label="Path pattern" min-width="260">
              <template #default="{ row }">
                <UiInput v-model="row.path_pattern" data-test="api-binding-path" placeholder="/api/users/{id}" />
              </template>
            </UiTableColumn>
            <UiTableColumn label="Actions" width="100">
              <template #default="{ $index }">
                <UiButton link type="danger" @click="removeBinding($index)">{{ $t('Delete') }}</UiButton>
              </template>
            </UiTableColumn>
          </UiTable>
        </section>
      </div>
    </section>
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted, ref } from 'vue'
import { ElMessage } from '@/ui/feedback'
import {
  listPermissionApis,
  listPermissions,
  setPermissionApis,
  type PermissionApiBinding,
  type PermissionResource
} from '@/api/system/permissions'
import { t } from '@/i18n'

const permissions = ref<PermissionResource[]>([])
const bindings = ref<PermissionApiBinding[]>([])
const selectedPermissionId = ref<number | null>(null)
const keyword = ref('')
const loading = ref(false)
const bindingLoading = ref(false)
const saving = ref(false)

const filteredPermissions = computed(() => {
  const q = keyword.value.trim().toLowerCase()
  if (!q) return permissions.value
  return permissions.value.filter((item) => [item.name, item.code].some((value) => value.toLowerCase().includes(q)))
})

async function loadPermissions() {
  loading.value = true
  try {
    permissions.value = await listPermissions()
    if (!selectedPermissionId.value) {
      selectedPermissionId.value = permissions.value[0]?.id || null
    }
    if (selectedPermissionId.value) {
      await loadBindings(selectedPermissionId.value)
    }
  } catch {
    ElMessage.error(t('Failed to load permissions'))
  } finally {
    loading.value = false
  }
}

async function selectPermission(id: number) {
  selectedPermissionId.value = id
  await loadBindings(id)
}

async function loadBindings(id: number) {
  bindingLoading.value = true
  try {
    bindings.value = await listPermissionApis(id)
  } catch {
    ElMessage.error(t('Failed to load API bindings'))
  } finally {
    bindingLoading.value = false
  }
}

function addBinding() {
  bindings.value = [...bindings.value, { method: 'GET', path_pattern: '' }]
}

function removeBinding(index: number) {
  bindings.value = bindings.value.filter((_, itemIndex) => itemIndex !== index)
}

async function saveBindings() {
  if (!selectedPermissionId.value) return
  saving.value = true
  try {
    const response = await setPermissionApis(
      selectedPermissionId.value,
      bindings.value.filter((item) => item.method.trim() && item.path_pattern.trim())
    )
    if (response.code === 'OK') {
      ElMessage.success(t('Saved'))
      await loadBindings(selectedPermissionId.value)
      return
    }
    ElMessage.error(response.message || t('Failed to save API bindings'))
  } catch {
    ElMessage.error(t('Failed to save API bindings'))
  } finally {
    saving.value = false
  }
}

onMounted(loadPermissions)
</script>

<style scoped>
.api-binding-layout {
  display: grid;
  grid-template-columns: minmax(220px, 280px) minmax(0, 1fr);
  gap: 14px;
}

.api-binding-sidebar {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.api-binding-item {
  display: grid;
  gap: 3px;
  border: 1px solid #e7e5e4;
  border-radius: 8px;
  background: #fff;
  padding: 10px 12px;
  color: #292524;
  text-align: left;
}

.api-binding-item small {
  color: #78716c;
}

.api-binding-item.is-active {
  border-color: #2563eb;
  background: #eff6ff;
}
</style>
