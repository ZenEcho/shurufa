import { ref, shallowRef } from 'vue'
import { defineStore } from 'pinia'
import type { NewUserDictionaryEntry, UserDictionaryEntry } from '@shurufa/shared-types'

export const useDictionaryStore = defineStore('dictionary', () => {
  const entries = ref<UserDictionaryEntry[]>([])
  const loading = shallowRef(false)
  const saving = shallowRef(false)
  const error = shallowRef<string | null>(null)

  const load = async (): Promise<void> => {
    loading.value = true
    error.value = null

    try {
      entries.value = await window.imeApi.listUserDictionary()
    } catch (loadError) {
      error.value = loadError instanceof Error ? loadError.message : '加载用户词典失败'
      throw loadError
    } finally {
      loading.value = false
    }
  }

  const createEntry = async (entry: NewUserDictionaryEntry): Promise<void> => {
    saving.value = true
    error.value = null

    try {
      entries.value = await window.imeApi.createUserDictionaryEntry(entry)
    } catch (saveError) {
      error.value = saveError instanceof Error ? saveError.message : '创建用户词条失败'
      throw saveError
    } finally {
      saving.value = false
    }
  }

  const deleteEntry = async (id: number): Promise<void> => {
    saving.value = true
    error.value = null

    try {
      entries.value = await window.imeApi.deleteUserDictionaryEntry(id)
    } catch (deleteError) {
      error.value = deleteError instanceof Error ? deleteError.message : '删除用户词条失败'
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
    createEntry,
    deleteEntry
  }
})
