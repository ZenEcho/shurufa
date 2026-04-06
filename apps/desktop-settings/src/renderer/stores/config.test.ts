import { createPinia, setActivePinia } from 'pinia'
import { beforeEach, describe, expect, it, vi } from 'vitest'
import { configFixture, createImeApiMock, runtimeStatusFixture, updatedConfigFixture } from '../../test/ime-api'
import { useConfigStore } from './config'

describe('useConfigStore', () => {
  beforeEach(() => {
    setActivePinia(createPinia())
  })

  it('loads persisted config into both persisted and draft state', async () => {
    createImeApiMock()
    const store = useConfigStore()

    await store.load()

    expect(store.persistedConfig).toEqual(configFixture)
    expect(store.draftConfig).toEqual(configFixture)
    expect(store.loading).toBe(false)
    expect(store.error).toBeNull()
    expect(store.isDirty).toBe(false)
  })

  it('patches input settings and saves the draft back to the runtime service', async () => {
    const api = createImeApiMock({
      getConfig: vi.fn(async () => configFixture),
      updateConfig: vi.fn(async () => runtimeStatusFixture)
    })
    const store = useConfigStore()

    await store.load()
    store.patchInputConfig(updatedConfigFixture.input)

    expect(store.inputConfig).toEqual(updatedConfigFixture.input)
    expect(store.isDirty).toBe(true)

    await expect(store.save()).resolves.toEqual(runtimeStatusFixture)

    expect(api.updateConfig).toHaveBeenCalledWith(updatedConfigFixture)
    expect(store.persistedConfig).toEqual(updatedConfigFixture)
    expect(store.isDirty).toBe(false)
    expect(store.lastSavedAt).not.toBeNull()
  })

  it('resets config state from the defaults endpoint', async () => {
    const api = createImeApiMock({
      resetConfig: vi.fn(async () => updatedConfigFixture)
    })
    const store = useConfigStore()

    await store.resetToDefaults()

    expect(api.resetConfig).toHaveBeenCalledOnce()
    expect(store.persistedConfig).toEqual(updatedConfigFixture)
    expect(store.draftConfig).toEqual(updatedConfigFixture)
    expect(store.saving).toBe(false)
  })

  it('stores the error message when loading fails', async () => {
    createImeApiMock({
      getConfig: vi.fn(async () => {
        throw new Error('config unavailable')
      })
    })
    const store = useConfigStore()

    await expect(store.load()).rejects.toThrow('config unavailable')

    expect(store.error).toBe('config unavailable')
    expect(store.loading).toBe(false)
  })
})
