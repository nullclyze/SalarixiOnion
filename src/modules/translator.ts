import { downloadJsonContent } from '../downloader/downloader';
import { log } from '../logger';

export type Language = 'ru' | 'en';

let cache_map: any = null;

export async function translate(lang: Language) {
  try {
    let map: any = null;

    if (!cache_map) {
      const content = await downloadJsonContent('https://raw.githubusercontent.com/nullclyze/SalarixiOnion/refs/heads/main/salarixi.lang.json');
    
      if (content) {
        map = content['map'];
      } else {
        log(`Ошибка загрузки перевода: Failed to load JSON-content`, 'error');
      }
    } else {
      map = cache_map;
    }

    if (map) {
      document.querySelectorAll<HTMLElement>('[translator-tag]').forEach(e => {
        const tag = e.getAttribute('translator-tag');

        if (tag) {
          for (const el of map) {
            if (el.tags.includes(tag)) {
              e.innerText = el.lang[lang];
            }
          }
        }
      });
    }
  } catch (error) {
    log(`Ошибка перевода: ${error}`, 'error');
  }
}