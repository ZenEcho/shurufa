import { contextBridge, ipcRenderer } from 'electron'
import type {
  AppConfig,
  DashboardSnapshot,
  ErrorLogEntry,
  HotkeyEntry,
  InputHistoryEntry,
  NewUserDictionaryEntry,
  RuntimeStatus,
  TypingKeyInput,
  TypingSessionState,
  TypingSnapshot,
  UserDictionaryEntry
} from '@shurufa/shared-types'

const api = {
  getDashboard: (): Promise<DashboardSnapshot> => ipcRenderer.invoke('ime:get-dashboard'),
  getRuntimeStatus: (): Promise<RuntimeStatus> => ipcRenderer.invoke('ime:get-runtime-status'),
  getConfig: (): Promise<AppConfig> => ipcRenderer.invoke('ime:get-config'),
  createTypingSession: (): Promise<TypingSessionState> => ipcRenderer.invoke('ime:create-typing-session'),
  processTypingKey: (session: TypingSessionState, key: TypingKeyInput): Promise<TypingSnapshot> =>
    ipcRenderer.invoke('ime:process-typing-key', session, key),
  updateConfig: (config: AppConfig): Promise<RuntimeStatus> => ipcRenderer.invoke('ime:update-config', config),
  resetConfig: (): Promise<AppConfig> => ipcRenderer.invoke('ime:reset-config'),
  listUserDictionary: (): Promise<UserDictionaryEntry[]> => ipcRenderer.invoke('ime:list-user-dictionary'),
  createUserDictionaryEntry: (entry: NewUserDictionaryEntry): Promise<UserDictionaryEntry[]> =>
    ipcRenderer.invoke('ime:create-user-dictionary-entry', entry),
  deleteUserDictionaryEntry: (id: number): Promise<UserDictionaryEntry[]> =>
    ipcRenderer.invoke('ime:delete-user-dictionary-entry', id),
  listHistory: (): Promise<InputHistoryEntry[]> => ipcRenderer.invoke('ime:list-history'),
  listHotkeys: (): Promise<HotkeyEntry[]> => ipcRenderer.invoke('ime:list-hotkeys'),
  saveHotkey: (entry: HotkeyEntry): Promise<HotkeyEntry[]> => ipcRenderer.invoke('ime:save-hotkey', entry),
  deleteHotkey: (id: string): Promise<HotkeyEntry[]> => ipcRenderer.invoke('ime:delete-hotkey', id),
  listErrorLogs: (): Promise<ErrorLogEntry[]> => ipcRenderer.invoke('ime:list-error-logs')
}

contextBridge.exposeInMainWorld('imeApi', api)
