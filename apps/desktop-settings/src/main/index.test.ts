/**
 * @vitest-environment node
 */

import { beforeEach, describe, expect, it, vi } from 'vitest'

const browserWindowCtor = vi.hoisted(() => vi.fn())
const loadFile = vi.hoisted(() => vi.fn(async () => undefined))
const loadURL = vi.hoisted(() => vi.fn(async () => undefined))
const openDevTools = vi.hoisted(() => vi.fn())
const ipcHandle = vi.hoisted(() => vi.fn())
const whenReady = vi.hoisted(() => vi.fn(() => Promise.resolve()))
const appOn = vi.hoisted(() => vi.fn())
const quit = vi.hoisted(() => vi.fn())
const getAllWindows = vi.hoisted(() => vi.fn(() => []))

vi.mock('electron', () => ({
  app: {
    whenReady,
    on: appOn,
    quit
  },
  BrowserWindow: Object.assign(
    class BrowserWindowMock {
      webContents = {
        openDevTools
      }

      constructor(options: unknown) {
        browserWindowCtor(options)
      }

      loadFile = loadFile
      loadURL = loadURL
    },
    {
      getAllWindows
    }
  ),
  ipcMain: {
    handle: ipcHandle
  }
}))

vi.mock('@electron-toolkit/utils', () => ({
  is: {
    dev: false
  }
}))

vi.mock('./service-client', () => ({
  getDashboard: vi.fn(),
  getRuntimeStatus: vi.fn(),
  getConfig: vi.fn(),
  createTypingSession: vi.fn(),
  processTypingKey: vi.fn(),
  updateConfig: vi.fn(),
  resetConfig: vi.fn(),
  listUserDictionary: vi.fn(),
  createUserDictionaryEntry: vi.fn(),
  deleteUserDictionaryEntry: vi.fn(),
  listHistory: vi.fn(),
  listHotkeys: vi.fn(),
  saveHotkey: vi.fn(),
  deleteHotkey: vi.fn(),
  listErrorLogs: vi.fn()
}))

describe('main process bootstrap', () => {
  beforeEach(() => {
    browserWindowCtor.mockReset()
    loadFile.mockReset()
    loadURL.mockReset()
    openDevTools.mockReset()
    ipcHandle.mockReset()
    whenReady.mockClear()
    appOn.mockReset()
    quit.mockReset()
    getAllWindows.mockReset()
    getAllWindows.mockReturnValue([])
    vi.resetModules()
  })

  it('points BrowserWindow preload to the built preload module path', async () => {
    await import('./index')
    await Promise.resolve()
    await Promise.resolve()

    const options = browserWindowCtor.mock.calls[0]?.[0] as {
      webPreferences: { preload: string; sandbox?: boolean }
    }

    expect(options.webPreferences.preload).toMatch(/[\\/]+preload[\\/]+index\.mjs$/)
    expect(options.webPreferences.sandbox).toBe(false)
  })
})
