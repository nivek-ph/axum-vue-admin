<template>
  <div class="admin-page">
    <section class="page-hero">
      <div class="page-hero-main">
        <span class="page-hero-kicker">{{ $t('User directory') }}</span>
        <h2 class="page-hero-title">{{ $t('Users') }}</h2>
        <p class="page-hero-subtitle">{{ $t('Review accounts, roles, status, and common actions in one place.') }}</p>

        <div class="page-metrics">
          <div class="page-metric">
            <div class="page-metric-label">{{ $t('Total users') }}</div>
            <div class="page-metric-value">{{ total }}</div>
          </div>
          <div class="page-metric">
            <div class="page-metric-label">{{ $t('Enabled users') }}</div>
            <div class="page-metric-value">{{ enabledCount }}</div>
          </div>
          <div class="page-metric">
            <div class="page-metric-label">{{ $t('Role types') }}</div>
            <div class="page-metric-value">{{ roleCount }}</div>
          </div>
        </div>
      </div>

      <aside class="page-hero-side">
        <div>
          <div class="page-note-label">{{ $t('Current data') }}</div>
          <div class="page-note-value">{{ summary }}</div>
        </div>
        <div class="page-hero-actions">
          <UiButton @click="loadUsers" :loading="loading">{{ $t('Refresh list') }}</UiButton>
        </div>
      </aside>
    </section>

    <section class="page-panel">
      <div class="page-panel-header">
        <div>
          <h3 class="page-panel-title">{{ $t('Accounts') }}</h3>
          <p class="page-panel-subtitle">{{ $t('Common actions stay inline for faster operation.') }}</p>
        </div>
        <UiButton data-test="new-user-button" type="primary" @click="openCreateDialog">
          {{ $t('New user') }}
        </UiButton>
      </div>

      <div class="surface-card">
        <UiTable :data="users" :loading="loading" style="width: 100%">
          <UiTableColumn prop="ID" label="ID" width="80" />
          <UiTableColumn prop="userName" label="Username" min-width="140" />
          <UiTableColumn prop="nickName" label="Nickname" min-width="140" />
          <UiTableColumn prop="phone" label="Phone" min-width="140" />
          <UiTableColumn prop="email" label="Email" min-width="180" />
          <UiTableColumn label="Role" min-width="140">
            <template #default="{ row }">
              {{ row.authority?.authorityName || '-' }}
            </template>
          </UiTableColumn>
          <UiTableColumn label="Status" width="100">
            <template #default="{ row }">
              <UiTag :type="row.enable === 1 ? 'success' : 'danger'">
                {{ $t(row.enable === 1 ? 'Enabled' : 'Disabled') }}
              </UiTag>
            </template>
          </UiTableColumn>
          <UiTableColumn label="Actions" width="220">
            <template #default="{ row }">
              <UiButton link @click="handleResetPassword(row.ID)">{{ $t('Reset password') }}</UiButton>
              <UiButton link type="danger" @click="handleDelete(row.ID)">{{ $t('Delete') }}</UiButton>
            </template>
          </UiTableColumn>
        </UiTable>
      </div>
    </section>

    <UiDialog v-model="createDialogVisible" title="New user" width="560px">
      <UiForm class="user-form" @submit.prevent="handleCreateUser">
        <UiFormItem label="Username">
          <UiInput v-model="createForm.userName" placeholder="Username" />
        </UiFormItem>
        <UiFormItem label="Nickname">
          <UiInput v-model="createForm.nickName" placeholder="Nickname" />
        </UiFormItem>
        <UiFormItem label="Password">
          <UiInput v-model="createForm.password" type="password" placeholder="Password" showPassword />
        </UiFormItem>
        <UiFormItem label="Role">
          <UiSelect v-model="createForm.authorityId" data-test="user-role-select" placeholder="Select role">
            <UiOption
              v-for="role in roleOptions"
              :key="role.authorityId"
              :label="role.authorityName"
              :value="role.authorityId"
            />
          </UiSelect>
        </UiFormItem>
        <UiFormItem label="Phone">
          <UiInput v-model="createForm.phone" placeholder="Phone" />
        </UiFormItem>
        <UiFormItem label="Email">
          <UiInput v-model="createForm.email" placeholder="Email" />
        </UiFormItem>
        <UiFormItem label="Status">
          <UiSelect v-model="createForm.enable">
            <UiOption label="Enabled" :value="1" />
            <UiOption label="Disabled" :value="0" />
          </UiSelect>
        </UiFormItem>
      </UiForm>
      <template #footer>
        <UiButton @click="createDialogVisible = false">{{ $t('Cancel') }}</UiButton>
        <UiButton type="primary" :loading="createUserMutation.isPending.value" @click="handleCreateUser">
          {{ $t('Create') }}
        </UiButton>
      </template>
    </UiDialog>
  </div>
</template>

<script setup lang="ts">
import { computed, reactive, ref, watch } from 'vue'
import { ElMessage, ElMessageBox } from '@/ui/feedback'

import { usePageChrome } from '@/composables/usePageChrome'
import { getApiErrorMessage } from '@/api/http'
import { t } from '@/i18n'
import { useCreateUserMutation, useDeleteUserMutation, useResetUserPasswordMutation, useUserRolesQuery, useUsersQuery } from './userQueries'

const usersQuery = useUsersQuery()
const rolesQuery = useUserRolesQuery()
const createUserMutation = useCreateUserMutation()
const deleteUserMutation = useDeleteUserMutation()
const resetUserPasswordMutation = useResetUserPasswordMutation()
const users = computed(() => usersQuery.data.value?.list || [])
const loading = computed(() => usersQuery.isFetching.value || deleteUserMutation.isPending.value)
const createDialogVisible = ref(false)
const createForm = reactive({
  userName: '',
  nickName: '',
  password: '123456',
  phone: '',
  email: '',
  enable: 1,
  authorityId: undefined as number | undefined
})
const { total, summary } = usePageChrome(users, 'users')
const enabledCount = computed(() => users.value.filter((item) => item.enable === 1).length)
const roleCount = computed(
  () => new Set(users.value.map((item) => item.authority?.authorityName).filter(Boolean)).size
)
const roleOptions = computed(() => rolesQuery.data.value || [])

watch(roleOptions, (roles) => {
  if (createDialogVisible.value && !createForm.authorityId) {
    createForm.authorityId = roles[0]?.authorityId
  }
})

async function loadUsers() {
  try {
    await usersQuery.refetch()
  } catch (err) {
    ElMessage.error(getApiErrorMessage(err, t('Failed to load users')))
  }
}

function resetCreateForm() {
  createForm.userName = ''
  createForm.nickName = ''
  createForm.password = '123456'
  createForm.phone = ''
  createForm.email = ''
  createForm.enable = 1
  createForm.authorityId = roleOptions.value[0]?.authorityId
}

function openCreateDialog() {
  resetCreateForm()
  createDialogVisible.value = true
}

async function handleCreateUser() {
  if (!createForm.userName.trim() || !createForm.nickName.trim() || !createForm.password || !createForm.authorityId) {
    ElMessage.error(t('Username, nickname, password, and role are required'))
    return
  }

  try {
    const res = await createUserMutation.mutateAsync({ ...createForm })
    if (res.code === 'OK') {
      ElMessage.success(t('Created'))
      createDialogVisible.value = false
      resetCreateForm()
    } else {
      ElMessage.error(res.message || t('Create failed'))
    }
  } catch (err) {
    ElMessage.error(getApiErrorMessage(err, t('Create failed')))
  }
}

async function handleDelete(id: number) {
  try {
    await ElMessageBox.confirm(t('Delete this user?'), t('Notice'), {
      type: 'warning'
    })
  } catch {
    return
  }

  try {
    const res = await deleteUserMutation.mutateAsync(id)
    if (res.code === 'OK') {
      ElMessage.success(t('Deleted'))
    } else {
      ElMessage.error(res.message || t('Delete failed'))
    }
  } catch (err) {
    ElMessage.error(getApiErrorMessage(err, t('Delete failed')))
  }
}

async function handleResetPassword(id: number) {
  try {
    const res = await resetUserPasswordMutation.mutateAsync({ id, password: '123456' })
    if (res.code === 'OK') {
      ElMessage.success(t('Password reset to 123456'))
    } else {
      ElMessage.error(res.message || t('Reset failed'))
    }
  } catch (err) {
    ElMessage.error(getApiErrorMessage(err, t('Reset failed')))
  }
}
</script>

<style scoped>
.user-form {
  display: grid;
  gap: 14px;
}
</style>
