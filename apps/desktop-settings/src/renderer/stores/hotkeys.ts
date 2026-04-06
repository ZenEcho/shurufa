import { ref, shallowRef } from 'vue'
import { defineStore } from 'pinia'
import type { HotkeyEntry } from '@shurufa/shared-types'

export const useHotkeysStore = defineStore('hotkeys', () => {
  const entries = ref<HotkeyEntry[]>([])
  const loading = shallowRef(false)
  const saving = shallowRef(false)
  const error = shallowRef<string | null>(null)

  const load = async (): Promise<void> => {
    loading.value = true
    error.value = null

    try {
      entries.value = await window.imeApi.listHotkeys()
    } catch (loadError) {
      error.value = loadError instanceof Error ? loadError.message : '加载快捷键失败'
      throw loadError
    } finally {
      loading.value = false
    }
  }

  const saveEntry = async (entry: HotkeyEntry): Promise<void> => {
    saving.value = true
    error.value = null

    try {
      entries.value = await window.imeApi.saveHotkey(entry)
    } catch (saveError) {
      error.value = saveError instanceof Error ? saveError.message : '保存快捷键失败'
      throw saveError
    } finally {
      saving.value = false
    }
  }

  const deleteEntry = async (id: string): Promise<void> => {
    saving.value = true
    error.value = null

    try {
      entries.value = await window.imeApi.deleteHotkey(id)
    } catch (deleteError) {
      error.value = deleteError instanceof Error ? deleteError.message : '删除快捷键失败'
      throw deleteError
    } finally {
      saving.value = false
    }
  }

  return {
    entries,
    loading,
    saving,
    error,
    load,
    saveEntry,
    deleteEntry
  }
})
