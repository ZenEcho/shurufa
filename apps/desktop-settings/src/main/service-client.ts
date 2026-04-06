import { execFile } from 'node:child_process'
import { resolve } from 'node:path'
import { promisify } from 'node:util'
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

const execFileAsync = promisify(execFile)

const cargoExecutable = process.platform === 'win32' ? 'cargo.exe' : 'cargo'
const workspaceRoot = resolve(__dirname, '../../../../')
const manifestPath = resolve(workspaceRoot, 'rust', 'Cargo.toml')

type RustAppConfig = {
  general: {
    startup_with_system: boolean
    locale: string
  }
  input: {
    default_schema: string
    english_mode_by_default: boolean
    candidate_page_size: number
  }
  appearance: {
    theme: string
    font_size: number
    candidate_layout: string
  }
  logging: {
    level: string
    redact_input_content: boolean
  }
}

type RustRuntimeStatus = {
  service_status: string
  active_platform: string
  default_schema: string
}

type RustUserDictionaryEntry = {
  id: number
  schema_id: string
  code: string
  word: string
  weight: number
  source: string
  created_at: number
  updated_at: number
}

type RustInputHistoryEntry = {
  id: number
  schema_id: string
  input_code: string
  committed_text: string
  usage_count: number
  last_used_at: number
  created_at: number
}

type RustHotkeyEntry = {
  id: string
  action: string
  accelerator: string
  scope: string
  enabled: boolean
  updated_at: number
}

type RustErrorLogEntry = {
  id: number
  level: string
  module: string
  message: string
  context_json: string | null
  created_at: number
}

type RustTypingCandidate = {
  id: string
  text: string
  annotation: string | null
  hotkey: string | null
}

type RustTypingCandidatePage = {
  items: RustTypingCandidate[]
  page_index: number
  has_next_page: boolean
}

type RustTypingPreeditState = {
  composition_text: string
  cursor: number
}

type RustTypingResponse = {
  consumed: boolean
  commit_text: string | null
  preedit: RustTypingPreeditState
  candidates: RustTypingCandidatePage
  input_mode: 'Chinese' | 'English'
}

type RustTypingSessionState = {
  raw_keys: string
  composition_text: string
  selected_index: number
  candidates: RustTypingCandidate[]
  input_mode: 'Chinese' | 'English'
}

type RustTypingSnapshot = {
  session: RustTypingSessionState
  response: RustTypingResponse
}

const runServiceCommand = async <T>(command: string, payload?: string): Promise<T> => {
  const args = ['run', '--quiet', '--manifest-path', manifestPath, '-p', 'ime-service', '--', '--json', command]

  if (payload) {
    args.push(payload)
  }

  const { stdout } = await execFileAsync(cargoExecutable, args, {
    cwd: workspaceRoot,
    windowsHide: true,
    maxBuffer: 1024 * 1024 * 4
  })

  return JSON.parse(stdout.trim()) as T
}

const mapConfig = (config: RustAppConfig): AppConfig => ({
  general: {
    startupWithSystem: config.general.startup_with_system,
    locale: config.general.locale
  },
  input: {
    defaultSchema: config.input.default_schema,
    englishModeByDefault: config.input.english_mode_by_default,
    candidatePageSize: config.input.candidate_page_size
  },
  appearance: {
    theme: config.appearance.theme,
    fontSize: config.appearance.font_size,
    candidateLayout: config.appearance.candidate_layout
  },
  logging: {
    level: config.logging.level,
    redactInputContent: config.logging.redact_input_content
  }
})

const toRustConfigPayload = (config: AppConfig): RustAppConfig => ({
  general: {
    startup_with_system: config.general.startupWithSystem,
    locale: config.general.locale
  },
  input: {
    default_schema: config.input.defaultSchema,
    english_mode_by_default: config.input.englishModeByDefault,
    candidate_page_size: config.input.candidatePageSize
  },
  appearance: {
    theme: config.appearance.theme,
    font_size: config.appearance.fontSize,
    candidate_layout: config.appearance.candidateLayout
  },
  logging: {
    level: config.logging.level,
    redact_input_content: config.logging.redactInputContent
  }
})

const mapRuntimeStatus = (status: RustRuntimeStatus): RuntimeStatus => ({
  serviceStatus: status.service_status,
  activePlatform: status.active_platform,
  defaultSchema: status.default_schema
})

const mapUserDictionaryEntry = (entry: RustUserDictionaryEntry): UserDictionaryEntry => ({
  id: entry.id,
  schemaId: entry.schema_id,
  code: entry.code,
  word: entry.word,
  weight: entry.weight,
  source: entry.source,
  createdAt: entry.created_at,
  updatedAt: entry.updated_at
})

const mapHistoryEntry = (entry: RustInputHistoryEntry): InputHistoryEntry => ({
  id: entry.id,
  schemaId: entry.schema_id,
  inputCode: entry.input_code,
  committedText: entry.committed_text,
  usageCount: entry.usage_count,
  lastUsedAt: entry.last_used_at,
  createdAt: entry.created_at
})

const mapHotkeyEntry = (entry: RustHotkeyEntry): HotkeyEntry => ({
  id: entry.id,
  action: entry.action,
  accelerator: entry.accelerator,
  scope: entry.scope,
  enabled: entry.enabled,
  updatedAt: entry.updated_at
})

const mapErrorLogEntry = (entry: RustErrorLogEntry): ErrorLogEntry => ({
  id: entry.id,
  level: entry.level,
  module: entry.module,
  message: entry.message,
  contextJson: entry.context_json,
  createdAt: entry.created_at
})

const mapTypingCandidate = (entry: RustTypingCandidate) => ({
  id: entry.id,
  text: entry.text,
  annotation: entry.annotation,
  hotkey: entry.hotkey
})

const mapTypingCandidatePage = (page: RustTypingCandidatePage) => ({
  items: page.items.map(mapTypingCandidate),
  pageIndex: page.page_index,
  hasNextPage: page.has_next_page
})

const mapTypingSessionState = (session: RustTypingSessionState): TypingSessionState => ({
  rawKeys: session.raw_keys,
  compositionText: session.composition_text,
  selectedIndex: session.selected_index,
  candidates: session.candidates.map(mapTypingCandidate),
  inputMode: session.input_mode
})

const mapTypingSnapshot = (snapshot: RustTypingSnapshot): TypingSnapshot => ({
  session: mapTypingSessionState(snapshot.session),
  response: {
    consumed: snapshot.response.consumed,
    commitText: snapshot.response.commit_text,
    preedit: {
      compositionText: snapshot.response.preedit.composition_text,
      cursor: snapshot.response.preedit.cursor
    },
    candidates: mapTypingCandidatePage(snapshot.response.candidates),
    inputMode: snapshot.response.input_mode
  }
})

const toRustTypingSessionState = (session: TypingSessionState): RustTypingSessionState => ({
  raw_keys: session.rawKeys,
  composition_text: session.compositionText,
  selected_index: session.selectedIndex,
  candidates: session.candidates.map((candidate) => ({
    id: candidate.id,
    text: candidate.text,
    annotation: candidate.annotation,
    hotkey: candidate.hotkey
  })),
  input_mode: session.inputMode
})

const toRustTypingKeyPayload = (session: TypingSessionState, key: TypingKeyInput) => ({
  session: toRustTypingSessionState(session),
  event:
    key.kind === 'char'
      ? { Char: key.char }
      : key.kind === 'number'
        ? { Number: key.number }
        : key.kind === 'backspace'
          ? 'Backspace'
          : key.kind === 'enter'
            ? 'Enter'
            : key.kind === 'space'
              ? 'Space'
              : key.kind === 'escape'
                ? 'Escape'
                : 'ToggleInputMode'
})

export const getConfig = async (): Promise<AppConfig> => {
  const config = await runServiceCommand<RustAppConfig>('get-config')
  return mapConfig(config)
}

export const updateConfig = async (config: AppConfig): Promise<RuntimeStatus> => {
  const runtime = await runServiceCommand<RustRuntimeStatus>(
    'set-config',
    JSON.stringify(toRustConfigPayload(config))
  )

  return mapRuntimeStatus(runtime)
}

export const resetConfig = async (): Promise<AppConfig> => {
  const config = await runServiceCommand<RustAppConfig>('reset-config')
  return mapConfig(config)
}

export const getRuntimeStatus = async (): Promise<RuntimeStatus> => {
  const runtime = await runServiceCommand<RustRuntimeStatus>('get-runtime-status')
  return mapRuntimeStatus(runtime)
}

export const getDashboard = async (): Promise<DashboardSnapshot> => {
  const [config, runtime] = await Promise.all([getConfig(), getRuntimeStatus()])

  return {
    metrics: [
      {
        label: '当前平台',
        value: runtime.activePlatform,
        hint: '当前已接入共享 Rust 输入内核的原生宿主'
      },
      {
        label: '服务状态',
        value: runtime.serviceStatus,
        hint: '由 Rust 服务层返回的运行时健康状态'
      },
      {
        label: '默认方案',
        value: config.input.defaultSchema,
        hint: '已写入 SQLite，并通过 Rust 服务提供'
      },
      {
        label: '候选每页数量',
        value: String(config.input.candidatePageSize),
        hint: '输入配置中的共享设置，会影响候选分页'
      }
    ]
  }
}

export const listUserDictionary = async (): Promise<UserDictionaryEntry[]> => {
  const entries = await runServiceCommand<RustUserDictionaryEntry[]>('list-user-dictionary')
  return entries.map(mapUserDictionaryEntry)
}

export const createUserDictionaryEntry = async (
  entry: NewUserDictionaryEntry
): Promise<UserDictionaryEntry[]> => {
  const entries = await runServiceCommand<RustUserDictionaryEntry[]>(
    'create-user-dictionary-entry',
    JSON.stringify({
      schema_id: entry.schemaId,
      code: entry.code,
      word: entry.word,
      weight: entry.weight,
      source: entry.source
    })
  )

  return entries.map(mapUserDictionaryEntry)
}

export const deleteUserDictionaryEntry = async (id: number): Promise<UserDictionaryEntry[]> => {
  const entries = await runServiceCommand<RustUserDictionaryEntry[]>(
    'delete-user-dictionary-entry',
    JSON.stringify({ id })
  )

  return entries.map(mapUserDictionaryEntry)
}

export const listHistory = async (): Promise<InputHistoryEntry[]> => {
  const entries = await runServiceCommand<RustInputHistoryEntry[]>('list-history')
  return entries.map(mapHistoryEntry)
}

export const listHotkeys = async (): Promise<HotkeyEntry[]> => {
  const entries = await runServiceCommand<RustHotkeyEntry[]>('list-hotkeys')
  return entries.map(mapHotkeyEntry)
}

export const saveHotkey = async (entry: HotkeyEntry): Promise<HotkeyEntry[]> => {
  const entries = await runServiceCommand<RustHotkeyEntry[]>(
    'save-hotkey',
    JSON.stringify({
      id: entry.id,
      action: entry.action,
      accelerator: entry.accelerator,
      scope: entry.scope,
      enabled: entry.enabled,
      updated_at: entry.updatedAt
    })
  )

  return entries.map(mapHotkeyEntry)
}

export const deleteHotkey = async (id: string): Promise<HotkeyEntry[]> => {
  const entries = await runServiceCommand<RustHotkeyEntry[]>(
    'delete-hotkey',
    JSON.stringify({ id })
  )

  return entries.map(mapHotkeyEntry)
}

export const listErrorLogs = async (): Promise<ErrorLogEntry[]> => {
  const entries = await runServiceCommand<RustErrorLogEntry[]>('list-error-logs')
  return entries.map(mapErrorLogEntry)
}

export const createTypingSession = async (): Promise<TypingSessionState> => {
  const session = await runServiceCommand<RustTypingSessionState>('create-typing-session')
  return mapTypingSessionState(session)
}

export const processTypingKey = async (
  session: TypingSessionState,
  key: TypingKeyInput
): Promise<TypingSnapshot> => {
  const snapshot = await runServiceCommand<RustTypingSnapshot>(
    'process-typing-key',
    JSON.stringify(toRustTypingKeyPayload(session, key))
  )

  return mapTypingSnapshot(snapshot)
}
