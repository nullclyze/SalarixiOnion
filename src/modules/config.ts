import { plugins } from '../common/structs';
import { log } from '../logger';
import { updatePluginState } from '../main';


interface ConfigElement {
  id: string;
  value: string | number | boolean | null;
}

export function initConfig(): void {
  if (!localStorage.getItem('salarixionion:config')) {
    localStorage.setItem('salarixionion:config', JSON.stringify({}, null, 2));
  }

  let latest: any = null;

  setInterval(async () => {
    const elements = document.querySelectorAll('[keep="true"]');
    const config: Record<string, ConfigElement> = {};

    for (const element of elements) {
      const el = element as HTMLInputElement;
      if (el.type === 'checkbox') {
        config[el.id] = {
          id: el.id,
          value: el.checked
        };
      } else {
        config[el.id] = {
          id: el.id,
          value: el.type === 'number' ? parseInt(el.value) : el.value
        };
      }
    }

    if (JSON.stringify(config) === JSON.stringify(latest)) return;

    localStorage.setItem('salarixionion:config', JSON.stringify(config, null, 2))

    latest = config;
  }, 1500);
}

export function loadConfig(): void {
  log('Загрузка конфига...', 'system');

  try {
    let config = localStorage.getItem('salarixionion:config');

    if (!config) {
      log('Ошибка загрузки конфига: Config not found', 'error');
      return;
    }

    for (const [id, el] of Object.entries<ConfigElement>(JSON.parse(config))) {
      const doc = document.getElementById(id) as HTMLInputElement;

      if (doc) {
        if (doc.type === 'checkbox') {
          doc.checked = Boolean(el.value);
        } else {
          if (typeof el.value === 'number') {
            doc.valueAsNumber = el.value ? el.value : 0;
          } else {
            doc.value = el.value ? el.value.toString() : '';
          }
        }
      } 
    }

    for (const name in plugins) {
      const state = localStorage.getItem(`plugin-state:${name}`) === 'true';
      updatePluginState(name, state);
    }

    log('Конфиг успешно загружен', 'system');
  } catch (error) {
    log(`Ошибка загрузки конфига: ${error}`, 'error');
  }
}