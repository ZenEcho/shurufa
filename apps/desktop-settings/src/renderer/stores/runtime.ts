import { ref, shallowRef } from 'vue'
import { defineStore } from 'pinia'
import type { DashboardSnapshot, RuntimeStatus } from '@shurufa/shared-types'

export const useRuntimeStore = defineStore('runtime', () => {
  const dashboard = ref<DashboardSnapshot | null>(null)
  const runtime = ref<RuntimeStatus | null>(null)
  const loading = shallowRef(false)
  const error = shallowRef<string | null>(null)

  const load = async (): Promise<void> => {
    loading.value = true
    error.value = null

    try {
      const [nextDashboard, nextRuntime] = await Promise.all([
        window.imeApi.getDashboard(),
        window.imeApi.getRuntimeStatus()
      ])

      dashboard.value = nextDashboard
      runtime.value = nextRuntime
    } catch (loadError) {
      error.value = loadError instanceof Error ? loadError.message : '加载运行时状态失败'
      throw loadError
    } finally {
      loading.value = false
    }
  }

  const setRuntime = (nextRuntime: RuntimeStatus): void => {
    runtime.value = nextRuntime
  }

  return {
    dashboard,
    runtime,
    loading,
    error,
    load,
    setRuntime
  }
})
