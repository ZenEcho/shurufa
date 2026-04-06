import { computed, ref, shallowRef } from 'vue'
import { defineStore } from 'pinia'
import type { AppConfig, InputConfig } from '@shurufa/shared-types'

export const useConfigStore = defineStore('config', () => {
  const persistedConfig = ref<AppConfig | null>(null)
  const draftConfig = ref<AppConfig | null>(null)
  const loading = shallowRef(false)
  const saving = shallowRef(false)
  const error = shallowRef<string | null>(null)
  const lastSavedAt = shallowRef<string | null>(null)

  const isDirty = computed(() =>
    JSON.stringify(persistedConfig.value) !== JSON.stringify(draftConfig.value)
  )

  const inputConfig = computed(() => draftConfig.value?.input ?? null)

  const cloneConfig = (config: AppConfig): AppConfig => JSON.parse(JSON.stringify(config)) as AppConfig

  const load = async (): Promise<void> => {
    loading.value = true
    error.value = null

    try {
      const config = await window.imeApi.getConfig()
      persistedConfig.value = config
      draftConfig.value = cloneConfig(config)
    } catch (loadError) {
      error.value = loadError instanceof Error ? loadError.message : '加载配置失败'
      throw loadError
    } finally {
      loading.value = false
    }
  }

  const patchInputConfig = (patch: Partial<InputConfig>): void => {
    if (!draftConfig.value) {
      return
    }

    draftConfig.value = {
      ...draftConfig.value,
      input: {
        ...draftConfig.value.input,
        ...patch
      }
    }
  }

  const save = async () => {
    if (!draftConfig.value) {
      return null
    }

    const nextConfig = cloneConfig(draftConfig.value)

    saving.value = true
    error.value = null

    try {
      const runtime = await window.imeApi.updateConfig(nextConfig)
      persistedConfig.value = nextConfig
      lastSavedAt.value = new Date().toISOString()
      return runtime
    } catch (saveError) {
      error.value = saveError instanceof Error ? saveError.message : '保存配置失败'
      throw saveError
    } finally {
      saving.value = false
    }
  }

  const resetToDefaults = async (): Promise<void> => {
    saving.value = true
    error.value = null

    try {
      const config = await window.imeApi.resetConfig()
      persistedConfig.value = config
      draftConfig.value = cloneConfig(config)
      lastSavedAt.value = new Date().toISOString()
    } catch (resetError) {
      error.value = resetError instanceof Error ? resetError.message : '重置配置失败'
      throw resetError
    } finally {
      saving.value = false
    }
  }

  return {
    persistedConfig,
    draftConfig,
    inputConfig,
    loading,
    saving,
    error,
    isDirty,
    lastSavedAt,
    load,
    save,
    resetToDefaults,
    patchInputConfig
  }
})
