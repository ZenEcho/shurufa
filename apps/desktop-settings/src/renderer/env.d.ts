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

declare global {
  interface Window {
    imeApi: {
      getDashboard: () => Promise<DashboardSnapshot>
      getRuntimeStatus: () => Promise<RuntimeStatus>
      getConfig: () => Promise<AppConfig>
      createTypingSession: () => Promise<TypingSessionState>
      processTypingKey: (session: TypingSessionState, key: TypingKeyInput) => Promise<TypingSnapshot>
      updateConfig: (config: AppConfig) => Promise<RuntimeStatus>
      resetConfig: () => Promise<AppConfig>
      listUserDictionary: () => Promise<UserDictionaryEntry[]>
      createUserDictionaryEntry: (entry: NewUserDictionaryEntry) => Promise<UserDictionaryEntry[]>
      deleteUserDictionaryEntry: (id: number) => Promise<UserDictionaryEntry[]>
      listHistory: () => Promise<InputHistoryEntry[]>
      listHotkeys: () => Promise<HotkeyEntry[]>
      saveHotkey: (entry: HotkeyEntry) => Promise<HotkeyEntry[]>
      deleteHotkey: (id: string) => Promise<HotkeyEntry[]>
      listErrorLogs: () => Promise<ErrorLogEntry[]>
    }
  }
}

export {}
