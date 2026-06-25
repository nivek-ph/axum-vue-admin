<template>
  <div class="admin-page">
    <section class="role-workspace-header">
      <div>
        <span class="panel-kicker">{{ $t('Role permissions') }}</span>
        <h2 class="workspace-title">{{ $t('Roles') }}</h2>
        <p class="workspace-subtitle">{{ $t('Manage unified permissions by page and button, shared by frontend and backend.') }}</p>
      </div>
      <div class="workspace-actions">
        <UiButton @click="loadWorkbench" :loading="loading">{{ $t('Refresh') }}</UiButton>
        <UiButton v-if="canCreateRole" type="primary" @click="openCreateDialog">{{ $t('New') }}</UiButton>
      </div>
    </section>

    <section class="role-workbench">
      <aside class="role-sidebar">
        <div class="role-sidebar-header">
          <div>
            <h3 class="role-sidebar-title">{{ $t('Role') }}</h3>
            <p class="role-sidebar-subtitle">{{ $t('Total {count} roles', { count: authorityOptions.length }) }}</p>
          </div>
          <UiButton v-if="canCreateRole" type="primary" @click="openCreateDialog">+</UiButton>
        </div>

        <UiInput v-model="roleSearch" placeholder="Search role name/ID" />

        <div class="role-list" data-test="role-list">
          <button
            v-for="authority in filteredAuthorityOptions"
            :key="authority.authorityId"
            :class="['role-list-item', selectedAuthorityId === authority.authorityId && 'is-active']"
            type="button"
            @click="selectAuthority(authority)"
          >
            <span class="role-list-main">
              <span class="role-list-name">{{ authority.authorityName }}</span>
              <span class="role-list-meta">ID {{ authority.authorityId }} · {{ authority.defaultRouter || 'dashboard' }}</span>
            </span>
          </button>

          <div v-if="filteredAuthorityOptions.length === 0" class="empty-state">{{ $t('No matching roles') }}</div>
        </div>
      </aside>

      <section class="permission-panel">
        <div class="permission-panel-header">
          <div>
            <p class="panel-kicker">{{ $t(activeTab === 'users' ? 'Current role' : 'Function permissions') }}</p>
            <h3 class="permission-title">
              {{ selectedAuthority?.authorityName || $t('Select a role') }}
            </h3>
            <p class="permission-subtitle">
              {{
                activeTab === 'users'
                  ? selectedAuthority
                    ? $t('Default entry: {route}', { route: selectedAuthority.defaultRouter || 'dashboard' })
                    : $t('Select a role from the left to manage members')
                  : selectedAuthority
                    ? $t('Select page access and operation buttons. Hover to inspect the mapped API.')
                    : $t('Select a role from the left to manage function permissions')
              }}
            </p>
          </div>

          <div v-if="selectedAuthority && activeTab === 'users'" class="role-actions">
            <UiButton v-if="canUpdateRole" @click="openEditDialog(selectedAuthority)">{{ $t('Edit') }}</UiButton>
            <UiButton
              v-if="canDeleteRole"
              type="danger"
              :disabled="selectedAuthority.authorityId === 888"
              @click="handleDelete(selectedAuthority)"
            >
              {{ $t('Delete') }}
            </UiButton>
          </div>
        </div>

        <div v-if="authorityOptions.length > 0" class="permission-tabs">
          <button
            data-test="function-permissions-tab"
            :class="['permission-tab', activeTab === 'menus' && 'is-active']"
            type="button"
            @click="switchTab('menus')"
          >
            {{ $t('Function permissions') }}
          </button>
          <button
            v-if="canViewRoleUsers"
            data-test="role-users-tab"
            :class="['permission-tab', activeTab === 'users' && 'is-active']"
            type="button"
            @click="switchTab('users')"
          >
            {{ $t('Role users') }}
          </button>
        </div>

        <div v-if="authorityOptions.length === 0" class="empty-state large">{{ $t('No role data') }}</div>

        <div v-else-if="activeTab === 'menus' && selectedAuthority" class="permission-content">
          <div class="content-toolbar">
            <div>
              <h4 class="content-title">{{ $t('Function permissions') }}</h4>
              <p class="content-subtitle">{{ $t('Select page access to include button permissions under that page and avoid visible pages with 403 APIs.') }}</p>
            </div>
            <UiButton
              v-if="canManageFunctionPermissions"
              data-test="save-function-permissions"
              type="primary"
              :loading="functionSubmitting"
              @click="saveFunctionPermissions"
            >
              {{ $t('Save permissions') }}
            </UiButton>
          </div>

          <div class="permission-catalog-scroll">
            <table class="permission-catalog">
              <thead>
                <tr>
                  <th class="resource-column">{{ $t('Menu') }}</th>
                  <th class="actions-column">{{ $t('Permission') }}</th>
                </tr>
              </thead>
              <tbody>
                <tr v-for="menu in flatMenus" :key="menu.ID">
                  <td class="resource-cell" :style="{ '--indent': `${menu.level * 22}px` }">
                    <div class="menu-resource">
                      <span class="menu-resource-title">{{ menuTitle(menu) }}</span>
                      <span class="menu-resource-path">{{ menu.path }}</span>
                    </div>
                  </td>
                  <td class="actions-cell">
                    <div class="permission-actions">
                      <label
                        class="permission-chip"
                        :class="pageAccessChecked(menu) && 'is-checked'"
                        data-test="page-access-chip"
                      >
                        <input
                          :data-test="`menu-permission-${menu.ID}-${selectedAuthority.authorityId}`"
                          type="checkbox"
                          :disabled="!canManageFunctionPermissions"
                          :checked="pageAccessChecked(menu)"
                          @change="onPageAccessChange(menu, $event)"
                        />
                        <span class="permission-chip-label">{{ $t('Page access') }}</span>
                        <span class="permission-chip-tip">{{ $t('Page visible') }} · {{ menu.path }}</span>
                      </label>

                      <label
                        v-for="action in actionsForMenu(menu)"
                        :key="action.ID"
                        class="permission-chip"
                        :class="actionAccessChecked(action) && 'is-checked'"
                      >
                        <input
                          :data-test="`action-permission-${action.permission || action.ID}-${selectedAuthority.authorityId}`"
                          type="checkbox"
                          :disabled="!canManageFunctionPermissions"
                          :checked="actionAccessChecked(action)"
                          @change="onActionAccessChange(menu, action, $event)"
                        />
                        <span class="permission-chip-label">{{ actionTitle(action) }}</span>
                        <span class="permission-chip-tip">{{ actionTip(action) }}</span>
                      </label>

                      <span v-if="actionsForMenu(menu).length === 0" class="permission-empty">{{ $t('No button permissions') }}</span>
                    </div>
                  </td>
                </tr>
              </tbody>
            </table>

            <div v-if="flatMenus.length === 0" class="empty-state">{{ $t('No menu data') }}</div>
          </div>
        </div>

        <div v-else-if="selectedAuthority" class="permission-content">
          <div class="content-toolbar">
            <div>
              <h4 class="content-title">{{ $t('Role users') }}</h4>
              <p class="content-subtitle">{{ $t('Maintain users assigned to the current role.') }}</p>
            </div>
            <div class="member-count">
              <span>{{ $t('Selected members') }}</span>
              <strong>{{ selectedUserIds.length }} / {{ userOptions.length }}</strong>
            </div>
          </div>

          <div class="member-tools">
            <UiInput v-model="memberSearch" placeholder="Search users" />
            <UiButton
              v-if="canAssignRoleUsers"
              type="primary"
              :loading="userSubmitting"
              @click="submitRoleUsers"
            >
              {{ $t('Save members') }}
            </UiButton>
          </div>

          <div class="member-list" data-test="member-list">
            <label
              v-for="user in filteredUserOptions"
              :key="user.ID"
              :class="['member-card', selectedUserIdSet.has(user.ID) && 'is-selected']"
            >
              <input
                class="member-checkbox"
                type="checkbox"
                :checked="selectedUserIdSet.has(user.ID)"
                @change="toggleUserSelection(user.ID)"
              />
              <span class="member-checkmark">
                <span v-if="selectedUserIdSet.has(user.ID)">✓</span>
              </span>
              <span class="member-avatar">{{ userInitial(user) }}</span>
              <span class="member-main">
                <span class="member-name">{{ user.nickName || user.userName }}</span>
                <span class="member-meta">{{ user.userName }}<span v-if="user.email"> · {{ user.email }}</span></span>
              </span>
            </label>

            <div v-if="filteredUserOptions.length === 0" class="empty-state">
              {{ $t(userOptions.length === 0 ? 'No users available' : 'No matching users') }}
            </div>
          </div>
        </div>

        <div v-else class="empty-state large">{{ $t('Select a role from the left first') }}</div>
      </section>
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
import {
  fetchPermissionMenuRoleMatrix,
  fetchPermissionMenuList,
  setAuthorityMenus,
  type MenuRecord
} from '@/api/menus'
import { useAuthStore } from '@/stores/auth'
import { fetchUsers, type UserRecord } from '@/api/users'
import { t } from '@/i18n'

type DialogMode = 'create' | 'edit'
type PermissionTab = 'menus' | 'users'
type FlatMenu = MenuRecord & { level: number }

const authorities = ref<AuthorityRecord[]>([])
const menus = ref<MenuRecord[]>([])
const menuRoleMatrix = ref<Record<number, number[]>>({})
const loading = ref(false)
const accessLoading = ref(false)
const dialogVisible = ref(false)
const dialogMode = ref<DialogMode>('create')
const submitting = ref(false)
const menuSubmitting = ref(false)
const functionSubmitting = computed(() => menuSubmitting.value)
const userSubmitting = ref(false)
const selectedAuthorityId = ref<number | null>(null)
const activeTab = ref<PermissionTab>('menus')
const roleSearch = ref('')
const memberSearch = ref('')
const userOptions = ref<UserRecord[]>([])
const selectedUserIds = ref<number[]>([])
const permissionsDirty = ref(false)
const form = reactive({
  authorityId: 0,
  authorityName: '',
  parentId: 0,
  defaultRouter: 'dashboard'
})

const authStore = useAuthStore()
const authorityOptions = computed(() => flattenAuthorities(authorities.value))
const selectedAuthority = computed(
  () => authorityOptions.value.find((item) => item.authorityId === selectedAuthorityId.value) || null
)
const selectedUserIdSet = computed(() => new Set(selectedUserIds.value))
const flatMenus = computed(() => flattenMenus(menus.value))
const canCreateRole = computed(() => authStore.can('system:role:create'))
const canUpdateRole = computed(() => authStore.can('system:role:update'))
const canDeleteRole = computed(() => authStore.can('system:role:delete'))
const canAssignRoleUsers = computed(() => authStore.can('system:role:assign-users'))
const canViewRoleUsers = computed(() => authStore.can('system:role:list-users') && authStore.can('system:user:list'))
const canManageFunctionPermissions = computed(() => authStore.can('system:role:update-permission'))
const filteredAuthorityOptions = computed(() => {
  const keyword = roleSearch.value.trim().toLowerCase()
  if (!keyword) return authorityOptions.value
  return authorityOptions.value.filter((authority) =>
    [authority.authorityName, authority.authorityId, authority.defaultRouter]
      .filter(Boolean)
      .some((value) => String(value).toLowerCase().includes(keyword))
  )
})
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

function flattenMenus(list: MenuRecord[], level = 0): FlatMenu[] {
  return list
    .filter((item) => item.menuType !== 'action')
    .flatMap((item) => [
      { ...item, level },
      ...flattenMenus((item.children || []).filter((child) => child.menuType !== 'action'), level + 1)
    ])
}

function resetForm() {
  form.authorityId = 0
  form.authorityName = ''
  form.parentId = 0
  form.defaultRouter = 'dashboard'
}

async function loadWorkbench() {
  loading.value = true
  try {
    const [authorityList, menuTree, menuMatrix] = await Promise.all([
      fetchAuthorities(),
      fetchPermissionMenuList(),
      fetchPermissionMenuRoleMatrix()
    ])
    authorities.value = authorityList
    menus.value = menuTree
    menuRoleMatrix.value = menuMatrix
    permissionsDirty.value = false

    const stillExists = authorityOptions.value.some((item) => item.authorityId === selectedAuthorityId.value)
    if (!stillExists) {
      selectedAuthorityId.value = authorityOptions.value[0]?.authorityId || null
    } else if (!selectedAuthorityId.value) {
      selectedAuthorityId.value = authorityOptions.value[0]?.authorityId || null
    }

    if (selectedAuthorityId.value && activeTab.value === 'users' && canViewRoleUsers.value) {
      await loadRoleAccess()
    }
  } catch {
    ElMessage.error(t('Failed to load roles'))
  } finally {
    loading.value = false
  }
}

async function loadRoleAccess() {
  if (!selectedAuthorityId.value) return

  accessLoading.value = true
  try {
    const [users, userIds] = await Promise.all([
      fetchUsers(1, 200),
      fetchAuthorityUsers(selectedAuthorityId.value)
    ])
    userOptions.value = users.list
    selectedUserIds.value = userIds
  } catch {
    ElMessage.error(t('Failed to load role permissions'))
  } finally {
    accessLoading.value = false
  }
}

function selectAuthority(authority: AuthorityRecord) {
  if (selectedAuthorityId.value === authority.authorityId) return
  selectedAuthorityId.value = authority.authorityId
  memberSearch.value = ''
  if (activeTab.value === 'users' && canViewRoleUsers.value) {
    loadRoleAccess()
  }
}

function switchTab(tab: PermissionTab) {
  activeTab.value = tab
  if (tab === 'users' && canViewRoleUsers.value) {
    loadRoleAccess()
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
      selectedAuthorityId.value = form.authorityId
      await loadWorkbench()
      return
    }

    ElMessage.error(response.message || t('Failed to save role'))
  } catch {
    ElMessage.error(t('Failed to save role'))
  } finally {
    submitting.value = false
  }
}

function actionsForMenu(menu: MenuRecord) {
  return (menu.children || [])
    .filter((child) => child.menuType === 'action')
    .sort((left, right) => left.sort - right.sort || left.ID - right.ID)
}

function pageAccessChecked(menu: FlatMenu) {
  if (!selectedAuthorityId.value) return false
  return isMenuRoleChecked(menu.ID, selectedAuthorityId.value)
}

function actionAccessChecked(action: MenuRecord) {
  if (!selectedAuthorityId.value) return false
  return isMenuRoleChecked(action.ID, selectedAuthorityId.value)
}

function setMenuAccess(menuId: number, authorityId: number, enabled: boolean) {
  const current = isMenuRoleChecked(menuId, authorityId)
  if (current !== enabled) {
    toggleMenuRole(menuId, authorityId)
  }
}

function onPageAccessChange(menu: FlatMenu, event: Event) {
  if (!selectedAuthorityId.value || !canManageFunctionPermissions.value) return

  const enabled = (event.target as HTMLInputElement).checked
  setMenuAccess(menu.ID, selectedAuthorityId.value, enabled)
  for (const action of actionsForMenu(menu)) {
    setMenuAccess(action.ID, selectedAuthorityId.value, enabled)
  }
}

function onActionAccessChange(menu: FlatMenu, action: MenuRecord, event: Event) {
  if (!selectedAuthorityId.value || !canManageFunctionPermissions.value) return

  const enabled = (event.target as HTMLInputElement).checked
  setMenuAccess(action.ID, selectedAuthorityId.value, enabled)
  if (enabled) {
    setMenuAccess(menu.ID, selectedAuthorityId.value, true)
  }
}

async function saveFunctionPermissions() {
  await saveMenuPermissions()
}

function isMenuRoleChecked(menuId: number, authorityId: number) {
  return (menuRoleMatrix.value[menuId] || []).includes(authorityId)
}

function toggleMenuRole(menuId: number, authorityId: number) {
  if (!canManageFunctionPermissions.value) return

  const current = new Set(menuRoleMatrix.value[menuId] || [])
  if (current.has(authorityId)) {
    current.delete(authorityId)
  } else {
    current.add(authorityId)
  }
  menuRoleMatrix.value = {
    ...menuRoleMatrix.value,
    [menuId]: [...current].sort((a, b) => a - b)
  }
  permissionsDirty.value = true
}

async function saveMenuPermissions(): Promise<boolean> {
  if (!selectedAuthorityId.value || !canManageFunctionPermissions.value) return false

  if (!permissionsDirty.value) {
    ElMessage.success(t('Role permissions updated'))
    return true
  }

  menuSubmitting.value = true
  try {
    const roleMenuIds = Object.entries(menuRoleMatrix.value)
      .filter(([, authorityIds]) => authorityIds.includes(selectedAuthorityId.value as number))
      .map(([menuId]) => Number(menuId))
      .filter((menuId) => Number.isFinite(menuId))
      .sort((left, right) => left - right)

    await setAuthorityMenus(selectedAuthorityId.value, roleMenuIds)
    menuRoleMatrix.value = await fetchPermissionMenuRoleMatrix()
    permissionsDirty.value = false
    ElMessage.success(t('Role permissions updated'))
    return true
  } catch {
    ElMessage.error(t('Failed to save permissions'))
    return false
  } finally {
    menuSubmitting.value = false
  }
}

function toggleUserSelection(userId: number) {
  selectedUserIds.value = selectedUserIds.value.includes(userId)
    ? selectedUserIds.value.filter((id) => id !== userId)
    : [...selectedUserIds.value, userId].sort((a, b) => a - b)
}

function userInitial(user: UserRecord) {
  return (user.nickName || user.userName || '?').slice(0, 1).toUpperCase()
}

function menuTitle(menu: MenuRecord) {
  return t(menu.meta?.title || menu.name || menu.path)
}

function actionTitle(action: MenuRecord) {
  return t(action.meta?.title || action.permission || action.name)
}

function actionTip(action: MenuRecord) {
  const route = [action.method, action.apiPath].filter(Boolean).join(' ')
  return action.permission ? `${action.permission}${route ? ` · ${route}` : ''}` : route
}

async function submitRoleUsers() {
  if (!selectedAuthorityId.value) return

  userSubmitting.value = true
  try {
    const response = await setRoleUsers(selectedAuthorityId.value, selectedUserIds.value)
    if (response.code === 'OK') {
      ElMessage.success(t('Role members updated'))
      await loadRoleAccess()
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
      if (selectedAuthorityId.value === authority.authorityId) {
        selectedAuthorityId.value = null
      }
      await loadWorkbench()
      return
    }

    ElMessage.error(response.message || t('Failed to delete role'))
  } catch {
    ElMessage.error(t('Failed to delete role'))
  }
}

onMounted(() => {
  loadWorkbench()
})
</script>

<style scoped>
.role-workbench {
  display: grid;
  grid-template-columns: minmax(220px, 260px) minmax(0, 1fr);
  min-height: calc(100vh - 178px);
  overflow: hidden;
  border: 1px solid #e7e5e4;
  border-radius: 18px;
  background: rgba(255, 255, 255, 0.94);
  box-shadow: 0 18px 48px rgba(24, 24, 27, 0.06);
}

.role-workspace-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 16px;
  border: 1px solid #e7e5e4;
  border-radius: 18px;
  background: rgba(255, 255, 255, 0.94);
  padding: 16px 18px;
  box-shadow: 0 10px 28px rgba(24, 24, 27, 0.04);
}

.workspace-title {
  margin: 4px 0 0;
  color: #18181b;
  font-size: 24px;
  line-height: 1.12;
  font-weight: 780;
}

.workspace-subtitle {
  margin: 6px 0 0;
  color: var(--text-muted);
  font-size: 13px;
}

.workspace-actions {
  display: flex;
  align-items: center;
  flex-wrap: nowrap;
  justify-content: flex-end;
  gap: 10px;
}

.role-sidebar,
.permission-panel {
  background: transparent;
}

.role-sidebar {
  display: grid;
  align-content: start;
  gap: 14px;
  border-right: 1px solid #e7e5e4;
  padding: 18px;
}

.role-sidebar-header,
.permission-panel-header,
.content-toolbar,
.member-tools {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 14px;
}

.role-sidebar-title,
.permission-title,
.content-title {
  margin: 0;
  color: #18181b;
  font-weight: 760;
}

.role-sidebar-title {
  font-size: 18px;
}

.role-sidebar-subtitle,
.permission-subtitle,
.content-subtitle,
.panel-kicker {
  margin: 4px 0 0;
  color: var(--text-muted);
  font-size: 13px;
}

.role-list {
  display: grid;
  gap: 6px;
  max-height: 420px;
  overflow-y: auto;
}

.role-list-item {
  display: flex;
  width: 100%;
  border: 1px solid transparent;
  border-radius: 12px;
  background: transparent;
  padding: 12px;
  text-align: left;
  cursor: pointer;
  transition: background 0.16s ease, border-color 0.16s ease;
}

.role-list-item:hover,
.role-list-item.is-active {
  border-color: #d6d3d1;
  background: #f5f5f4;
}

.role-list-item.is-active {
  box-shadow: inset 3px 0 0 #18181b;
}

.role-list-main,
.role-list-name,
.role-list-meta {
  display: block;
  min-width: 0;
}

.role-list-name {
  overflow: hidden;
  color: #18181b;
  font-size: 15px;
  font-weight: 760;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.role-list-meta {
  margin-top: 4px;
  overflow: hidden;
  color: var(--text-muted);
  font-size: 12px;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.permission-panel {
  min-width: 0;
  overflow: auto;
}

.permission-panel-header {
  padding: 18px 22px;
}

.permission-title {
  font-size: 24px;
}

.role-actions {
  display: flex;
  gap: 8px;
}

.permission-tabs {
  display: flex;
  gap: 4px;
  border-top: 1px solid #e7e5e4;
  border-bottom: 1px solid #e7e5e4;
  padding: 0 20px;
}

.permission-tab {
  border: 0;
  border-bottom: 2px solid transparent;
  background: transparent;
  color: #52525b;
  padding: 15px 16px 13px;
  font-size: 15px;
  font-weight: 760;
  cursor: pointer;
}

.permission-tab:hover,
.permission-tab.is-active {
  color: #18181b;
}

.permission-tab.is-active {
  border-bottom-color: #18181b;
}

.permission-content {
  display: grid;
  gap: 18px;
  padding: 18px 22px 22px;
}

.content-toolbar {
  align-items: flex-start;
}

.permission-matrix-scroll,
.permission-catalog-scroll,
.member-list {
  border: 1px solid #e7e5e4;
  border-radius: 14px;
  overflow: hidden;
  background: #ffffff;
}

.permission-matrix-scroll,
.permission-catalog-scroll {
  overflow-x: auto;
}

.permission-catalog {
  width: 100%;
  min-width: 720px;
  border-collapse: separate;
  border-spacing: 0;
}

.permission-catalog th,
.permission-catalog td {
  border-bottom: 1px solid #f1f1f0;
  padding: 14px 16px;
  text-align: left;
  vertical-align: top;
}

.permission-catalog th {
  position: sticky;
  top: 0;
  z-index: 1;
  background: #f5f5f4;
  color: #52525b;
  font-size: 13px;
  font-weight: 760;
}

.actions-column {
  min-width: 420px;
}

.menu-resource {
  display: grid;
  gap: 4px;
}

.menu-resource-title {
  color: #18181b;
  font-weight: 700;
}

.menu-resource-path {
  color: var(--text-muted);
  font-size: 12px;
}

.permission-actions {
  display: grid;
  grid-template-columns: repeat(auto-fill, 136px);
  align-items: center;
  gap: 8px 10px;
}

.permission-chip {
  position: relative;
  display: inline-flex;
  align-items: center;
  gap: 8px;
  width: max-content;
  max-width: 136px;
  min-height: 34px;
  border: 1px solid #e7e5e4;
  border-radius: 999px;
  background: #ffffff;
  padding: 0 12px 0 10px;
  cursor: pointer;
  transition: border-color 0.16s ease, background 0.16s ease, box-shadow 0.16s ease;
}

.permission-chip:hover,
.permission-chip.is-checked {
  border-color: #18181b;
  background: #fafaf9;
}

.permission-chip:hover {
  box-shadow: 0 10px 24px rgba(24, 24, 27, 0.08);
}

.permission-chip input {
  accent-color: #18181b;
}

.permission-chip-label {
  color: #18181b;
  font-size: 13px;
  font-weight: 650;
  white-space: nowrap;
}

.permission-chip-tip {
  position: absolute;
  left: 50%;
  bottom: calc(100% + 10px);
  z-index: 3;
  transform: translateX(-50%) translateY(4px);
  min-width: max-content;
  max-width: 360px;
  border: 1px solid #e7e5e4;
  border-radius: 10px;
  background: #18181b;
  color: #fafafa;
  padding: 8px 10px;
  font-size: 12px;
  line-height: 1.45;
  white-space: nowrap;
  opacity: 0;
  pointer-events: none;
  transition: opacity 0.16s ease, transform 0.16s ease;
}

.permission-chip-tip::after {
  content: '';
  position: absolute;
  top: 100%;
  left: 50%;
  transform: translateX(-50%);
  border: 6px solid transparent;
  border-top-color: #18181b;
}

.permission-chip:hover .permission-chip-tip {
  opacity: 1;
  transform: translateX(-50%) translateY(0);
}

.permission-empty {
  display: inline-flex;
  align-items: center;
  min-height: 34px;
  color: var(--text-muted);
  font-size: 13px;
}

.permission-matrix {
  width: 100%;
  min-width: 520px;
  border-collapse: separate;
  border-spacing: 0;
}

.permission-matrix th,
.permission-matrix td {
  border-bottom: 1px solid #f1f1f0;
  padding: 13px 14px;
  text-align: left;
  vertical-align: middle;
}

.permission-matrix th {
  position: sticky;
  top: 0;
  z-index: 1;
  background: #f5f5f4;
  color: #52525b;
  font-size: 13px;
  font-weight: 760;
  white-space: nowrap;
}

.permission-matrix th:not(.resource-column):not(.route-column),
.check-cell {
  min-width: 86px;
  text-align: center;
}

.resource-column {
  min-width: 170px;
}

.route-column {
  min-width: 140px;
}

.resource-cell {
  padding-left: calc(14px + var(--indent, 0px));
  color: #18181b;
  font-weight: 700;
  white-space: nowrap;
}

.route-cell {
  accent-color: #18181b;
  overflow: hidden;
  color: var(--text-muted);
  text-overflow: ellipsis;
  white-space: nowrap;
  max-width: 340px;
}

.check-cell input,
.member-checkbox {
  accent-color: #18181b;
}

.api-resource {
  display: flex;
  align-items: center;
  gap: 10px;
}

.member-tools {
  align-items: stretch;
}

.member-tools :deep(.ui-input) {
  flex: 1;
}

.member-count {
  display: grid;
  justify-items: end;
  gap: 4px;
}

.member-count span {
  color: var(--text-muted);
  font-size: 12px;
}

.member-count strong {
  color: #18181b;
  font-size: 18px;
}

.member-list {
  display: grid;
  gap: 8px;
  max-height: 420px;
  overflow-y: auto;
  border: 0;
  background: transparent;
}

.member-card {
  display: grid;
  grid-template-columns: 22px 34px minmax(0, 1fr);
  align-items: center;
  gap: 12px;
  min-height: 58px;
  border: 1px solid #e7e5e4;
  border-radius: 12px;
  background: #ffffff;
  padding: 10px 12px;
  cursor: pointer;
  transition: border-color 0.16s ease, background 0.16s ease;
}

.member-card:hover,
.member-card.is-selected {
  border-color: #18181b;
  background: #fafaf9;
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

.empty-state {
  border: 1px dashed #d6d3d1;
  border-radius: 12px;
  padding: 20px;
  color: var(--text-muted);
  text-align: center;
}

.empty-state.large {
  margin: 24px;
  padding: 48px;
}

.w-full {
  width: 100%;
}

@media (max-width: 720px) {
  .role-workspace-header {
    align-items: stretch;
    flex-direction: column;
  }

  .workspace-actions {
    justify-content: flex-start;
  }

  .role-workbench {
    grid-template-columns: 1fr;
    min-height: auto;
  }

  .role-sidebar {
    border-right: 0;
    border-bottom: 1px solid #e7e5e4;
  }

  .role-actions,
  .content-toolbar,
  .member-tools {
    align-items: stretch;
    flex-direction: column;
  }
}

@media (min-width: 721px) and (max-width: 1180px) {
  .role-workbench {
    grid-template-columns: minmax(170px, 190px) minmax(0, 1fr);
  }

  .role-sidebar {
    padding: 14px;
  }

  .role-list-item {
    padding: 10px;
  }

  .route-column,
  .route-cell {
    display: none;
  }

  .permission-matrix {
    min-width: 0;
    table-layout: fixed;
  }

  .permission-matrix-scroll {
    overflow-x: hidden;
  }

  .permission-matrix th,
  .permission-matrix td {
    padding: 11px 8px;
  }

  .permission-matrix th:not(.resource-column):not(.route-column),
  .check-cell {
    width: 78px;
    min-width: 78px;
  }

  .permission-matrix th:not(.resource-column):not(.route-column) {
    white-space: normal;
    word-break: break-word;
  }

  .resource-column {
    min-width: 0;
    width: auto;
  }

  .resource-cell {
    white-space: normal;
    word-break: break-word;
  }

  .api-resource {
    align-items: flex-start;
    flex-direction: column;
    gap: 5px;
  }
}
</style>
