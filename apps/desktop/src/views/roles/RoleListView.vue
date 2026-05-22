<template>
  <div class="admin-page">
    <section class="page-hero">
      <div class="page-hero-main">
        <span class="page-hero-kicker">Authority Center</span>
        <h2 class="page-hero-title">角色管理</h2>
        <p class="page-hero-subtitle">维护核心 admin 的角色结构、默认路由和成员归属。</p>

        <div class="page-metrics">
          <div class="page-metric">
            <div class="page-metric-label">角色总数</div>
            <div class="page-metric-value">{{ total }}</div>
          </div>
          <div class="page-metric">
            <div class="page-metric-label">根角色</div>
            <div class="page-metric-value">{{ rootRoleCount }}</div>
          </div>
          <div class="page-metric">
            <div class="page-metric-label">默认入口</div>
            <div class="page-metric-value">{{ defaultRouterLabel }}</div>
          </div>
        </div>
      </div>

      <aside class="page-hero-side">
        <div>
          <div class="page-note-label">模块摘要</div>
          <div class="page-note-value">{{ summary }}</div>
        </div>
        <div class="page-hero-actions">
          <el-button @click="loadAuthorities" :loading="loading">刷新列表</el-button>
          <el-button type="primary" @click="openCreateDialog">新增角色</el-button>
        </div>
      </aside>
    </section>

    <section class="page-panel">
      <div class="page-panel-header">
        <div>
          <h3 class="page-panel-title">角色列表</h3>
          <p class="page-panel-subtitle">查看树形列表，维护角色所需字段。</p>
        </div>
      </div>

      <div class="surface-card">
        <el-table
          :data="authorities"
          row-key="authorityId"
          default-expand-all
          v-loading="loading"
          style="width: 100%"
        >
          <el-table-column prop="authorityId" label="角色 ID" width="120" />
          <el-table-column prop="authorityName" label="角色名称" min-width="180" />
          <el-table-column prop="parentId" label="父角色" width="120" />
          <el-table-column prop="defaultRouter" label="默认路由" min-width="160" />
          <el-table-column label="操作" width="260">
            <template #default="{ row }">
              <el-button link type="primary" @click="openEditDialog(row)">编辑</el-button>
              <el-button link @click="openUserDialog(row)">分配成员</el-button>
              <el-button
                link
                type="danger"
                :disabled="row.authorityId === 888"
                @click="handleDelete(row)"
              >
                删除
              </el-button>
            </template>
          </el-table-column>
        </el-table>
      </div>
    </section>

    <el-dialog
      v-model="dialogVisible"
      :title="dialogMode === 'create' ? '新增角色' : '编辑角色'"
      width="520px"
    >
      <el-form label-width="100px" @submit.prevent="submitAuthority">
        <el-form-item label="角色 ID">
          <el-input-number
            v-model="form.authorityId"
            :disabled="dialogMode === 'edit'"
            :min="1"
            :precision="0"
            class="w-full"
          />
        </el-form-item>
        <el-form-item label="角色名称">
          <el-input v-model="form.authorityName" placeholder="例如：运营管理员" />
        </el-form-item>
        <el-form-item label="父角色">
          <el-select v-model="form.parentId" class="w-full">
            <el-option :value="0" label="顶级角色" />
            <el-option
              v-for="item in authorityOptions"
              :key="item.authorityId"
              :label="`${item.authorityName} (${item.authorityId})`"
              :value="item.authorityId"
            />
          </el-select>
        </el-form-item>
        <el-form-item label="默认路由">
          <el-input v-model="form.defaultRouter" placeholder="dashboard" />
        </el-form-item>
      </el-form>

      <template #footer>
        <el-button @click="dialogVisible = false">取消</el-button>
        <el-button type="primary" :loading="submitting" @click="submitAuthority">保存</el-button>
      </template>
    </el-dialog>

    <el-dialog v-model="userDialogVisible" title="分配成员" width="520px">
      <div class="dialog-summary">
        {{ selectedAuthority?.authorityName || '角色' }}
      </div>
      <el-select
        v-model="selectedUserIds"
        multiple
        filterable
        class="w-full"
        placeholder="选择用户"
      >
        <el-option
          v-for="user in userOptions"
          :key="user.ID"
          :label="`${user.nickName || user.userName} (${user.userName})`"
          :value="user.ID"
        />
      </el-select>

      <template #footer>
        <el-button @click="userDialogVisible = false">取消</el-button>
        <el-button type="primary" :loading="userSubmitting" @click="submitRoleUsers">
          保存成员
        </el-button>
      </template>
    </el-dialog>
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
const { total, summary } = usePageChrome(authorities, '条角色')
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
    ElMessage.error('获取角色列表失败')
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
    ElMessage.warning('请填写完整角色信息')
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
      ElMessage.success(dialogMode.value === 'create' ? '角色已创建' : '角色已更新')
      dialogVisible.value = false
      await loadAuthorities()
      return
    }

    ElMessage.error(response.message || '保存角色失败')
  } catch {
    ElMessage.error('保存角色失败')
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
    ElMessage.error('获取角色成员失败')
  }
}

async function submitRoleUsers() {
  if (!selectedAuthority.value) return

  userSubmitting.value = true
  try {
    const response = await setRoleUsers(selectedAuthority.value.authorityId, selectedUserIds.value)
    if (response.code === 'OK') {
      ElMessage.success('角色成员已更新')
      userDialogVisible.value = false
      return
    }

    ElMessage.error(response.message || '保存成员失败')
  } catch {
    ElMessage.error('保存成员失败')
  } finally {
    userSubmitting.value = false
  }
}

async function handleDelete(authority: AuthorityRecord) {
  try {
    await ElMessageBox.confirm(`确定删除角色“${authority.authorityName}”吗？`, '提示', {
      type: 'warning'
    })
  } catch {
    return
  }

  try {
    const response = await deleteAuthority(authority.authorityId)
    if (response.code === 'OK') {
      ElMessage.success('角色已删除')
      await loadAuthorities()
      return
    }

    ElMessage.error(response.message || '删除角色失败')
  } catch {
    ElMessage.error('删除角色失败')
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
