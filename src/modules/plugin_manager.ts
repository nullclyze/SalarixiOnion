import { logger } from '../utils/logger';
import { download } from '../utils/downloader';
import { plugins } from '../common/structs';
import { process } from '../main';

class PluginManager {
  /** Метод инициализации функций, связанных с плагинами. */
  public async init(): Promise<void> {
    this.createCards();
    for (const name in plugins) this.updatePluginState(name, localStorage.getItem(`plugin-state:${name}`) === 'true');
  }

  /** Метод обновления и сохранения состояния плагина (включен / выключен). */
  private updatePluginState(name: string, state: boolean): void {
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

  /** Метод создания и инициализации карточек плагинов. */
  private createCards(): void {
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
            <button class="btn" plugin-toggler="true" plugin-name="${name}" state="true" id="${name}-toggler" style="color: #d61818;">Выключен</button>
            <button class="btn" plugin-open-description="true" path="${name}-plugin-description">Описание</button>
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
              <button class="btn min" plugin-close-description="true" path="${name}-plugin-description">
                ⨉
              </button>
            </div>
          </div>
  
          <div class="plugin-description" id="${name}-description"></div>
          <p class="plugin-latest-update">Последнее обновление плагина: <span id="${name}-latest-update">?</span></p>
        `;
  
        document.getElementById('plugins-container')?.appendChild(element);
      }
  
      document.querySelectorAll('[plugin-toggler="true"]').forEach(t => t.addEventListener('click', () => {
        const name = t.getAttribute('plugin-name') || '';
        const state = t.getAttribute('state') === 'true';
        this.updatePluginState(name, state);
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

  /** Метод загрузки описаний плагинов. */
  public async loadDescriptions(): Promise<void> {
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
}

const pluginManager = new PluginManager();

export { pluginManager }