import { app, BrowserWindow } from 'electron'

let window: BrowserWindow

app.on('ready', () => {
  window = new BrowserWindow({
    width: 1280,
    height: 720,
  })

  window.loadURL('https://google.com')
})
