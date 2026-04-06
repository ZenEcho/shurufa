import { ref, shallowRef } from 'vue'
import { defineStore } from 'pinia'
import type { InputHistoryEntry } from '@shurufa/shared-types'

export const useHistoryStore = defineStore('history', () => {
  const entries = ref<InputHistoryEntry[]>([])
  const loading = shallowRef(false)
  const error = shallowRef<string | null>(null)

  const load = async (): Promise<void> => {
    loading.value = true
    error.value = null

    try {
      entries.value = await window.imeApi.listHistory()
    } catch (loadError) {
      error.value = loadError instanceof Error ? loadError.message : '加载输入历史失败'
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
