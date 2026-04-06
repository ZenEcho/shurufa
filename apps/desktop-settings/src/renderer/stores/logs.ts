import { ref, shallowRef } from 'vue'
import { defineStore } from 'pinia'
import type { ErrorLogEntry } from '@shurufa/shared-types'

export const useLogsStore = defineStore('logs', () => {
  const entries = ref<ErrorLogEntry[]>([])
  const loading = shallowRef(false)
  const error = shallowRef<string | null>(null)

  const load = async (): Promise<void> => {
    loading.value = true
    error.value = null

    try {
      entries.value = await window.imeApi.listErrorLogs()
    } catch (loadError) {
      error.value = loadError instanceof Error ? loadError.message : '加载日志失败'
      throw loadError
    } finally {
      loading.value = false
    }
  }

  return {
    entries,
    loading,
    error,
    load
  }
})
