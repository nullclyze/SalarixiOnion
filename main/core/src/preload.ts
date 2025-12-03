import { contextBridge, ipcRenderer } from 'electron';

contextBridge.exposeInMainWorld('client', {
  port: () => ipcRenderer.invoke('client', 'port').then(answer => answer.result),
  info: () => ipcRenderer.invoke('client', 'get-info').then(answer => answer.result),
  window: (action: string) => ipcRenderer.invoke('client', 'window', { action: action }).then(answer => answer.result)
});
