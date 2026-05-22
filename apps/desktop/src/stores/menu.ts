import { defineStore } from 'pinia'
import { ref } from 'vue'

export interface CoreMenuItem {
  key: string
  label: string
  path: string
}

const coreMenuItems: CoreMenuItem[] = [
  { key: 'dashboard', label: '仪表盘', path: '/dashboard' },
  { key: 'users', label: '用户', path: '/users' },
  { key: 'roles', label: '角色', path: '/roles' },
  { key: 'menus', label: '菜单', path: '/menus' },
  { key: 'apis', label: 'API', path: '/apis' },
  { key: 'params', label: '参数', path: '/params' },
  { key: 'dictionaries', label: '字典', path: '/dictionaries' },
  { key: 'files', label: '文件', path: '/files' },
  { key: 'login-logs', label: '登录日志', path: '/login-logs' },
  { key: 'operation-logs', label: '操作日志', path: '/operation-logs' },
  { key: 'profile', label: '个人中心', path: '/profile' },
  { key: 'system-config', label: '系统配置', path: '/system-config' },
  { key: 'system-state', label: '系统状态', path: '/system-state' }
]

export interface RemoteMenuItem {
  name: string
  path?: string
  meta?: {
    title?: string
  }
}

export function buildCoreMenuItems(remoteMenus: RemoteMenuItem[] = []) {
  if (!remoteMenus.length) {
    return coreMenuItems
  }

  const remoteMap = new Map(
    remoteMenus
      .filter((item) => item.name)
      .map((item) => [
        item.name,
        {
          key: item.name,
          label: item.meta?.title || item.name,
          path: `/${(item.path || item.name).replace(/^\/+/, '')}`
        } satisfies CoreMenuItem
      ])
  )

  return coreMenuItems.map((item) => remoteMap.get(item.key) || item)
}

export const useMenuStore = defineStore('menu', () => {
  const items = ref<CoreMenuItem[]>(coreMenuItems)

  function setItems(nextItems: CoreMenuItem[]) {
    items.value = nextItems
  }

  return {
    items,
    setItems
  }
})
