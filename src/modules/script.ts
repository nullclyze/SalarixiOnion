import { invoke } from '@tauri-apps/api/core';

import { log } from '../logger';


export class ScriptManager {
	public async execute() {
		try {
			const script = (document.getElementById('user-script') as HTMLTextAreaElement).value;

			if (script === '') return;

			await invoke('execute_script', {
				script: script
			});
		} catch (error) {
			log(`Ошибка выполнения скрипта: ${error}`, 'error');
		}
	}

  public async stop() {
		try {
			await invoke('stop_script');
		} catch (error) {
			log(`Ошибка остановки скрипта: ${error}`, 'error');
		}
	}
}