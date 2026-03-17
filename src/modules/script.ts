import { invoke } from '@tauri-apps/api/core';

import { logger } from '../utils/logger';
import { setQuickTasksAllowed } from '../main';

export class ScriptManager {
	public init(): void {
		const scriptEditor = document.getElementById('user-script') as HTMLTextAreaElement;
		const lineCounter = document.getElementById('line-counter') as HTMLDivElement;

		if (scriptEditor && lineCounter) {
			scriptEditor.addEventListener('keydown', (e) => {
				if (e.key === 'Tab') {
					e.preventDefault();
					const start = scriptEditor.selectionStart;
					const end = scriptEditor.selectionEnd;
					const value = scriptEditor.value;
					scriptEditor.value = value.substring(0, start) + '  ' + value.substring(end);
					scriptEditor.selectionStart = scriptEditor.selectionEnd = start + 2;
					updateLineCounter();
				}
			});

			scriptEditor.addEventListener('mouseenter', () => {
				setQuickTasksAllowed(false);
			});

			scriptEditor.addEventListener('mouseleave', () => {
				setQuickTasksAllowed(true);
			});

			scriptEditor.addEventListener('input', () => {
				updateLineCounter();
			});

			scriptEditor.addEventListener('scroll', () => {
				lineCounter.scrollTop = scriptEditor.scrollTop;
			});

			updateLineCounter();
		}

		function updateLineCounter() {
			const scriptEditor = document.getElementById('user-script') as HTMLTextAreaElement;
			const lineCounter = document.getElementById('line-counter') as HTMLElement;
			
			if (scriptEditor && lineCounter) {
				const lines = scriptEditor.value.split('\n').length;
				let numbers = '';

				for (let i = 1; i <= lines; i++) {
					numbers += `<p>${i}</p>\n`;
				}

				lineCounter.innerHTML = numbers;
			}
		}
	}

	public async execute(): Promise<void> {
		try {
			const script = (document.getElementById('user-script') as HTMLTextAreaElement).value;

			if (script === '') return;

			await invoke('execute_script', {
				script: script
			});
		} catch (error) {
			logger.log(`Ошибка выполнения скрипта: ${error}`, 'error');
		}
	}

  public async stop(): Promise<void> {
		try {
			await invoke('stop_script');
		} catch (error) {
			logger.log(`Ошибка остановки скрипта: ${error}`, 'error');
		}
	}
}