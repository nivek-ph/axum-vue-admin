<template>
  <div class="admin-page">
    <section class="page-hero">
      <div class="page-hero-main">
        <span class="page-hero-kicker">{{ $t('Role permissions') }}</span>
        <h2 class="page-hero-title">{{ $t('Roles') }}</h2>
        <p class="page-hero-subtitle">{{ $t('Manage role structure, default routes, and members.') }}</p>

        <div class="page-metrics">
          <div class="page-metric">
            <div class="page-metric-label">{{ $t('Total roles') }}</div>
            <div class="page-metric-value">{{ total }}</div>
          </div>
          <div class="page-metric">
            <div class="page-metric-label">{{ $t('Root roles') }}</div>
            <div class="page-metric-value">{{ rootRoleCount }}</div>
          </div>
          <div class="page-metric">
            <div class="page-metric-label">{{ $t('Default entry') }}</div>
            <div class="page-metric-value">{{ defaultRouterLabel }}</div>
          </div>
        </div>
      </div>

      <aside class="page-hero-side">
        <div>
          <div class="page-note-label">{{ $t('Current data') }}</div>
          <div class="page-note-value">{{ summary }}</div>
        </div>
        <div class="page-hero-actions">
          <UiButton @click="loadAuthorities" :loading="loading">{{ $t('Refresh list') }}</UiButton>
          <UiButton type="primary" @click="openCreateDialog">{{ $t('New role') }}</UiButton>
        </div>
      </aside>
    </section>

    <section class="page-panel">
      <div class="page-panel-header">
        <div>
          <h3 class="page-panel-title">{{ $t('Role list') }}</h3>
          <p class="page-panel-subtitle">{{ $t('Review the tree and maintain role fields.') }}</p>
        </div>
      </div>

      <div class="surface-card">
        <UiTable
          :data="authorities"
          row-key="authorityId"
          default-expand-all
          :loading="loading"
          style="width: 100%"
        >
          <UiTableColumn prop="authorityId" label="Role ID" width="120" />
          <UiTableColumn prop="authorityName" label="Role name" min-width="180" />
          <UiTableColumn prop="parentId" label="Parent role" width="120" />
          <UiTableColumn prop="defaultRouter" label="Default route" min-width="160" />
          <UiTableColumn label="Actions" width="260">
            <template #default="{ row }">
              <UiButton link type="primary" @click="openEditDialog(row)">{{ $t('Edit') }}</UiButton>
              <UiButton link @click="openUserDialog(row)">{{ $t('Assign members') }}</UiButton>
              <UiButton
                link
                type="danger"
                :disabled="row.authorityId === 888"
                @click="handleDelete(row)"
              >
                {{ $t('Delete') }}
              </UiButton>
            </template>
          </UiTableColumn>
        </UiTable>
      </div>
    </section>

    <UiDialog
      v-model="dialogVisible"
      :title="dialogMode === 'create' ? 'New role' : 'Edit role'"
      width="520px"
    >
      <UiForm labelWidth="100px" @submit.prevent="submitAuthority">
        <UiFormItem label="Role ID">
          <UiInputNumber
            v-model="form.authorityId"
            :disabled="dialogMode === 'edit'"
            :min="1"
            :precision="0"
            class="w-full"
          />
        </UiFormItem>
        <UiFormItem label="Role name">
          <UiInput v-model="form.authorityName" placeholder="Example: operator admin" />
        </UiFormItem>
        <UiFormItem label="Parent role">
          <UiSelect v-model="form.parentId" class="w-full">
            <UiOption :value="0" label="Top-level role" />
            <UiOption
              v-for="item in authorityOptions"
              :key="item.authorityId"
              :label="`${item.authorityName} (${item.authorityId})`"
              :value="item.authorityId"
            />
          </UiSelect>
        </UiFormItem>
        <UiFormItem label="Default route">
          <UiInput v-model="form.defaultRouter" placeholder="dashboard" />
        </UiFormItem>
      </UiForm>

      <template #footer>
        <UiButton @click="dialogVisible = false">{{ $t('Cancel') }}</UiButton>
        <UiButton type="primary" :loading="submitting" @click="submitAuthority">{{ $t('Save') }}</UiButton>
      </template>
    </UiDialog>

    <UiDialog v-model="userDialogVisible" title="Assign members" width="520px">
      <div class="dialog-summary">
        {{ selectedAuthority?.authorityName || $t('Role') }}
      </div>
      <UiSelect
        v-model="selectedUserIds"
        multiple
        filterable
        class="w-full"
        placeholder="Select users"
      >
        <UiOption
          v-for="user in userOptions"
          :key="user.ID"
          :label="`${user.nickName || user.userName} (${user.userName})`"
          :value="user.ID"
        />
      </UiSelect>

      <template #footer>
        <UiButton @click="userDialogVisible = false">{{ $t('Cancel') }}</UiButton>
        <UiButton type="primary" :loading="userSubmitting" @click="submitRoleUsers">
          {{ $t('Save members') }}
        </UiButton>
      </template>
    </UiDialog>
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted, reactive, ref } from 'vue'
import { ElMessage, ElMessageBox } from '@/ui/feedback'

import {
  createAuthority,
  deleteAuthority,
  fetchAuthorities,
  fetchAuthorityUsers,
  setRoleUsers,
  updateAuthority,
  type AuthorityRecord
} from '@/api/authorities'
import { usePageChrome } from '@/composables/usePageChrome'
import { fetchUsers, type UserRecord } from '@/api/users'
import { t } from '@/i18n'

type DialogMode = 'create' | 'edit'

const authorities = ref<AuthorityRecord[]>([])
const loading = ref(false)
const dialogVisible = ref(false)
const dialogMode = ref<DialogMode>('create')
const submitting = ref(false)
const userDialogVisible = ref(false)
const userSubmitting = ref(false)
const selectedAuthority = ref<AuthorityRecord | null>(null)
const userOptions = ref<UserRecord[]>([])
const selectedUserIds = ref<number[]>([])
const form = reactive({
  authorityId: 0,
  authorityName: '',
  parentId: 0,
  defaultRouter: 'dashboard'
})

const authorityOptions = computed(() => flattenAuthorities(authorities.value))
const { total, summary } = usePageChrome(authorities, 'roles')
const rootRoleCount = computed(() => authorityOptions.value.filter((item) => item.parentId === 0).length)
const defaultRouterLabel = computed(() => authorityOptions.value[0]?.defaultRouter || 'dashboard')

function flattenAuthorities(list: AuthorityRecord[]): AuthorityRecord[] {
  return list.flatMap((item) => [item, ...flattenAuthorities(item.children || [])])
}

function resetForm() {
  form.authorityId = 0
  form.authorityName = ''
  form.parentId = 0
  form.defaultRouter = 'dashboard'
}

async function loadAuthorities() {
  loading.value = true
  try {
    authorities.value = await fetchAuthorities()
  } catch {
    ElMessage.error(t('Failed to load roles'))
  } finally {
    loading.value = false
  }
}

function openCreateDialog() {
  dialogMode.value = 'create'
  resetForm()
  dialogVisible.value = true
}

function openEditDialog(authority: AuthorityRecord) {
  dialogMode.value = 'edit'
  form.authorityId = authority.authorityId
  form.authorityName = authority.authorityName
  form.parentId = authority.parentId
  form.defaultRouter = authority.defaultRouter || 'dashboard'
  dialogVisible.value = true
}

async function submitAuthority() {
  if (!form.authorityId || !form.authorityName.trim()) {
    ElMessage.warning(t('Please complete role information'))
    return
  }

  submitting.value = true
  try {
    const response =
      dialogMode.value === 'create'
        ? await createAuthority({
            authorityId: form.authorityId,
            authorityName: form.authorityName.trim(),
            parentId: form.parentId
          })
        : await updateAuthority({
            authorityId: form.authorityId,
            authorityName: form.authorityName.trim(),
            parentId: form.parentId,
            defaultRouter: form.defaultRouter.trim() || 'dashboard'
          })

    if (response.code === 'OK') {
      ElMessage.success(t(dialogMode.value === 'create' ? 'Role created' : 'Role updated'))
      dialogVisible.value = false
      await loadAuthorities()
      return
    }

    ElMessage.error(response.message || t('Failed to save role'))
  } catch {
    ElMessage.error(t('Failed to save role'))
  } finally {
    submitting.value = false
  }
}

async function openUserDialog(authority: AuthorityRecord) {
  selectedAuthority.value = authority
  userDialogVisible.value = true
  try {
    const [users, userIds] = await Promise.all([
      fetchUsers(1, 200),
      fetchAuthorityUsers(authority.authorityId)
    ])
    userOptions.value = users.list
    selectedUserIds.value = userIds
  } catch {
    ElMessage.error(t('Failed to load role members'))
  }
}

async function submitRoleUsers() {
  if (!selectedAuthority.value) return

  userSubmitting.value = true
  try {
    const response = await setRoleUsers(selectedAuthority.value.authorityId, selectedUserIds.value)
    if (response.code === 'OK') {
      ElMessage.success(t('Role members updated'))
      userDialogVisible.value = false
      return
    }

    ElMessage.error(response.message || t('Failed to save members'))
  } catch {
    ElMessage.error(t('Failed to save members'))
  } finally {
    userSubmitting.value = false
  }
}

async function handleDelete(authority: AuthorityRecord) {
  try {
    await ElMessageBox.confirm(t('Delete role "{name}"?', { name: authority.authorityName }), t('Notice'), {
      type: 'warning'
    })
  } catch {
    return
  }

  try {
    const response = await deleteAuthority(authority.authorityId)
    if (response.code === 'OK') {
      ElMessage.success(t('Role deleted'))
      await loadAuthorities()
      return
    }

    ElMessage.error(response.message || t('Failed to delete role'))
  } catch {
    ElMessage.error(t('Failed to delete role'))
  }
}

onMounted(() => {
  loadAuthorities()
})
</script>

<style scoped>
.dialog-summary {
  margin-bottom: 12px;
  color: #334155;
  font-weight: 600;
}

.w-full {
  width: 100%;
}
</style>
