import { createPinia, setActivePinia } from 'pinia'
import { beforeEach, describe, expect, it, vi } from 'vitest'
import {
  createImeApiMock,
  dashboardFixture,
  dictionaryEntryFixture,
  errorLogFixture,
  historyEntryFixture,
  hotkeyEntryFixture,
  newDictionaryEntryFixture,
  runtimeStatusFixture
} from '../../test/ime-api'
import { useDictionaryStore } from './dictionary'
import { useHistoryStore } from './history'
import { useHotkeysStore } from './hotkeys'
import { useLogsStore } from './logs'
import { useRuntimeStore } from './runtime'

describe('resource stores', () => {
  beforeEach(() => {
    setActivePinia(createPinia())
  })

  it('loads the dashboard snapshot and runtime status together', async () => {
    const api = createImeApiMock()
    const store = useRuntimeStore()

    await store.load()

    expect(api.getDashboard).toHaveBeenCalledOnce()
    expect(api.getRuntimeStatus).toHaveBeenCalledOnce()
    expect(store.dashboard).toEqual(dashboardFixture)
    expect(store.runtime).toEqual(runtimeStatusFixture)
    expect(store.error).toBeNull()
  })

  it('updates dictionary entries on load, create, and delete', async () => {
    const api = createImeApiMock({
      createUserDictionaryEntry: vi.fn(async () => [dictionaryEntryFixture]),
      deleteUserDictionaryEntry: vi.fn(async () => [])
    })
    const store = useDictionaryStore()

    await store.load()
    expect(store.entries).toEqual([dictionaryEntryFixture])

    await store.createEntry(newDictionaryEntryFixture)
    expect(api.createUserDictionaryEntry).toHaveBeenCalledWith(newDictionaryEntryFixture)
    expect(store.entries).toEqual([dictionaryEntryFixture])

    await store.deleteEntry(dictionaryEntryFixture.id)
    expect(api.deleteUserDictionaryEntry).toHaveBeenCalledWith(dictionaryEntryFixture.id)
    expect(store.entries).toEqual([])
  })

  it('updates hotkey entries on load, save, and delete', async () => {
    const api = createImeApiMock({
      saveHotkey: vi.fn(async () => [hotkeyEntryFixture]),
      deleteHotkey: vi.fn(async () => [])
    })
    const store = useHotkeysStore()

    await store.load()
    expect(store.entries).toEqual([hotkeyEntryFixture])

    await store.saveEntry(hotkeyEntryFixture)
    expect(api.saveHotkey).toHaveBeenCalledWith(hotkeyEntryFixture)

    await store.deleteEntry(hotkeyEntryFixture.id)
    expect(api.deleteHotkey).toHaveBeenCalledWith(hotkeyEntryFixture.id)
    expect(store.entries).toEqual([])
  })

  it.each([
    ['history', useHistoryStore, 'listHistory', [historyEntryFixture]],
    ['logs', useLogsStore, 'listErrorLogs', [errorLogFixture]]
  ])('loads %s entries from imeApi', async (_label, useStore, methodName, expectedEntries) => {
    const api = createImeApiMock()
    const store = useStore()

    await store.load()

    expect(api[methodName as 'listHistory' | 'listErrorLogs']).toHaveBeenCalledOnce()
    expect(store.entries).toEqual(expectedEntries)
    expect(store.loading).toBe(false)
    expect(store.error).toBeNull()
  })

  it('surfaces runtime loading failures', async () => {
    createImeApiMock({
      getDashboard: vi.fn(async () => {
        throw new Error('service offline')
      })
    })
    const store = useRuntimeStore()

    await expect(store.load()).rejects.toThrow('service offline')

    expect(store.error).toBe('service offline')
    expect(store.loading).toBe(false)
  })
})
