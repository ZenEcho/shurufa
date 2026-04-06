import { defaultConfig, type AppConfig, type DashboardSnapshot, type ErrorLogEntry, type HotkeyEntry, type InputHistoryEntry, type NewUserDictionaryEntry, type RuntimeStatus, type TypingKeyInput, type TypingSessionState, type TypingSnapshot, type UserDictionaryEntry } from '@shurufa/shared-types'
import { vi } from 'vitest'

type ImeApi = Window['imeApi']

export const runtimeStatusFixture: RuntimeStatus = {
  serviceStatus: 'running',
  activePlatform: 'windows-tsf',
  defaultSchema: 'pinyin'
}

export const dashboardFixture: DashboardSnapshot = {
  metrics: [
    { label: 'Platform', value: 'windows-tsf', hint: 'Runtime platform' },
    { label: 'Service', value: 'running', hint: 'Background service state' }
  ]
}

export const configFixture: AppConfig = structuredClone(defaultConfig)

export const updatedConfigFixture: AppConfig = {
  ...structuredClone(defaultConfig),
  input: {
    ...defaultConfig.input,
    defaultSchema: 'double-pinyin',
    candidatePageSize: 5
  }
}

export const dictionaryEntryFixture: UserDictionaryEntry = {
  id: 1,
  schemaId: 'pinyin',
  code: 'nihao',
  word: '你好',
  weight: 100,
  source: 'user',
  createdAt: 1_710_000_000_000,
  updatedAt: 1_710_000_000_000
}

export const newDictionaryEntryFixture: NewUserDictionaryEntry = {
  schemaId: 'pinyin',
  code: 'shurufa',
  word: '输入法',
  weight: 90,
  source: 'manual'
}

export const historyEntryFixture: InputHistoryEntry = {
  id: 2,
  schemaId: 'pinyin',
  inputCode: 'nihao',
  committedText: '你好',
  usageCount: 8,
  lastUsedAt: 1_710_000_000_000,
  createdAt: 1_709_000_000_000
}

export const hotkeyEntryFixture: HotkeyEntry = {
  id: 'toggle',
  action: 'toggle-input-mode',
  accelerator: 'Ctrl+Space',
  scope: 'global',
  enabled: true,
  updatedAt: 1_710_000_000_000
}

export const errorLogFixture: ErrorLogEntry = {
  id: 3,
  level: 'error',
  module: 'ime-service',
  message: 'panic recovered',
  contextJson: '{"trace":"demo"}',
  createdAt: 1_710_000_000_000
}

export const typingSessionFixture: TypingSessionState = {
  rawKeys: '',
  compositionText: '',
  selectedIndex: 0,
  candidates: [],
  inputMode: 'Chinese'
}

export const typingSnapshotFixture: TypingSnapshot = {
  session: {
    ...typingSessionFixture,
    rawKeys: 'n',
    compositionText: '你',
    candidates: [
      { id: '1', text: '你', annotation: null, hotkey: '1' }
    ]
  },
  response: {
    consumed: true,
    commitText: null,
    preedit: {
      compositionText: '你',
      cursor: 1
    },
    candidates: {
      items: [{ id: '1', text: '你', annotation: null, hotkey: '1' }],
      pageIndex: 0,
      hasNextPage: false
    },
    inputMode: 'Chinese'
  }
}

export const createImeApiMock = (overrides: Partial<ImeApi> = {}) => {
  const api = {
    getDashboard: vi.fn(async () => dashboardFixture),
    getRuntimeStatus: vi.fn(async () => runtimeStatusFixture),
    getConfig: vi.fn(async () => configFixture),
    createTypingSession: vi.fn(async () => typingSessionFixture),
    processTypingKey: vi.fn(async (_session: TypingSessionState, _key: TypingKeyInput) => typingSnapshotFixture),
    updateConfig: vi.fn(async (_config: AppConfig) => runtimeStatusFixture),
    resetConfig: vi.fn(async () => configFixture),
    listUserDictionary: vi.fn(async () => [dictionaryEntryFixture]),
    createUserDictionaryEntry: vi.fn(async (_entry: NewUserDictionaryEntry) => [dictionaryEntryFixture]),
    deleteUserDictionaryEntry: vi.fn(async (_id: number) => []),
    listHistory: vi.fn(async () => [historyEntryFixture]),
    listHotkeys: vi.fn(async () => [hotkeyEntryFixture]),
    saveHotkey: vi.fn(async (_entry: HotkeyEntry) => [hotkeyEntryFixture]),
    deleteHotkey: vi.fn(async (_id: string) => []),
    listErrorLogs: vi.fn(async () => [errorLogFixture]),
    ...overrides
  } satisfies ImeApi

  Object.defineProperty(window, 'imeApi', {
    configurable: true,
    writable: true,
    value: api
  })

  return api
}
