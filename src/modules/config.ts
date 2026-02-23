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
    const elements = document.querySelectorAll<HTMLElement>('[keep="true"]');
    const config: Record<string, ConfigElement> = {};

    for (const element of elements) {
      if (element.tagName.toLocaleLowerCase() === 'input') {
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
      } else if (element.tagName.toLocaleLowerCase() === 'textarea') {
        const el = element as HTMLTextAreaElement;

        config[el.id] = {
          id: el.id,
          value: el.type === 'number' ? parseInt(el.value) : el.value
        };
      } else {
        const el = element as HTMLSelectElement;
        
        config[el.id] = {
          id: el.id,
          value: el.selectedIndex
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
      if (id === '') continue;

      const doc = document.getElementById(id) as HTMLElement;

      if (doc.tagName.toLocaleLowerCase() === 'input') {
        const input = doc as HTMLInputElement;

        if (input.type === 'checkbox') {
          input.checked = Boolean(el.value);
        } else {
          if (typeof el.value === 'number') {
            input.valueAsNumber = el.value;
          } else {
            input.value = el.value ? el.value.toString() : '';
          }
        }
      } else if (doc.tagName.toLocaleLowerCase() === 'textarea') {
        const textarea = doc as HTMLTextAreaElement;
        textarea.value = el.value ? el.value.toString() : '';
      } else {
        const select = doc as HTMLSelectElement;

        if (typeof el.value === 'number') {
          select.selectedIndex = el.value;
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