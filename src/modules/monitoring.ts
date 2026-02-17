import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';

import { log } from '../logger';
import { date } from '../helpers/date';


interface BotProfile {
  status: string;
  nickname: string;
  version: string;
  password: string;
  proxy: string;
  ping: number;
  health: number;
  satiety: number;
  registered: boolean;
  skin_is_set: boolean;
  captcha_caught: boolean;
  plugins_loaded: boolean;
  group: string;
}

export class MonitoringManager {
  private active: boolean = false;

  private usernameList: string[] = [];

  private statusText: HTMLElement | null = null;
  private botCardsContainer: HTMLElement | null = null;

  private chatMessageCounter: Record<string, number> = {};
  private chatHistoryFilters: Record<string, string> = {};

  private listeners: Map<string, any> = new Map();

  private activeMapRenderings: Map<string, boolean> = new Map();
  private mapBase64Codes: Map<string, string> = new Map();

  public extendedMonitoring: boolean = true;
  public chatMonitoring: boolean = true;
  public mapMonitoring: boolean = false;
  public maxChatHistoryLength: number = 0;
  public antiCaptchaType: string | null = null;

  public async init(): Promise<void> {
    this.statusText = document.getElementById('monitoring-status-text');
    this.botCardsContainer = document.getElementById('bot-cards-container');

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

    await listen('anti-web-captcha', (event) => {
      try {
        const payload = event.payload as { captcha_url: string; nickname: string; };
        const captcha_url = payload.captcha_url;
        const nickname = payload.nickname;

        document.getElementById(`solve-captcha-${nickname}`)?.setAttribute('captcha-url', captcha_url);
      } catch (error) {
        log(`Ошибка мониторинга (receive-payload): ${error}`, 'error');
      }
    });

    await listen('anti-map-captcha', (event) => {
      try {
        const payload = event.payload as { base64_code: string; nickname: string; };
        const base64_code = payload.base64_code;
        const nickname = payload.nickname;

        const img = document.createElement('img');
        img.className = 'bot-captcha-image';
        img.src = `data:image/png;base64,${base64_code}`;
        img.draggable = false;

        document.getElementById(`map-captcha-image-container-${nickname}`)?.appendChild(img);
      } catch (error) {
        log(`Ошибка мониторинга (receive-payload): ${error}`, 'error');
      }
    });
  }

  public wait(): void {
    this.statusText!.innerText = 'Ожидание активных ботов...';
    this.statusText!.style.color = '#646464f7';

    this.botCardsContainer!.innerHTML = '';
    this.botCardsContainer!.style.display = 'grid';
  }

  public enable(delay: number): void {
    try {
      this.active = true;

      let isFirst = true;

      const interval = setInterval(async () => {
        if (!this.active) {
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

            if (this.usernameList.includes(nickname)) {
              this.updateBotCard(nickname, profile, statusColor)
            } else {
              this.createBotCard(nickname, profile, statusColor);
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
    this.active = false;
    
    for (const [id, data] of this.listeners) {
      document.getElementById(id)?.removeEventListener(data.event, data.listener);
    }

    this.listeners.clear();

    this.botCardsContainer!.innerHTML = '';
    this.chatMessageCounter = {};
    this.chatHistoryFilters = {};
    this.usernameList = [];

    this.statusText!.innerText = 'Объекты ботов отсутствуют';
    this.statusText!.style.color = '#646464f7';
    this.statusText!.style.display = 'flex';

    this.botCardsContainer!.innerHTML = '';
    this.botCardsContainer!.style.display = 'none';
  }

  private addTempListener(id: string, event: string, listener: EventListener): void {
    document.getElementById(id)?.addEventListener(event, listener);
    this.listeners.set(id, { event: event, listener: listener });
  }

  private createBotCard(nickname: string, profile: BotProfile, statusColor: string): void {
    const steveIconPath = document.getElementById('steve-img') as HTMLImageElement;

    const groupNameExamples = [
      'killaura', 'bow_aim', 'AutoFarm', 
      'AFK', 'Travelers', 'miner', 
      'Stealer', 'Farmer', 'Spamming', 
      'PvE', 'PvP', 'afk_group'
    ];
    
    const groupNameExample = groupNameExamples[Math.floor(Math.random() * groupNameExamples.length)];

    const card = document.createElement('div');
    card.className = 'bot-card';
    card.id = `bot-card-${nickname}`;

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
        <p>Версия:<span class="value" id="bot-version-${nickname}">${profile.version}</span></p>
        <p>Пароль:<span class="value" id="bot-password-${nickname}">${profile.password}</span></p>
        <p>Прокси:<span class="value" id="bot-proxy-${nickname}">${profile.proxy}</span></p>
        <p id="extended-monitoring-ping-${nickname}" style="display: none;">Пинг:<span class="value" id="bot-ping-${nickname}">${profile.ping} мс</span></p>
        <p id="extended-monitoring-health-${nickname}" style="display: none;">Здоровье:<span class="value" id="bot-health-${nickname}">${profile.health} / 20</span></p>
        <p id="extended-monitoring-satiety-${nickname}" style="display: none;">Сытость:<span class="value" id="bot-satiety-${nickname}">${profile.satiety} / 20</span></p>
      </div>

      <div class="sep"></div>

      <input type="text" class="group-input" id="bot-group-${nickname}" placeholder="Группа, например: ${groupNameExample}">

      <div class="sep"></div>

      <button class="btn spec" id="open-chat-${nickname}" style="display: none;">Открыть чат</button>
      <button class="btn spec" id="open-map-${nickname}" style="display: none;">Открыть карту</button>
      <button class="btn spec" id="solve-captcha-${nickname}" style="display: none;">Решить капчу</button>
      <button class="btn spec" id="reset-${nickname}">Сбросить</button>
      <button class="btn spec" id="disconnect-${nickname}" style="margin-bottom: 12px;">Отключить</button>

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
              ⨉
            </button>
          </div>
        </div>

        <div class="chat-content">
          <div class="monitoring-content" id="monitoring-chat-content-${nickname}"></div>
        </div>

        <div style="display: flex; justify-content: center; align-items: center; gap: 10px; margin-top: 10px;">
          <p class="signature">${nickname}:</p>

          <input type="text" class="glass-input" control="this" id="this-chat-message-${nickname}" placeholder="Введите сообщение" style="height: 32px; width: 250px;">
        </div>
      </div>

      <div class="cover" id="map-${nickname}">
        <div class="panel">
          <div class="right">
            <button class="btn min pretty" id="remove-map-${nickname}">
              <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-trash-fill" viewBox="0 0 16 16">
                <path d="M2.5 1a1 1 0 0 0-1 1v1a1 1 0 0 0 1 1H3v9a2 2 0 0 0 2 2h6a2 2 0 0 0 2-2V4h.5a1 1 0 0 0 1-1V2a1 1 0 0 0-1-1H10a1 1 0 0 0-1-1H7a1 1 0 0 0-1 1zm3 4a.5.5 0 0 1 .5.5v7a.5.5 0 0 1-1 0v-7a.5.5 0 0 1 .5-.5M8 5a.5.5 0 0 1 .5.5v7a.5.5 0 0 1-1 0v-7A.5.5 0 0 1 8 5m3 .5v7a.5.5 0 0 1-1 0v-7a.5.5 0 0 1 1 0"/>
              </svg>
            </button>

            <button class="btn min pretty" id="close-map-${nickname}">
              ⨉
            </button>
          </div>
        </div>

        <div class="bot-map-wrap" id="map-wrap-${nickname}">
          <div class="bot-map-render-status" id="map-render-status-${nickname}">Генерация карты, пожайлуста, подождите...</div>
          <div class="bot-map-render-progress" id="map-render-progress-${nickname}">Прогресс (блоков): <span class="bot-map-render-progress-count" id="map-render-progress-count-${nickname}">0</span> / 65536</div>

          <div id="save-map-wrap-${nickname}" style="display: none; margin-top: 25px; gap: 15px;">
            <input type="text" class="glass-input" id="save-map-path-${nickname}" placeholder="/home/User/MinecraftMaps/" style="height: 32px; width: 250px;">
            
            <button class="btn min" id="save-map-${nickname}">Сохранить</button>
          </div>  
        </div>
      </div>
    `;

    this.botCardsContainer?.appendChild(card);
    this.chatHistoryFilters[nickname] = 'all';
    this.usernameList.push(nickname);
    this.initializeBotCard(nickname);
  }

  private updateBotCard(nickname: string, profile: BotProfile, statusColor: string): void {
    const status = document.getElementById(`bot-status-${nickname}`) as HTMLElement;
    const proxy = document.getElementById(`bot-proxy-${nickname}`) as HTMLElement;
    const ping = document.getElementById(`bot-ping-${nickname}`) as HTMLElement;
    const health = document.getElementById(`bot-health-${nickname}`) as HTMLElement;
    const satiety = document.getElementById(`bot-satiety-${nickname}`) as HTMLElement;

    if (health.innerText.split('/')[0].replace(' ', '') != profile.health.toString() || satiety.innerText.split('/')[0].replace(' ', '') != profile.satiety.toString()) {
      const card = document.getElementById(`bot-card-${nickname}`);
      
      card?.classList.add('glow');

      setTimeout(() => {
        card?.classList.remove('glow');
      }, 300);
    }

    status.innerHTML = `<span style="color: ${statusColor};">• ${profile.status}</span>`;
    proxy.innerText = profile.proxy;
    ping.innerText = `${profile.ping} мс`
    health.innerText = `${profile.health} / 20`;
    satiety.innerText = `${profile.satiety} / 20`;
  }

  private initializeBotCard(nickname: string): void {
    const chat = document.getElementById(`chat-${nickname}`);
    const map = document.getElementById(`map-${nickname}`);

    const ping = document.getElementById(`extended-monitoring-ping-${nickname}`);
    const health = document.getElementById(`extended-monitoring-health-${nickname}`);
    const satiety = document.getElementById(`extended-monitoring-satiety-${nickname}`);

    if (this.extendedMonitoring) {
      (ping as HTMLElement).style.display = 'flex';
      (health as HTMLElement).style.display = 'flex';
      (satiety as HTMLElement).style.display = 'flex';
    } else {
      ping?.remove();
      health?.remove();
      satiety?.remove();
    }

    if (this.chatMonitoring) {
      (document.getElementById(`open-chat-${nickname}`) as HTMLButtonElement).style.display = 'flex';
    } else {
      chat?.remove();
    }

    if (this.mapMonitoring) {
      (document.getElementById(`open-map-${nickname}`) as HTMLButtonElement).style.display = 'flex';
    } else {
      map?.remove();
    }

    if (this.antiCaptchaType) {
      (document.getElementById(`solve-captcha-${nickname}`) as HTMLButtonElement).style.display = 'flex';
    }

    this.addTempListener(`bot-group-${nickname}`, 'input', async () => {
      try {
        const group = (document.getElementById(`bot-group-${nickname}`) as HTMLInputElement).value.replace(' ', '');

        await invoke('set_group', {
          nickname: nickname,
          group: group !== '' ? group : 'global'
        });
      } catch (error) {
        log(`Ошибка изменения группы ${nickname}: ${error}`, 'error');
      }
    });

    this.addTempListener(`open-chat-${nickname}`, 'click', () => (chat as HTMLElement).style.display = 'flex');
    this.addTempListener(`close-chat-${nickname}`, 'click', () => (chat as HTMLElement).style.display = 'none');

    this.addTempListener(`open-map-${nickname}`, 'click', async () => {
      (map as HTMLElement).style.display = 'flex';

      try {
        if (!document.getElementById(`map-image-${nickname}`) && !this.activeMapRenderings.get(nickname)) {
          this.activeMapRenderings.set(nickname, true);

          const old_map = document.getElementById(`map-image-${nickname}`);

          if (old_map) {
            old_map.remove();
          }

          (document.getElementById(`map-render-status-${nickname}`) as HTMLElement).style.display = 'flex';
          (document.getElementById(`map-render-progress-${nickname}`) as HTMLElement).style.display = 'flex';
          
          const base64_code = await invoke('render_map', { nickname: nickname }) as string;

          this.mapBase64Codes.set(nickname, base64_code);

          (document.getElementById(`map-render-status-${nickname}`) as HTMLElement).style.display = 'none';
          (document.getElementById(`map-render-progress-${nickname}`) as HTMLElement).style.display = 'none';

          this.activeMapRenderings.delete(nickname);

          const img = document.createElement('img');
          img.className = 'bot-map-image';
          img.id = `map-image-${nickname}`;
          img.src = `data:image/png;base64,${base64_code}`;
          img.draggable = false;

          const wrap = document.getElementById(`map-wrap-${nickname}`);

          wrap?.insertBefore(img, wrap.firstChild);

          (document.getElementById(`save-map-wrap-${nickname}`) as HTMLElement).style.display = 'flex';
        }
      } catch (error) {
        log(`Ошибка мониторинга (render-map): ${error}`, 'error');
      }
    });

    this.addTempListener(`remove-map-${nickname}`, 'click', async () => {
      this.mapBase64Codes.delete(nickname);
      document.getElementById(`map-image-${nickname}`)?.remove();
      (document.getElementById(`save-map-wrap-${nickname}`) as HTMLElement).style.display = 'none';
    });

    this.addTempListener(`save-map-${nickname}`, 'click', async () => {
      try {
        const base64code = this.mapBase64Codes.get(nickname);

        if (base64code) {
          const path = document.getElementById(`save-map-path-${nickname}`) as HTMLInputElement;

          await invoke('save_map', { 
            nickname: nickname, 
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

    this.addTempListener(`close-map-${nickname}`, 'click', () => (map as HTMLElement).style.display = 'none');

    this.addTempListener(`disconnect-${nickname}`, 'click', async () => {
      try {
        await invoke('disconnect_bot', {
          nickname: nickname
        });
      } catch (error) {
        log(`Ошибка отключения бота ${nickname}: ${error}`, 'error');
      }
    });

    this.addTempListener(`reset-${nickname}`, 'click', async () => {
      try {
        await invoke('reset_bot', {
          nickname: nickname
        });
      } catch (error) {
        log(`Ошибка сбрасывания задач и состояний бота ${nickname}: ${error}`, 'error');
      }
    });

    this.addTempListener(`filter-chat-${nickname}`, 'click', () => {
      try {
        const content = document.getElementById(`monitoring-chat-content-${nickname}`);
        const type = document.getElementById(`select-chat-filter-${nickname}`) as HTMLSelectElement;

        const history = [...document.querySelectorAll(`#monitoring-message-${nickname}`).values()];
        
        content!.innerHTML = '';

        this.chatHistoryFilters[nickname] = type.value;

        history.forEach(m => this.filterMessage(type.value, m.textContent || '') ? content?.appendChild(m) : null);
      } catch (error) {
        log(`Ошибка фильтровки чата: ${error}`, 'error');
      }
    });

    this.addTempListener(`clear-chat-${nickname}`, 'click', () => {
      const messages = document.querySelectorAll(`#monitoring-message-${nickname}`);
      messages.forEach(msg => msg.remove());
      this.chatMessageCounter[nickname] = 0;
    });

    const sendMsg = async (input_id: string) => {
      const message = document.getElementById(input_id) as HTMLInputElement;

      await invoke('send_message', { 
        nickname: nickname,
        message: message.value
      });

      message.value = '';
    }

    this.addTempListener(`chat-${nickname}`, 'keydown', async (e: Event) => (e as KeyboardEvent).key === 'Enter' ? await sendMsg(`this-chat-message-${nickname}`) : null);
  
    switch (this.antiCaptchaType) {
      case 'web':
        document.getElementById(`solve-captcha-${nickname}`)?.setAttribute('captcha-url', 'none');

        this.addTempListener(`solve-captcha-${nickname}`, 'click', async () => {
          const captcha_url = (document.getElementById(`solve-captcha-${nickname}`) as HTMLButtonElement).getAttribute('captcha-url');

          if (captcha_url && captcha_url !== 'none') {
            await invoke('open_url', { url: captcha_url });
          }
        });

        break;

      case 'map':
        const container = document.createElement('div');
        container.className = 'cover';
        container.id = `map-captcha-image-${nickname}`;

        container.innerHTML = `
          <div class="panel">
            <div class="right">
              <button class="btn min pretty" id="close-map-captcha-image-${nickname}">
                ⨉
              </button>
            </div>
          </div>

          <div id="map-captcha-image-container-${nickname}" style="width: 100%; height: 410px;"></div>

          <div style="display: flex; justify-content: center; align-items: center; gap: 10px; margin-top: 20px;">
            <p class="signature">${nickname}:</p>

            <input type="text" class="glass-input" id="send-captcha-code-${nickname}" placeholder="Введите код с капчи" style="height: 32px; width: 250px;">
          </div>
        `;

        document.getElementById(`bot-card-${nickname}`)?.appendChild(container);

        this.addTempListener(`map-captcha-image-${nickname}`, 'keydown', async (e: Event) => (e as KeyboardEvent).key === 'Enter' ? await sendMsg(`send-captcha-code-${nickname}`) : null);

        this.addTempListener(`solve-captcha-${nickname}`, 'click', () => {
          (document.getElementById(`map-captcha-image-${nickname}`) as HTMLElement).style.display = 'flex';
        });

        this.addTempListener(`close-map-captcha-image-${nickname}`, 'click', () => {
          (document.getElementById(`map-captcha-image-${nickname}`) as HTMLElement).style.display = 'none';
        });

        break;
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