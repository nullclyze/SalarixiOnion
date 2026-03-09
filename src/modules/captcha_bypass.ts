import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';

import { log } from '../logger';

interface Captcha {
  type: 'web' | 'map';
  data: {
    url: string | null;
    base64: string | null;
  }
}

export class CaptchaBypassManager {
  private settings: HTMLElement | null = null;
  private cards: HTMLElement | null = null;
  private wrappers: HTMLElement | null = null;

  private listeners: Map<string, any> = new Map();

  public async init(): Promise<void> {
    this.settings = document.getElementById('anti-captcha-settings');
    this.cards = document.getElementById('anti-captcha-cards');
    this.wrappers = document.getElementById('anti-captcha-wrappers');

    await listen('anti-web-captcha', (event) => {
      try {
        const payload = event.payload as { captcha_url: string; nickname: string; };
        const captcha_url = payload.captcha_url;
        const nickname = payload.nickname;

        this.createCaptchaCard(nickname, {
          type: 'web',
          data: {
            url: captcha_url,
            base64: null
          }
        });
      } catch (error) {
        log(`Ошибка receive-payload: ${error}`, 'error');
      }
    });

    await listen('anti-map-captcha', (event) => {
      try {
        const payload = event.payload as { base64_code: string; nickname: string; };
        const base64 = payload.base64_code;
        const nickname = payload.nickname;

        this.createCaptchaCard(nickname, {
          type: 'map',
          data: {
            url: null,
            base64: base64
          }
        });
      } catch (error) {
        log(`Ошибка receive-payload: ${error}`, 'error');
      }
    });
  }

  public enable(type: string, mode: string): void {
    if (type === 'map' || mode === 'manual') {
      this.settings!.style.display = 'none';

      this.cards!.innerHTML = '';
      this.cards!.style.display = 'flex';
    }
  }

  public disable(): void {
    for (const [id, data] of this.listeners) {
      document.getElementById(id)?.removeEventListener(data.event, data.listener);
    }

    this.listeners.clear();

    this.cards!.innerHTML = '';
    this.cards!.style.display = 'none';

    this.wrappers!.innerHTML = '';

    this.settings!.style.display = 'flex';
  }

  private addTempListener(id: string, event: string, listener: EventListener): void {
    document.getElementById(id)?.addEventListener(event, listener);
    this.listeners.set(id, { event: event, listener: listener });
  }

  private createCaptchaCard(username: string, captcha: Captcha): void {
    const oldCard = document.getElementById(`bot-captcha-${username}`);

    if (oldCard) {
      oldCard.remove();
      document.getElementById(`captcha-wrapper-${username}`)?.remove();
    }

    const card = document.createElement('div');
    card.className = 'bot-captcha';
    card.id = `bot-captcha-${username}`;
    
    card.innerHTML = `
      <div class="head">
        <div class="username">${username}</div>
      </div>

      <div class="buttons">
        <button class="btn min pretty" id="solve-captcha-${username}">Решить</button>
        <button class="btn min pretty" id="remove-captcha-${username}">Удалить</button>
      </div>
    `;

    this.cards?.appendChild(card);

    this.initializeCaptchaCard(username, captcha);
  }

  private initializeCaptchaCard(username: string, captcha: Captcha): void {
    if (captcha.type === 'web') {
      this.addTempListener(`solve-captcha-${username}`, 'click', async () => {
        const url = captcha.data.url;

        if (url && url !== '') {
          await invoke('open_url', { url: url });
        }
      });
    } else {
      const wrapper = document.createElement('div');
      wrapper.className = 'cover';
      wrapper.id = `captcha-wrapper-${username}`;
      
      wrapper.innerHTML = `
        <div class="panel with-header" style="margin-bottom: 20px;">
          <div class="left">
            <div class="header">Капча бота ${username}</div>
          </div>

          <div class="right">
            <button class="btn min pretty" id="close-captcha-wrapper-${username}">
              ⨉
            </button>
          </div>
        </div>

        <div style="width: 100%; display: flex; justify-content: center; align-items: center; overflow-y: auto; height: 100%;">
          <img id="captcha-img-${username}" src="data:image/png;base64,${captcha.data.base64}" draggable="false" style="max-width: 100%; height: auto; image-rendering: pixelated;">
        </div>

        <div class="pretty-input-wrapper" style="margin-top: 20px;">
          <p class="signature">${username}</p>
          <input type="text" id="captcha-code-${username}" placeholder="Введите и отправьте код с капчи, нажав «Enter»">
        </div>
      `;

      this.wrappers?.appendChild(wrapper);

      const img = document.getElementById(`captcha-img-${username}`) as HTMLImageElement;
      img.onload = () => {
        const scale = Math.min(2, 800 / img.naturalWidth);
        img.style.width = (img.naturalWidth * scale) + 'px';
        img.style.height = (img.naturalHeight * scale) + 'px';
      };

      this.addTempListener(`solve-captcha-${username}`, 'click', () => wrapper.style.display = 'flex');
      this.addTempListener(`close-captcha-wrapper-${username}`, 'click', () => wrapper.style.display = 'none');

      this.addTempListener(`captcha-wrapper-${username}`, 'keydown', async (e: Event) => {
        if ((e as KeyboardEvent).key === 'Enter') {
          const input = document.getElementById(`captcha-code-${username}`) as HTMLInputElement;

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

    this.addTempListener(`remove-captcha-${username}`, 'click', async () => {
      document.getElementById(`bot-captcha-${username}`)?.remove();
      document.getElementById(`captcha-wrapper-${username}`)?.remove();
    });
  }
}   