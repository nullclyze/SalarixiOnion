import { log } from '../logger';


export function initConfig(): void {
  if (!localStorage.getItem('salarixionion:config')) {
    localStorage.setItem('salarixionion:config', JSON.stringify({}, null, 2));
  }

  let latest: any = null;

  setInterval(async () => {
    const elements = document.querySelectorAll('[keep="true"]');

    const config: Record<string, any> = {};

    for (const element of elements) {
      const el = element as HTMLInputElement;
      if (el.type === 'checkbox') {
        config[el.name] = {
          id: el.id,
          value: el.checked
        };
      } else {
        config[el.name] = {
          id: el.id,
          value: el.type === 'number' ? parseInt(el.value) : el.value
        };
      }
    }

    if (JSON.stringify(config) === JSON.stringify(latest)) return;

    for (let attempts = 0; attempts < 4; attempts++) {
      localStorage.setItem('salarixionion:config', JSON.stringify(config, null, 2))
      break;
    }

    latest = config;
  }, 1500);
}

export function loadConfig(): void {
  log('Загрузка конфига...', 'system');

  try {
    let config = null;

    for (let attempts = 0; attempts < 8; attempts++) {
      const data = localStorage.getItem('salarixionion:config');

      if (!data) {
        continue;
      } else {
        config = JSON.parse(data);
        break;
      } 
    }

    if (!config) {
      log('Ошибка загрузки конфига: Файл конфигурации отсутствует или повреждён', 'error');
      return;
    }

    for (const [_, element] of Object.entries(config)) {
      if ((element as any).value) {
        const html = document.getElementById((element as any).id) as HTMLInputElement;

        if (html) {
          if (String((element as any).id).startsWith('use')) {
            html.checked = (element as any).value;
          } else {
            if (typeof (element as any).value === 'number') {
              html.valueAsNumber = (element as any).value ? (element as any).value : 0;
            } else {
              html.value = (element as any).value;
            }
          }
        } 
      }
    }

    log('Конфиг успешно загружен', 'system');
  } catch (error) {
    log(`Ошибка загрузки конфига: ${error}`, 'error');
  }
}