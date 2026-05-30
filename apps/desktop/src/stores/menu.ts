import { defineStore } from 'pinia'
import { ref } from 'vue'

export interface CoreMenuItem {
  key: string
  label: string
  path: string
}

const coreMenuItems: CoreMenuItem[] = [
  { key: 'dashboard', label: 'Dashboard', path: '/dashboard' },
  { key: 'users', label: 'User', path: '/users' },
  { key: 'roles', label: 'Role', path: '/roles' },
  { key: 'menus', label: 'Menu', path: '/menus' },
  { key: 'apis', label: 'API', path: '/apis' },
  { key: 'params', label: 'Param', path: '/params' },
  { key: 'dictionaries', label: 'Dictionary', path: '/dictionaries' },
  { key: 'files', label: 'File', path: '/files' },
  { key: 'login-logs', label: 'Login logs', path: '/login-logs' },
  { key: 'operation-logs', label: 'Operation logs', path: '/operation-logs' },
  { key: 'profile', label: 'Profile', path: '/profile' },
  { key: 'system-config', label: 'System config', path: '/system-config' },
  { key: 'system-state', label: 'System status', path: '/system-state' }
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
