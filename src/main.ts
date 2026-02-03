import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { Chart, registerables } from 'chart.js';

import { log, changeLogsVisibility, eraseLogs } from './logger';
import { initConfig, loadConfig } from './modules/config';
import { ProxyManager } from './modules/proxy';
import { ChartManager } from './modules/chart';
import { MonitoringManager } from './modules/monitoring';
import { RadarManager } from './modules/radar';
import { translate, Language } from './modules/translator';
import { changeMessagesVisibility, spawnMessage } from './message';


Chart.register(...registerables);

const client = {
  version: '1.0.7'
};

let process: 'active' | 'sleep' = 'sleep';

const proxyManager = new ProxyManager();
const chartManager = new ChartManager();
const monitoringManager = new MonitoringManager();
const radarManager = new RadarManager();

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

export const pluginList = ['auto-armor', 'auto-totem', 'auto-eat', 'auto-potion', 'auto-look', 'auto-shield', 'auto-repair'];

export const plugins: { [x: string]: { enable: boolean, date: string } } = {
  'auto-armor': {
    enable: false,
    date: '01.02.2026'
  },
  'auto-totem': {
    enable: false,
    date: '01.02.2026'
  },
  'auto-eat': {
    enable: false,
    date: '01.02.2026'
  },
  'auto-potion': {
    enable: false,
    date: '01.02.2026'
  },
  'auto-look': {
    enable: false,
    date: '01.02.2026'
  },
  'auto-shield': {
    enable: false,
    date: '01.02.2026'
  },
  'auto-repair': {
    enable: false,
    date: '01.02.2026'
  }
};

let globalContainers: Array<{ id: string; el: HTMLElement }> = [];
let controlContainers: Array<{ id: string; el: HTMLElement }> = [];

export function updatePluginState(name: string | null, state: boolean) {
  if (name && process === 'sleep') {
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
      toggler.style.color = '#d61818';
      toggler.innerText = 'Выключен';
    }
  }
}

async function startBots(): Promise<void> {
  log('Запуск ботов на сервер...', 'info');

  if (process === 'active') {
    log('Запуск невозможен, существуют активные боты', 'warning');
    spawnMessage('Предупреждение', `Запуск невозможен, существуют активные боты`);
    return;
  }

  process = 'active';

  const address = (document.getElementById('address') as HTMLInputElement).value;
  const version = (document.getElementById('version') as HTMLInputElement).value;
  const botsCount = parseInt((document.getElementById('bots-count') as HTMLInputElement).value);
  const joinDelay = parseFloat((document.getElementById('join-delay') as HTMLInputElement).value);

  const nicknameType = (document.getElementById('nickname-type-select') as HTMLSelectElement).value;
  const passwordType = (document.getElementById('password-type-select') as HTMLSelectElement).value;
  const nicknameTemplate = (document.getElementById('nickname-template') as HTMLInputElement).value;
  const passwordTemplate = (document.getElementById('password-template') as HTMLInputElement).value;

  const registerCommand = (document.getElementById('register-command') as HTMLInputElement).value;
  const registerTemplate = (document.getElementById('register-template') as HTMLInputElement).value;
  const registerMinDelay = parseFloat((document.getElementById('register-min-delay') as HTMLInputElement).value);
  const registerMaxDelay = parseFloat((document.getElementById('register-max-delay') as HTMLInputElement).value);
  const loginCommand = (document.getElementById('login-command') as HTMLInputElement).value;
  const loginTemplate = (document.getElementById('login-template') as HTMLInputElement).value;
  const loginMinDelay = parseFloat((document.getElementById('login-min-delay') as HTMLInputElement).value);
  const loginMaxDelay = parseFloat((document.getElementById('login-max-delay') as HTMLInputElement).value);
  const rejoinDelay = parseInt((document.getElementById('rejoin-delay') as HTMLInputElement).value);
  const chatHistoryLength = parseInt((document.getElementById('chat-history-length') as HTMLInputElement).value);
  const viewDistance = parseInt((document.getElementById('view-distance') as HTMLInputElement).value);
  const language = (document.getElementById('language') as HTMLInputElement).value;
  const chatColors = (document.getElementById('chat-colors') as HTMLInputElement).value;
  const humanoidArm = (document.getElementById('humanoid-arm') as HTMLInputElement).value;

  const useAutoRegister = (document.getElementById('use-auto-register') as HTMLInputElement).checked;
  const useAutoLogin = (document.getElementById('use-auto-login') as HTMLInputElement).checked;
  const useProxy = (document.getElementById('use-proxy') as HTMLInputElement).checked;
  const useAntiCaptcha = (document.getElementById('use-anti-captcha') as HTMLInputElement).checked;
  const useWebhook = (document.getElementById('use-webhook') as HTMLInputElement).checked;
  const useAutoRejoin = (document.getElementById('use-auto-rejoin') as HTMLInputElement).checked;
  const useChatSigning = (document.getElementById('use-chat-signing') as HTMLInputElement).checked;

  const proxyList = (document.getElementById('proxy-list') as HTMLTextAreaElement).value;

  const skinType = (document.getElementById('select-skin-type') as HTMLSelectElement).value;
  const setSkinCommand = (document.getElementById('set-skin-command') as HTMLInputElement).value;
  const customSkinByNickname = (document.getElementById('skin-by-nickname') as HTMLInputElement).value;

  const captchaType = (document.getElementById('select-captcha-type') as HTMLSelectElement).value;

  const antiWebCaptchaOptions: {
    regex: null | string;
    required_url_part: null | string;
  } = { 
    regex: (document.getElementById('anti-web-captcha-regex') as HTMLInputElement).value,
    required_url_part: (document.getElementById('anti-web-captcha-required-url-part') as HTMLInputElement).value
  };

  const webhookOptions: {
    url: null | string;
    information: boolean;
    data: boolean;
    actions: boolean;
  } = { 
    url: (document.getElementById('webhook-url') as HTMLInputElement).value,
    information: (document.getElementById('use-webhook-information') as HTMLInputElement).checked,
    data: (document.getElementById('use-webhook-data') as HTMLInputElement).checked,
    actions: (document.getElementById('use-webhook-actions') as HTMLInputElement).checked
  };

  spawnMessage('Cистема', `Запуск ${botsCount} ботов с версией ${version} на сервер ${address}...`);

  const result = await invoke('launch_bots', { options: {
    address: address || 'localhost',
    version: version || '1.21.11',
    bots_count: botsCount || 1,
    join_delay: joinDelay || 1000,
    nickname_type: nicknameType,
    password_type: passwordType,
    nickname_template: nicknameTemplate || 'player_#m#m',
    password_template: passwordTemplate || '#m#m#l#n',
    register_command: registerCommand || '/reg',
    register_template: registerTemplate || '@cmd @pass',
    register_min_delay: registerMinDelay || 2000,
    register_max_delay: registerMaxDelay || 3500,
    login_command: loginCommand || '/login',
    login_template: loginTemplate || '@cmd @pass',
    login_min_delay: loginMinDelay || 2000,
    login_max_delay: loginMaxDelay || 3500,
    rejoin_delay: rejoinDelay || 3000,
    view_distance: viewDistance || 8,
    language: language || 'en_us',
    chat_colors: chatColors === 'true',
    humanoid_arm: humanoidArm,
    use_auto_register: useAutoRegister,
    use_auto_login: useAutoLogin,
    use_proxy: useProxy,
    use_anti_captcha: useAntiCaptcha,
    use_webhook: useWebhook,
    use_auto_rejoin: useAutoRejoin,
    use_chat_signing: useChatSigning,
    proxy_list: proxyList,
    skin_settings: {
      skin_type: skinType,
      set_skin_command: setSkinCommand,
      custom_skin_by_nickname: customSkinByNickname 
    },
    anti_captcha_settings: {
      captcha_type: captchaType,
      options: {
        web: antiWebCaptchaOptions
      }
    },
    webhook_settings: webhookOptions,
    plugins: {
      auto_armor: plugins['auto-armor'].enable,
      auto_totem: plugins['auto-totem'].enable,
      auto_eat: plugins['auto-eat'].enable,
      auto_potion: plugins['auto-potion'].enable,
      auto_look: plugins['auto-look'].enable,
      auto_shield: plugins['auto-shield'].enable,
      auto_repair: plugins['auto-repair'].enable
    }
  }}) as Array<string>;

  log(String(result[1]), result[0]);

  chartManager.enable();

  monitoringManager.maxChatHistoryLength = chatHistoryLength ? chatHistoryLength : 50;
  monitoringManager.antiCaptchaType = useAntiCaptcha ? captchaType : null;

  monitoringManager.enable();
  monitoringManager.wait();

  radarManager.enable();
}

async function stopBots(): Promise<void> {
  log('Остановка ботов...', 'info');

  try {
    const result = await invoke('stop_bots') as Array<string>;

    log(String(result[1]), result[0]);

    process = 'sleep';

    log('Выключение мониторинга...', 'system');

    chartManager.disable();
    monitoringManager.disable();
    radarManager.disable();

    log('Мониторинг выключен', 'system');
  } catch (error) {
    log(`Ошибка (stop-bots-process): ${error}`, 'error');
  }
}

function changeElementsDisplay(condition: boolean, view: string, show: string[] = [], hide: string[] = []): void {
  hide.forEach(id => {
    const element = document.getElementById(id);
    if (element) element.style.display = condition ? 'none' : view;
  });
  
  show.forEach(id => {
    const element = document.getElementById(id);
    if (element) element.style.display = condition ? view : 'none';
  });
}

class ElementManager {
  private latestControlContainer: HTMLElement | null = null;

  public async init(): Promise<void> {
    const startBotsProcessBtn = document.getElementById('start') as HTMLButtonElement;
    const stopBotsProcessBtn = document.getElementById('stop') as HTMLButtonElement;
    const setRandomValuesBtn =  document.getElementById('random') as HTMLButtonElement;
    const clearInputValuesBtn = document.getElementById('clear') as HTMLButtonElement;

    const panelBtns = document.querySelectorAll('.panel-btn');
    const controlBtns = document.querySelectorAll('.control-btn');
    
    panelBtns.forEach(btn => {
      if (btn.id === 'main') btn.classList.add('selected');

      btn.addEventListener('click', () => {
        this.showGlobalContainer(btn.getAttribute('path') || '');
        panelBtns.forEach(b => b.classList.remove('selected'));
        btn.classList.add('selected');
      });
    });
    
    controlBtns.forEach(btn => {
      if (btn.id === 'control-chat') btn.classList.add('selected');

      btn.addEventListener('click', () => {
        if (btn.getAttribute('path')) {
          this.showControlContainer(btn.getAttribute('path') || '');
          controlBtns.forEach(b => b.classList.remove('selected'));
          btn.classList.add('selected');
          this.latestControlContainer = document.getElementById(btn.getAttribute('path') || '');
        }
      });
    });

    document.querySelectorAll('[plugin-toggler="true"]').forEach(t => t.addEventListener('click', () => {
      const name = t.getAttribute('plugin-name');
      const state = t.getAttribute('state') === 'true';
      updatePluginState(name, state);
    }));  

    document.querySelectorAll('[plugin-open-description="true"]').forEach(e => e.addEventListener('click', () => {
      const path = e.getAttribute('path');
      if (path) {
        const container = document.getElementById(path) as HTMLElement;
        container.style.display = 'flex';
      }
    })); 

    document.querySelectorAll('[plugin-close-description="true"]').forEach(e => e.addEventListener('click', () => {
      const path = e.getAttribute('path');
      if (path) {
        const container = document.getElementById(path) as HTMLElement;
        container.style.display = 'none';
      }
    })); 

    await this.initPluginDescriptions();

    document.querySelectorAll('[control-toggler="true"]').forEach(e => e.addEventListener('click', async () => { 
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

      await this.control((e as any).name, state);
    }));

    (document.getElementById('clear-journal') as HTMLButtonElement).addEventListener('click', () => eraseLogs());

    document.addEventListener('keydown', async (e) => {
      if (process === 'active') {
        const key = e.key.toLowerCase();
    
        for (const k in pressedKeys) {
          key === k ? pressedKeys[key] = true : null;
        }
      
        if (pressedKeys.alt && pressedKeys.i && pressedKeys.c) {
          await invoke('quick_task', { name: 'clear-inventory' });
        } else if (pressedKeys.alt && pressedKeys.f && pressedKeys.c) {
          await invoke('quick_task', { name: 'form-circle' });
        } else if (pressedKeys.alt && pressedKeys.f && pressedKeys.l) {
          await invoke('quick_task', { name: 'form-line' });
        } else if (pressedKeys.alt && pressedKeys.p && pressedKeys.s) {
          await invoke('quick_task', { name: 'pathfinder-stop' });
        } else if (pressedKeys.alt && pressedKeys.g && pressedKeys.f) {
          await invoke('quick_task', { name: 'fly' });
        } else if (pressedKeys.alt && pressedKeys.g && pressedKeys.r) {
          await invoke('quick_task', { name: 'rise' });
        } else if (pressedKeys.alt && pressedKeys.g && pressedKeys.c) {
          await invoke('quick_task', { name: 'capsule' });
        } else if (pressedKeys.alt && pressedKeys.shift && pressedKeys.q) {
          await invoke('quick_task', { name: 'quit' });
        } else if (pressedKeys.alt && pressedKeys.w) {
          await invoke('quick_task', { name: 'move-forward' });
        } else if (pressedKeys.alt && pressedKeys.s) {
          await invoke('quick_task', { name: 'move-backward' });
        } else if (pressedKeys.alt && pressedKeys.a) {
          await invoke('quick_task', { name: 'move-left' });
        } else if (pressedKeys.alt && pressedKeys.d) {
          await invoke('quick_task', { name: 'move-right' });
        } else if (pressedKeys.alt && pressedKeys.j) {
          await invoke('quick_task', { name: 'jump' });
        } else if (pressedKeys.alt && pressedKeys.c) {
          await invoke('quick_task', { name: 'shift' });
        } else if (pressedKeys.alt && pressedKeys.u) {
          await invoke('quick_task', { name: 'unite' });
        } else if (pressedKeys.alt && pressedKeys.t) {
          await invoke('quick_task', { name: 'turn' });
        } else if (pressedKeys.alt && pressedKeys.z) {
          await invoke('quick_task', { name: 'zero' });
        }
      }
    });

    document.addEventListener('keyup', (e) => {
      if (process === 'active') {
        const key = e.key.toLowerCase();
    
        for (const k in pressedKeys) {
          key === k ? pressedKeys[key] = false : null;
        }
      }
    });

    const switcher = (value: boolean) => {
      const version = document.getElementById('version') as HTMLInputElement;

      if (value) {
        version.value = '1.21.11';
        version.disabled = true;
      } else {
        version.disabled = false;
      }
    }

    const useProxyChbx = document.getElementById('use-proxy') as HTMLInputElement;

    useProxyChbx.addEventListener('change', () => switcher(useProxyChbx.checked));

    switcher(useProxyChbx.checked);

    document.getElementById('select-captcha-type')?.addEventListener('change', function (this: HTMLSelectElement) {
      const antiWebCaptchaOptionsContainer = document.getElementById('anti-web-captcha-options') as HTMLElement;

      if (this.value === 'web') {
        antiWebCaptchaOptionsContainer.style.display = 'flex';
      } else if (this.value === 'map') {
        antiWebCaptchaOptionsContainer.style.display = 'none';
      } else if (this.value === 'frame') {
        antiWebCaptchaOptionsContainer.style.display = 'none';
      }
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

      const setRandomValue = (e: string) => {
        let current = document.getElementById(e) as HTMLInputElement; 

        switch (e) {
          case 'version':
            const randomVersion = String(versions[Math.floor(Math.random() * versions.length)]);
            current.value = (document.getElementById('use-proxy') as HTMLInputElement).checked ? '1.21.11' : randomVersion; break;
          case 'bots-count':
            const randomQuantity = Math.floor(Math.random() * (50 - 10 + 1) + 10);
            current.valueAsNumber = randomQuantity; break;
          case 'join-delay':
            const randomDelay = Math.floor(Math.random() * (7000 - 1000 + 1) + 1000);
            current.valueAsNumber = randomDelay; break;
        }
      }

      setRandomValue('version');
      setRandomValue('bots-count');
      setRandomValue('join-delay');
    });

    clearInputValuesBtn.addEventListener('click', () => {
      (document.getElementById('address') as HTMLInputElement).value = '';
      (document.getElementById('version') as HTMLInputElement).value = '';
      (document.getElementById('bots-count') as HTMLInputElement).value = '';
      (document.getElementById('join-delay') as HTMLInputElement).value = '';
    });

    document.getElementById('nickname-type-select')?.addEventListener('change', function (this: HTMLSelectElement) {
      if (this.value === 'custom') {
        (document.getElementById('custom-nickname-template-container') as HTMLInputElement).style.display = 'flex';
      } else {
        (document.getElementById('custom-nickname-template-container') as HTMLInputElement).style.display = 'none';
      }
    });

    document.getElementById('password-type-select')?.addEventListener('change', function (this: HTMLSelectElement) {
      if (this.value === 'custom') {
        (document.getElementById('custom-password-template-container') as HTMLInputElement).style.display = 'flex';
      } else {
        (document.getElementById('custom-password-template-container') as HTMLInputElement).style.display = 'none';
      }
    });

    document.getElementById('chat-mode')?.addEventListener('change', function (this: HTMLSelectElement) {
      const chatSpammingChbxContainer = document.getElementById('chat-spamming-chbx-container') as HTMLElement;
      const chatSpammingInputContainer = document.getElementById('chat-spamming-input-container') as HTMLElement;
            
      const chatDefaultBtnsContainer = document.getElementById('chat-default-btns-container') as HTMLElement;
      const chatSpammingBtnsContainer = document.getElementById('chat-spamming-btns-container') as HTMLElement;

      if (this.value === 'spamming') {
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

    document.getElementById('select-inventory-mode')?.addEventListener('change', function (this: HTMLSelectElement) {
      const basicBtnContainer = document.getElementById('inventory-basic-btn-container') as HTMLElement;

      const swapInputContainer = document.getElementById('inventory-swap-input-container') as HTMLElement;
      const swapBtnContainer = document.getElementById('inventory-swap-btn-container') as HTMLElement;

      if (this.value === 'basic') {
        basicBtnContainer.style.display = 'flex';

        swapInputContainer.style.display = 'none';
        swapBtnContainer.style.display = 'none';
      } else if (this.value === 'swap') {
        swapInputContainer.style.display = 'flex';
        swapBtnContainer.style.display = 'flex';

        basicBtnContainer.style.display = 'none';
      } 
    });

    document.getElementById('select-movement-mode')?.addEventListener('change', function (this: HTMLSelectElement) {
      const selectMovementDirectionContainer = document.getElementById('select-movement-direction-container') as HTMLElement;
      const movementPathfinderInputContainer = document.getElementById('movement-pathfinder-input-container') as HTMLElement;
      const movementImpulsivenessContainer = document.getElementById('movement-impulsiveness-container') as HTMLElement;

      if (this.value === 'default') {
        selectMovementDirectionContainer.style.display = 'flex';
        movementPathfinderInputContainer.style.display = 'none';
        movementImpulsivenessContainer.style.display = 'flex';
      } else {
        selectMovementDirectionContainer.style.display = 'none';
        movementPathfinderInputContainer.style.display = 'flex';
        movementImpulsivenessContainer.style.display = 'none';
      }
    });

    document.getElementById('select-flight-settings')?.addEventListener('change', function (this: HTMLSelectElement) {
      const manualSettingsContainer = document.getElementById('flight-manual-settings-container') as HTMLElement;
      const selectAntiCheatContainer = document.getElementById('flight-select-anti-cheat-container') as HTMLElement;

      if (this.value === 'adaptive') {
        manualSettingsContainer.style.display = 'none';
        selectAntiCheatContainer.style.display = 'grid';
      } else {
        manualSettingsContainer.style.display = 'flex';
        selectAntiCheatContainer.style.display = 'none';
      }
    });

    document.getElementById('select-killaura-settings')?.addEventListener('change', function (this: HTMLSelectElement) {
      const manualSettingsContainer = document.getElementById('killaura-manual-settings-container') as HTMLElement;

      if (this.value === 'adaptive') {
        manualSettingsContainer.style.display = 'none';
      } else {
        manualSettingsContainer.style.display = 'flex';
      }
    });

    document.getElementById('use-killaura-auto-weapon')?.addEventListener('change', function (this: HTMLInputElement) {
      const selectWeaponContainer = document.getElementById('select-killaura-weapon-container') as HTMLElement;
      const weaponSlotContainer = document.getElementById('killaura-weapon-slot-container') as HTMLElement;

      if (this.checked) {
        selectWeaponContainer.style.display = 'grid';
        weaponSlotContainer.style.display = 'none';
      } else {
        selectWeaponContainer.style.display = 'none';
        weaponSlotContainer.style.display = 'flex';
      }
    });


    document.getElementById('use-killaura-chase')?.addEventListener('input', function (this: HTMLInputElement) {
      const chaseSettingsContainer = document.getElementById('killaura-chase-settings-container') as HTMLElement;

      if (this.checked) {
        chaseSettingsContainer.style.display = 'flex';
      } else {
        chaseSettingsContainer.style.display = 'none';
      }
    });

    document.getElementById('select-bow-aim-target')?.addEventListener('change', function (this: HTMLSelectElement) {
      const customGoalInputContainer = document.getElementById('bow-aim-custom-goal-input-container') as HTMLElement;

      if (this.value === 'custom-goal') {
        customGoalInputContainer.style.display = 'flex';
      } else {
        customGoalInputContainer.style.display = 'none';
      }
    });

    document.getElementById('miner-mode-select')?.addEventListener('change', function (this: HTMLSelectElement) {
      const tunnelSelectContainer = document.getElementById('miner-tunnel-select-container') as HTMLElement;
      const lookSelectContainer = document.getElementById('miner-look-select-container') as HTMLElement;

      if (this.value === 'default') {
        tunnelSelectContainer.style.display = 'none';
        lookSelectContainer.style.display = 'none';
      } else {
        tunnelSelectContainer.style.display = 'grid';
        lookSelectContainer.style.display = 'grid';
      }
    });

    (document.getElementById('use-auto-register') as HTMLInputElement).addEventListener('change', function () { changeElementsDisplay((this as HTMLInputElement).checked, 'flex', ['auto-register-input-container']); });
    (document.getElementById('use-auto-login') as HTMLInputElement).addEventListener('change', function () { changeElementsDisplay((this as HTMLInputElement).checked, 'flex', ['auto-login-input-container']); });
    (document.getElementById('use-auto-rejoin') as HTMLInputElement).addEventListener('change', function () { changeElementsDisplay((this as HTMLInputElement).checked, 'flex', ['auto-rejoin-input-container']); });

    changeElementsDisplay((document.getElementById('use-auto-register') as HTMLInputElement).checked, 'flex', ['auto-register-input-container']);
    changeElementsDisplay((document.getElementById('use-auto-login') as HTMLInputElement).checked, 'flex', ['auto-login-input-container']);
    changeElementsDisplay((document.getElementById('use-auto-rejoin') as HTMLInputElement).checked, 'flex', ['auto-rejoin-input-container']);

    document.getElementById('show-system-log')?.addEventListener('change', function (this: HTMLInputElement) { 
      changeLogsVisibility('system', this.checked);
    });

    document.getElementById('show-extended-log')?.addEventListener('change', function (this: HTMLInputElement) {
      changeLogsVisibility('extended', this.checked);
    });

    document.getElementById('proxy-finder-algorithm')?.addEventListener('change', function (this: HTMLSelectElement) {
      const selectProxyFinderCountry = document.getElementById('proxy-finder-country') as HTMLSelectElement;

      if (this.value === 'apic') {
        selectProxyFinderCountry.disabled = false;
      } else {
        selectProxyFinderCountry.disabled = true;
      }
    });

    document.getElementById('select-skin-type')?.addEventListener('change', function (this: HTMLSelectElement) {
      const setSkinCommandContainer = document.getElementById('set-skin-command-container') as HTMLElement;
      const customSkinContainer = document.getElementById('custom-skin-container') as HTMLElement;

      if (this.value === 'default') {
        setSkinCommandContainer.style.display = 'none';
        customSkinContainer.style.display = 'none';
      } else if (this.value === 'random') {
        setSkinCommandContainer.style.display = 'flex';
        customSkinContainer.style.display = 'none';
      } else if (this.value === 'custom') {
        setSkinCommandContainer.style.display = 'flex';
        customSkinContainer.style.display = 'flex';
      }
    });

    document.getElementById('interface-theme')?.addEventListener('change', () => {
      this.setInterfaceTheme((document.getElementById('interface-theme') as HTMLSelectElement).value);
    });

    document.getElementById('interface-show-messages')?.addEventListener('change', () => {
      this.setInterfaceShowMessages((document.getElementById('interface-show-messages') as HTMLSelectElement).value);
    });

    document.getElementById('interface-global-font-family')?.addEventListener('change', () => {
      this.setInterfaceGlobalFontFamily((document.getElementById('interface-global-font-family') as HTMLSelectElement).value);
    });

    document.getElementById('interface-show-panel-icons')?.addEventListener('change', () => {
      this.setInterfaceShowPanelIcons((document.getElementById('interface-show-panel-icons') as HTMLSelectElement).value);
    });

    document.getElementById('interface-panel-font-family')?.addEventListener('change', () => {
      this.setInterfacePanelFontFamily((document.getElementById('interface-panel-font-family') as HTMLSelectElement).value);
    });

    document.getElementById('interface-panel-font-size')?.addEventListener('change', () => {
      this.setInterfacePanelFontSize((document.getElementById('interface-panel-font-size') as HTMLSelectElement).value);
    });

    document.getElementById('interface-panel-internal-gap')?.addEventListener('change', () => {
      this.setInterfacePanelInternalGap((document.getElementById('interface-panel-internal-gap') as HTMLSelectElement).value);
    });

    document.getElementById('interface-client-language')?.addEventListener('change', async () => {
      await translate((document.getElementById('interface-client-language') as HTMLSelectElement).value as Language);
    });

    this.setInterfaceTheme((document.getElementById('interface-theme') as HTMLSelectElement).value);
    this.setInterfaceGlobalFontFamily((document.getElementById('interface-global-font-family') as HTMLSelectElement).value);
    this.setInterfaceShowMessages((document.getElementById('interface-show-messages') as HTMLSelectElement).value);
    this.setInterfaceShowPanelIcons((document.getElementById('interface-show-panel-icons') as HTMLSelectElement).value);
    this.setInterfacePanelFontFamily((document.getElementById('interface-panel-font-family') as HTMLSelectElement).value);
    this.setInterfacePanelFontSize((document.getElementById('interface-panel-font-size') as HTMLSelectElement).value);
    this.setInterfacePanelInternalGap((document.getElementById('interface-panel-internal-gap') as HTMLSelectElement).value);
    await translate((document.getElementById('interface-client-language') as HTMLSelectElement).value as Language);
  } 

  private setInterfaceTheme(theme: string): void {
    try {
      const root = document.documentElement;

      switch (theme) {
        case 'onion': 
          root.style.setProperty('--title-color', '#7e47ff');
          root.style.setProperty('--spec-color', '#6523ff');
          root.style.setProperty('--dull-spec-color', '#6a39dd');
          root.style.setProperty('--chbx-spec-color', '#6a3fce');
          root.style.setProperty('--chbx-dull-spec-color', '#5426be');
          break;
        case 'toxic': 
          root.style.setProperty('--title-color', '#3ec71c');
          root.style.setProperty('--spec-color', '#40cf33');
          root.style.setProperty('--dull-spec-color', '#51be26');
          root.style.setProperty('--chbx-spec-color', '#25a115');
          root.style.setProperty('--chbx-dull-spec-color', '#1b7210');
          break;
        case 'ice': 
          root.style.setProperty('--title-color', '#1c96c7');
          root.style.setProperty('--spec-color', '#46b2f0');
          root.style.setProperty('--dull-spec-color', '#26b9be');
          root.style.setProperty('--chbx-spec-color', '#26a9e6');
          root.style.setProperty('--chbx-dull-spec-color', '#0d7696');
          break;
        case 'blood':
          root.style.setProperty('--title-color', '#e42525');
          root.style.setProperty('--spec-color', '#f03333');
          root.style.setProperty('--dull-spec-color', '#d62727');
          root.style.setProperty('--chbx-spec-color', '#ce2323');
          root.style.setProperty('--chbx-dull-spec-color', '#991717');
          break;
        case 'gold': 
          root.style.setProperty('--title-color', '#c4c71c');
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
          root.style.setProperty('--title-color', '#9f23e7');
          root.style.setProperty('--spec-color', '#9215e6');
          root.style.setProperty('--dull-spec-color', '#8e13be');
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
      }
    } catch (error) {
      log(`Ошибка изменения темы: ${error}`, 'error');
    }
  }

  private setInterfaceGlobalFontFamily(fontFamily: string): void {
    try {
      const root = document.documentElement;

      switch (fontFamily) {
        case 'inter':
          root.style.setProperty('--global-font-family', `'Inter', sans-serif`);
          break;
        case 'jetbrains-mono':
          root.style.setProperty('--global-font-family', `'Segoe UI', Tahoma, Geneva, Verdana, sans-serif`);
          break;
        case 'segoe-ui':
          root.style.setProperty('--global-font-family', `'Gill Sans', 'Gill Sans MT', 'Trebuchet MS', sans-serif`);
          break;
      }
    } catch (error) {
      log(`Ошибка изменения шрифта: ${error}`, 'error');
    }
  }

  private setInterfaceShowMessages(state: string): void {
    try {
      changeMessagesVisibility(state);

      document.querySelectorAll<HTMLElement>('.message').forEach(m => {
        if (state === 'hide') {
          m.style.display = 'none';
        } else {
          m.style.display = 'flex';
        }
      });
    } catch (error) {
      log(`Ошибка изменения состояния сообщений: ${error}`, 'error');
    }
  }

  private setInterfaceShowPanelIcons(state: string): void {
    try {
      document.querySelectorAll<SVGElement>('[panel-btn-icon="true"]').forEach(i => {
        if (state === 'hide') {
          i.style.display = 'none';
        } else {
          i.style.display = 'flex';
        }
      });
    } catch (error) {
      log(`Ошибка изменения состояния иконок на панели: ${error}`, 'error');
    }
  }

  private setInterfacePanelFontFamily(fontFamily: string): void {
    try {
      const root = document.documentElement;

      switch (fontFamily) {
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
      log(`Ошибка изменения шрифта панели: ${error}`, 'error');
    }
  }

  private setInterfacePanelFontSize(size: string): void {
    try {
      const root = document.documentElement;
      root.style.setProperty('--panel-font-size', size);
    } catch (error) {
      log(`Ошибка изменения размера шрифта панели: ${error}`, 'error');
    }
  }

  private setInterfacePanelInternalGap(size: string): void {
    try {
      const root = document.documentElement;
      root.style.setProperty('--panel-btn-gap', size);
    } catch (error) {
      log(`Ошибка изменения внутреннего отступа кнопок панели: ${error}`, 'error');
    }
  }

  private async initPluginDescriptions(): Promise<void> {
    try {
      const response = await fetch('https://raw.githubusercontent.com/nullclyze/SalarixiOnion/refs/heads/main/salarixi.plugins.json');

      if (response.ok) {
        const data = await response.json();

        const list = data['list'];

        for (const plugin of list) {
          const name = plugin['name'];
          const description = plugin['description'];
          const latestUpdate = plugin['latest-update'];

          if (plugins[name]?.date) {
            if (latestUpdate != plugins[name].date) {
              document.getElementById(`${name}-plugin`)?.classList.add('deprecated');
              
              const tag = document.createElement('span');
              tag.className = 'tag';
              tag.innerText = 'Deprecated';

              document.getElementById(`${name}-name`)?.appendChild(tag);
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
                  <div class="name">${plugin['header']} <span class="tag">Unavailable</span></div>
                  <div class="meta">Статус: <span class="status">Недоступен</span></div>
                </div>
              </div>
            `;

            document.getElementById('plugin-list')?.appendChild(pluginCard);
          }
        }
      }
    } catch (error) {
      log(`Ошибка initPluginDescriptions: ${error}`, 'error');
    }
  }

  private showGlobalContainer(id: string): void {
    globalContainers.forEach(c => c && c.id === id ? c.el.style.display = 'flex' : c.el.style.display = 'none');
    controlContainers.forEach(c => c.el.style.display = 'none');

    if (id === 'control-container') {
      this.latestControlContainer ? this.latestControlContainer.style.display = 'flex' : (document.getElementById('control-chat-container') as HTMLElement).style.display = 'flex';
    }
  }

  private showControlContainer(id: string): void {
    controlContainers.forEach(c => c && c.id === id ? c.el.style.display = 'flex' : c.el.style.display = 'none');
  }

  private async control(name: string, state: boolean | string): Promise<void> {
    try {
      const elements = document.querySelectorAll(`[control="${name}"]`);

      let options: Record<string, any> = {};

      elements.forEach(e => {
        if (e.tagName.toLowerCase() === 'select') {
          options[(e as HTMLSelectElement).name] = (e as HTMLSelectElement).value;
        } else if (e.tagName.toLowerCase() === 'input') {
          if ((e as HTMLInputElement).type === 'checkbox') {
            options[(e as HTMLInputElement).name] = (e as HTMLInputElement).checked;
          } else {
            options[(e as HTMLInputElement).name] = (e as HTMLInputElement).type === 'number' ? Number((e as HTMLInputElement).value) : (e as HTMLInputElement).value;
          }
        }
      });

      const group = (document.getElementById('control-group') as HTMLInputElement).value.replace(' ', '');

      await invoke('control', {
        name: name,
        options: {
          ...options,
          state: state
        },
        group: group !== '' ? group : 'global'
      });
    } catch (error) {
      log(`Ошибка управления (${name}): ${error}`, 'error');
    }
  }
}

function addOpeningUrlTo(id: string, event: string, url: string): void {
  const el = document.getElementById(id);

  if (el) {
    el.addEventListener(event, async () => {
      try {
        await invoke('open_url', { url: url });
      } catch (error) {
        log(`Ошибка открытия URL: ${error}`, 'error');
      }
    });
  }
}

async function checkUpdate(): Promise<void> {
  try {
    document.getElementById('close-notice-btn')?.addEventListener('click', () => {
      const notice = document.getElementById('notice');

      if (notice) {
        notice.style.display = 'none';
      }
    });

    const notice = document.getElementById('notice') as HTMLElement;
    const newVersion = document.getElementById('new-client-version') as HTMLElement;
    const newTag = document.getElementById('new-client-tag') as HTMLElement;
    const newReleaseDate = document.getElementById('new-client-release-date') as HTMLElement;

    const response = await fetch('https://raw.githubusercontent.com/nullclyze/SalarixiOnion/refs/heads/main/salarixi.version.json', { method: 'GET' });

    if (!response.ok) return;

    const data = await response.json();

    if (data && data.version !== client.version) {
      const tag = `v${data.version}-${String(data.type).toLowerCase()}`;

      newVersion.innerText = data.version;
      newTag.innerText = tag;
      newReleaseDate.innerText = data.releaseDate;
      
      addOpeningUrlTo('open-client-release', 'click', `https://github.com/nullclyze/SalarixiOnion/releases/tag/${tag}`); 
      
      setTimeout(() => {
        notice.style.display = 'flex';
      }, 4000);
    }
  } catch (error) {
    log(`Ошибка проверки обновлений: ${error}`, 'error');
  }
}

document.addEventListener('DOMContentLoaded', async () => {
  log('Клиент запущен', 'info');

  try {
    log('Инициализация...', 'extended');

    const elementManager = new ElementManager();

    await listen('log', (event) => {
      try {
        const payload = event.payload as { name: string; message: string; };
        const name = payload.name;
        const message = payload.message;

        log(message, name);
      } catch (error) {
        log(`Ошибка принятие log-события: ${error}`, 'error');
      }
    });

    await listen('message', (event) => {
      try {
        const payload = event.payload as { name: string; content: string; };
        const name = payload.name;
        const content = payload.content;

        spawnMessage(name, content);
      } catch (error) {
        log(`Ошибка принятие message-события: ${error}`, 'error');
      }
    });

    (document.getElementById('title-version') as HTMLElement).innerText = `v${client.version}`;

    (document.getElementById('window-minimize') as HTMLButtonElement).addEventListener('click', async () => await getCurrentWindow().minimize());
    (document.getElementById('window-close') as HTMLButtonElement).addEventListener('click', async () => await invoke('exit'));

    document.querySelectorAll('[global="true"]').forEach(c => globalContainers.push({ id: c.id, el: c as HTMLElement }));
    document.querySelectorAll('[sector="true"]').forEach(c => controlContainers.push({ id: c.id, el: c as HTMLElement }));

    addOpeningUrlTo('telegram', 'click', 'https://t.me/salarixionion'); 
    addOpeningUrlTo('github', 'click', 'https://github.com/nullclyze/SalarixiOnion'); 
    addOpeningUrlTo('youtube', 'click', 'https://www.youtube.com/@salarixionion'); 

    initConfig();
    loadConfig();

    proxyManager.init();
    chartManager.init();
    radarManager.init();

    await elementManager.init();
    await monitoringManager.init();

    log('Инициализация прошла успешно', 'extended');

    //await checkUpdate();
  } catch (error) {
    log(`Ошибка инициализации: ${error}`, 'error');
  }
});