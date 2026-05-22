<template>
  <div class="admin-page">
    <section class="page-hero">
      <div class="page-hero-main">
        <span class="page-hero-kicker">User Directory</span>
        <h2 class="page-hero-title">用户管理</h2>
        <p class="page-hero-subtitle">保持后台最常用的账号管理节奏，用更清楚的层次展示状态、角色和操作入口。</p>

        <div class="page-metrics">
          <div class="page-metric">
            <div class="page-metric-label">用户总数</div>
            <div class="page-metric-value">{{ total }}</div>
          </div>
          <div class="page-metric">
            <div class="page-metric-label">启用用户</div>
            <div class="page-metric-value">{{ enabledCount }}</div>
          </div>
          <div class="page-metric">
            <div class="page-metric-label">角色种类</div>
            <div class="page-metric-value">{{ roleCount }}</div>
          </div>
        </div>
      </div>

      <aside class="page-hero-side">
        <div>
          <div class="page-note-label">模块摘要</div>
          <div class="page-note-value">{{ summary }}</div>
        </div>
        <div class="page-hero-actions">
          <el-button @click="loadUsers" :loading="loading">刷新列表</el-button>
          <el-button type="primary" @click="router.push('/roles')">查看角色</el-button>
        </div>
      </aside>
    </section>

    <section class="page-panel">
      <div class="page-panel-header">
        <div>
          <h3 class="page-panel-title">账号列表</h3>
          <p class="page-panel-subtitle">高频动作保留在行内，减少认知负担。</p>
        </div>
      </div>

      <div class="surface-card">
        <el-table :data="users" v-loading="loading" style="width: 100%">
          <el-table-column prop="ID" label="ID" width="80" />
          <el-table-column prop="userName" label="用户名" min-width="140" />
          <el-table-column prop="nickName" label="昵称" min-width="140" />
          <el-table-column prop="phone" label="手机号" min-width="140" />
          <el-table-column prop="email" label="邮箱" min-width="180" />
          <el-table-column label="角色" min-width="140">
            <template #default="{ row }">
              {{ row.authority?.authorityName || '-' }}
            </template>
          </el-table-column>
          <el-table-column label="状态" width="100">
            <template #default="{ row }">
              <el-tag :type="row.enable === 1 ? 'success' : 'danger'">
                {{ row.enable === 1 ? '启用' : '禁用' }}
              </el-tag>
            </template>
          </el-table-column>
          <el-table-column label="操作" width="220">
            <template #default="{ row }">
              <el-button link type="primary" @click="handleResetPassword(row.ID)">重置密码</el-button>
              <el-button link type="danger" @click="handleDelete(row.ID)">删除</el-button>
            </template>
          </el-table-column>
        </el-table>
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

const router = useRouter()
const users = ref<UserRecord[]>([])
const loading = ref(false)
const { total, summary } = usePageChrome(users, '位用户')
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
    ElMessage.error(getApiErrorMessage(err, '获取用户列表失败'))
  } finally {
    loading.value = false
  }
}

async function handleDelete(id: number) {
  try {
    await ElMessageBox.confirm('确定删除该用户吗？', '提示', {
      type: 'warning'
    })
  } catch {
    return
  }

  try {
    const res = await deleteUser(id)
    if (res.code === 'OK') {
      ElMessage.success('删除成功')
      await loadUsers()
    } else {
      ElMessage.error(res.message || '删除失败')
    }
  } catch (err) {
    ElMessage.error(getApiErrorMessage(err, '删除失败'))
  }
}

async function handleResetPassword(id: number) {
  try {
    const res = await resetUserPassword(id, '123456')
    if (res.code === 'OK') {
      ElMessage.success('密码已重置为 123456')
    } else {
      ElMessage.error(res.message || '重置失败')
    }
  } catch (err) {
    ElMessage.error(getApiErrorMessage(err, '重置失败'))
  }
}

onMounted(() => {
  loadUsers()
})
</script>
