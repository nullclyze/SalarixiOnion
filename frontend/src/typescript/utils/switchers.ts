import { controlWrappers, globalWrappers, latestControlWrapper } from '../main';
import { Language, translator } from '../modules/translator';

/**
 * Функция переключения видимости глобальных обёрток.
 * 
 * При открытии "control-container" функция показывает `latestControlWrapper` (последняя открытая обёртка).
 * Если `latestControlWrapper` не инициализирован, показывается "control-chat-container".
 */
const switchGlobalWrapper = (id: string, name: string): void => {
  globalWrappers.forEach(w => w && w.id === id ? w.el.style.display = 'flex' : w.el.style.display = 'none');
  controlWrappers.forEach(w => w.el.style.display = 'none');

  const split = id.split('-');
  let translatorTag = 'title:';

  if (split.length > 2) {
    let counter = 1;

    for (const pat of split) {
      if (counter < split.length) {
        counter === 1 ? translatorTag += pat : translatorTag += `-${pat}`;
      }

      counter++;
    }
  } else {
    translatorTag += split[0];
  }

  const currentSectionName = document.getElementById('current-section-name') as HTMLElement;
  currentSectionName.innerText = name;
  currentSectionName.setAttribute('translator-tag', translatorTag);

  const currentLanguage = (document.getElementById('interface_select_client-language') as HTMLSelectElement).value as Language;

  if (currentLanguage !== 'ru') {
    const translatorCache = translator.getCache();

    if (translatorCache) {
      for (const cacheEl of translatorCache) cacheEl.tags.includes(translatorTag) ? currentSectionName.innerText = cacheEl.lang[currentLanguage] : null;
    }
  }

  id === 'control-container' ? latestControlWrapper ? latestControlWrapper.style.display = 'flex' : (document.getElementById('control-chat-container') as HTMLElement).style.display = 'flex' : null;
}

/** Функция переключения видимости обёрток управления. */
const switchControlWrapper = (id: string): void => controlWrappers.forEach(c => c && c.id === id ? c.el.style.display = 'flex' : c.el.style.display = 'none');

export {
  switchGlobalWrapper,
  switchControlWrapper
}