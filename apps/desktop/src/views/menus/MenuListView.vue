<template>
  <div class="admin-page">
    <section class="page-hero">
      <div class="page-hero-main">
        <span class="page-hero-kicker">Menu Topology</span>
        <h2 class="page-hero-title">菜单管理</h2>
        <p class="page-hero-subtitle">维护菜单层级、可见性与角色绑定，并按核心后台收敛入口。</p>

        <div class="page-metrics">
          <div class="page-metric">
            <div class="page-metric-label">菜单总数</div>
            <div class="page-metric-value">{{ total }}</div>
          </div>
          <div class="page-metric">
            <div class="page-metric-label">顶级菜单</div>
            <div class="page-metric-value">{{ rootMenuCount }}</div>
          </div>
          <div class="page-metric">
            <div class="page-metric-label">隐藏菜单</div>
            <div class="page-metric-value">{{ hiddenMenuCount }}</div>
          </div>
        </div>
      </div>

      <aside class="page-hero-side">
        <div>
          <div class="page-note-label">模块摘要</div>
          <div class="page-note-value">{{ summary }}</div>
        </div>
        <div class="page-hero-actions">
          <el-button @click="loadData" :loading="loading">刷新结构</el-button>
          <el-button type="primary" @click="openCreateDialog">新增菜单</el-button>
        </div>
      </aside>
    </section>

    <section class="page-panel">
      <div class="page-panel-header">
        <div>
          <h3 class="page-panel-title">菜单结构</h3>
          <p class="page-panel-subtitle">高频信息在前，树形结构保留，复杂字段放进弹窗里处理。</p>
        </div>
      </div>

      <div class="surface-card">
        <el-table
          :data="menus"
          row-key="ID"
          default-expand-all
          v-loading="loading"
          style="width: 100%"
        >
          <el-table-column prop="ID" label="ID" width="80" />
          <el-table-column label="标题" min-width="180">
            <template #default="{ row }">
              {{ row.meta?.title || row.name }}
            </template>
          </el-table-column>
          <el-table-column prop="path" label="路径" min-width="140" />
          <el-table-column prop="name" label="名称" min-width="140" />
          <el-table-column prop="component" label="组件" min-width="220" />
          <el-table-column prop="sort" label="排序" width="90" />
          <el-table-column label="隐藏" width="90">
            <template #default="{ row }">
              <el-tag :type="row.hidden ? 'info' : 'success'">
                {{ row.hidden ? '隐藏' : '显示' }}
              </el-tag>
            </template>
          </el-table-column>
          <el-table-column label="操作" width="280">
            <template #default="{ row }">
              <el-button link type="primary" @click="openEditDialog(row)">编辑</el-button>
              <el-button link @click="openRoleDialog(row)">分配角色</el-button>
              <el-button link type="danger" @click="handleDelete(row)">删除</el-button>
            </template>
          </el-table-column>
        </el-table>
      </div>
    </section>

    <el-dialog
      v-model="dialogVisible"
      :title="dialogMode === 'create' ? '新增菜单' : '编辑菜单'"
      width="620px"
    >
      <el-form label-width="110px" @submit.prevent="submitMenu">
        <el-form-item label="父菜单">
          <el-select v-model="form.parentId" class="w-full">
            <el-option :value="0" label="顶级菜单" />
            <el-option
              v-for="item in menuOptions"
              :key="item.ID"
              :label="`${item.meta?.title || item.name} (${item.name})`"
              :value="item.ID"
            />
          </el-select>
        </el-form-item>
        <el-form-item label="路径">
          <el-input v-model="form.path" placeholder="users" />
        </el-form-item>
        <el-form-item label="名称">
          <el-input v-model="form.name" placeholder="users" />
        </el-form-item>
        <el-form-item label="标题">
          <el-input v-model="form.meta.title" placeholder="用户管理" />
        </el-form-item>
        <el-form-item label="组件">
          <el-input v-model="form.component" placeholder="view/users/index.vue" />
        </el-form-item>
        <el-form-item label="排序">
          <el-input-number v-model="form.sort" :min="0" :precision="0" class="w-full" />
        </el-form-item>
        <el-form-item label="图标">
          <el-input v-model="form.meta.icon" placeholder="setting" />
        </el-form-item>
        <el-form-item label="激活路由">
          <el-input v-model="form.meta.activeName" placeholder="users" />
        </el-form-item>
        <el-form-item label="过渡">
          <el-input v-model="form.meta.transitionType" placeholder="fade" />
        </el-form-item>
        <el-form-item label="显示设置">
          <div class="switch-group">
            <el-switch v-model="form.hidden" active-text="隐藏" inactive-text="显示" />
            <el-switch
              v-model="form.meta.keepAlive"
              active-text="缓存"
              inactive-text="不缓存"
            />
            <el-switch
              v-model="form.meta.defaultMenu"
              active-text="默认菜单"
              inactive-text="普通菜单"
            />
          </div>
        </el-form-item>
      </el-form>

      <template #footer>
        <el-button @click="dialogVisible = false">取消</el-button>
        <el-button type="primary" :loading="submitting" @click="submitMenu">保存</el-button>
      </template>
    </el-dialog>

    <el-dialog v-model="roleDialogVisible" title="分配角色" width="520px">
      <el-alert
        v-if="defaultRouterAuthorityIds.length"
        type="warning"
        :closable="false"
        class="dialog-alert"
      >
        有角色将此菜单作为默认路由：{{ defaultRouterAuthorityIds.join(', ') }}
      </el-alert>
      <el-select
        v-model="selectedAuthorityIds"
        multiple
        filterable
        class="w-full"
        placeholder="选择可见角色"
      >
        <el-option
          v-for="authority in authorityOptions"
          :key="authority.authorityId"
          :label="`${authority.authorityName} (${authority.authorityId})`"
          :value="authority.authorityId"
        />
      </el-select>

      <template #footer>
        <el-button @click="roleDialogVisible = false">取消</el-button>
        <el-button type="primary" :loading="roleSubmitting" @click="submitMenuRoles">
          保存角色
        </el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted, reactive, ref } from 'vue'
import { ElMessage, ElMessageBox } from '@/ui/feedback'

import { fetchAuthorities, type AuthorityRecord } from '@/api/authorities'
import { usePageChrome } from '@/composables/usePageChrome'
import {
  createMenu,
  deleteMenu,
  fetchMenuList,
  fetchMenuRoles,
  setMenuRoles,
  updateMenu,
  type MenuRecord
} from '@/api/menus'

type DialogMode = 'create' | 'edit'

const menus = ref<MenuRecord[]>([])
const authorities = ref<AuthorityRecord[]>([])
const loading = ref(false)
const dialogVisible = ref(false)
const dialogMode = ref<DialogMode>('create')
const submitting = ref(false)
const roleDialogVisible = ref(false)
const roleSubmitting = ref(false)
const selectedMenu = ref<MenuRecord | null>(null)
const selectedAuthorityIds = ref<number[]>([])
const defaultRouterAuthorityIds = ref<number[]>([])
const form = reactive(createEmptyMenu())

const menuOptions = computed(() => flattenMenus(menus.value))
const authorityOptions = computed(() => flattenAuthorities(authorities.value))
const { total, summary } = usePageChrome(menus, '条菜单')
const rootMenuCount = computed(() => menuOptions.value.filter((item) => item.parentId === 0).length)
const hiddenMenuCount = computed(() => menuOptions.value.filter((item) => item.hidden).length)

function createEmptyMenu(): MenuRecord {
  return {
    ID: 0,
    parentId: 0,
    path: '',
    name: '',
    hidden: false,
    component: '',
    sort: 0,
    meta: {
      activeName: '',
      keepAlive: false,
      defaultMenu: false,
      title: '',
      icon: '',
      closeTab: false,
      transitionType: ''
    },
    parameters: [],
    menuBtn: [],
    children: []
  }
}

function cloneMenu(menu: MenuRecord): MenuRecord {
  return {
    ID: menu.ID,
    parentId: menu.parentId,
    path: menu.path,
    name: menu.name,
    hidden: menu.hidden,
    component: menu.component,
    sort: menu.sort,
    meta: { ...menu.meta },
    parameters: menu.parameters.map((item) => ({ ...item })),
    menuBtn: menu.menuBtn.map((item) => ({ ...item })),
    children: []
  }
}

function assignForm(next: MenuRecord) {
  Object.assign(form, next)
  form.meta = { ...next.meta }
  form.parameters = next.parameters.map((item) => ({ ...item }))
  form.menuBtn = next.menuBtn.map((item) => ({ ...item }))
  form.children = []
}

function flattenMenus(list: MenuRecord[]): MenuRecord[] {
  return list.flatMap((item) => [item, ...flattenMenus(item.children || [])])
}

function flattenAuthorities(list: AuthorityRecord[]): AuthorityRecord[] {
  return list.flatMap((item) => [item, ...flattenAuthorities(item.children || [])])
}

async function loadData() {
  loading.value = true
  try {
    const [menuList, authorityList] = await Promise.all([fetchMenuList(), fetchAuthorities()])
    menus.value = menuList
    authorities.value = authorityList
  } catch {
    ElMessage.error('获取菜单数据失败')
  } finally {
    loading.value = false
  }
}

function openCreateDialog() {
  dialogMode.value = 'create'
  assignForm(createEmptyMenu())
  dialogVisible.value = true
}

function openEditDialog(menu: MenuRecord) {
  dialogMode.value = 'edit'
  assignForm(cloneMenu(menu))
  dialogVisible.value = true
}

async function submitMenu() {
  if (!form.name.trim() || !form.path.trim() || !form.meta.title.trim() || !form.component.trim()) {
    ElMessage.warning('请填写完整菜单信息')
    return
  }

  submitting.value = true
  try {
    const payload = cloneMenu(form)
    const response =
      dialogMode.value === 'create' ? await createMenu(payload) : await updateMenu(payload)

    if (response.code === 'OK') {
      ElMessage.success(dialogMode.value === 'create' ? '菜单已创建' : '菜单已更新')
      dialogVisible.value = false
      await loadData()
      return
    }

    ElMessage.error(response.message || '保存菜单失败')
  } catch {
    ElMessage.error('保存菜单失败')
  } finally {
    submitting.value = false
  }
}

async function openRoleDialog(menu: MenuRecord) {
  selectedMenu.value = menu
  roleDialogVisible.value = true
  try {
    const selection = await fetchMenuRoles(menu.ID)
    selectedAuthorityIds.value = selection.authorityIds
    defaultRouterAuthorityIds.value = selection.defaultRouterAuthorityIds
  } catch {
    ElMessage.error('获取菜单角色失败')
  }
}

async function submitMenuRoles() {
  if (!selectedMenu.value) return

  roleSubmitting.value = true
  try {
    const response = await setMenuRoles(selectedMenu.value.ID, selectedAuthorityIds.value)
    if (response.code === 'OK') {
      ElMessage.success('菜单角色已更新')
      roleDialogVisible.value = false
      return
    }

    ElMessage.error(response.message || '保存菜单角色失败')
  } catch {
    ElMessage.error('保存菜单角色失败')
  } finally {
    roleSubmitting.value = false
  }
}

async function handleDelete(menu: MenuRecord) {
  try {
    await ElMessageBox.confirm(`确定删除菜单“${menu.meta?.title || menu.name}”吗？`, '提示', {
      type: 'warning'
    })
  } catch {
    return
  }

  try {
    const response = await deleteMenu(menu.ID)
    if (response.code === 'OK') {
      ElMessage.success('菜单已删除')
      await loadData()
      return
    }

    ElMessage.error(response.message || '删除菜单失败')
  } catch {
    ElMessage.error('删除菜单失败')
  }
}

onMounted(() => {
  loadData()
})
</script>

<style scoped>
.switch-group {
  display: flex;
  flex-wrap: wrap;
  gap: 12px;
}

.dialog-alert {
  margin-bottom: 12px;
}

.w-full {
  width: 100%;
}
</style>
