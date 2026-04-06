export type SettingsSectionId =
  | 'overview'
  | 'typing'
  | 'input'
  | 'candidates'
  | 'appearance'
  | 'dictionary'
  | 'history'
  | 'hotkeys'
  | 'logs'
  | 'about'

export interface NavSection {
  id: SettingsSectionId
  label: string
  description: string
}

export interface GeneralConfig {
  startupWithSystem: boolean
  locale: string
}

export interface InputConfig {
  defaultSchema: string
  englishModeByDefault: boolean
  candidatePageSize: number
}

export interface AppearanceConfig {
  theme: string
  fontSize: number
  candidateLayout: string
}

export interface LoggingConfig {
  level: string
  redactInputContent: boolean
}

export interface AppConfig {
  general: GeneralConfig
  input: InputConfig
  appearance: AppearanceConfig
  logging: LoggingConfig
}

export interface RuntimeStatus {
  serviceStatus: string
  activePlatform: string
  defaultSchema: string
}

export interface MetricItem {
  label: string
  value: string
  hint: string
}

export interface DashboardSnapshot {
  metrics: MetricItem[]
}

export type TypingKeyKind =
  | 'char'
  | 'backspace'
  | 'enter'
  | 'space'
  | 'escape'
  | 'number'
  | 'toggleInputMode'

export type InputModeValue = 'Chinese' | 'English'

export interface TypingKeyInput {
  kind: TypingKeyKind
  char?: string
  number?: number
}

export interface TypingCandidate {
  id: string
  text: string
  annotation: string | null
  hotkey: string | null
}

export interface TypingCandidatePage {
  items: TypingCandidate[]
  pageIndex: number
  hasNextPage: boolean
}

export interface TypingPreeditState {
  compositionText: string
  cursor: number
}

export interface TypingResponse {
  consumed: boolean
  commitText: string | null
  preedit: TypingPreeditState
  candidates: TypingCandidatePage
  inputMode: InputModeValue
}

export interface TypingSessionState {
  rawKeys: string
  compositionText: string
  selectedIndex: number
  candidates: TypingCandidate[]
  inputMode: InputModeValue
}

export interface TypingSnapshot {
  session: TypingSessionState
  response: TypingResponse
}

export interface UserDictionaryEntry {
  id: number
  schemaId: string
  code: string
  word: string
  weight: number
  source: string
  createdAt: number
  updatedAt: number
}

export interface NewUserDictionaryEntry {
  schemaId: string
  code: string
  word: string
  weight: number
  source: string
}

export interface InputHistoryEntry {
  id: number
  schemaId: string
  inputCode: string
  committedText: string
  usageCount: number
  lastUsedAt: number
  createdAt: number
}

export interface HotkeyEntry {
  id: string
  action: string
  accelerator: string
  scope: string
  enabled: boolean
  updatedAt: number
}

export interface ErrorLogEntry {
  id: number
  level: string
  module: string
  message: string
  contextJson: string | null
  createdAt: number
}

export const defaultConfig: AppConfig = {
  general: {
    startupWithSystem: false,
    locale: 'zh-CN'
  },
  input: {
    defaultSchema: 'pinyin',
    englishModeByDefault: false,
    candidatePageSize: 9
  },
  appearance: {
    theme: 'system',
    fontSize: 16,
    candidateLayout: 'vertical'
  },
  logging: {
    level: 'info',
    redactInputContent: true
  }
}

export const sectionMeta: NavSection[] = [
  { id: 'overview', label: '总览', description: '运行状态与关键指标' },
  { id: 'typing', label: '输入测试', description: '直接打字验证 Rust 输入内核闭环' },
  { id: 'input', label: '输入设置', description: '默认方案与输入行为配置' },
  { id: 'candidates', label: '候选设置', description: '候选分页与候选展示行为' },
  { id: 'appearance', label: '外观设置', description: '主题、字号与界面外观' },
  { id: 'dictionary', label: '用户词典', description: '管理用户词条与短语数据' },
  { id: 'history', label: '输入历史', description: '查看输入记录与学习数据' },
  { id: 'hotkeys', label: '快捷键', description: '管理快捷键绑定与冲突检查' },
  { id: 'logs', label: '日志中心', description: '查看运行日志与诊断信息' },
  { id: 'about', label: '关于与调试', description: '产品信息与调试入口' }
]
