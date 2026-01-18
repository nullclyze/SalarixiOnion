import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';

import { log } from '../logger';
import { date } from '../helpers/date';


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

export class MonitoringManager {
  private active: boolean = false;

  private usernameList: string[] = [];

  private statusText: HTMLElement | null = null;
  private botCardsContainer: HTMLElement | null = null;

  public maxChatHistoryLength: number | null = null;
  private chatMessageCounter: Record<string, number> = {};
  private chatHistoryFilters: Record<string, string> = {};

  private listeners: Map<string, any> = new Map();

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

  public enable(): void {
    try {
      this.active = true;

      const steveIconPath = document.getElementById('steve-img') as HTMLImageElement;

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

              this.usernameList.push(nickname);

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

  private addListener(id: string, event: string, listener: EventListener): void {
    document.getElementById(id)?.addEventListener(event, listener);
    this.listeners.set(id, { event: event, listener: listener });
  }

  private initializeBotCard(nickname: string): void {
    const chat = document.getElementById(`chat-${nickname}`);

    this.addListener(`open-chat-${nickname}`, 'click', () => (chat as HTMLElement).style.display = 'flex');
    this.addListener(`close-chat-${nickname}`, 'click', () => (chat as HTMLElement).style.display = 'none');

    this.addListener(`disconnect-${nickname}`, 'click', async () => {
      try {
        const result = await invoke('disconnect_bot', {
          nickname: nickname
        }) as Array<string>;

        log(result[1], `log-${result[0]}`);
      } catch (error) {
        log(`Ошибка отключения бота ${nickname}: ${error}`, 'log-error');
      }
    });

    this.addListener(`solve-captcha-${nickname}`, 'click', () => {
      const captcha_url = (document.getElementById(`solve-captcha-${nickname}`) as HTMLButtonElement).getAttribute('captcha-url');

      if (captcha_url && captcha_url !== 'none') {
        invoke('open_url', { url: captcha_url });
      }
    });

    this.addListener(`filter-chat-${nickname}`, 'click', () => {
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

    this.addListener(`clear-chat-${nickname}`, 'click', () => {
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

    this.addListener(`chat-${nickname}`, 'keydown', async (e: Event) => (e as KeyboardEvent).key === 'Enter' ? await sendMsg() : null);
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