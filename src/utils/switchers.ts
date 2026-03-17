import { controlWrappers, globalWrappers, latestControlWrapper } from '../main';

/**
 * Функция переключения видимости глобальных обёрток.
 * 
 * При открытии "control-wrapper" функция показывает `latestControlWrapper` (последняя открытая обёртка).
 * Если `latestControlWrapper` не инициализирован, показывается "control-chat-container".
 */
const switchGlobalWrapper = (id: string): void => {
  globalWrappers.forEach(w => w && w.id === id ? w.el.style.display = 'flex' : w.el.style.display = 'none');
  controlWrappers.forEach(w => w.el.style.display = 'none');
  id === 'control-wrapper' ? latestControlWrapper ? latestControlWrapper.style.display = 'flex' : (document.getElementById('control-chat-container') as HTMLElement).style.display = 'flex' : null;
}

/** Функция переключения видимости обёрток управления. */
const switchControlWrapper = (id: string): void => controlWrappers.forEach(c => c && c.id === id ? c.el.style.display = 'flex' : c.el.style.display = 'none');

export {
  switchGlobalWrapper,
  switchControlWrapper
}