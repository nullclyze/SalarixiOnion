import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { Chart, registerables } from 'chart.js';

import { plugins } from './common/structs';
import { logger } from './utils/logger';
import { configurator } from './modules/configurator';
import { accountManager } from './modules/account_manager';
import { proxyCollector } from './modules/proxy_collector';
import { graphic } from './modules/graphic';
import { scriptExecutor } from './modules/script_executor';
import { monitoring } from './modules/monitoring';
import { captchaBypass } from './modules/captcha_bypass';
import { radar } from './modules/radar';
import { pinger } from './modules/pinger';
import { translator } from './modules/translator';
import { messages } from './utils/message';
import { download } from './utils/downloader';
import { disableParticles, enableParticles } from './particles';
import { switchControlWrapper, switchGlobalWrapper } from './utils/switchers';
import { triggerRegistry } from './modules/trigger_registry';


Chart.register(...registerables);

const client = {
  version: '1.0.9'
};

export let process: 'active' | 'sleep' = 'sleep';
export let quickTasksAllowed: boolean = true;

const pressedKeys: { [x: string]: boolean } = {
  alt: false,
  shift: false,
  f: false,
  i: false,
  c: false,
  j: false,
  w: false,
  a: false,
  s: false,
  d: false,
  q: false,
  h: false,
  u: false,
  l: false,
  p: false,
  g: false,
  r: false,
  t: false,
  z: false
};

export let globalWrappers: Array<{ id: string; el: HTMLElement }> = [];
export let controlWrappers: Array<{ id: string; el: HTMLElement }> = [];
export let latestControlWrapper: HTMLElement | null = null;

/** Функция задания значения для ```quickTasksAllowed```. */
export function setQuickTasksAllowed(value: boolean): void {
  quickTasksAllowed = value;
}

/** Функция инициализации руководства по использованию. */
async function initUserGuide(): Promise<void> {
  try {
    const content = await download('https://raw.githubusercontent.com/nullclyze/SalarixiOnion/refs/heads/main/salarixi.guide.json');

    if (!content) return;

    (document.getElementById('guide-latest-update') as HTMLElement).innerText = content['latest-update'];

    const guide = document.getElementById('guide-wrapper') as HTMLElement; 

    const sections = content['sections'];

    for (const section of sections) {
      const sectionElement = document.createElement('div');
      sectionElement.className = 'section';

      const header = section['header'];
      const subsections = section['subsections'];

      if (header) {
        const el = document.createElement('div');
        el.className = 'header';
        el.innerText = header;

        sectionElement.appendChild(el);
      }

      for (const subsection of subsections) {
        const subsectionElement = document.createElement('div');
        subsectionElement.className = 'subsection';

        const subheader = subsection['subheader'];
        const paragraphs = subsection['paragraphs'];

        if (subheader) {
          const el = document.createElement('div');
          el.className = 'subheader';
          el.innerText = subheader;

          subsectionElement.appendChild(el);
        }

        for (const paragraph of paragraphs) {
          const el = document.createElement('p');

          const html = String(paragraph)
            .replaceAll('/*', '<span class="bold">')
            .replaceAll('/!', '<span class="highlight">')
            .replaceAll('/#', '<span class="link">')
            .replaceAll('/:', '<span class="code">')
            .replaceAll('/&', '</span>');

          el.innerHTML = html;

          subsectionElement.appendChild(el);
        }

        sectionElement.appendChild(subsectionElement);
      }

      guide.appendChild(sectionElement);
    }
  } catch (error) {
    logger.log(`Ошибка загрузки руководства: ${error}`, 'error');
  }
}

/** Функция обновления и сохранения состояния плагина (включен / выключен). */
export function updatePluginState(name: string, state: boolean) {
  if (process === 'sleep') {
    plugins[name].enable = state;

    localStorage.setItem(`plugin-state:${name}`, String(state));

    const status = document.getElementById(`${name}-status`) as HTMLElement;
    const toggler = document.getElementById(`${name}-toggler`) as HTMLButtonElement;

    if (state) {
      status.innerText = 'Включен';

      toggler.setAttribute('state', 'false');
      toggler.style.color = '#4ed618';
      toggler.innerText = 'Включен';
    } else {
      status.innerText = 'Выключен';

      toggler.setAttribute('state', 'true');
      toggler.style.color = '#e61919';
      toggler.innerText = 'Выключен';
    }
  } else {
    logger.log(`Не удалось изменить состояние плагина ${name}: Plugin states can only be changed before startup`, 'warning');
  }
}

/** Функция инициализации карточек плагинов. */
function initPlugins(): void {
  try {
    for (const name in plugins) {
      const plugin = plugins[name];

      const pluginCard = document.createElement('div');
      pluginCard.className = 'plugin-card';
      pluginCard.id = `${name}-plugin`;

      pluginCard.innerHTML = `
        <div class="head">
          <svg class="image" xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" viewBox="0 0 16 16">
            <path fill-rule="evenodd" d="M1 8a7 7 0 1 1 2.898 5.673c-.167-.121-.216-.406-.002-.62l1.8-1.8a3.5 3.5 0 0 0 4.572-.328l1.414-1.415a.5.5 0 0 0 0-.707l-.707-.707 1.559-1.563a.5.5 0 1 0-.708-.706l-1.559 1.562-1.414-1.414 1.56-1.562a.5.5 0 1 0-.707-.706l-1.56 1.56-.707-.706a.5.5 0 0 0-.707 0L5.318 5.975a3.5 3.5 0 0 0-.328 4.571l-1.8 1.8c-.58.58-.62 1.6.121 2.137A8 8 0 1 0 0 8a.5.5 0 0 0 1 0"/>
          </svg>

          <div class="text">
            <div class="header" id="${name}-header">${plugin.header}</div>
            <div class="meta">
              <span translator-tag="label:plugin-status">Статус:</span> <span id="${name}-status">Выключен</span>
            </div>
          </div>
        </div>

        <div class="controls">
          <button class="button" plugin-toggler="true" plugin-name="${name}" state="true" id="${name}-toggler" style="color: #d61818;">Выключен</button>
          <button class="button" plugin-open-description="true" path="${name}-plugin-description">Описание</button>
        </div>
      `;

      document.getElementById('plugin-list')?.appendChild(pluginCard);
      
      const element = document.createElement('div');

      element.className = 'cover';
      element.id = `${name}-plugin-description`;

      element.innerHTML = `
        <div class="panel with-header">
          <div class="left">
            <div class="header">${plugin.header} - Описание</div>
          </div>

          <div class="right">
            <button class="btn min pretty" plugin-close-description="true" path="${name}-plugin-description">
              ⨉
            </button>
          </div>
        </div>

        <div class="plugin-description-body">
          <div class="description" id="${name}-description"></div>
          <p class="plugin-latest-update">Последнее обновление: <span id="${name}-latest-update">?</span></p>
        </div>
      `;

      document.getElementById('plugins-container')?.appendChild(element);
    }

    document.querySelectorAll('[plugin-toggler="true"]').forEach(t => t.addEventListener('click', () => {
      const name = t.getAttribute('plugin-name') || '';
      const state = t.getAttribute('state') === 'true';
      updatePluginState(name, state);
    }));  

    document.querySelectorAll('[plugin-open-description="true"]').forEach(e => e.addEventListener('click', () => {
      const path = e.getAttribute('path');
      if (!path) return;
      (document.getElementById(path) as HTMLElement).style.display = 'flex';
    })); 

    document.querySelectorAll('[plugin-close-description="true"]').forEach(e => e.addEventListener('click', () => {
      const path = e.getAttribute('path');
      if (!path) return;
      (document.getElementById(path) as HTMLElement).style.display = 'none';
    })); 
  } catch (error) {
    logger.log(`Ошибка инициализации плагинов: ${error}`, 'error');
  }
}

/** Функция инициализации описаний плагинов. */
async function initPluginDescriptions(): Promise<void> {
  try {
    const content = await download('https://raw.githubusercontent.com/nullclyze/SalarixiOnion/refs/heads/main/salarixi.plugins.json');
    
    if (!content) return;

    for (const plugin of content['list']) {
      const name = plugin['name'];
      const description = plugin['description'];
      const latestUpdate = plugin['latest-update'];

      if (plugins[name]?.date) {
        if (latestUpdate != plugins[name].date) {
          document.getElementById(`${name}-plugin`)?.classList.add('deprecated');
          
          const tag = document.createElement('span');
          tag.className = 'tag';
          tag.innerText = 'Устарел';

          document.getElementById(`${name}-header`)?.appendChild(tag);
        }

        const pluginDescriptionContainer = document.getElementById(`${name}-description`) as HTMLElement;
        const pluginLatestUpdateContainer = document.getElementById(`${name}-latest-update`) as HTMLElement;

        for (const p of description) {
          const el = document.createElement('p');
          el.innerText = p;

          pluginDescriptionContainer.appendChild(el);
        }

        pluginLatestUpdateContainer.innerText = latestUpdate;
      } else {
        const pluginCard = document.createElement('div');
        pluginCard.className = 'plugin-card';
        pluginCard.classList.add('unavailable');

        pluginCard.innerHTML = `
          <div class="head">
            <svg class="image" xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" viewBox="0 0 16 16">
              <path fill-rule="evenodd" d="M1 8a7 7 0 1 1 2.898 5.673c-.167-.121-.216-.406-.002-.62l1.8-1.8a3.5 3.5 0 0 0 4.572-.328l1.414-1.415a.5.5 0 0 0 0-.707l-.707-.707 1.559-1.563a.5.5 0 1 0-.708-.706l-1.559 1.562-1.414-1.414 1.56-1.562a.5.5 0 1 0-.707-.706l-1.56 1.56-.707-.706a.5.5 0 0 0-.707 0L5.318 5.975a3.5 3.5 0 0 0-.328 4.571l-1.8 1.8c-.58.58-.62 1.6.121 2.137A8 8 0 1 0 0 8a.5.5 0 0 0 1 0"/>
            </svg>

            <div class="text">
              <div class="header">${plugin['header']} <span class="tag">Недоступен</span></div>
              <div class="meta">Статус: <span class="status">Недоступен</span></div>
            </div>
          </div>
        `;

        document.getElementById('plugin-list')?.appendChild(pluginCard);
      }
    }
  } catch (error) {
    logger.log(`Ошибка загрузки информации о плагинах: ${error}`, 'error');
  }
}

/** Функция получения количества скачиваний программы. */
async function initDownloadCount(): Promise<void> {
  try {
    const content = await download('https://api.github.com/repos/nullclyze/SalarixiOnion/releases');
    if (!content) return;

    let globalDownloadCount = 0;
    for (const release of content) for (const build of release['assets']) globalDownloadCount += build['download_count'] ? build['download_count'] as number : 0;
    
    (document.getElementById('download-count') as HTMLElement).innerText = globalDownloadCount.toString();
  } catch (error) {
    logger.log(`Ошибка загрузки количества скачиваний: ${error}`, 'error');
  }
}

/** Функция запуска ботов (передача информации в поток Rust). */
async function startBots(): Promise<void> {
  try {
    const options: {
      basic: any;
      accounts: any;
      plugins: any;
      captcha_bypass: any;
      webhook: any;
    } = {
      basic: {},
      accounts: {},
      plugins: {},
      captcha_bypass: {},
      webhook: {}
    };

    document.querySelectorAll<HTMLElement>('[launch-option]').forEach(o => {
      const optionSection = o.getAttribute('section') || 'basic';
      const optionKey = o.getAttribute('key') || '';

      const current = options[optionSection as 'basic' | 'plugins' | 'captcha_bypass' | 'webhook'];

      if (o.tagName.toLocaleLowerCase() === 'input') {
        const input = o as HTMLInputElement;

        if (input.type === 'checkbox') {
          current[optionKey] = input.checked;
        } else {
          const defaultValue = input.getAttribute('default');

          if (input.type === 'number') {
            current[optionKey] = input.value ? parseInt(input.value) : defaultValue ? parseInt(defaultValue!) : null;
          } else {
            current[optionKey] = input.value || defaultValue;
          }
        }
      } else if (o.tagName.toLocaleLowerCase() === 'select') {
        const select = o as HTMLSelectElement;
        current[optionKey] = select.value;
      } else if (o.tagName.toLocaleLowerCase() === 'textarea') {
        const textarea = o as HTMLTextAreaElement;
        current[optionKey] = textarea.value;
      }
    });

    options.accounts = accountManager.getAccounts();

    for (const name in plugins) options.plugins[name.replaceAll('-', '_')] = plugins[name].enable;

    const text = `Запуск ботов на сервер ${options.basic.address}...`;

    logger.log(text, 'info');
    messages.message('Cистема', text);

    const success = await invoke('launch_bots', { options: options }) as boolean;

    if (!success) return;

    process = 'active';

    graphic.enable();

    monitoring.maxChatHistoryLength = parseInt((document.getElementById('monitoring_option_chat-history-length') as HTMLInputElement).value || '50');

    monitoring.enable(parseInt((document.getElementById('monitoring_option_update-frequency') as HTMLInputElement).value || '1800'));
    monitoring.wait();

    if (options.basic.use_anti_captcha) captchaBypass.enable(options.captcha_bypass.captcha_type, options.captcha_bypass.solve_mode);

    radar.enable();
  } catch (error) {
    logger.log(`Ошибка (start-bots-process): ${error}`, 'error');
  }
}

/** Функция остановки ботов (передача информации в поток Rust). */
async function stopBots(): Promise<void> {
  logger.log('Остановка ботов...', 'info');

  try {
    const success = await invoke('stop_bots') as boolean;

    if (!success) return;

    process = 'sleep';

    logger.log('Выключение мониторинга...', 'system');

    graphic.disable();
    monitoring.disable();
    captchaBypass.disable();
    radar.disable();

    logger.log('Мониторинг выключен', 'system');
  } catch (error) {
    logger.log(`Ошибка (stop-bots-process): ${error}`, 'error');
  }
}

/** Функция отправки данных в поток Rust для модулей управления. */
async function controlBots(name: string, state: boolean | string): Promise<void> {
  try {
    const elements = document.querySelectorAll(`[control="${name}"]`);

    let options: Record<string, any> = {};

    elements.forEach(e => e.tagName.toLowerCase() === 'select' ? options[(e as HTMLSelectElement).name] = (e as HTMLSelectElement).value : (e as HTMLInputElement).type === 'checkbox' ? options[(e as HTMLInputElement).name] = (e as HTMLInputElement).checked : options[(e as HTMLInputElement).name] = (e as HTMLInputElement).type === 'number' ? Number((e as HTMLInputElement).value) : (e as HTMLInputElement).value);

    const group = (document.getElementById('control-group') as HTMLInputElement).value.replace(' ', '');

    await invoke('control_bots', {
      name: name,
      options: {
        ...options,
        state: state
      },
      group: group !== '' ? group : 'global'
    });
  } catch (error) {
    logger.log(`Ошибка управления (${name}): ${error}`, 'error');
  }
}

/** Функция инициализации различных функций. */
async function initFunctions(): Promise<void> {
  const startBotsProcessBtn = document.getElementById('start') as HTMLButtonElement;
  const stopBotsProcessBtn = document.getElementById('stop') as HTMLButtonElement;
  const setRandomValuesBtn =  document.getElementById('random') as HTMLButtonElement;
  const clearInputValuesBtn = document.getElementById('clear') as HTMLButtonElement;

  const dashboardBtns = document.querySelectorAll<HTMLButtonElement>('.dashboard .pretty-btn');
  const controlBtns = document.querySelectorAll<HTMLButtonElement>('.control-btn');
  
  dashboardBtns.forEach(btn => {
    if (btn.id === 'main') btn.classList.add('selected');
    btn.addEventListener('click', () => {
      const path = btn.getAttribute('path');
      if (!path) return;
      switchGlobalWrapper(path);
      dashboardBtns.forEach(b => b.classList.remove('selected'));
      btn.classList.add('selected');
    });
  });
  
  controlBtns.forEach(btn => {
    if (btn.id === 'control-chat') btn.classList.add('selected');
    btn.addEventListener('click', () => {
      const path = btn.getAttribute('path');
      if (!path) return;
      switchControlWrapper(path);
      controlBtns.forEach(b => b.classList.remove('selected'));
      btn.classList.add('selected');
      latestControlWrapper = document.getElementById(path);
    });
  });

  document.querySelectorAll<HTMLButtonElement>('[control-toggler="true"]').forEach(e => e.addEventListener('click', async () => { 
    let state: boolean | string = false;
    let attribute = e.getAttribute('state');

    switch (attribute) {
      case 'true':
        state = true;
        break;
      case 'false':
        state = false;
        break;
      case null:
        state = false;
        break;
      default:
        state = attribute;
        break;
    }

    await controlBots(e.name, state);
  }));

  document.addEventListener('keydown', async (e) => {
    if (process === 'sleep' || !quickTasksAllowed) return;
    const key = e.key.toLowerCase();
    for (const k in pressedKeys) key === k ? pressedKeys[key] = true : null;
    if (pressedKeys.shift && pressedKeys.i && pressedKeys.c) {
      await invoke('quick_task', { name: 'clear-inventory' });
    } else if (pressedKeys.shift && pressedKeys.f && pressedKeys.c) {
      await invoke('quick_task', { name: 'form-circle' });
    } else if (pressedKeys.shift && pressedKeys.f && pressedKeys.l) {
      await invoke('quick_task', { name: 'form-line' });
    } else if (pressedKeys.shift && pressedKeys.p && pressedKeys.s) {
      await invoke('quick_task', { name: 'pathfinder-stop' });
    } else if (pressedKeys.shift && pressedKeys.g && pressedKeys.f) {
      await invoke('quick_task', { name: 'fly' });
    } else if (pressedKeys.shift && pressedKeys.g && pressedKeys.r) {
      await invoke('quick_task', { name: 'rise' });
    } else if (pressedKeys.shift && pressedKeys.g && pressedKeys.c) {
      await invoke('quick_task', { name: 'capsule' });
    } else if (pressedKeys.alt && pressedKeys.shift && pressedKeys.q) {
      await invoke('quick_task', { name: 'quit' });
    } else if (pressedKeys.shift && pressedKeys.w) {
      await invoke('quick_task', { name: 'move-forward' });
    } else if (pressedKeys.shift && pressedKeys.s) {
      await invoke('quick_task', { name: 'move-backward' });
    } else if (pressedKeys.shift && pressedKeys.a) {
      await invoke('quick_task', { name: 'move-left' });
    } else if (pressedKeys.shift && pressedKeys.d) {
      await invoke('quick_task', { name: 'move-right' });
    } else if (pressedKeys.shift && pressedKeys.j) {
      await invoke('quick_task', { name: 'jump' });
    } else if (pressedKeys.shift && pressedKeys.c) {
      await invoke('quick_task', { name: 'shift' });
    } else if (pressedKeys.shift && pressedKeys.u) {
      await invoke('quick_task', { name: 'unite' });
    } else if (pressedKeys.shift && pressedKeys.t) {
      await invoke('quick_task', { name: 'turn' });
    } else if (pressedKeys.shift && pressedKeys.z) {
      await invoke('quick_task', { name: 'zero' });
    }
  });

  document.addEventListener('keyup', (e) => {
    if (process === 'sleep') return;
    const key = e.key.toLowerCase();
    for (const k in pressedKeys) key === k ? pressedKeys[key] = false : null;
  }); 

  startBotsProcessBtn.addEventListener('click', async () => await startBots());
  stopBotsProcessBtn.addEventListener('click', async () => await stopBots());

  setRandomValuesBtn.addEventListener('click', () => {
    const versions = [
      '1.8.9', '1.12.2', '1.12',
      '1.14', '1.16.4', '1.16.5',
      '1.19', '1.20.1', '1.20.3',
      '1.21', '1.21.1', '1.21.3',
      '1.21.6', '1.21.5', '1.20.4',
      '1.21.10', '1.21.11', '1.21.8'
    ];

    const gen = (e: string): void => {
      let current = document.getElementById(e) as HTMLInputElement; 
      switch (e) {
        case 'settings_option_version':
          const randomVersion = String(versions[Math.floor(Math.random() * versions.length)]);
          current.value = (document.getElementById('settings_chbx_use-proxy') as HTMLInputElement).checked ? '1.21.11' : randomVersion; break;
        case 'settings_option_bots-count':
          const randomQuantity = Math.floor(Math.random() * (50 - 10 + 1) + 10);
          current.valueAsNumber = randomQuantity; break;
        case 'settings_option_join-delay':
          const randomDelay = Math.floor(Math.random() * (7000 - 1000 + 1) + 1000);
          current.valueAsNumber = randomDelay; break;
      }
    }

    gen('settings_option_version');
    gen('settings_option_bots-count');
    gen('settings_option_join-delay');
  });

  clearInputValuesBtn.addEventListener('click', () => {
    (document.getElementById('settings_option_address') as HTMLInputElement).value = '';
    (document.getElementById('settings_option_version') as HTMLInputElement).value = '';
    (document.getElementById('settings_option_bots-count') as HTMLInputElement).value = '';
    (document.getElementById('settings_option_join-delay') as HTMLInputElement).value = '';
  });

  document.getElementById('interface_select_discord-rpc')?.addEventListener('change', async () => await invoke('set_discord_rpc', { state: (document.getElementById('interface_select_discord-rpc') as HTMLSelectElement).value === 'enable' }));
  await invoke('set_discord_rpc', { state: (document.getElementById('interface_select_discord-rpc') as HTMLSelectElement).value === 'enable' });

  await initUserGuide();
  await initPluginDescriptions();
  await initDownloadCount();
} 

/** Обёрточная функция для пополнения реестра триггеров. */
export function replenishTriggerRegistry(): void {
  triggerRegistry.register('settings_chbx_use-proxy', 'checkbox', (current: HTMLInputElement) => {
    const version = document.getElementById('settings_option_version') as HTMLInputElement;

    if (current.checked) {
      version.value = '1.21.11';
      version.disabled = true;
    } else {
      version.disabled = false;
    }
  });

  triggerRegistry.register('settings_select_nickname-type', 'select', (current: HTMLSelectElement) => {
    const container = document.getElementById('custom-nickname-template-container') as HTMLInputElement;

    if (current.value === 'custom') {
      container.style.display = 'flex';
    } else {
      container.style.display = 'none';
    }
  });

  triggerRegistry.register('settings_select_password-type', 'select', (current: HTMLSelectElement) => {
    const container = document.getElementById('custom-password-template-container') as HTMLInputElement;

    if (current.value === 'custom') {
      container.style.display = 'flex';
    } else {
      container.style.display = 'none';
    }
  });

  triggerRegistry.register('settings_select_register-mode', 'select', (current: HTMLSelectElement) => {
    if (current.value === 'default') {
      (document.getElementById('register-min-delay-container') as HTMLInputElement).style.display = 'flex';
      (document.getElementById('register-max-delay-container') as HTMLInputElement).style.display = 'flex';
      (document.getElementById('register-trigger-container') as HTMLInputElement).style.display = 'none';
    } else {
      (document.getElementById('register-min-delay-container') as HTMLInputElement).style.display = 'none';
      (document.getElementById('register-max-delay-container') as HTMLInputElement).style.display = 'none';
      (document.getElementById('register-trigger-container') as HTMLInputElement).style.display = 'flex';
    }
  });

  triggerRegistry.register('settings_select_login-mode', 'select', (current: HTMLSelectElement) => {
    if (current.value === 'default') {
      (document.getElementById('login-min-delay-container') as HTMLInputElement).style.display = 'flex';
      (document.getElementById('login-max-delay-container') as HTMLInputElement).style.display = 'flex';
      (document.getElementById('login-trigger-container') as HTMLInputElement).style.display = 'none';
    } else {
      (document.getElementById('login-min-delay-container') as HTMLInputElement).style.display = 'none';
      (document.getElementById('login-max-delay-container') as HTMLInputElement).style.display = 'none';
      (document.getElementById('login-trigger-container') as HTMLInputElement).style.display = 'flex';
    }
  });

  triggerRegistry.register('settings_chbx_use-auto-register', 'checkbox', (current: HTMLInputElement) => {
    const container = document.getElementById('auto-register-input-container') as HTMLElement;

    if (current.checked) {
      container.style.display = 'flex';
    } else {
      container.style.display = 'none';
    }
  });

  triggerRegistry.register('settings_chbx_use-auto-login', 'checkbox', (current: HTMLInputElement) => {
    const container = document.getElementById('auto-login-input-container') as HTMLElement;

    if (current.checked) {
      container.style.display = 'flex';
    } else {
      container.style.display = 'none';
    }
  });

  triggerRegistry.register('settings_chbx_use-auto-rejoin', 'checkbox', (current: HTMLInputElement) => {
    const container = document.getElementById('auto-rejoin-input-container') as HTMLElement;

    if (current.checked) {
      container.style.display = 'flex';
    } else {
      container.style.display = 'none';
    }
  });

  triggerRegistry.register('settings_chbx_use-accounts', 'checkbox', (current: HTMLInputElement) => {
    const openAccountsSectionBtn = document.getElementById('accounts') as HTMLButtonElement;
    const openProxySectionBtn = document.getElementById('proxy') as HTMLButtonElement;

    const useProxyChbx = document.getElementById('settings_chbx_use-proxy') as HTMLInputElement;
    const botsCountInput = document.getElementById('settings_option_bots-count') as HTMLInputElement;
    const selectNicknameType = document.getElementById('settings_select_nickname-type') as HTMLSelectElement;
    const selectPasswordType = document.getElementById('settings_select_password-type') as HTMLSelectElement;
    const nicknameTemplateInput = document.getElementById('settings_option_nickname-template') as HTMLInputElement;
    const passwordTemplateInput = document.getElementById('settings_option_password-template') as HTMLInputElement;

    if (current.checked) {
      openProxySectionBtn.style.display = 'none';
      openAccountsSectionBtn.style.display = 'flex';

      useProxyChbx.disabled = true;
      botsCountInput.disabled = true;
      selectNicknameType.disabled = true;
      selectPasswordType.disabled = true;
      nicknameTemplateInput.disabled = true;
      passwordTemplateInput.disabled = true;
    } else {
      openProxySectionBtn.style.display = 'flex';
      openAccountsSectionBtn.style.display = 'none';

      useProxyChbx.disabled = false;
      botsCountInput.disabled = false;
      selectNicknameType.disabled = false;
      selectPasswordType.disabled = false;
      nicknameTemplateInput.disabled = false;
      passwordTemplateInput.disabled = false;
    }
  });

  triggerRegistry.register('captcha-bypass_select_captcha-type', 'select', (current: HTMLSelectElement) => {
    const antiWebCaptchaOptionsContainer = document.getElementById('anti-web-captcha-options') as HTMLElement;
    const antiMapCaptchaOptionsContainer = document.getElementById('anti-map-captcha-options') as HTMLElement;
    const antiWebCaptchaSelectsContainer = document.getElementById('anti-web-captha-selects-container') as HTMLElement;
    const antiMapCaptchaSelectsContainer = document.getElementById('anti-map-captha-selects-container') as HTMLElement;

    if (current.value === 'web') {
      antiWebCaptchaOptionsContainer.style.display = 'flex';
      antiWebCaptchaSelectsContainer.style.display = 'flex';
      antiMapCaptchaOptionsContainer.style.display = 'none';
      antiMapCaptchaSelectsContainer.style.display = 'none';
    } else if (current.value === 'map') {
      antiWebCaptchaOptionsContainer.style.display = 'none';
      antiWebCaptchaSelectsContainer.style.display = 'none';
      antiMapCaptchaSelectsContainer.style.display = 'flex';
    }
  });

  triggerRegistry.register('captcha-bypass_select_solve-mode', 'select', (current: HTMLSelectElement) => {
    const antiWebCaptchaWebDriverServerUrlContainer = document.getElementById('anti-web-captcha-webdriver-server-url-container') as HTMLElement;
    const antiWebCaptchaSelectBrowserContainer = document.getElementById('select-captcha-browser-container') as HTMLElement;

    if (current.value === 'auto') {
      antiWebCaptchaWebDriverServerUrlContainer.style.display = 'flex';
      antiWebCaptchaSelectBrowserContainer.style.display = 'grid';
    } else {
      antiWebCaptchaWebDriverServerUrlContainer.style.display = 'none';
      antiWebCaptchaSelectBrowserContainer.style.display = 'none';
    }
  });

  triggerRegistry.register('captcha-bypass_select_captcha-subtype', 'select', (current: HTMLSelectElement) => {
    const antiMapCaptchaOptionsContainer = document.getElementById('anti-map-captcha-options') as HTMLElement;

    if (current.value === 'inventory') {
      antiMapCaptchaOptionsContainer.style.display = 'none';
    } else if (current.value === 'frame') {
      antiMapCaptchaOptionsContainer.style.display = 'flex';
    }
  });

  triggerRegistry.register('module_chat_select_mode', 'select', (current: HTMLSelectElement) => {
    const chatSpammingChbxContainer = document.getElementById('chat-spamming-chbx-container') as HTMLElement;
    const chatSpammingInputContainer = document.getElementById('chat-spamming-input-container') as HTMLElement;
          
    const chatDefaultBtnsContainer = document.getElementById('chat-default-btns-container') as HTMLElement;
    const chatSpammingBtnsContainer = document.getElementById('chat-spamming-btns-container') as HTMLElement;

    if (current.value === 'spamming') {
      chatSpammingChbxContainer.style.display = 'flex';
      chatSpammingInputContainer.style.display = 'flex';
      chatSpammingBtnsContainer.style.display = 'flex';

      chatDefaultBtnsContainer.style.display = 'none';
    } else {
      chatDefaultBtnsContainer.style.display = 'flex';

      chatSpammingChbxContainer.style.display = 'none';
      chatSpammingInputContainer.style.display = 'none';
      chatSpammingBtnsContainer.style.display = 'none';
    }
  });

  triggerRegistry.register('module_inventory_select_mode', 'select', (current: HTMLSelectElement) => {
    const basicBtnContainer = document.getElementById('inventory-basic-btn-container') as HTMLElement;
    const swapInputContainer = document.getElementById('inventory-swap-input-container') as HTMLElement;
    const swapBtnContainer = document.getElementById('inventory-swap-btn-container') as HTMLElement;

    if (current.value === 'basic') {
      basicBtnContainer.style.display = 'flex';
      swapInputContainer.style.display = 'none';
      swapBtnContainer.style.display = 'none';
    } else if (current.value === 'swap') {
      swapInputContainer.style.display = 'flex';
      swapBtnContainer.style.display = 'flex';
      basicBtnContainer.style.display = 'none';
    } 
  });

  triggerRegistry.register('module_movement_select_mode', 'select', (current: HTMLSelectElement) => {
    const selectMovementDirectionContainer = document.getElementById('select-movement-direction-container') as HTMLElement;
    const movementPathfinderInputContainer = document.getElementById('movement-pathfinder-input-container') as HTMLElement;
    const movementImpulsivenessContainer = document.getElementById('movement-impulsiveness-container') as HTMLElement;

    if (current.value === 'default') {
      selectMovementDirectionContainer.style.display = 'flex';
      movementPathfinderInputContainer.style.display = 'none';
      movementImpulsivenessContainer.style.display = 'flex';
    } else {
      selectMovementDirectionContainer.style.display = 'none';
      movementPathfinderInputContainer.style.display = 'flex';
      movementImpulsivenessContainer.style.display = 'none';
    }
  });

  triggerRegistry.register('module_flight_select_settings', 'select', (current: HTMLSelectElement) => {
    const manualSettingsContainer = document.getElementById('flight-manual-settings-container') as HTMLElement;
    const selectAntiCheatContainer = document.getElementById('flight-select-anti-cheat-container') as HTMLElement;

    if (current.value === 'adaptive') {
      manualSettingsContainer.style.display = 'none';
      selectAntiCheatContainer.style.display = 'grid';
    } else {
      manualSettingsContainer.style.display = 'flex';
      selectAntiCheatContainer.style.display = 'none';
    }
  });

  triggerRegistry.register('module_killaura_select_target', 'select', (current: HTMLSelectElement) => {
    const customGoalInputContainer = document.getElementById('killaura-custom-goal-input-container') as HTMLElement;

    if (current.value === 'custom') {
      customGoalInputContainer.style.display = 'flex';
    } else {
      customGoalInputContainer.style.display = 'none';
    }
  });

  triggerRegistry.register('module_killaura_select_settings', 'select', (current: HTMLSelectElement) => {
    const manualSettingsContainer = document.getElementById('killaura-manual-settings-container') as HTMLElement;

    if (current.value === 'adaptive') {
      manualSettingsContainer.style.display = 'none';
    } else {
      manualSettingsContainer.style.display = 'flex';
    }
  });

  triggerRegistry.register('module_killaura_chbx_use-auto-weapon', 'checkbox', (current: HTMLInputElement) => {
    const selectWeaponContainer = document.getElementById('select-killaura-weapon-container') as HTMLElement;
    const weaponSlotContainer = document.getElementById('killaura-weapon-slot-container') as HTMLElement;

    if (current.checked) {
      selectWeaponContainer.style.display = 'grid';
      weaponSlotContainer.style.display = 'none';
    } else {
      selectWeaponContainer.style.display = 'none';
      weaponSlotContainer.style.display = 'flex';
    }
  });

  triggerRegistry.register('module_killaura_chbx_use-chase', 'checkbox', (current: HTMLInputElement) => {
    const chaseSettingsContainer = document.getElementById('killaura-chase-settings-container') as HTMLElement;

    if (current.checked) {
      chaseSettingsContainer.style.display = 'flex';
    } else {
      chaseSettingsContainer.style.display = 'none';
    }
  });
  
  triggerRegistry.register('module_bow-aim_select_target', 'select', (current: HTMLSelectElement) => {
    const customGoalInputContainer = document.getElementById('bow-aim-custom-goal-input-container') as HTMLElement;

    if (current.value === 'custom') {
      customGoalInputContainer.style.display = 'flex';
    } else {
      customGoalInputContainer.style.display = 'none';
    }
  });

  triggerRegistry.register('module_miner_select_mode', 'select', (current: HTMLSelectElement) => {
    const tunnelSelectContainer = document.getElementById('miner-tunnel-select-container') as HTMLElement;
    const lookSelectContainer = document.getElementById('miner-look-select-container') as HTMLElement;

    if (current.value === 'default') {
      tunnelSelectContainer.style.display = 'none';
      lookSelectContainer.style.display = 'none';
    } else {
      tunnelSelectContainer.style.display = 'grid';
      lookSelectContainer.style.display = 'grid';
    }
  });

  triggerRegistry.register('settings_select_skin-type', 'select', (current: HTMLSelectElement) => {
    const setSkinCommandContainer = document.getElementById('set-skin-command-container') as HTMLElement;
    const customSkinContainer = document.getElementById('custom-skin-container') as HTMLElement;

    if (current.value === 'default') {
      setSkinCommandContainer.style.display = 'none';
      customSkinContainer.style.display = 'none';
    } else if (current.value === 'random') {
      setSkinCommandContainer.style.display = 'flex';
      customSkinContainer.style.display = 'none';
    } else if (current.value === 'custom') {
      setSkinCommandContainer.style.display = 'flex';
      customSkinContainer.style.display = 'flex';
    }
  });

  triggerRegistry.register('journal_chbx_show-system-logs', 'checkbox', (current: HTMLInputElement) => logger.setVisibility('system', current.checked));
  triggerRegistry.register('journal_chbx_show-extended-logs', 'checkbox', (current: HTMLInputElement) => logger.setVisibility('extended', current.checked));

  triggerRegistry.register('proxy-finder_select_algorithm', 'select', (current: HTMLSelectElement) => {
    const selectProxyFinderCountry = document.getElementById('proxy-finder_select_country') as HTMLSelectElement;

    if (current.value === 'apic') {
      selectProxyFinderCountry.disabled = false;
    } else {
      selectProxyFinderCountry.disabled = true;
    }
  });

  triggerRegistry.register('interface_select_particles', 'select', (current: HTMLSelectElement) => {
    try {
      if (current.value === 'enable') {
        enableParticles();
      } else {
        disableParticles();
      }
    } catch (error) {
      logger.log(`Ошибка изменения состояния партиклов: ${error}`, 'error');
    }
  });

  triggerRegistry.register('interface_select_theme', 'select', (current: HTMLSelectElement) => {
    try {
      const root = document.documentElement;

      switch (current.value) {
        case 'onion': 
          root.style.setProperty('--title-color', '#7e47ff');
          root.style.setProperty('--spec-color', '#7946f0');
          root.style.setProperty('--dull-spec-color', '#5426be');
          root.style.setProperty('--chbx-spec-color', '#7946f0');
          root.style.setProperty('--chbx-dull-spec-color', '#5426be');
          break;
        case 'toxic': 
          root.style.setProperty('--title-color', '#3ec71c');
          root.style.setProperty('--spec-color', '#40cf33');
          root.style.setProperty('--dull-spec-color', '#51be26');
          root.style.setProperty('--chbx-spec-color', '#25a115');
          root.style.setProperty('--chbx-dull-spec-color', '#1d7e11');
          break;
        case 'ice': 
          root.style.setProperty('--title-color', '#17b2f0');
          root.style.setProperty('--spec-color', '#46b2f0');
          root.style.setProperty('--dull-spec-color', '#26b9be');
          root.style.setProperty('--chbx-spec-color', '#26a9e6');
          root.style.setProperty('--chbx-dull-spec-color', '#1386a8');
          break;
        case 'blood':
          root.style.setProperty('--title-color', '#f72323');
          root.style.setProperty('--spec-color', '#f03333');
          root.style.setProperty('--dull-spec-color', '#d62727');
          root.style.setProperty('--chbx-spec-color', '#ce2323');
          root.style.setProperty('--chbx-dull-spec-color', '#aa1818');
          break;
        case 'gold': 
          root.style.setProperty('--title-color', '#f8fc12');
          root.style.setProperty('--spec-color', '#ccbd33');
          root.style.setProperty('--dull-spec-color', '#bbbe26');
          root.style.setProperty('--chbx-spec-color', '#95a319');
          root.style.setProperty('--chbx-dull-spec-color', '#7e7c0f');
          break;
        case 'dark': 
          root.style.setProperty('--title-color', '#818181');
          root.style.setProperty('--spec-color', '#646464');
          root.style.setProperty('--dull-spec-color', '#555555');
          root.style.setProperty('--chbx-spec-color', '#494949');
          root.style.setProperty('--chbx-dull-spec-color', '#3f3f3f');
          break;
        case 'magenta': 
          root.style.setProperty('--title-color', '#f727e6');
          root.style.setProperty('--spec-color', '#eb28db');
          root.style.setProperty('--dull-spec-color', '#be13a1');
          root.style.setProperty('--chbx-spec-color', '#9e0b97');
          root.style.setProperty('--chbx-dull-spec-color', '#960676');
          break;
        case 'snow': 
          root.style.setProperty('--title-color', '#e2e2e2');
          root.style.setProperty('--spec-color', '#cfcecf');
          root.style.setProperty('--dull-spec-color', '#979797');
          root.style.setProperty('--chbx-spec-color', '#a5a5a5');
          root.style.setProperty('--chbx-dull-spec-color', '#999999');
          break;
        case 'fire': 
          root.style.setProperty('--title-color', '#ff5e00');
          root.style.setProperty('--spec-color', '#ff6600');
          root.style.setProperty('--dull-spec-color', '#d14d25');
          root.style.setProperty('--chbx-spec-color', '#ee4c01');
          root.style.setProperty('--chbx-dull-spec-color', '#d64a1f');
          break;
        case 'ocean': 
          root.style.setProperty('--title-color', '#0051ff');
          root.style.setProperty('--spec-color', '#0044ff');
          root.style.setProperty('--dull-spec-color', '#254dd1');
          root.style.setProperty('--chbx-spec-color', '#0140ee');
          root.style.setProperty('--chbx-dull-spec-color', '#1f41d6');
          break;
      }
    } catch (error) {
      logger.log(`Ошибка изменения темы: ${error}`, 'error');
    }
  });

  triggerRegistry.register('interface_select_global-font-family', 'select', (current: HTMLSelectElement) => {
    try {
      const root = document.documentElement;

      switch (current.value) {
        case 'inter':
          root.style.setProperty('--global-font-family', `'Inter', sans-serif`);
          break;
        case 'jetbrains-mono':
          root.style.setProperty('--global-font-family', `'JetBrains Mono', 'Fira Code', monospace`);
          break;
        case 'segoe-ui':
          root.style.setProperty('--global-font-family', `'Segoe UI', Tahoma, Geneva, Verdana, sans-serif`);
          break;
      }
    } catch (error) {
      logger.log(`Ошибка изменения шрифта: ${error}`, 'error');
    }
  });

  triggerRegistry.register('interface_select_show-messages', 'select', (current: HTMLSelectElement) => {
    try {
      messages.visibility(current.value);

      document.querySelectorAll<HTMLElement>('.message').forEach(m => {
        if (current.value === 'hide') {
          m.style.display = 'none';
        } else {
          m.style.display = 'flex';
        }
      });
    } catch (error) {
      logger.log(`Ошибка изменения состояния сообщений: ${error}`, 'error');
    }
  });

  triggerRegistry.register('interface_select_show-panel-icons', 'select', (current: HTMLSelectElement) => {
    try {
      document.querySelectorAll<SVGElement>('[panel-btn-icon="true"]').forEach(i => {
        if (current.value === 'hide') {
          i.style.display = 'none';
        } else {
          i.style.display = 'flex';
        }
      });
    } catch (error) {
      logger.log(`Ошибка изменения состояния иконок на панели: ${error}`, 'error');
    }
  });

  triggerRegistry.register('interface_select_panel-font-family', 'select', (current: HTMLSelectElement) => {
    try {
      const root = document.documentElement;

      switch (current.value) {
        case 'inter':
          root.style.setProperty('--panel-font-family', `'Inter', sans-serif`);
          root.style.setProperty('--panel-text-margin-top', '1px');
          break;
        case 'jetbrains-mono':
          root.style.setProperty('--panel-font-family', `'JetBrains Mono', 'Fira Code', monospace`);
          root.style.setProperty('--panel-text-margin-top', '2px');
          break;
        case 'segoe-ui':
          root.style.setProperty('--panel-font-family', `'Segoe UI', Tahoma, Geneva, Verdana, sans-serif`);
          root.style.setProperty('--panel-text-margin-top', '-1px');
          break;
        case 'courier-new':
          root.style.setProperty('--panel-font-family', `'Courier New', Courier, monospace`);
          root.style.setProperty('--panel-text-margin-top', '4px');
          break;
        case 'trebuchet-ms':
          root.style.setProperty('--panel-font-family', `'Trebuchet MS', 'Lucida Sans Unicode', 'Lucida Grande', 'Lucida Sans', Arial, sans-serif`);
          root.style.setProperty('--panel-text-margin-top', '1px');
          break;
      }
    } catch (error) {
      logger.log(`Ошибка изменения шрифта панели: ${error}`, 'error');
    }
  });

  triggerRegistry.register('interface_select_panel-font-size', 'select', (current: HTMLSelectElement) => {
    try {
      const root = document.documentElement;
      root.style.setProperty('--panel-font-size', current.value);
    } catch (error) {
      logger.log(`Ошибка изменения размера шрифта панели: ${error}`, 'error');
    }
  });

  triggerRegistry.register('interface_select_panel-internal-gap', 'select', (current: HTMLSelectElement) => {
    try {
      const root = document.documentElement;
      root.style.setProperty('--panel-btn-gap', current.value);
    } catch (error) {
      logger.log(`Ошибка изменения внутреннего отступа кнопок панели: ${error}`, 'error');
    }
  });

  triggerRegistry.register('interface_select_panel-internal-gap', 'select', (current: HTMLSelectElement) => {
    try {
      const root = document.documentElement;
      root.style.setProperty('--panel-btn-gap', current.value);
    } catch (error) {
      logger.log(`Ошибка изменения внутреннего отступа кнопок панели: ${error}`, 'error');
    }
  });
}

/** Функция добавления открывающейся ссылки для определённого события элемента. */
function addOpeningUrlTo(id: string, event: string, url: string): void {
  const el = document.getElementById(id);
  if (!el) return;
  el.addEventListener(event, async () => {
    try {
      await invoke('open_url', { url: url });
    } catch (error) {
      logger.log(`Ошибка открытия URL: ${error}`, 'error');
    }
  });
}

/** Функция проверки наличия обновлений программы. */
async function checkUpdate(): Promise<void> {
  try {
    const notice = document.getElementById('notice') as HTMLElement;
    const newVersion = document.getElementById('new-client-version') as HTMLElement;
    const newTag = document.getElementById('new-client-tag') as HTMLElement;
    const newReleaseDate = document.getElementById('new-client-release-date') as HTMLElement;

    document.getElementById('close-notice-btn')?.addEventListener('click', () => notice.style.display = 'none');

    const response = await fetch('https://raw.githubusercontent.com/nullclyze/SalarixiOnion/refs/heads/main/salarixi.version.json', { method: 'GET' });

    if (!response.ok) return;

    const data = await response.json();

    if (data && data.version !== client.version) {
      const tag = `v${data.version}-${String(data.type).toLowerCase()}`;

      newVersion.innerText = data.version;
      newTag.innerText = tag;
      newReleaseDate.innerText = data.releaseDate;
      
      addOpeningUrlTo('open-client-release', 'click', `https://github.com/nullclyze/SalarixiOnion/releases/tag/${tag}`); 
      
      setTimeout(() => notice.style.display = 'flex', 4000);
    }
  } catch (error) {
    logger.log(`Ошибка проверки обновлений: ${error}`, 'error');
  }
}

/** Вспомогательная функция инициализации титул бара. */
function initTitleBar(): void {
  (document.getElementById('title-version') as HTMLElement).innerText = `v${client.version}`;
  (document.getElementById('window-minimize') as HTMLButtonElement).addEventListener('click', async () => await getCurrentWindow().minimize());
  (document.getElementById('window-close') as HTMLButtonElement).addEventListener('click', async () => await invoke('exit'));
}

/** Функция инициализации слушателей основных событий. */
async function listenEvents(): Promise<void> {
  await listen('log', (event) => {
    try {
      const payload = event.payload as { text: string; class: string; };
      logger.log(payload.text, payload.class);
    } catch (error) {
      logger.log(`Ошибка принятие log-события: ${error}`, 'error');
    }
  });

  await listen('message', (event) => {
    try {
      const payload = event.payload as { name: string; content: string; };
      messages.message(payload.name, payload.content);
    } catch (error) {
      logger.log(`Ошибка принятие message-события: ${error}`, 'error');
    }
  });
}

// Вызывающийся слушатель при загрузке DOMContent.
document.addEventListener('DOMContentLoaded', async () => {
  logger.init();
  logger.log('Клиент запущен', 'info');

  messages.init();

  try {
    logger.log('Инициализация, пожайлуста, подождите...', 'extended');

    initTitleBar();
    
    replenishTriggerRegistry();

    document.querySelectorAll('[global="true"]').forEach(c => globalWrappers.push({ id: c.id, el: c as HTMLDivElement }));
    document.querySelectorAll('[sector="true"]').forEach(c => controlWrappers.push({ id: c.id, el: c as HTMLDivElement }));

    enableParticles();

    initPlugins();

    addOpeningUrlTo('telegram', 'click', 'https://t.me/salarixionion'); 
    addOpeningUrlTo('discord', 'click', 'https://discord.gg/meSaZdARX'); 
    addOpeningUrlTo('github', 'click', 'https://github.com/nullclyze/SalarixiOnion'); 
    addOpeningUrlTo('youtube', 'click', 'https://www.youtube.com/@salarixionion'); 

    configurator.init();

    accountManager.init();
    proxyCollector.init();
    graphic.init();
    radar.init();
    scriptExecutor.init();
    pinger.init();

    await listenEvents();

    await monitoring.init();
    await captchaBypass.init();
    
    await initFunctions();
    await translator.init();

    logger.log('Инициализация прошла успешно', 'extended');

    // await checkUpdate();
  } catch (error) {
    logger.log(`Ошибка инициализации: ${error}`, 'error');
  }
});