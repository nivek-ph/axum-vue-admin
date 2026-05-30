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
          <UiButton type="primary" @click="router.push('/roles')">{{ $t('View roles') }}</UiButton>
        </div>
      </aside>
    </section>

    <section class="page-panel">
      <div class="page-panel-header">
        <div>
          <h3 class="page-panel-title">{{ $t('Accounts') }}</h3>
          <p class="page-panel-subtitle">{{ $t('Common actions stay inline for faster operation.') }}</p>
        </div>
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
              <UiButton link type="primary" @click="handleResetPassword(row.ID)">{{ $t('Reset password') }}</UiButton>
              <UiButton link type="danger" @click="handleDelete(row.ID)">{{ $t('Delete') }}</UiButton>
            </template>
          </UiTableColumn>
        </UiTable>
      </div>
    </section>
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted, ref } from 'vue'
import { useRouter } from 'vue-router'
import { ElMessage, ElMessageBox } from '@/ui/feedback'

import { usePageChrome } from '@/composables/usePageChrome'
import { deleteUser, fetchUsers, resetUserPassword, type UserRecord } from '@/api/users'
import { getApiErrorMessage } from '@/api/http'
import { t } from '@/i18n'

const router = useRouter()
const users = ref<UserRecord[]>([])
const loading = ref(false)
const { total, summary } = usePageChrome(users, 'users')
const enabledCount = computed(() => users.value.filter((item) => item.enable === 1).length)
const roleCount = computed(
  () => new Set(users.value.map((item) => item.authority?.authorityName).filter(Boolean)).size
)

async function loadUsers() {
  loading.value = true
  try {
    const result = await fetchUsers()
    users.value = result.list
  } catch (err) {
    ElMessage.error(getApiErrorMessage(err, t('Failed to load users')))
  } finally {
    loading.value = false
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
    const res = await deleteUser(id)
    if (res.code === 'OK') {
      ElMessage.success(t('Deleted'))
      await loadUsers()
    } else {
      ElMessage.error(res.message || t('Delete failed'))
    }
  } catch (err) {
    ElMessage.error(getApiErrorMessage(err, t('Delete failed')))
  }
}

async function handleResetPassword(id: number) {
  try {
    const res = await resetUserPassword(id, '123456')
    if (res.code === 'OK') {
      ElMessage.success(t('Password reset to 123456'))
    } else {
      ElMessage.error(res.message || t('Reset failed'))
    }
  } catch (err) {
    ElMessage.error(getApiErrorMessage(err, t('Reset failed')))
  }
}

onMounted(() => {
  loadUsers()
})
</script>
