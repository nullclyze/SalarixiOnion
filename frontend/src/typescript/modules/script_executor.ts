import { invoke } from '@tauri-apps/api/core';

import { logger } from '../utils/logger';
import { setQuickTasksAllowed } from '../main';

class ScriptExecutor {
	private editor: HTMLTextAreaElement | null;
	private lineCounter: HTMLDivElement | null; 
	
	constructor() {
		this.editor = null;
		this.lineCounter = null;
	}

	/** Метод инициализации функций, связанных с скриптингом. */
	public init(): void {
		this.editor = document.getElementById('user-script') as HTMLTextAreaElement;
		this.lineCounter = document.getElementById('line-counter') as HTMLDivElement;

		if (!this.editor) return;

		this.editor.addEventListener('keydown', (e) => {
			if (e.key === 'Tab' && this.editor) {
				e.preventDefault();
				const start = this.editor.selectionStart;
				const end = this.editor.selectionEnd;
				const value = this.editor.value;
				this.editor.value = value.substring(0, start) + '  ' + value.substring(end);
				this.editor.selectionStart = this.editor.selectionEnd = start + 2;
				this.updateLineCounter();
			}
		});

		this.editor.addEventListener('mouseenter', () => setQuickTasksAllowed(false));
		this.editor.addEventListener('mouseleave', () => setQuickTasksAllowed(true));
		this.editor.addEventListener('input', () => this.updateLineCounter());
		this.editor.addEventListener('scroll', () => this.lineCounter && this.editor ? this.lineCounter.scrollTop = this.editor.scrollTop : null);

		document.getElementById('execute-script')?.addEventListener('click', async () => await this.execute());
  	document.getElementById('stop-script')?.addEventListener('click', async () => await this.stop());

		this.updateLineCounter();
	}

	/** Метод обновления счётчика строк. */
	private updateLineCounter(): void {
		if (!this.editor || !this.lineCounter) return;
		const lines = this.editor.value.split('\n').length;
		let numbers = '';
		for (let i = 1; i <= lines; i++) numbers += `<p>${i}</p>\n`;
		this.lineCounter.innerHTML = numbers;
	}

	/** Метод исполнения пользовательского сценария. */
	private async execute(): Promise<void> {
		try {
			if (!this.editor) return;
			const script = this.editor.value;
			if (script === '') return;
			await invoke('execute_script', { script: script });
		} catch (error) {
			logger.log(`Ошибка выполнения скрипта: ${error}`, 'error');
		}
	}

	/** Метод остановки пользовательского сценария. */
  private async stop(): Promise<void> {
		try {
			await invoke('stop_script');
		} catch (error) {
			logger.log(`Ошибка остановки скрипта: ${error}`, 'error');
		}
	}
}

const scriptExecutor = new ScriptExecutor();

export { scriptExecutor }