import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { isAbsolute } from '@tauri-apps/api/path';
import { Chart, registerables } from 'chart.js';

import { collect } from './collector';


Chart.register(...registerables);


interface Statistics {
  isInitialized: boolean;
  logIndex: number;
  showSystemLogs: boolean;
  showExtendedLogs: boolean;
  latestConfig: Record<string, any>;
  usernameList: string[];
  latestControlContainer: HTMLElement | null;
  listeners: Map<string, { event: string; listener: EventListener }>;
}

interface Activity {
  botting: boolean;
  profileDataMonitoring: boolean;
  chatHistoryMonitoring: boolean;
}

interface LogEventPayload {
  name: string;
  message: string;
}

interface ChatEventPayload {
  receiver: string;
  message: string;
}

interface BotProfile {
  status: string;
  nickname: string;
  version: string;
  password: string;
  proxy: string;
  captcha_url: string | null;
  registered: boolean;
}

interface RadarInfo {
  status: string;
  uuid: string;
  x: number;
  y: number;
  z: number;
  observer: {
    x: number;
    z: number;
  };
}

const client = {
  version: '1.0.4'
};

let statistics: Statistics = {
  isInitialized: false,
  logIndex: 0,
  showSystemLogs: true,
  showExtendedLogs: false,
  latestConfig: {},
  usernameList: [],
  latestControlContainer: null,
  listeners: new Map()
};

let activity: Activity = {
  botting: false,
  profileDataMonitoring: false,
  chatHistoryMonitoring: false
};

const pressedKeys: { [key: string]: boolean } = {
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

let globalContainers: Array<{ id: string; el: HTMLElement }> = [];
let controlContainers: Array<{ id: string; el: HTMLElement }> = [];

function date(format: string = 'H:M:S'): string {
  const date = new Date();

  const hours = date.getHours().toString().padStart(2, '0');
  const minutes = date.getMinutes().toString().padStart(2, '0');
  const seconds = date.getSeconds().toString().padStart(2, '0');

  if (format === 'H:M:S') {
    return `${hours}:${minutes}:${seconds}`;
  } else if (format === 'H:M') {
    return `${hours}:${minutes}`;
  } else if (format === 'M:S') {
    return `${minutes}:${seconds}`;
  } else {
    return `${hours}:${minutes}:${seconds}`;
  }
}

function log(text: string, type: string): void {
  const logContent = document.getElementById('log-content') as HTMLElement;

  if (!logContent) return;

  if (statistics.logIndex >= 400) {
    statistics.logIndex = 399;
    logContent.firstChild?.remove();
  }

  statistics.logIndex++;

  const container = document.createElement('div');

  container.className = 'log-line';

  if (type === 'log-system') {
    container.className += ' log-line-system';
  }

  if (type === 'log-extended') {
    container.className += ' log-line-extended';
  }

  text = text
    .replace(/%hcg/g, '<span style="color: #21d618ba;">')
    .replace(/%hcy/g, '<span style="color: #d6d018b6;">')
    .replace(/%hcr/g, '<span style="color: #d61b1893;">')
    .replace(/%sc/g, '</span>');

  container.innerHTML = `
    <div class="log-line-date">${date()}</div>
    <div class="log-line-content ${type}">${text}</div>
  `;

  if ((container.className.includes('log-line-system') && !statistics.showSystemLogs) || (container.className.includes('log-line-extended') && !statistics.showExtendedLogs)) {
    container.style.display = 'none';
  }

  logContent.appendChild(container);

  if ((document.getElementById('auto-scroll-log') as HTMLInputElement).checked) {
    logContent.scrollTo({
      top: logContent.scrollHeight,
      behavior: 'smooth'
    });
  }
}

async function clear(): Promise<void> {
  statistics.usernameList = [];
  listenerManager.cleanup();
}

async function startBots(): Promise<void> {
  log('Запуск ботов на сервер...', 'log-info');

  activity.botting = true;

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
  const useAutoRejoin = (document.getElementById('use-auto-rejoin') as HTMLInputElement).checked;
  const useAutoLogin = (document.getElementById('use-auto-login') as HTMLInputElement).checked;
  const useProxy = (document.getElementById('use-proxy') as HTMLInputElement).checked;

  const proxyList = (document.getElementById('proxy-list') as HTMLTextAreaElement).value;

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
    humanoid_arm: humanoidArm || 'right',
    use_auto_register: useAutoRegister,
    use_auto_rejoin: useAutoRejoin,
    use_auto_login: useAutoLogin,
    use_proxy: useProxy,
    proxy_list: proxyList
  }}) as Array<string>;

  log(String(result[1]), `log-${result[0]}`);

  log('Включение мониторинга...', 'log-system');

  graphicManager.enableGraphics();

  monitoringManager.maxChatHistoryLength = chatHistoryLength ? chatHistoryLength : 50;

  await monitoringManager.enableMonitoring();
  monitoringManager.wait();

  log('Мониторинг включён', 'log-system');
}

async function stopBots(): Promise<void> {
  log('Остановка ботов...', 'log-info');

  try {
    const result = await invoke('stop_bots') as Array<string>;

    log(String(result[1]), `log-${result[0]}`);

    activity.botting = false;

    log('Выключение мониторинга...', 'log-system');

    graphicManager.disableGraphics();
    monitoringManager.clear();

    log('Мониторинг выключен', 'log-system');
  } catch (error) {
    log(`Ошибка (stop-bots-process): ${error}`, 'log-error');
  }
}

class ListenerManager {
  public add(id: string, event: string, listener: EventListener): void {
    document.getElementById(id)?.addEventListener(event, listener);
    statistics.listeners.set(id, { event: event, listener: listener });
  }

  public cleanup(): void {
    for (const [id, data] of statistics.listeners) {
      document.getElementById(id)?.removeEventListener(data.event, data.listener);
    }

    statistics.listeners.clear();
  }
}

class FunctionManager {
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
          statistics.latestControlContainer = document.getElementById(btn.getAttribute('path') || '');
        }
      });
    });

    document.querySelectorAll('[toggler="true"]').forEach(e => e.addEventListener('click', async () => { 
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

    (document.getElementById('clear-journal') as HTMLButtonElement).addEventListener('click', () => {
      const logs = document.querySelectorAll('.log-line');
      logs.forEach(e => e.remove());
      statistics.logIndex = 0;
    });

    document.addEventListener('keydown', async (e) => {
      if (activity.botting) {
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
      if (activity.botting) {
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
  }

  private showGlobalContainer(id: string): void {
    globalContainers.forEach(c => c && c.id === id ? c.el.style.display = 'flex' : c.el.style.display = 'none');
    controlContainers.forEach(c => c.el.style.display = 'none');

    if (id === 'control-container') {
      statistics.latestControlContainer ? statistics.latestControlContainer.style.display = 'flex' : (document.getElementById('control-chat-container') as HTMLElement).style.display = 'flex';
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

      await invoke('control', {
        name: name,
        options: {
          ...options,
          state: state
        }
      });
    } catch (error) {
      log(`Ошибка управления (${name}): ${error}`, 'log-error');
    }
  }
}

class GraphicManager {
  private statusText: HTMLElement | null = null;

  private graphicActiveBots: HTMLElement | null = null;
  private graphicMemoryUsage: HTMLElement | null = null;

  private chartActiveBots: any = null;
  private chartMemoryUsage: any = null;

  private intervals: { activeBots: any, memoryUsage: any } = { activeBots: null, memoryUsage: null };

  public async init(): Promise<void> {
    this.graphicActiveBots = document.getElementById('graphic-active-bots-container');
    this.graphicMemoryUsage = document.getElementById('graphic-memory-usage-container');

    this.statusText = document.getElementById('graphic-status-text');
  }

  public enableGraphics(): void {
    try {
      this.createGraphicActiveBots();
      this.createGraphicMemoryUsage();

      this.graphicActiveBots!.style.display = 'flex';
      this.graphicMemoryUsage!.style.display = 'flex';

      this.statusText!.style.display = 'none';

      this.intervals.activeBots = setInterval(async () => {
        if (!activity.botting) {
          clearInterval(this.intervals.activeBots);
          return;
        }

        try {
          const data = await invoke('get_active_bots_count') as number;
          this.addGraphicDataActiveBots(data || 0);
        } catch (error) {
          log(`Ошибка графика активных ботов: ${error}`, 'log-error');
        }
      }, 1800);

      this.intervals.memoryUsage = setInterval(async () => {
        if (!activity.botting) {
          clearInterval(this.intervals.memoryUsage);
          return;
        }

        try {
          const data = await invoke('get_memory_usage') as number;
          this.addGraphicDataMemoryUsage(parseFloat(data.toFixed(3)) || 0);
        } catch (error) {
          log(`Ошибка графика используемой памяти: ${error}`, 'log-error');
        }
      }, 1800);
    } catch (error) {
      log(`Ошибка инициализации графика активных ботов: ${error}`, 'log-error');
    }
  }

  public disableGraphics(): void {
    if (this.chartActiveBots) {
      this.chartActiveBots.destroy();
      this.chartActiveBots = null;
    }

    if (this.chartMemoryUsage) {
      this.chartMemoryUsage.destroy();
      this.chartMemoryUsage = null;
    }
    
    if (this.intervals.activeBots) {
      clearInterval(this.intervals.activeBots);
      this.intervals.activeBots = null;
    }

    if (this.intervals.memoryUsage) {
      clearInterval(this.intervals.memoryUsage);
      this.intervals.memoryUsage = null;
    }

    this.graphicActiveBots!.style.display = 'none';
    this.graphicMemoryUsage!.style.display = 'none';

    this.statusText!.innerText = 'Данные отсутствуют';
    this.statusText!.style.display = 'flex';
  }

  private createGraphic(context: CanvasRenderingContext2D, label: string, title: string, maxY: number, tag: string): any {
    const initialLabels: string[] = [];
    const initialData: number[] = [];
    
    for (let i = 0; i < 31; i++) {
      initialLabels.push(date());
      initialData.push(0);
    }

    const chart = new Chart(context, {
      type: 'line',
      data: {
        labels: initialLabels,
        datasets: [
          {
            label: ` ${label}`,
            data: initialData,
            fill: true,
            borderWidth: 2,
            borderColor: '#6ff34ef1',
            backgroundColor: '#83ff3b23',
            tension: 0.1,
            pointStyle: 'line',
            pointRadius: 2
          }
        ]
      },
      options: {
        responsive: true,
        animation: { duration: 400 },
        scales: {
          x: {
            ticks: { color: '#858585ff' },
            border: { color: '#383838ab' },
            grid: { color: '#383838ab' }
          },
          y: {
            min: 0,
            max: maxY, 
            ticks: { 
              callback: (value) => { return tag === 'memory-usage' ? value + ' MB' : value },
              color: '#a3a3a3ff' 
            },
            border: { color: '#383838ab' },
            grid: { color: '#383838ab' }
          }
        },
        plugins: {
          title: {
            text: title,
            display: true,
            color: '#a2a2a2ff'
          },
          legend: {
            display: false,
            position: 'top',
            labels: {
              color: '#979797ff',
              font: { size: 12 },
              usePointStyle: true
            }
          },
          tooltip: {
            mode: 'index',
            intersect: false,
            backgroundColor: 'rgba(10, 10, 10, 0.8)',
            titleColor: '#ffffff',
            bodyColor: '#ffffff'
          }
        }
      }
    });

    return chart;
  }
  
  private createGraphicActiveBots(): void {
    const context = (document.getElementById('graphic-active-bots') as HTMLCanvasElement).getContext('2d');
    if (!context) return;
    this.chartActiveBots = this.createGraphic(context, 'Активные боты', 'График активных ботов', 500, 'active-bots');
  }

  private createGraphicMemoryUsage(): void {
    const context = (document.getElementById('graphic-memory-usage') as HTMLCanvasElement).getContext('2d');
    if (!context) return;
    this.chartMemoryUsage = this.createGraphic(context, 'Используемая память', 'График используемой памяти', 1024, 'memory-usage');
  }

  private addGraphicDataActiveBots(activeBotsQuantity: number): void {
    this.chartActiveBots.data.labels?.push(date());
    this.chartActiveBots.data.datasets[0].data.push(activeBotsQuantity);

    if (this.chartActiveBots.data.labels && this.chartActiveBots.data.labels.length > 31) {
      this.chartActiveBots.data.labels.shift();
      this.chartActiveBots.data.datasets[0].data.shift();
    }
      
    this.chartActiveBots.update(); 
  }

  private addGraphicDataMemoryUsage(memoryUsage: number): void {
    this.chartMemoryUsage.data.labels?.push(date());
    this.chartMemoryUsage.data.datasets[0].data.push(memoryUsage);

    if (this.chartMemoryUsage.data.labels && this.chartMemoryUsage.data.labels.length > 31) {
      this.chartMemoryUsage.data.labels.shift();
      this.chartMemoryUsage.data.datasets[0].data.shift();
    }
      
    this.chartMemoryUsage.update(); 
  }
}

class MonitoringManager {
  private statusText: HTMLElement | null = null;
  private botCardsContainer: HTMLElement | null = null;

  public maxChatHistoryLength: number | null = null;
  private chatMessageCounter: Record<string, number> = {};
  private chatHistoryFilters: Record<string, string> = {};

  public async init(): Promise<void> {
    this.statusText = document.getElementById('monitoring-status-text');
    this.botCardsContainer = document.getElementById('bot-cards-container');

    this.statusText!.innerText = 'Объекты ботов отсутствуют';
    this.statusText!.style.color = '#646464f7';
    this.statusText!.style.display = 'flex';

    await listen('chat', (event) => {
      try {
        const payload = event.payload as ChatEventPayload;
        const receiver = payload.receiver;
        const message = payload.message;

        if (!this.chatHistoryFilters[receiver]) {
          this.chatHistoryFilters[receiver] = 'all';
        }

        if (!this.filterMessage(this.chatHistoryFilters[receiver], String(message))) return;

        const chat = document.getElementById(`monitoring-chat-content-${receiver}`);

        if (!chat) return;

        const container = document.createElement('div');

        container.className = 'monitoring-line';
        container.id = `monitoring-message-${receiver}`;

        container.innerHTML = `
          <div class="monitoring-line-time">${date()}</div>
          <div class="monitoring-line-content">${String(message).replace('%hb', '<span class="bot-tag">').replace('%sc', '</span>')}</div>
        `;

        chat.appendChild(container);

        if (!this.chatMessageCounter[receiver]) {
          this.chatMessageCounter[receiver] = 1;
        } else {
          this.chatMessageCounter[receiver] = this.chatMessageCounter[receiver] + 1;

          if (this.chatMessageCounter[receiver] > this.maxChatHistoryLength!) {
            this.chatMessageCounter[receiver] = this.chatMessageCounter[receiver] - 1;
            chat.firstChild?.remove();
          }
        }
      } catch (error) {
        log('error', `Ошибка мониторинга чата: ${error}`);
      }
    });
  }

  public wait(): void {
    this.statusText!.innerText = 'Ожидание активных ботов...';
    this.statusText!.style.color = '#646464f7';

    this.botCardsContainer!.innerHTML = '';
    this.botCardsContainer!.style.display = 'grid';
  }

  public clear(): void {
    listenerManager.cleanup();
    
    this.botCardsContainer!.innerHTML = '';
    this.chatMessageCounter = {};
    this.chatHistoryFilters = {};
    statistics.usernameList = [];

    this.statusText!.innerText = 'Объекты ботов отсутствуют';
    this.statusText!.style.color = '#646464f7';
    this.statusText!.style.display = 'flex';

    this.botCardsContainer!.innerHTML = '';
    this.botCardsContainer!.style.display = 'none';
  }

  private initializeBotCard(nickname: string): void {
    const chat = document.getElementById(`chat-${nickname}`);

    listenerManager.add(`open-chat-${nickname}`, 'click', () => (chat as HTMLElement).style.display = 'flex');
    listenerManager.add(`close-chat-${nickname}`, 'click', () => (chat as HTMLElement).style.display = 'none');

    listenerManager.add(`disconnect-${nickname}`, 'click', async () => {
      try {
        const result = await invoke('disconnect_bot', {
          nickname: nickname
        }) as Array<string>;

        log(result[1], `log-${result[0]}`);
      } catch (error) {
        log(`Ошибка отключения бота ${nickname}: ${error}`, 'log-error');
      }
    });

    listenerManager.add(`solve-captcha-${nickname}`, 'click', () => {
      const captcha_url = (document.getElementById(`solve-captcha-${nickname}`) as HTMLButtonElement).getAttribute('captcha-url');

      if (captcha_url && captcha_url !== 'none') {
        invoke('open_url', { url: captcha_url });
      }
    });

    listenerManager.add(`filter-chat-${nickname}`, 'click', () => {
      try {
        const content = document.getElementById(`monitoring-chat-content-${nickname}`);
        const type = document.getElementById(`select-chat-filter-${nickname}`) as HTMLSelectElement;

        const history = [...document.querySelectorAll(`#monitoring-message-${nickname}`).values()];
        
        content!.innerHTML = '';

        this.chatHistoryFilters[nickname] = type.value;

        history.forEach(m => this.filterMessage(type.value, m.textContent || '') ? content?.appendChild(m) : null);
      } catch (error) {
        log(`Ошибка фильтровки чата: ${error}`, 'log-error');
      }
    });

    listenerManager.add(`clear-chat-${nickname}`, 'click', () => {
      const messages = document.querySelectorAll(`#monitoring-message-${nickname}`);
      messages.forEach(msg => msg.remove());
      this.chatMessageCounter[nickname] = 0;
    });

    const sendMsg = async () => {
      const message = document.getElementById(`this-chat-message-${nickname}`) as HTMLInputElement;

      const result = await invoke('send_message', { 
        nickname: nickname,
        message: message.value
      }) as Array<string>;

      if (result[0] !== 'error') {
        message.value = '';
      }

      log(result[1], `log-${result[0]}`);
    }

    listenerManager.add(`chat-${nickname}`, 'keydown', async (e: Event) => (e as KeyboardEvent).key === 'Enter' ? await sendMsg() : null);
  }

  public async enableMonitoring(): Promise<void> {
    try {
      const steveIconPath = document.getElementById('steve-img') as HTMLImageElement;

      let isFirst = true;

      const interval = setInterval(async () => {
        if (!activity.botting) {
          clearInterval(interval);
          return;
        }

        try {
          const profiles = await invoke('get_bot_profiles') as Record<string, BotProfile>;

          for (const nickname in profiles) {
            if (isFirst) {
              this.statusText!.style.display = 'none';
              isFirst = false;
            }

            const profile = profiles[nickname];

            let statusColor = '';

            switch (profile.status) {
              case 'Соединение...':
                statusColor = '#8f8f8fff'; break;
              case 'Онлайн':
                statusColor = '#22ed17ff'; break;
              case 'Оффлайн':
                statusColor = '#ed1717ff'; break;
              case 'Повреждён':
                statusColor = '#ed1717ff'; break;
            }

            if (statistics.usernameList.includes(nickname)) {
              (document.getElementById(`bot-status-${nickname}`) as HTMLElement).innerHTML = `<span style="color: ${statusColor};">• ${profile.status}</span>`;
              (document.getElementById(`bot-version-${nickname}`) as HTMLElement).innerHTML = `  ${profile.version}`;
              (document.getElementById(`bot-password-${nickname}`) as HTMLElement).innerHTML = `  ${profile.password}`;
              (document.getElementById(`bot-proxy-${nickname}`) as HTMLElement).innerHTML = `  ${profile.proxy}`;
              (document.getElementById(`solve-captcha-${nickname}`) as HTMLButtonElement).setAttribute('captcha-url', profile.captcha_url ? profile.captcha_url : 'none');
            } else {
              const card = document.createElement('div');
              card.className = 'bot-card';

              card.innerHTML = `
                <div class="bot-card-head">
                  <img src="${steveIconPath.src}" class="image" draggable="false">
                  <div class="text">
                    <div>
                      <div class="bot-nickname" style="user-select: text; -moz-user-select: text;">${nickname}</div>
                      <div class="bot-status"><span id="bot-status-${nickname}" style="color: ${statusColor};">• ${profile.status}</span></div>
                    </div>
                  </div>
                </div>

                <div class="bot-advanced-info">
                  <p>Версия:<span id="bot-version-${nickname}">  ${profile.version}</span></p>
                  <p>Пароль:<span id="bot-password-${nickname}">  ${profile.password}</span></p>
                  <p>Прокси:<span id="bot-proxy-${nickname}">  ${profile.proxy}</span></p>
                </div>

                <button class="btn spec" id="open-chat-${nickname}">Открыть чат</button>
                <button class="btn spec" id="solve-captcha-${nickname}" captcha-url="none">Решить капчу</button>
                <button class="btn red spec" id="disconnect-${nickname}" style="margin-bottom: 12px;">Отключить</button>

                <div class="chat-container cover" id="chat-${nickname}">
                  <div class="panel">
                    <div class="left">
                      <div class="custom-select">
                        <select id="select-chat-filter-${nickname}">
                          <option value="all">Все сообщения</option>
                          <option value="bans">Блокировки</option>
                          <option value="mentions">Упоминания</option>
                          <option value="warnings">Предупреждения</option>
                          <option value="links">Ссылки</option>
                        </select>
                      </div>

                      <button class="btn min pretty" id="filter-chat-${nickname}">
                        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-funnel-fill" viewBox="0 0 16 16">
                          <path d="M1.5 1.5A.5.5 0 0 1 2 1h12a.5.5 0 0 1 .5.5v2a.5.5 0 0 1-.128.334L10 8.692V13.5a.5.5 0 0 1-.342.474l-3 1A.5.5 0 0 1 6 14.5V8.692L1.628 3.834A.5.5 0 0 1 1.5 3.5z"/>
                        </svg>
                      </button>
                    </div>

                    <div class="right">
                      <button class="btn min pretty" id="clear-chat-${nickname}">
                        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-trash-fill" viewBox="0 0 16 16">
                          <path d="M2.5 1a1 1 0 0 0-1 1v1a1 1 0 0 0 1 1H3v9a2 2 0 0 0 2 2h6a2 2 0 0 0 2-2V4h.5a1 1 0 0 0 1-1V2a1 1 0 0 0-1-1H10a1 1 0 0 0-1-1H7a1 1 0 0 0-1 1zm3 4a.5.5 0 0 1 .5.5v7a.5.5 0 0 1-1 0v-7a.5.5 0 0 1 .5-.5M8 5a.5.5 0 0 1 .5.5v7a.5.5 0 0 1-1 0v-7A.5.5 0 0 1 8 5m3 .5v7a.5.5 0 0 1-1 0v-7a.5.5 0 0 1 1 0"/>
                        </svg>
                      </button>

                      <button class="btn min pretty" id="close-chat-${nickname}">
                        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-x-lg" viewBox="0 0 16 16">
                          <path d="M2.146 2.854a.5.5 0 1 1 .708-.708L8 7.293l5.146-5.147a.5.5 0 0 1 .708.708L8.707 8l5.147 5.146a.5.5 0 0 1-.708.708L8 8.707l-5.146 5.147a.5.5 0 0 1-.708-.708L7.293 8z"/>
                        </svg>
                      </button>
                    </div>
                  </div>

                  <div class="chat-content">
                    <div class="monitoring-content" id="monitoring-chat-content-${nickname}"></div>
                  </div>

                  <div style="display: flex; justify-content: center; align-items: center; gap: 10px;">
                    <p class="signature">${nickname}:</p>

                    <input type="text" control="this" id="this-chat-message-${nickname}" placeholder="Сообщение" style="height: 28px; width: 250px;">
                  </div>
                </div>
              `;

              this.botCardsContainer!.appendChild(card);
              this.chatHistoryFilters[nickname] = 'all';

              statistics.usernameList.push(nickname);

              setTimeout(() => this.initializeBotCard(nickname), 200);
            }
          }
        } catch (error) {
          log('error', `Ошибка мониторинга профилей: ${error}`);
        }
      }, 1800);
    } catch (error) {
      log(`Ошибка инициализации мониторинга: ${error}`, 'log-error');
    }
  }

  private createTrigrams(word: string): string[] {
    const trigrams = [];

    for (let i = 0; i <= word.length - 3; i++) {
      trigrams.push(word.substr(i, 3));
    }

    return trigrams;
  }

  private checkPatterns(word: string, patterns: string[]): boolean {
    if (word.length < 3) return false;

    let totalTrigrams = 0;
    let similarTrigrams = 0;

    const wts = this.createTrigrams(word);
    totalTrigrams = wts.length;

    for (const p of patterns) {
      const pts = this.createTrigrams(p);
      for (const wt of wts) {
        for (const pt of pts) {
          if (wt.toLowerCase() == pt.toLowerCase()) {
            similarTrigrams++;
          }
        }
      }
    }

    if (similarTrigrams >= totalTrigrams / 2) {
      return true;
    } 

    return false;
  }

  private filterMessage(type: string, message: string): boolean {
    if (type === 'all') {
      return true;
    } else if (type === 'links') {
      const patterns = [
        'http://', 'https://', 'socks5://', 'socks4://', 
        '.dev', '.com', '.org', '.io', '.ai', '.net',
        '.pro', '.gov', '.lv', '.ru', '.onion', '.ie',
        '.co', '.fun', '.gg', '.xyz', '.club', '.eu',
        '.me', '.us', '.online', '.br', '.cc', '.no'
      ];

      let result = false;
      
      patterns.forEach(p => message.toLowerCase().includes(p) ? result = true : null);

      return result;
    } else {
      const patterns: Record<string, string[]> = {
        bans: [
          'banned', 'ban', 'kicked',
          'kick', 'кикнут', 'заблокированный',
          'заблокирован', 'блокировка',
          'заблокировали', 'забанен', 
          'забанили', 'бан', 'blocked'
        ],
        warnings: [
          'предупреждение', 'warn', 'warning', 
          'важно', 'important', 'предупреждает', 
          'важная', 'уведомление', 'осведомление',
          'notice', 'уведомлять', 'замечание'
        ],
        mentions: [
          'упомянут', 'mention', 'reference', 
          'упоминает', 'упоминание', 'ссылаться'
        ]
      };

      let results: boolean[] = [];

      for (const word of message.split(' ')) {
        const result = this.checkPatterns(word, patterns[type]);
        results.push(result);
      }

      if (results.includes(true)) return true;
    }
      
    return false;
  }
}

class ProxyManager {
  private proxyList: HTMLTextAreaElement | null = null;
  private proxyCounter: HTMLElement | null = null;

  private proxyFinderStatus: HTMLElement | null = null;

  public async init(): Promise<void> {
    this.proxyList = document.getElementById('proxy-list') as HTMLTextAreaElement;
    this.proxyFinderStatus = document.getElementById('proxy-finder-status') as HTMLElement;
    this.proxyCounter = document.getElementById('proxy-counter') as HTMLElement;

    this.proxyList.addEventListener('input', () => this.updateCount());

    document.getElementById('clear-proxy-list')?.addEventListener('click', () => {
      this.proxyList!.value = '';
      this.updateCount();
    });

    document.getElementById('find-proxy')?.addEventListener('click', () => this.collectProxy());

    this.updateCount();
  }

  private updateCount(): void {
    let counter = 0;

    String(this.proxyList!.value).split('\n').forEach(element => {
      if (element.match(/((?:\d{1,3}\.){3}\d{1,3}):(\d+)/g) || element.match(/(\w+)\:\/\/((?:\d{1,3}\.){3}\d{1,3}):(\d+)/g)) {
        counter++;
      }
    });

    this.proxyCounter!.innerText = counter.toString();
  }
  
  private async collectProxy(): Promise<void> {
    try {
      const algorithm = (document.getElementById('proxy-finder-algorithm') as HTMLSelectElement).value;
      const protocol = (document.getElementById('proxy-finder-protocol') as HTMLSelectElement).value;
      const country = (document.getElementById('proxy-finder-country') as HTMLSelectElement).value;
      const count = (document.getElementById('proxy-finder-count') as HTMLInputElement).value;

      this.proxyFinderStatus!.innerText = 'Поиск прокси...';

      const proxies = await collect({
        algorithm: algorithm,
        protocol: protocol,
        country: country,
        count: count
      });

      if (proxies) {
        this.proxyFinderStatus!.style.color = '#0cd212ff';
        this.proxyFinderStatus!.innerText = 'Поиск окончен';
        this.proxyList!.value = Array.from(String(proxies).split('\n')).filter(p => p && p.trim() !== '').join('\n');
      } else {
        this.proxyFinderStatus!.style.color = '#cc1d1dff';
        this.proxyFinderStatus!.innerText = 'Ошибка поиска';

        log(`Ошибка поиска прокси`, 'log-error');
      }
    } catch (error) {
      this.proxyFinderStatus!.style.color = '#cc1d1dff';
      this.proxyFinderStatus!.innerText = 'Ошибка поиска';

      log(`Ошибка поиска прокси: ${error}`, 'log-error');
    } finally {
      this.updateCount();
      setTimeout(() => {
        this.proxyFinderStatus!.style.color = '#848080';
        this.proxyFinderStatus!.innerText = 'Поиск неактивен';
      }, 2000);
    }
  }
}

class ConfigManager {
  public async init(): Promise<void> {
    await this.create();
  }

  private async create(): Promise<void> {
    if (!localStorage.getItem('salarixionion:config')) {
      localStorage.setItem('salarixionion:config', JSON.stringify({}, null, 2));
    }
  }

  public async load(): Promise<any> {
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

      return config;
    } catch {
      return null;
    }
  }

  public async save(config: any): Promise<void> {
    for (let attempts = 0; attempts < 4; attempts++) {
      localStorage.setItem('salarixionion:config', JSON.stringify(config, null, 2))
      break;
    }
  }
}

class RadarManager {
  private targetCardsContainer: HTMLElement | null = null;

  private updateFrequency: number = 1500;
  private targets: Map<string, { data: any, interval: any, chart: any }> = new Map();

  public async init(): Promise<void> {
    this.targetCardsContainer = document.getElementById('radar-target-cards-container') as HTMLElement;

    const addTargetBtn = document.getElementById('radar-add-target') as HTMLButtonElement;
    const setUpdateFrequency = document.getElementById('radar-update-frequency') as HTMLSelectElement;
    const openSettingsBtn = document.getElementById('radar-open-settings') as HTMLButtonElement;
    const closeSettingsBtn = document.getElementById('radar-close-settings') as HTMLButtonElement;
    const removeAllTargetsBtn = document.getElementById('radar-remove-all-targets') as HTMLElement;

    addTargetBtn.addEventListener('click', () => {
      if (!activity.botting) return;

      const nicknameInput = (document.getElementById('radar-target-nickname') as HTMLInputElement);
      const nickname = nicknameInput.value;

      if (this.targets.has(nickname)) return;

      nicknameInput.value = '';

      this.targets.set(nickname, { data: null, interval: null, chart: null });

      const card = document.createElement('div');
      card.className = 'radar-target';
      card.id = `radar-target-${nickname}`;

      card.innerHTML = `
        <svg class="icon" xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-person-fill" viewBox="0 0 16 16">
          <path d="M3 14s-1 0-1-1 1-4 6-4 6 3 6 4-1 1-1 1zm5-6a3 3 0 1 0 0-6 3 3 0 0 0 0 6"/>
        </svg>

        <div class="sep"></div>

        <div class="info" style="min-width: 220px; max-width: 220px;">
          <p>Никнейм: ${nickname.length <= 16 ? nickname : nickname.substr(0, 16) + '...'}</p>
          <p>Статус: <span id="radar-target-status-${nickname}">Не найден</span></p>
          <p>UUID: <span id="radar-target-uuid-${nickname}">?</span></p>
        </div>

        <div class="sep"></div>

        <div class="info" style="min-width: 150px; max-width: 150px;">
          <p>X: <span id="radar-target-x-${nickname}">?</span></p>
          <p>Y: <span id="radar-target-y-${nickname}">?</span></p>
          <p>Z: <span id="radar-target-z-${nickname}">?</span></p>
        </div>

        <div class="sep"></div>

        <div class="btn-group">
          <div class="btn-group-flex" style="margin-top: 0;">
            <button class="btn min pretty" id="radar-open-route-${nickname}">
              <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-geo-alt-fill" viewBox="0 0 16 16">
                <path d="M8 16s6-5.686 6-10A6 6 0 0 0 2 6c0 4.314 6 10 6 10m0-7a3 3 0 1 1 0-6 3 3 0 0 1 0 6"/>
              </svg>
            </button>

            <button class="btn min pretty" id="radar-remove-target-${nickname}">
              <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-trash-fill" viewBox="0 0 16 16">
                <path d="M2.5 1a1 1 0 0 0-1 1v1a1 1 0 0 0 1 1H3v9a2 2 0 0 0 2 2h6a2 2 0 0 0 2-2V4h.5a1 1 0 0 0 1-1V2a1 1 0 0 0-1-1H10a1 1 0 0 0-1-1H7a1 1 0 0 0-1 1zm3 4a.5.5 0 0 1 .5.5v7a.5.5 0 0 1-1 0v-7a.5.5 0 0 1 .5-.5M8 5a.5.5 0 0 1 .5.5v7a.5.5 0 0 1-1 0v-7A.5.5 0 0 1 8 5m3 .5v7a.5.5 0 0 1-1 0v-7a.5.5 0 0 1 1 0"/>
              </svg>
            </button>
          </div>

          <div class="btn-group-flex" style="margin-top: 0;">
            <button class="btn min pretty" disabled>
              <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-crosshair" viewBox="0 0 16 16">
                <path d="M8.5.5a.5.5 0 0 0-1 0v.518A7 7 0 0 0 1.018 7.5H.5a.5.5 0 0 0 0 1h.518A7 7 0 0 0 7.5 14.982v.518a.5.5 0 0 0 1 0v-.518A7 7 0 0 0 14.982 8.5h.518a.5.5 0 0 0 0-1h-.518A7 7 0 0 0 8.5 1.018zm-6.48 7A6 6 0 0 1 7.5 2.02v.48a.5.5 0 0 0 1 0v-.48a6 6 0 0 1 5.48 5.48h-.48a.5.5 0 0 0 0 1h.48a6 6 0 0 1-5.48 5.48v-.48a.5.5 0 0 0-1 0v.48A6 6 0 0 1 2.02 8.5h.48a.5.5 0 0 0 0-1zM8 10a2 2 0 1 0 0-4 2 2 0 0 0 0 4"/>
              </svg>
            </button>

            <button class="btn min pretty" id="radar-copy-target-info-${nickname}">
              <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-copy" viewBox="0 0 16 16">
                <path fill-rule="evenodd" d="M4 2a2 2 0 0 1 2-2h8a2 2 0 0 1 2 2v8a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2zm2-1a1 1 0 0 0-1 1v8a1 1 0 0 0 1 1h8a1 1 0 0 0 1-1V2a1 1 0 0 0-1-1zM2 5a1 1 0 0 0-1 1v8a1 1 0 0 0 1 1h8a1 1 0 0 0 1-1v-1h1v1a2 2 0 0 1-2 2H2a2 2 0 0 1-2-2V6a2 2 0 0 1 2-2h1v1z"/>
              </svg>
            </button>
          </div>
        </div>

        <div class="cover" id="radar-route-${nickname}">
          <div class="panel">
            <div class="left">
            </div>

            <div class="right">
              <button class="btn min pretty" id="radar-close-route-${nickname}">
                <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-x-lg" viewBox="0 0 16 16">
                  <path d="M2.146 2.854a.5.5 0 1 1 .708-.708L8 7.293l5.146-5.147a.5.5 0 0 1 .708.708L8.707 8l5.147 5.146a.5.5 0 0 1-.708.708L8 8.707l-5.146 5.147a.5.5 0 0 1-.708-.708L7.293 8z"/>
                </svg>
              </button>
            </div>
          </div>

          <canvas class="radar-chart" id="radar-chart-${nickname}"></canvas>
        </div>
      `;

      this.targetCardsContainer!.appendChild(card);

      setTimeout(() => this.initializeTargetCard(nickname), 200);
    });

    setUpdateFrequency.addEventListener('change', () => {
      this.updateFrequency = setUpdateFrequency.value ? parseInt(setUpdateFrequency.value) : 1500;

      this.targets.forEach((i, n) => {
        clearInterval(i.interval);
        this.setTargetUpdateInterval(n, this.updateFrequency);
      });
    });

    openSettingsBtn.addEventListener('click', () => {
      const settings = document.getElementById('radar-settings') as HTMLElement;
      settings.style.display = 'flex';
    }); 

    closeSettingsBtn.addEventListener('click', () => {
      const settings = document.getElementById('radar-settings') as HTMLElement;
      settings.style.display = 'none';
    }); 

    removeAllTargetsBtn.addEventListener('click', () => {
      this.targets.forEach((v, n) => {
        const card = document.getElementById(`radar-target-${n}`) as HTMLElement;
        v.chart.destroy();
        card.remove();
        clearInterval(v.interval);
      });

      this.targets.clear();
    });
  }

  private initializeTargetCard(nickname: string): void {
    try {
      const openRouteBtn = document.getElementById(`radar-open-route-${nickname}`) as HTMLButtonElement;
      const closeRouteBtn = document.getElementById(`radar-close-route-${nickname}`) as HTMLButtonElement;
      const removeTargetBtn = document.getElementById(`radar-remove-target-${nickname}`) as HTMLButtonElement;
      const copyTargetInfoBtn = document.getElementById(`radar-copy-target-info-${nickname}`) as HTMLButtonElement;

      openRouteBtn.addEventListener('click', () => {
        const routeContainer = document.getElementById(`radar-route-${nickname}`) as HTMLElement;
        routeContainer.style.display = 'flex';
      });

      closeRouteBtn.addEventListener('click', () => {
        const routeContainer = document.getElementById(`radar-route-${nickname}`) as HTMLElement;
        routeContainer.style.display = 'none';
      });

      removeTargetBtn.addEventListener('click', () => {
        const card = document.getElementById(`radar-target-${nickname}`) as HTMLElement;
        this.targets.get(nickname)?.chart.destroy();
        card.remove();
        clearInterval(this.targets.get(nickname)?.interval);
        this.targets.delete(nickname);
      });

      copyTargetInfoBtn.addEventListener('click', async () => {
        try {
          const status = (document.getElementById(`radar-target-status-${nickname}`) as HTMLElement).textContent;
          const uuid = this.targets.get(nickname)?.data?.fullUUID ? this.targets.get(nickname)?.data.fullUUID : '?';
          const x = (document.getElementById(`radar-target-x-${nickname}`) as HTMLElement).textContent;
          const y = (document.getElementById(`radar-target-y-${nickname}`) as HTMLElement).textContent;
          const z = (document.getElementById(`radar-target-z-${nickname}`) as HTMLElement).textContent;

          const text = `
Никнейм: ${nickname}
Статус: ${status}
UUID: ${uuid}
Координата X: ${x}
Координата Y: ${y}
Координата Z: ${z}
          `.trim();

          await navigator.clipboard.writeText(text);
        } catch (error) {
          log(`Ошибка копирования radar-данных: ${error}`, 'log-error');
        }
      });

      this.targets.set(nickname, { data: null, interval: null, chart: null });

      this.createTargetChart(nickname);
      this.setTargetUpdateInterval(nickname, this.updateFrequency);
    } catch (error) {
      log(`Ошибка инициализации radar-цели: ${error}`, 'log-error');
    }
  }

  private setTargetUpdateInterval(nickname: string, frequency: number) {
    let lx = '';
    let ly = '';
    let lz = '';

    const target = this.targets.get(nickname);

    if (target) {
      target.interval = setInterval(async () => {
        if (!activity.botting) {
          clearInterval(target.interval);
          return;
        }

        try {
          const data = await invoke('get_radar_data', { target: nickname }) as RadarInfo;

          if (data) {
            const x = data.x.toFixed(3);
            const y = data.y.toFixed(3);
            const z = data.z.toFixed(3);

            (document.getElementById(`radar-target-status-${nickname}`) as HTMLElement).innerText = data.status;
            (document.getElementById(`radar-target-uuid-${nickname}`) as HTMLElement).innerText = data.uuid.substr(0, 12) + '...';
            (document.getElementById(`radar-target-x-${nickname}`) as HTMLElement).innerText = x;
            (document.getElementById(`radar-target-y-${nickname}`) as HTMLElement).innerText = y;
            (document.getElementById(`radar-target-z-${nickname}`) as HTMLElement).innerText = z;

            if (target) {
              target.data = {
                fullUUID: data.uuid
              };
            }

            if (lx !== x || ly !== y || lz !== z) {
              const card = document.getElementById(`radar-target-${nickname}`) as HTMLElement;
              card.classList.add('glow');
              setTimeout(() => card.classList.remove('glow'), 300);
            }

            lx = data.x.toFixed(3);
            ly = data.y.toFixed(3);
            lz = data.z.toFixed(3);

            this.addRoutePointToChart(nickname, parseFloat(x), parseFloat(z), data.observer);

            if ((document.getElementById('radar-auto-save') as HTMLInputElement).checked) {
              const path = (document.getElementById('radar-path') as HTMLInputElement).value;
              const filename = (document.getElementById('radar-filename') as HTMLInputElement).value;

              if (await isAbsolute(path)) {
                await invoke('save_radar_data', {
                  target: nickname,
                  path: path,
                  filename: filename || 'radar_#t',
                  x: parseFloat(x),
                  y: parseFloat(y),
                  z: parseFloat(z)
                });
              }
            }
          }
        } catch (error) {
          log(`Ошибка обновления radar-цели ${nickname}: ${error}`, 'log-error');
        }
      }, frequency);
    }
  }

  private createTargetChart(nickname: string) {
    const ctx = document.getElementById(`radar-chart-${nickname}`) as HTMLCanvasElement;

    const chart = new Chart(ctx, {
      type: 'scatter', 
      data: {
        datasets: [
          {
            label: ` Маршрут ${nickname}`,
            data: [], 
            backgroundColor: '#39a10fff',
            borderColor: '#0f8f0bff', 
            showLine: true, 
            fill: false,
            pointRadius: 2,
            tension: 0,
            borderWidth: 1
          },
          {
            label: ` Метка наблюдателя`,
            data: [], 
            backgroundColor: '#d31212ff',
            borderColor: '#800c0cff', 
            showLine: false, 
            fill: false,
            pointRadius: 3,
            tension: 0,
            borderWidth: 1
          }
        ]
      },
      options: {
        responsive: true,
        animation: {
          duration: 300
        },
        scales: {
          x: {
            type: 'linear',
            position: 'bottom',
            title: {
              display: true,
              text: 'X'
            },
            min: -200,
            max: 200, 
            grid: { 
              color: '#30303086'
            },
            ticks: {
              stepSize: 50
            }
          },
          y: {
            type: 'linear',
            position: 'left',
            title: {
              display: true,
              text: 'Z'
            },
            min: -200,
            max: 200, 
            grid: { 
              color: '#30303086' 
            },
            ticks: {
              stepSize: 50
            }
          }
        },
        plugins: {
          title: {
            text: `Маршрут ${nickname}`,
            display: true,
            color: '#a2a2a2ff'
          },
          legend: {
            display: false 
          },
          tooltip: {
            enabled: false 
          }
        }
      }
    });

    const target = this.targets.get(nickname);

    if (target) {
      target.chart = chart;
    }
  }

  private addRoutePointToChart(nickname: string, x: number, z: number, observer: { x: number, z: number }) {
    const target = this.targets.get(nickname);

    if (target) {
      target.chart.data.datasets[0].data.push({ x: x, y: z });
      target.chart.data.datasets[1].data.push({ x: observer.x, y: observer.z });

      if (target.chart.data.datasets[0].data.length > 30) target.chart.data.datasets[0].data.shift();
      if (target.chart.data.datasets[1].data.length > 1) target.chart.data.datasets[1].data.shift();

      const xMin = x - 200;
      const xMax = x + 200;
      const zMin = z - 200;
      const zMax = z + 200;
      
      target.chart.options.scales.x.min = xMin;
      target.chart.options.scales.x.max = xMax;
      target.chart.options.scales.y.min = zMin;
      target.chart.options.scales.y.max = zMax;

      target.chart.update();
    }
  }
}                 

const listenerManager = new ListenerManager();
const functionManager = new FunctionManager();
const graphicManager = new GraphicManager();
const monitoringManager = new MonitoringManager();
const proxyManager = new ProxyManager();
const configManager = new ConfigManager();
const radarManager = new RadarManager();

setInterval(async () => {
  if (!statistics.isInitialized) return;

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

  if (JSON.stringify(config) === JSON.stringify(statistics.latestConfig)) return;

  await configManager.save(config);

  statistics.latestConfig = config;
}, 1400);

async function loadConfig(): Promise<void> {
  log('Загрузка конфига...', 'log-system');

  const config = await configManager.load();

  if (!config) {
    log(`Ошибка (load-config): Файл конфигурации отсутствует или повреждён`, 'log-error');
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

  log('Конфиг успешно загружен', 'log-system');
}

async function initSocialButtons(): Promise<void> {
  (document.getElementById('telegram') as HTMLElement).addEventListener('click', async () => await invoke('open_url', { url: 'https://t.me/salarixionion' })); 
  (document.getElementById('github') as HTMLElement).addEventListener('click', async () => await invoke('open_url', { url: 'https://github.com/nullclyze/SalarixiOnion' }));
  (document.getElementById('youtube') as HTMLElement).addEventListener('click', async () => await invoke('open_url', { url: 'https://www.youtube.com/@salarixionion' }));
}

function updateDisplayOfElements(condition: boolean, view: string, show: string[] = [], hide: string[] = []): void {
  hide.forEach(id => {
    const element = document.getElementById(id);
    if (element) element.style.display = condition ? 'none' : view;
  });
  
  show.forEach(id => {
    const element = document.getElementById(id);
    if (element) element.style.display = condition ? view : 'none';
  });
}

async function initCheckboxes(): Promise<void> {
  try {
    const update = () => {
      updateDisplayOfElements((document.getElementById('use-auto-register') as HTMLInputElement).checked, 'flex', ['auto-register-input-container']);
      updateDisplayOfElements((document.getElementById('use-auto-login') as HTMLInputElement).checked, 'flex', ['auto-login-input-container']);
      updateDisplayOfElements((document.getElementById('use-auto-rejoin') as HTMLInputElement).checked, 'flex', ['auto-rejoin-input-container']);
    }

    (document.getElementById('use-auto-register') as HTMLInputElement).addEventListener('change', function () { updateDisplayOfElements((this as HTMLInputElement).checked, 'flex', ['auto-register-input-container']); });
    (document.getElementById('use-auto-login') as HTMLInputElement).addEventListener('change', function () { updateDisplayOfElements((this as HTMLInputElement).checked, 'flex', ['auto-login-input-container']); });
    (document.getElementById('use-auto-rejoin') as HTMLInputElement).addEventListener('change', function () { updateDisplayOfElements((this as HTMLInputElement).checked, 'flex', ['auto-rejoin-input-container']); });

    (document.getElementById('show-system-log') as HTMLInputElement).addEventListener('change', function () {
      const systemLogs = document.querySelectorAll('.log-line-system');
      
      if (this.checked) {
        statistics.showSystemLogs = true;
        systemLogs.forEach(element => (element as HTMLElement).style.display = 'flex');
      } else {
        statistics.showSystemLogs = false;
        systemLogs.forEach(element => (element as HTMLElement).style.display = 'none');
      }
    });

    (document.getElementById('show-extended-log') as HTMLInputElement).addEventListener('change', function () {
      const extendedLogs = document.querySelectorAll('.log-line-extended');
      
      if (this.checked) {
        statistics.showExtendedLogs = true;
        extendedLogs.forEach(element => (element as HTMLElement).style.display = 'flex');
      } else {
        statistics.showExtendedLogs = false;
        extendedLogs.forEach(element => (element as HTMLElement).style.display = 'none');
      }
    });

    update();
  } catch (error) {
    log(`Ошибка операции initCheckboxes: ${error}`, 'log-error');
  }
}

async function initSelects(): Promise<void> {
  try {
    const nicknameTypeSelect = document.getElementById('nickname-type-select') as HTMLSelectElement;
    const passwordTypeSelect = document.getElementById('password-type-select') as HTMLSelectElement;
    const chatModeSelect = document.getElementById('chat-mode') as HTMLSelectElement;
    const inventoryModeSelect = document.getElementById('select-inventory-mode') as HTMLSelectElement;
    const movementModeSelect = document.getElementById('select-movement-mode') as HTMLSelectElement;
    const flightSettings = document.getElementById('select-flight-settings') as HTMLSelectElement;
    const bowAimTarget = document.getElementById('select-bow-aim-target') as HTMLSelectElement;
    const minerModeSelect = document.getElementById('miner-mode-select') as HTMLSelectElement;

    nicknameTypeSelect.addEventListener('change', () => {
      if (nicknameTypeSelect.value === 'custom') {
        (document.getElementById('custom-nickname-template-container') as HTMLInputElement).style.display = 'flex';
      } else {
        (document.getElementById('custom-nickname-template-container') as HTMLInputElement).style.display = 'none';
      }
    });

    passwordTypeSelect.addEventListener('change', () => {
      if (passwordTypeSelect.value === 'custom') {
        (document.getElementById('custom-password-template-container') as HTMLInputElement).style.display = 'flex';
      } else {
        (document.getElementById('custom-password-template-container') as HTMLInputElement).style.display = 'none';
      }
    });

    chatModeSelect.addEventListener('change', () => {
      const chatSpammingChbxContainer = document.getElementById('chat-spamming-chbx-container') as HTMLElement;
      const chatSpammingInputContainer = document.getElementById('chat-spamming-input-container') as HTMLElement;
            
      const chatDefaultBtnsContainer = document.getElementById('chat-default-btns-container') as HTMLElement;
      const chatSpammingBtnsContainer = document.getElementById('chat-spamming-btns-container') as HTMLElement;

      if (chatModeSelect.value === 'Spamming') {
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

    inventoryModeSelect.addEventListener('change', () => {
      const basicBtnContainer = document.getElementById('inventory-basic-btn-container') as HTMLElement;

      const swapInputContainer = document.getElementById('inventory-swap-input-container') as HTMLElement;
      const swapBtnContainer = document.getElementById('inventory-swap-btn-container') as HTMLElement;

      if (inventoryModeSelect.value === 'basic') {
        basicBtnContainer.style.display = 'flex';

        swapInputContainer.style.display = 'none';
        swapBtnContainer.style.display = 'none';
      } else if (inventoryModeSelect.value === 'swap') {
        swapInputContainer.style.display = 'flex';
        swapBtnContainer.style.display = 'flex';

        basicBtnContainer.style.display = 'none';
      } 
    });

    movementModeSelect.addEventListener('change', () => {
      const selectMovementDirectionContainer = document.getElementById('select-movement-direction-container') as HTMLElement;
      const movementPathfinderInputContainer = document.getElementById('movement-pathfinder-input-container') as HTMLElement;
      const movementImpulsivenessContainer = document.getElementById('movement-impulsiveness-container') as HTMLElement;

      if (movementModeSelect.value === 'default') {
        selectMovementDirectionContainer.style.display = 'flex';
        movementPathfinderInputContainer.style.display = 'none';
        movementImpulsivenessContainer.style.display = 'flex';
      } else {
        selectMovementDirectionContainer.style.display = 'none';
        movementPathfinderInputContainer.style.display = 'flex';
        movementImpulsivenessContainer.style.display = 'none';
      }
    });

    flightSettings.addEventListener('change', () => {
      const manualSettingsContainer = document.getElementById('flight-manual-settings-container') as HTMLElement;
      const selectAntiCheatContainer = document.getElementById('flight-select-anti-cheat-container') as HTMLElement;

      if (flightSettings.value === 'adaptive') {
        manualSettingsContainer.style.display = 'none';
        selectAntiCheatContainer.style.display = 'grid';
      } else {
        manualSettingsContainer.style.display = 'flex';
        selectAntiCheatContainer.style.display = 'none';
      }
    });

    bowAimTarget.addEventListener('change', () => {
      const customGoalInputContainer = document.getElementById('bow-aim-custom-goal-input-container') as HTMLElement;

      if (bowAimTarget.value === 'custom-goal') {
        customGoalInputContainer.style.display = 'flex';
      } else {
        customGoalInputContainer.style.display = 'none';
      }
    });

    minerModeSelect.addEventListener('change', () => {
      const selectBlockContainer = document.getElementById('miner-select-block-container') as HTMLElement;
      const manualSettingsContainer = document.getElementById('miner-manual-options-container') as HTMLElement;

      if (minerModeSelect.value === 'manual') {
        manualSettingsContainer.style.display = 'flex';
        selectBlockContainer.style.display = 'none';
      } else {
        manualSettingsContainer.style.display = 'none';
        selectBlockContainer.style.display = 'grid';
      }
    });
  } catch (error) {
    log(`Ошибка операции initSelects: ${error}`, 'log-error');
  }
}

async function checkUpdate(): Promise<void> {
  try {
    const closeNoticeBtn = document.getElementById('close-notice-btn');

    closeNoticeBtn?.addEventListener('click', () => {
      const notice = document.getElementById('notice');
      if (!notice) return;
      notice.style.display = 'none';
    });

    const notice = document.getElementById('notice') as HTMLElement;
    const newVersion = document.getElementById('new-client-version') as HTMLElement;
    const newTag = document.getElementById('new-client-tag') as HTMLElement;
    const newReleaseDate = document.getElementById('new-client-release-date') as HTMLElement;
    const openClientRelease = document.getElementById('open-client-release') as HTMLElement;

    const response = await fetch('https://raw.githubusercontent.com/nullclyze/SalarixiOnion/refs/heads/main/salarixi.version.json', { method: 'GET' });

    if (!response.ok) return;

    const data = await response.json();

    if (data && data.version !== client.version) {
      const tag = `v${data.version}-${String(data.type).toLowerCase()}`;

      newVersion.innerText = data.version;
      newTag.innerText = tag;
      newReleaseDate.innerText = data.releaseDate;
      
      openClientRelease.addEventListener('click', async () => await invoke('open_url', { url: `https://github.com/nullclyze/SalarixiOnion/releases/tag/${tag}` }));
      
      setTimeout(() => {
        notice.style.display = 'flex';
      }, 4000);
    }
  } catch (error) {
    log(`Ошибка проверки обновлений: ${error}`, 'log-error');
  }
}

document.addEventListener('DOMContentLoaded', async () => {
  log('Клиент запущен', 'log-info');

  try {
    log('[ debug ] Инициализация...', 'log-extended');

    await listen('log', (event) => {
      try {
        const payload = event.payload as LogEventPayload;
        const name = payload.name;
        const message = payload.message;

        log(message, `log-${name}`);
      } catch (error) {
        log(`Ошибка принятие log-события: ${error}`, 'log-error');
      }
    });

    (document.getElementById('title-version') as HTMLElement).innerText = `v${client.version}`;

    (document.getElementById('window-minimize') as HTMLButtonElement).addEventListener('click', async () => await getCurrentWindow().minimize());
    (document.getElementById('window-close') as HTMLButtonElement).addEventListener('click', async () => await invoke('exit'));

    document.querySelectorAll('[global="true"]').forEach(c => globalContainers.push({ id: c.id, el: c as HTMLElement }));
    document.querySelectorAll('[sector="true"]').forEach(c => controlContainers.push({ id: c.id, el: c as HTMLElement }));

    await clear();

    await configManager.init();

    await loadConfig();

    await initSocialButtons();

    await functionManager.init();
    await proxyManager.init();
    await graphicManager.init();
    await monitoringManager.init();
    await radarManager.init();
    
    await initCheckboxes();
    await initSelects();

    statistics.isInitialized = true;

    log('[ debug ] Инициализация прошла успешно', 'log-extended');

    const loadingContainer = document.getElementById('loading-container');

    if (loadingContainer) {
      setTimeout(() => {
        loadingContainer.classList.add('hide');
        setTimeout(() => {
          loadingContainer.style.display = 'none';
        }, 590);
      }, 1500);
    }

    await checkUpdate();
  } catch (error) {
    log(`Ошибка инициализации: ${error}`, 'log-error');
  }
});