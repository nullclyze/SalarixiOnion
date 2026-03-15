import { readFile, writeFile } from '@tauri-apps/plugin-fs';
import { path } from '@tauri-apps/api';

import { plugins } from '../common/structs';
import { log } from '../logger';
import { invokeElementFunction, updatePluginState } from '../main';
import { message } from '../message';
import { open } from '@tauri-apps/plugin-dialog';

type ConfigValue = string | number | boolean | null;

function setValue(id: string, value: ConfigValue): void {
  if (id === '') return;

  try {
    const doc = document.getElementById(id.replaceAll('.', '_'));

    if (!doc) return;

    if (doc.tagName.toLocaleLowerCase() === 'input') {
      const input = doc as HTMLInputElement;

      if (input.type === 'checkbox') {
        input.checked = Boolean(value);
      } else {
        if (typeof value === 'number') {
          input.valueAsNumber = value;
        } else {
          input.value = value ? value.toString() : '';
        }
      }
    } else if (doc.tagName.toLocaleLowerCase() === 'textarea') {
      const textarea = doc as HTMLTextAreaElement;
      textarea.value = value ? value.toString() : '';
    } else {
      const select = doc as HTMLSelectElement;

      if (typeof value === 'number') {
        select.selectedIndex = value;
      }
    }
  } catch (error) {
    log(`Ошибка установки значения для ${id}: ${error}`, 'error');
  }
}

export function initConfig(): void {
  const current_config = localStorage.getItem('salarixionion:storage:config');

  if (current_config) {
    log('Загрузка конфига...', 'system');

    for (const [id, value] of Object.entries<ConfigValue>(JSON.parse(current_config))) {
      setValue(id, value);
    }

    for (const name in plugins) {
      const state = localStorage.getItem(`plugin-state:${name}`) === 'true';
      updatePluginState(name, state);
    }

    document.querySelectorAll<HTMLElement>('[trigger]').forEach(e => invokeElementFunction(e.id));

    log('Конфиг успешно загружен', 'system');
  } else {
    localStorage.setItem('salarixionion:storage:config', JSON.stringify({}, null, 2));
  }

  document.getElementById('upload-config')?.addEventListener('click', async () => await uploadConfig());
  document.getElementById('share-config')?.addEventListener('click', async () => await shareConfig());

  setInterval(() => {
    const elements = document.querySelectorAll<HTMLElement>('[keep="true"]');
    const config: Record<string, ConfigValue> = {};

    for (const element of elements) {
      const id = element.id.replaceAll('_', '.');

      if (element.tagName.toLocaleLowerCase() === 'input') {
        const el = element as HTMLInputElement;
        if (el.type === 'checkbox') {
          config[id] = el.checked;
        } else {
          config[id] = el.type === 'number' ? el.value.includes('.') || el.value.includes(',') ? parseFloat(el.value) : parseInt(el.value) : el.value;
        }
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

async function uploadConfig(): Promise<void> {
  try {
    const path = await open({
      directory: false,
      multiple: false,
      filters: [
        {
          name: 'Config',
          extensions: ['json']
        }
      ]
    });

    if (!path) return;

    const buffer = await readFile(path);

    if (buffer) return;

    const decoder = new TextDecoder();
    const config = JSON.parse(decoder.decode(buffer));

    for (const [id, value] of Object.entries<ConfigValue>(config)) {
      if (!value && value !== 0) continue;
      setValue(id, value);
    }

    document.querySelectorAll<HTMLElement>('[trigger]').forEach(e => invokeElementFunction(e.id));

    message('Конфиг', `Конфиг успешно загружен из ${path}`);
  } catch (error) {
    log(`Не удалось загрузить конфиг: ${error}`, 'error');
  }
}

async function shareConfig(): Promise<void> {
  try {
    const directory = await open({
      directory: true,
      multiple: false
    });

    if (!directory) return;

    const config = localStorage.getItem('salarixionion:storage:config');

    if (!config) return;

    const clean_config: Record<string, ConfigValue> = {};

    for (const [id, value] of Object.entries<ConfigValue>(JSON.parse(config))) {
      const el = document.getElementById(id.replaceAll('.', '_'));

      if (el && !el.getAttribute('ignore-config')) {
        clean_config[id] = value;
      }
    }

    let encoder = new TextEncoder();
    let buffer = encoder.encode(JSON.stringify(clean_config, null, 2));

    await writeFile(await path.join(directory, 'salarixi.config.json'), buffer);

    message('Конфиг', `Публичный конфиг успешно создан в ${directory}`);
  } catch (error) {
    log(`Не удалось поделиться конфигом: ${error}`, 'error');
  }
}