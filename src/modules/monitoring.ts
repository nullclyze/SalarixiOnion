import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';

import { log } from '../logger';
import { date } from '../helpers/date';
import { process } from '../main';


interface BotProfile {
  status: any;
  username: string;
  password: string | null;
  proxy: { 
    ip_address: string;
    proxy: string | null;
    username: string | null;
    password: string | null;
  };
  ping: number;
  health: number;
  registered: boolean;
  skin_is_set: boolean;
  captcha_caught: boolean;
  group: string;
}

export class MonitoringManager {
  private usernameList: string[] = [];

  private statusText: HTMLElement | null = null;
  private cards: HTMLElement | null = null;
  private wrappers: HTMLElement | null = null;

  private chatMessageCounter: Record<string, number> = {};
  private chatHistoryFilters: Record<string, string> = {};

  private listeners: Map<string, any> = new Map();

  public extendedMonitoring: boolean = true;
  public chatMonitoring: boolean = true;
  public mapMonitoring: boolean = false;
  public maxChatHistoryLength: number = 0;

  public async init(): Promise<void> {
    this.statusText = document.getElementById('monitoring-status-text');
    this.cards = document.getElementById('bot-cards-container');
    this.wrappers = document.getElementById('bot-wrappers-container');

    this.statusText!.innerText = 'Объекты ботов отсутствуют';
    this.statusText!.style.color = '#646464f7';
    this.statusText!.style.display = 'flex';

    await listen('chat-message', (event) => {
      if (this.chatMonitoring) {
        try {
          const payload = event.payload as { receiver: string; message: string; };
          const receiver = payload.receiver;
          const message = payload.message;

          if (!this.chatHistoryFilters[receiver]) {
            this.chatHistoryFilters[receiver] = 'all';
          }

          if (!this.filterMessage(this.chatHistoryFilters[receiver], message)) return;

          const chat = document.getElementById(`monitoring-chat-content-${receiver}`);

          if (!chat) return;

          const line = document.createElement('div');
          line.className = 'line';
          line.setAttribute('monitoring-message', receiver);

          line.innerHTML = `
            <div class="time">${date()}</div>
            <div class="msg">${message}</div>
          `;

          chat.appendChild(line);

          chat.scrollTo({
            top: chat.scrollHeight,
            behavior: 'smooth'
          });

          if (!this.chatMessageCounter[receiver]) {
            this.chatMessageCounter[receiver] = 1;
          } else {
            this.chatMessageCounter[receiver] = this.chatMessageCounter[receiver] + 1;

            if (this.chatMessageCounter[receiver] > this.maxChatHistoryLength) {
              this.chatMessageCounter[receiver] = this.chatMessageCounter[receiver] - 1;
              chat.firstChild?.remove();
            }
          }
        } catch (error) {
          log(`Ошибка мониторинга (receive-payload): ${error}`, 'error');
        }
      }
    });

    /* Временно

    await listen('map-render-progress', (event) => {
      if (this.mapMonitoring) {
        try {
          const payload = event.payload as { nickname: string; progress: number; };
          const nickname = payload.nickname;
          const progress = payload.progress;

          const doc = document.getElementById(`map-render-progress-count-${nickname}`);

          if (doc) {
            doc.innerText = progress.toString();
          }
        } catch (error) {
          log(`Ошибка мониторинга (receive-payload): ${error}`, 'error');
        }
      }
    });

    */
  }

  public wait(): void {
    this.statusText!.innerText = 'Ожидание активных ботов...';
    this.statusText!.style.color = '#646464f7';

    this.cards!.innerHTML = '';
    this.cards!.style.display = 'flex';
  }

  public enable(delay: number): void {
    try {
      let isFirst = true;

      const interval = setInterval(async () => {
        if (process === 'sleep') {
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

            if (this.usernameList.includes(nickname)) {
              this.updateBotCard(nickname, profile);
            } else {
              this.createBotCard(nickname, profile);
            }
          }
        } catch (error) {
          log(`Ошибка мониторинга профилей: ${error}`, 'error');
        }
      }, delay);
    } catch (error) {
      log(`Ошибка инициализации мониторинга: ${error}`, 'error');
    }
  }

  public disable(): void {
    for (const [id, data] of this.listeners) {
      document.getElementById(id)?.removeEventListener(data.event, data.listener);
    }

    this.listeners.clear();

    this.cards!.innerHTML = '';
    this.chatMessageCounter = {};
    this.chatHistoryFilters = {};
    this.usernameList = [];

    this.statusText!.innerText = 'Объекты ботов отсутствуют';
    this.statusText!.style.color = '#646464f7';
    this.statusText!.style.display = 'flex';

    this.cards!.style.display = 'none';
  }

  private addTempListener(id: string, event: string, listener: EventListener): void {
    document.getElementById(id)?.addEventListener(event, listener);
    this.listeners.set(id, { event: event, listener: listener });
  }

  private createBotCard(username: string, profile: BotProfile): void {
    const steveIconPath = document.getElementById('steve-img') as HTMLImageElement;

    const groupNameExamples = [
      'killaura', 'bow_aim', 'AutoFarm', 
      'AFK', 'Travelers', 'miner', 
      'Stealer', 'Farmer', 'Spamming', 
      'PvE', 'PvP', 'afk_group',
      'MyGroup', 'Group', 'group1',
      'GroupOne', 'Stalkers', 'my_group'
    ];
    
    const groupNameExample = groupNameExamples[Math.floor(Math.random() * groupNameExamples.length)];

    const card = document.createElement('div');
    card.className = 'bot-card';
    card.id = `bot-card-${username}`;
    
    card.innerHTML = `
      <div class="head">
        <div class="top">
          <img src="${steveIconPath.src}" class="image" draggable="false">
          <div class="text">
            <div class="username">${username}</div>
            <div class="status" id="monitoring-status-${username}">Preparation</div>
          </div>
        </div>

        <div class="bottom">
          <input type="text" class="bot-group" id="bot-group-${username}" placeholder="Группа, например, ${groupNameExample}">
        </div>
      </div>

      <div class="info">
        <p class="line">Пинг: <span id="monitoring-ping-${username}">${profile.ping}</span>ms</p>
        <p class="line">Здоровье: <span id="monitoring-health-${username}">${profile.health}</span> / 20</p>
        <p class="line">Прокси: <span id="monitoring-proxy-${username}">${profile.proxy.ip_address}</span></p>
        <p class="line">Пароль: <span>${profile.password ?? 'No password'}</span></p>
      </div>

      <div class="buttons">
        <button class="btn min pretty" id="open-chat-${username}">Открыть чат</button>
        <button class="btn min pretty" id="reset-${username}">Сбросить</button>
        <button class="btn min pretty" id="disconnect-${username}">Отключить</button>
      </div>
    `;

    this.cards?.appendChild(card);
    this.initializeBotCard(username);
  }

  private createChatWrapper(username: string): HTMLDivElement {
    const wrapper = document.createElement('div');
    wrapper.className = 'cover';
    wrapper.id = `chat-${username}`;

    wrapper.innerHTML = `
      <div class="panel">
        <div class="left">
          <div class="custom-select">
            <select id="select-chat-filter-${username}">
              <option value="all">Все сообщения</option>
              <option value="bans">Блокировки</option>
              <option value="mentions">Упоминания</option>
              <option value="warnings">Предупреждения</option>
              <option value="links">Ссылки</option>
            </select>
          </div>

          <button class="btn min pretty" id="filter-chat-${username}">
            <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-funnel-fill" viewBox="0 0 16 16">
              <path d="M1.5 1.5A.5.5 0 0 1 2 1h12a.5.5 0 0 1 .5.5v2a.5.5 0 0 1-.128.334L10 8.692V13.5a.5.5 0 0 1-.342.474l-3 1A.5.5 0 0 1 6 14.5V8.692L1.628 3.834A.5.5 0 0 1 1.5 3.5z"/>
            </svg>
          </button>
        </div>

        <div class="right">
          <button class="btn min pretty" id="clear-chat-${username}">
            <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-trash-fill" viewBox="0 0 16 16">
              <path d="M2.5 1a1 1 0 0 0-1 1v1a1 1 0 0 0 1 1H3v9a2 2 0 0 0 2 2h6a2 2 0 0 0 2-2V4h.5a1 1 0 0 0 1-1V2a1 1 0 0 0-1-1H10a1 1 0 0 0-1-1H7a1 1 0 0 0-1 1zm3 4a.5.5 0 0 1 .5.5v7a.5.5 0 0 1-1 0v-7a.5.5 0 0 1 .5-.5M8 5a.5.5 0 0 1 .5.5v7a.5.5 0 0 1-1 0v-7A.5.5 0 0 1 8 5m3 .5v7a.5.5 0 0 1-1 0v-7a.5.5 0 0 1 1 0"/>
            </svg>
          </button>

          <button class="btn min pretty" id="close-chat-${username}">
            ⨉
          </button>
        </div>
      </div>

      <div class="chat-content" id="monitoring-chat-content-${username}"></div>

      <div class="pretty-input-wrapper">
        <p class="signature">${username}</p>
        <input type="text" id="chat-message-${username}" placeholder="Введите и отправьте сообщение, нажав «Enter»">
      </div>
    `;

    this.wrappers?.appendChild(wrapper);

    return wrapper;
  }

  /* Временно

  private createMapWrapper(username: string): HTMLDivElement {
    const wrapper = document.createElement('div');
    wrapper.className = 'cover';
    wrapper.id = `map-${username}`;

    wrapper.innerHTML = `
      <div class="panel">
        <div class="right">
          <button class="btn min pretty" id="remove-map-${username}">
            <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-trash-fill" viewBox="0 0 16 16">
              <path d="M2.5 1a1 1 0 0 0-1 1v1a1 1 0 0 0 1 1H3v9a2 2 0 0 0 2 2h6a2 2 0 0 0 2-2V4h.5a1 1 0 0 0 1-1V2a1 1 0 0 0-1-1H10a1 1 0 0 0-1-1H7a1 1 0 0 0-1 1zm3 4a.5.5 0 0 1 .5.5v7a.5.5 0 0 1-1 0v-7a.5.5 0 0 1 .5-.5M8 5a.5.5 0 0 1 .5.5v7a.5.5 0 0 1-1 0v-7A.5.5 0 0 1 8 5m3 .5v7a.5.5 0 0 1-1 0v-7a.5.5 0 0 1 1 0"/>
            </svg>
          </button>

          <button class="btn min pretty" id="close-map-${username}">
            ⨉
          </button>
        </div>
      </div>

      <div class="bot-map-wrap" id="map-wrap-${username}">
        <div class="bot-map-render-status" id="map-render-status-${username}">Генерация карты, пожайлуста, подождите...</div>
        <div class="bot-map-render-progress" id="map-render-progress-${username}">Прогресс (блоков): <span class="bot-map-render-progress-count" id="map-render-progress-count-${username}">0</span> / 40000</div>

        <div id="save-map-wrap-${username}" style="display: none; margin-top: 25px; gap: 15px;">
          <input type="text" id="save-map-path-${username}" placeholder="/home/User/MinecraftMaps/" style="height: 32px; width: 250px;">
          
          <button class="btn min" id="save-map-${username}">Сохранить</button>
        </div>  
      </div>
    `;

    this.cards?.appendChild(wrapper);

    return wrapper;
  }

  */

  private updateBotCard(username: string, profile: BotProfile): void {
    const status = document.getElementById(`monitoring-status-${username}`) as HTMLElement;
    const proxy = document.getElementById(`monitoring-proxy-${username}`) as HTMLElement;
    const ping = document.getElementById(`monitoring-ping-${username}`) as HTMLElement;
    const health = document.getElementById(`monitoring-health-${username}`) as HTMLElement;

    if (health.innerText.split('/')[0].replace(' ', '') != profile.health.toString()) {
      const card = document.getElementById(`bot-card-${username}`);
      
      card?.classList.add('glow');

      setTimeout(() => {
        card?.classList.remove('glow');
      }, 300);
    }

    let statusColor = '';

    switch (profile.status) {
      case 'Connecting':
        statusColor = '#8f8f8fff'; break;
      case 'Online':
        statusColor = '#22ed17ff'; break;
      case 'Offline':
        statusColor = '#ed1717ff'; break;
    }

    status.innerText = profile.status;
    status.style.color = statusColor;

    proxy.innerText = profile.proxy.ip_address;
    ping.innerText = profile.ping.toString();
    health.innerText = profile.health.toString();
  }

  private initializeBotCard(username: string): void {
    this.chatHistoryFilters[username] = 'all';
    this.usernameList.push(username);

    const chatWrapper = this.createChatWrapper(username);
    // const mapWrapper = this.createMapWrapper(username);

    if (!this.chatMonitoring) {
      (document.getElementById(`open-chat-${username}`) as HTMLButtonElement).disabled = true;
      chatWrapper.remove();
    }

    /* Временно

    if (this.mapMonitoring) {
      (document.getElementById(`open-map-${username}`) as HTMLButtonElement).style.display = 'flex';
    } else {
      mapWrapper.remove();
    }

    */

    this.addTempListener(`bot-group-${username}`, 'input', async () => {
      try {
        const group = (document.getElementById(`bot-group-${username}`) as HTMLInputElement).value.replace(' ', '');

        await invoke('set_group', {
          nickname: username,
          group: group !== '' ? group : 'global'
        });
      } catch (error) {
        log(`Ошибка изменения группы ${username}: ${error}`, 'error');
      }
    });

    this.addTempListener(`open-chat-${username}`, 'click', () => chatWrapper.style.display = 'flex');
    this.addTempListener(`close-chat-${username}`, 'click', () => chatWrapper.style.display = 'none');

    /* Временно

    this.addTempListener(`open-map-${username}`, 'click', async () => {
      mapWrapper.style.display = 'flex';

      try {
        if (!document.getElementById(`map-image-${username}`) && !this.activeMapRenderings.get(username)) {
          this.activeMapRenderings.set(username, true);

          const old_map = document.getElementById(`map-image-${username}`);

          if (old_map) {
            old_map.remove();
          }

          (document.getElementById(`map-render-status-${username}`) as HTMLElement).style.display = 'flex';
          (document.getElementById(`map-render-progress-${username}`) as HTMLElement).style.display = 'flex';
          
          const base64_code = await invoke('render_map', { nickname: username }) as string;

          this.mapBase64Codes.set(username, base64_code);

          (document.getElementById(`map-render-status-${username}`) as HTMLElement).style.display = 'none';
          (document.getElementById(`map-render-progress-${username}`) as HTMLElement).style.display = 'none';

          this.activeMapRenderings.delete(username);

          const img = document.createElement('img');
          img.className = 'bot-map-image';
          img.id = `map-image-${username}`;
          img.src = `data:image/png;base64,${base64_code}`;
          img.draggable = false;

          const wrap = document.getElementById(`map-wrap-${username}`);

          wrap?.insertBefore(img, wrap.firstChild);

          (document.getElementById(`save-map-wrap-${username}`) as HTMLElement).style.display = 'flex';
        }
      } catch (error) {
        log(`Ошибка мониторинга (render-map): ${error}`, 'error');
      }
    });

    this.addTempListener(`remove-map-${username}`, 'click', async () => {
      this.mapBase64Codes.delete(username);
      document.getElementById(`map-image-${username}`)?.remove();
      (document.getElementById(`save-map-wrap-${username}`) as HTMLElement).style.display = 'none';
    });

    this.addTempListener(`save-map-${username}`, 'click', async () => {
      try {
        const base64code = this.mapBase64Codes.get(username);

        if (base64code) {
          const path = document.getElementById(`save-map-path-${username}`) as HTMLInputElement;

          await invoke('save_map', { 
            nickname: username, 
            path: path.value, 
            base64code: base64code 
          });
        } else {
          log('Ошибка мониторинга (save-map): Image not found', 'error');
        }
      } catch (error) {
        log(`Ошибка мониторинга (save-map): ${error}`, 'error');
      }
    });

    this.addTempListener(`close-map-${username}`, 'click', () => mapWrapper.style.display = 'none');

    */

    this.addTempListener(`disconnect-${username}`, 'click', async () => {
      try {
        await invoke('send_command', {
          command: 'disconnect_bot',
          options: {
            username: username
          }
        });
      } catch (error) {
        log(`Ошибка отключения бота ${username}: ${error}`, 'error');
      }
    });

    this.addTempListener(`reset-${username}`, 'click', async () => {
      try {
        await invoke('send_command', {
          command: 'reset_bot',
          options: {
            username: username
          }
        });
      } catch (error) {
        log(`Ошибка сбрасывания задач и состояний бота ${username}: ${error}`, 'error');
      }
    });

    this.addTempListener(`filter-chat-${username}`, 'click', () => {
      try {
        const content = document.getElementById(`monitoring-chat-content-${username}`);
        const type = document.getElementById(`select-chat-filter-${username}`) as HTMLSelectElement;

        const history = [...document.querySelectorAll(`[monitoring-message="${username}"]`).values()];
        
        content!.innerHTML = '';

        this.chatHistoryFilters[username] = type.value;

        history.forEach(m => this.filterMessage(type.value, m.textContent || '') ? content?.appendChild(m) : null);
      } catch (error) {
        log(`Ошибка фильтровки чата: ${error}`, 'error');
      }
    });

    this.addTempListener(`clear-chat-${username}`, 'click', () => {
      const messages = document.querySelectorAll(`[monitoring-message="${username}"]`);
      messages.forEach(msg => msg.remove());
      this.chatMessageCounter[username] = 0;
    });

    this.addTempListener(`chat-${username}`, 'keydown', async (e: Event) => {
      if ((e as KeyboardEvent).key === 'Enter') {
        const input = document.getElementById(`chat-message-${username}`) as HTMLInputElement;

        await invoke('send_command', { 
          command: 'send_message',
          options: {
            username: username,
            message: input.value
          }
        });

        input.value = '';
      }
    });
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