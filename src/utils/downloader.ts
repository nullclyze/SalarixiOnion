import { logger } from './logger';

/** Функция загрузки JSON-контента с указанного URL. */
const download = async (url: string): Promise<any> => {
  try {
    const response = await fetch(url);
    if (!response.ok) return;
    const data = await response.json();
    return data;
  } catch (error) {
    logger.log(`Ошибка загрузки JSON-контента: ${error}`, 'error');
  }
}

export { download }