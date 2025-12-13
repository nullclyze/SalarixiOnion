import { contextBridge, ipcRenderer } from 'electron';

contextBridge.exposeInMainWorld('client', {
  port: () => ipcRenderer.invoke('client', 'port').then(answer => answer.result),
  getInfo: () => ipcRenderer.invoke('client', 'get-info').then(answer => answer.result),
  window: (action: string) => ipcRenderer.invoke('client', 'window', { action: action }).then(answer => answer.result),
  openFile: () => ipcRenderer.invoke('client', 'open-file').then(answer => answer.result),
  openUrl: (url: string) => ipcRenderer.invoke('client', 'open-url', { url: url }).then(answer => answer.result)
});
