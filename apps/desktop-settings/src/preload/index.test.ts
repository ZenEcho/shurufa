import type { AppConfig, HotkeyEntry, TypingKeyInput, TypingSessionState } from '@shurufa/shared-types'
import { beforeEach, describe, expect, it, vi } from 'vitest'

const exposeInMainWorld = vi.hoisted(() => vi.fn())
const invoke = vi.hoisted(() => vi.fn())

vi.mock('electron', () => ({
  contextBridge: {
    exposeInMainWorld
  },
  ipcRenderer: {
    invoke
  }
}))

describe('preload ime api', () => {
  beforeEach(() => {
    exposeInMainWorld.mockReset()
    invoke.mockReset()
    vi.resetModules()
  })

  it('exposes an API that forwards every call to the expected ipc channel', async () => {
    await import('./index')

    const api = exposeInMainWorld.mock.calls[0]?.[1] as Window['imeApi']
    const config = {} as AppConfig
    const session = {} as TypingSessionState
    const key = { kind: 'char', char: 'a' } as TypingKeyInput
    const hotkey = {} as HotkeyEntry

    expect(exposeInMainWorld).toHaveBeenCalledWith('imeApi', expect.any(Object))

    await api.getDashboard()
    await api.getRuntimeStatus()
    await api.getConfig()
    await api.createTypingSession()
    await api.processTypingKey(session, key)
    await api.updateConfig(config)
    await api.resetConfig()
    await api.listUserDictionary()
    await api.createUserDictionaryEntry({
      schemaId: 'pinyin',
      code: 'nihao',
      word: '你好',
      weight: 100,
      source: 'manual'
    })
    await api.deleteUserDictionaryEntry(1)
    await api.listHistory()
    await api.listHotkeys()
    await api.saveHotkey(hotkey)
    await api.deleteHotkey('toggle')
    await api.listErrorLogs()

    expect(invoke.mock.calls).toEqual([
      ['ime:get-dashboard'],
      ['ime:get-runtime-status'],
      ['ime:get-config'],
      ['ime:create-typing-session'],
      ['ime:process-typing-key', session, key],
      ['ime:update-config', config],
      ['ime:reset-config'],
      ['ime:list-user-dictionary'],
      ['ime:create-user-dictionary-entry', {
        schemaId: 'pinyin',
        code: 'nihao',
        word: '你好',
        weight: 100,
        source: 'manual'
      }],
      ['ime:delete-user-dictionary-entry', 1],
      ['ime:list-history'],
      ['ime:list-hotkeys'],
      ['ime:save-hotkey', hotkey],
      ['ime:delete-hotkey', 'toggle'],
      ['ime:list-error-logs']
    ])
  })
})
