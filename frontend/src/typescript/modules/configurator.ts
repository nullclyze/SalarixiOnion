import { readFile, writeFile } from '@tauri-apps/plugin-fs';
import { path } from '@tauri-apps/api';

import { logger } from '../utils/logger';
import { messages } from '../utils/message';
import { open } from '@tauri-apps/plugin-dialog';
import { triggerRegistry } from './trigger_registry';

type ConfigValue = string | number | boolean | null;

class Configurator {
  /** Метод инициализации конфигуратора. */
  public init(): void {
    const current_config = localStorage.getItem('salarixionion:storage:config');

    if (current_config) {
      logger.log('Загрузка конфига...', 'system');

      for (const [id, value] of Object.entries<ConfigValue>(JSON.parse(current_config))) this.setValue(id, value);

      triggerRegistry.triggerAll();

      logger.log('Конфиг успешно загружен', 'system');
    } else {
      localStorage.setItem('salarixionion:storage:config', JSON.stringify({}, null, 2));
    }

    document.getElementById('upload-config')?.addEventListener('click', async () => await this.uploadConfig());
    document.getElementById('share-config')?.addEventListener('click', async () => await this.shareConfig());

    setInterval(() => {
      const elements = document.querySelectorAll<HTMLElement>('[keep="true"]');
      const config: Record<string, ConfigValue> = {};

      for (const element of elements) {
        const id = element.id.replaceAll('_', '.');

        if (element.tagName.toLocaleLowerCase() === 'input') {
          const el = element as HTMLInputElement;
          el.type === 'checkbox' ? config[id] = el.checked : config[id] = el.type === 'number' ? el.value.includes('.') || el.value.includes(',') ? parseFloat(el.value) : parseInt(el.value) : el.value;
        } else if (element.tagName.toLocaleLowerCase() === 'textarea') {
          const el = element as HTMLTextAreaElement;
          config[id] = el.type === 'number' ? parseInt(el.value) : el.value;
        } else {
          const el = element as HTMLSelectElement;
          config[id] = el.selectedIndex;
        }
      }
      
      localStorage.setItem('salarixionion:storage:config', JSON.stringify(config, null, 2))
    }, 1500);
  }

  /** Метод установки значения для элемента. */
  private setValue(id: string, value: ConfigValue): void {
    if (id === '') return;

    try {
      const doc = document.getElementById(id.replaceAll('.', '_'));
      if (!doc) return;

      if (doc.tagName.toLocaleLowerCase() === 'input') {
        const input = doc as HTMLInputElement;
        input.type === 'checkbox' ? input.checked = Boolean(value) : typeof value === 'number' ? input.valueAsNumber = value : input.value = value ? value.toString() : '';
      } else if (doc.tagName.toLocaleLowerCase() === 'textarea') {
        const textarea = doc as HTMLTextAreaElement;
        textarea.value = value ? value.toString() : '';
      } else {
        const select = doc as HTMLSelectElement;
        if (typeof value === 'number') select.selectedIndex = value;
      }
    } catch (error) {
      logger.log(`Ошибка установки значения для ${id}: ${error}`, 'error');
    }
  }

  /** Метод загрузки стороннего конфига. */
  private async uploadConfig(): Promise<void> {
    try {
      const path = await open({
        directory: false,
        multiple: false,
        filters: [{
          name: 'Config',
          extensions: ['json']
        }]
      });

      if (!path) return;

      const buffer = await readFile(path);
      if (!buffer) return;

      const decoder = new TextDecoder();
      const config = JSON.parse(decoder.decode(buffer));

      for (const [id, value] of Object.entries<ConfigValue>(config)) value || value === 0 ? this.setValue(id, value) : null;

      triggerRegistry.triggerAll();

      messages.message('Конфиг', `Конфиг успешно загружен из ${path}`);
    } catch (error) {
      logger.log(`Не удалось загрузить конфиг: ${error}`, 'error');
    }
  }
  
  /** Метод создания публичного конфига. */
  private async shareConfig(): Promise<void> {
    try {
      const directory = await open({
        directory: true,
        multiple: false
      });

      if (!directory) return;

      const config = localStorage.getItem('salarixionion:storage:config');
      if (!config) return;

      const public_config: Record<string, ConfigValue> = {};

      for (const [id, value] of Object.entries<ConfigValue>(JSON.parse(config))) {
        const el = document.getElementById(id.replaceAll('.', '_'));
        if (!el) continue;
        el.getAttribute('public') === 'false' ? null : public_config[id] = value;
      }

      let encoder = new TextEncoder();
      let buffer = encoder.encode(JSON.stringify(public_config, null, 2));

      await writeFile(await path.join(directory, 'salarixi.config.json'), buffer);

      messages.message('Конфиг', `Публичный конфиг успешно создан в ${directory}`);
    } catch (error) {
      logger.log(`Не удалось поделиться конфигом: ${error}`, 'error');
    }
  }
}

const configurator = new Configurator();

export { configurator }