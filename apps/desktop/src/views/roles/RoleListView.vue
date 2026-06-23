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
              <UiButton link data-test="assign-members-button" @click="openUserDialog(row)">
                {{ $t('Assign members') }}
              </UiButton>
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
      <div class="member-panel">
        <div class="member-panel-header">
          <div>
            <div class="member-role-label">{{ $t('Role') }}</div>
            <div class="member-role-name">{{ selectedAuthority?.authorityName || '-' }}</div>
          </div>
          <div class="member-count">
            <span>{{ $t('Selected members') }}</span>
            <strong>{{ selectedUserIds.length }} / {{ userOptions.length }}</strong>
          </div>
        </div>

        <UiInput v-model="memberSearch" placeholder="Search users" />

        <div class="member-list" data-test="member-list">
          <label
            v-for="user in filteredUserOptions"
            :key="user.ID"
            :class="['member-card', selectedUserIds.includes(user.ID) && 'is-selected']"
          >
            <input
              class="member-checkbox"
              type="checkbox"
              :checked="selectedUserIds.includes(user.ID)"
              @change="toggleUserSelection(user.ID)"
            />
            <span class="member-checkmark">
              <span v-if="selectedUserIds.includes(user.ID)">✓</span>
            </span>
            <span class="member-avatar">{{ userInitial(user) }}</span>
            <span class="member-main">
              <span class="member-name">{{ user.nickName || user.userName }}</span>
              <span class="member-meta">{{ user.userName }}<span v-if="user.email"> · {{ user.email }}</span></span>
            </span>
          </label>

          <div v-if="filteredUserOptions.length === 0" class="member-empty">
            {{ $t(userOptions.length === 0 ? 'No users available' : 'No matching users') }}
          </div>
        </div>
      </div>

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
const memberSearch = ref('')
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
const filteredUserOptions = computed(() => {
  const keyword = memberSearch.value.trim().toLowerCase()
  if (!keyword) return userOptions.value
  return userOptions.value.filter((user) =>
    [user.userName, user.nickName, user.email, user.phone]
      .filter(Boolean)
      .some((value) => String(value).toLowerCase().includes(keyword))
  )
})

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
  memberSearch.value = ''
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

function toggleUserSelection(userId: number) {
  selectedUserIds.value = selectedUserIds.value.includes(userId)
    ? selectedUserIds.value.filter((id) => id !== userId)
    : [...selectedUserIds.value, userId]
}

function userInitial(user: UserRecord) {
  return (user.nickName || user.userName || '?').slice(0, 1).toUpperCase()
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
.member-panel {
  display: grid;
  gap: 14px;
}

.member-panel-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 16px;
  border: 1px solid #e7e5e4;
  border-radius: 14px;
  background: #fafaf9;
  padding: 14px;
}

.member-role-label,
.member-count span {
  color: var(--text-muted);
  font-size: 12px;
}

.member-role-name {
  margin-top: 4px;
  color: #18181b;
  font-size: 16px;
  font-weight: 700;
}

.member-count {
  display: grid;
  justify-items: end;
  gap: 4px;
}

.member-count strong {
  color: #18181b;
  font-size: 18px;
}

.member-list {
  display: grid;
  gap: 8px;
  max-height: 320px;
  overflow-y: auto;
  padding-right: 4px;
}

.member-card {
  display: grid;
  grid-template-columns: 22px 34px minmax(0, 1fr);
  align-items: center;
  gap: 12px;
  min-height: 58px;
  border: 1px solid #e7e5e4;
  border-radius: 14px;
  background: #ffffff;
  padding: 10px 12px;
  cursor: pointer;
  transition: border-color 0.16s ease, background 0.16s ease, box-shadow 0.16s ease;
}

.member-card:hover,
.member-card.is-selected {
  border-color: #18181b;
  background: #fafaf9;
}

.member-card.is-selected {
  box-shadow: 0 8px 24px rgba(24, 24, 27, 0.08);
}

.member-checkbox {
  position: absolute;
  opacity: 0;
  pointer-events: none;
}

.member-checkmark {
  display: grid;
  width: 22px;
  height: 22px;
  place-items: center;
  border: 1px solid #d6d3d1;
  border-radius: 7px;
  color: #ffffff;
  background: #ffffff;
  font-size: 14px;
  font-weight: 800;
}

.member-card.is-selected .member-checkmark {
  border-color: #18181b;
  background: #18181b;
}

.member-avatar {
  display: grid;
  width: 34px;
  height: 34px;
  place-items: center;
  border-radius: 999px;
  background: #f5f5f4;
  color: #27272a;
  font-size: 13px;
  font-weight: 800;
}

.member-main {
  min-width: 0;
}

.member-name,
.member-meta {
  display: block;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.member-name {
  color: #18181b;
  font-size: 14px;
  font-weight: 700;
}

.member-meta {
  margin-top: 2px;
  color: var(--text-muted);
  font-size: 12px;
}

.member-empty {
  border: 1px dashed #d6d3d1;
  border-radius: 14px;
  padding: 24px;
  color: var(--text-muted);
  text-align: center;
}
</style>

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
