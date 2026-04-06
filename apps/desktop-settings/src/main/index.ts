import { app, BrowserWindow, ipcMain } from 'electron'
import { join } from 'node:path'
import { is } from '@electron-toolkit/utils'
import { dashboardSnapshot, runtimeStatus } from '@shurufa/shared-types'

const createWindow = async (): Promise<void> => {
  const window = new BrowserWindow({
    width: 1360,
    height: 900,
    minWidth: 1120,
    minHeight: 760,
    backgroundColor: '#f8fbff',
    titleBarStyle: 'hiddenInset',
    webPreferences: {
      preload: join(__dirname, '../preload/index.js'),
      contextIsolation: true,
      nodeIntegration: false
    }
  })

  ipcMain.handle('ime:get-dashboard', async () => dashboardSnapshot)
  ipcMain.handle('ime:get-runtime-status', async () => runtimeStatus)

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

