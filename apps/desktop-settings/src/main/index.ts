import { app, BrowserWindow, ipcMain } from 'electron'
import { join } from 'node:path'
import { is } from '@electron-toolkit/utils'
import {
  createUserDictionaryEntry,
  createTypingSession,
  deleteHotkey,
  deleteUserDictionaryEntry,
  getConfig,
  getDashboard,
  getRuntimeStatus,
  listErrorLogs,
  listHistory,
  listHotkeys,
  listUserDictionary,
  processTypingKey,
  resetConfig,
  saveHotkey,
  updateConfig
} from './service-client'

const createWindow = async (): Promise<void> => {
  const window = new BrowserWindow({
    width: 1360,
    height: 900,
    minWidth: 1120,
    minHeight: 760,
    backgroundColor: '#f8fbff',
    title: '书入法设置中心',
    titleBarStyle: 'hiddenInset',
    webPreferences: {
      preload: join(__dirname, '../preload/index.mjs'),
      sandbox: false,
      contextIsolation: true,
      nodeIntegration: false
    }
  })

  ipcMain.handle('ime:get-dashboard', async () => getDashboard())
  ipcMain.handle('ime:get-runtime-status', async () => getRuntimeStatus())
  ipcMain.handle('ime:get-config', async () => getConfig())
  ipcMain.handle('ime:create-typing-session', async () => createTypingSession())
  ipcMain.handle('ime:process-typing-key', async (_event, session, key) => processTypingKey(session, key))
  ipcMain.handle('ime:update-config', async (_event, config) => updateConfig(config))
  ipcMain.handle('ime:reset-config', async () => resetConfig())
  ipcMain.handle('ime:list-user-dictionary', async () => listUserDictionary())
  ipcMain.handle('ime:create-user-dictionary-entry', async (_event, entry) => createUserDictionaryEntry(entry))
  ipcMain.handle('ime:delete-user-dictionary-entry', async (_event, id) => deleteUserDictionaryEntry(id))
  ipcMain.handle('ime:list-history', async () => listHistory())
  ipcMain.handle('ime:list-hotkeys', async () => listHotkeys())
  ipcMain.handle('ime:save-hotkey', async (_event, entry) => saveHotkey(entry))
  ipcMain.handle('ime:delete-hotkey', async (_event, id) => deleteHotkey(id))
  ipcMain.handle('ime:list-error-logs', async () => listErrorLogs())

  if (is.dev) {
    await window.loadURL('http://localhost:5173')
    window.webContents.openDevTools({ mode: 'detach' })
    return
  }

  await window.loadFile(join(__dirname, '../renderer/index.html'))
}

app.whenReady().then(async () => {
  await createWindow()

  app.on('activate', async () => {
    if (BrowserWindow.getAllWindows().length === 0) {
      await createWindow()
    }
  })
})

app.on('window-all-closed', () => {
  if (process.platform !== 'darwin') {
    app.quit()
  }
})
