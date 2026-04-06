import { contextBridge, ipcRenderer } from 'electron'
import type { DashboardSnapshot, RuntimeStatus } from '@shurufa/shared-types'

const api = {
  getDashboard: (): Promise<DashboardSnapshot> => ipcRenderer.invoke('ime:get-dashboard'),
  getRuntimeStatus: (): Promise<RuntimeStatus> => ipcRenderer.invoke('ime:get-runtime-status')
}

contextBridge.exposeInMainWorld('imeApi', api)

