import { app, BrowserWindow, ipcMain, dialog, shell } from 'electron';
import { dirname } from 'path';
import { fileURLToPath } from 'url';
import path from 'path';

import { setup } from './api/setup.js';
    

const __dirname = dirname(fileURLToPath(import.meta.url));

let port = setup();

app.commandLine.appendSwitch('no-sandbox');

const client = {
  version: '1.0.2',
  type: 'Beta',
  releaseDate: '13.12.2025'
};

let win: BrowserWindow;

function createWindow() {
  win = new BrowserWindow({
    title: 'Salarixi Onion',
    width: 1060,
    height: 680,
    titleBarStyle: 'hidden',
    resizable: false,
    fullscreen: false,
    fullscreenable: false,
    center: true,
    webPreferences: {
      preload: path.join(__dirname, 'preload.js'),
      contextIsolation: true,
      nodeIntegration: false,
      sandbox: false,
      devTools: false,
      spellcheck: false
    }
  });

  win.setMenuBarVisibility(false);

  win.loadFile(path.join(__dirname, 'interface', 'index.html'));
}

app.whenReady().then(() => {
  createWindow();
});

ipcMain.handle('client', async (_, cmd, options) => {
  try {
    let result;

    switch (cmd) {
      case 'port':
        result = port; break;
      case 'get-info':
        result = client; break;
      case 'window':
        if (options.action === 'minimize') {
          win.minimize()
        } else if (options.action === 'close') {
          win.close();
          app.quit();
        }; break;
      case 'open-file':
        await dialog.showOpenDialog(win, { properties: ['openFile'] }).then(r => {
          result = r.filePaths[0];
        }); break;
      case 'open-url':
        result = await shell.openExternal(options.url); break;
    }

    return { success: true, result: result };
  } catch (error) {
    return { success: false, error: error };
  }
});