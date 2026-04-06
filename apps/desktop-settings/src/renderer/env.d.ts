import type { DashboardSnapshot, RuntimeStatus } from '@shurufa/shared-types'

declare global {
  interface Window {
    imeApi: {
      getDashboard: () => Promise<DashboardSnapshot>
      getRuntimeStatus: () => Promise<RuntimeStatus>
    }
  }
}

export {}

