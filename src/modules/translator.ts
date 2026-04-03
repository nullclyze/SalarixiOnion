import { download } from '../utils/downloader';
import { logger } from '../utils/logger';

export type Language = 'ru' | 'en';

class Translator {
  private cache: any;

  constructor() {
    this.cache = null;
  }

  /** Метод инициализации функций, связанных с переводчиком. */
  public async init(): Promise<void> {
    const langSelect = document.getElementById('interface_select_client-language') as HTMLSelectElement;
    langSelect.addEventListener('change', async () => await this.translate(langSelect.value as Language));
    if (langSelect.value !== 'ru') await this.translate(langSelect.value as Language);
  }

  /** Метод получения кэша. */
  public getCache(): any {
    return this.cache;
  }

  /** Метод получения и установки актуального перевода. */
  private async translate(lang: Language): Promise<void> {
    try {
      if (!this.cache) {
        const content = await download('https://raw.githubusercontent.com/nullclyze/SalarixiOnion/refs/heads/main/salarixi.lang.json');

        if (content) {
          this.cache = content['map'];
        } else {
          logger.log(`Ошибка загрузки перевода: Failed to load JSON-content`, 'error');
          return;
        }
      }

      document.querySelectorAll<HTMLElement>('[translator-tag]').forEach(e => {
        const tag = e.getAttribute('translator-tag');
        if (!tag) return;
        for (const el of this.cache) el.tags.includes(tag) ? e.innerText = el.lang[lang] : null;
      });
    } catch (error) {
      logger.log(`Ошибка перевода: ${error}`, 'error');
    }
  }
}

const translator = new Translator();

export { translator }