export type SettingsSectionId =
  | 'overview'
  | 'schemas'
  | 'candidates'
  | 'dictionary'
  | 'history'
  | 'hotkeys'
  | 'logs'

export interface NavSection {
  id: SettingsSectionId
  label: string
  description: string
}

export interface MetricItem {
  label: string
  value: string
  hint: string
}

export interface DashboardSnapshot {
  metrics: MetricItem[]
}

export interface RuntimeStatus {
  serviceStatus: string
  activePlatform: string
  defaultSchema: string
}

export const sectionMeta: NavSection[] = [
  { id: 'overview', label: '总览', description: '系统状态、核心指标与运行概览' },
  { id: 'schemas', label: '输入方案', description: '拼音、五笔与扩展输入方案管理' },
  { id: 'candidates', label: '候选面板', description: '候选样式、布局和上屏行为' },
  { id: 'dictionary', label: '词库管理', description: '用户词库、第三方词库与短语片段' },
  { id: 'history', label: '输入历史', description: '常用词、最近输入与学习结果' },
  { id: 'hotkeys', label: '快捷键', description: '切换键位、候选选择与冲突检查' },
  { id: 'logs', label: '日志调试', description: '错误日志、性能追踪与诊断导出' }
]

export const dashboardSnapshot: DashboardSnapshot = {
  metrics: [
    { label: '运行平台', value: 'Windows · macOS · Linux', hint: '三端使用原生宿主接入系统输入链路' },
    { label: '核心语言', value: 'Rust', hint: '状态机、候选引擎、词频学习和数据库访问' },
    { label: '本地存储', value: 'SQLite', hint: '配置、用户词库、历史和日志索引统一持久化' },
    { label: '设置中心', value: 'Electron + Vue 3', hint: '跨平台设置、词库和调试体验' }
  ]
}

export const runtimeStatus: RuntimeStatus = {
  serviceStatus: 'Scaffold Ready',
  activePlatform: 'desktop-settings',
  defaultSchema: 'pinyin'
}
