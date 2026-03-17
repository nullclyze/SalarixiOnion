import { logger } from '../utils/logger';

export async function downloadJsonContent(url: string): Promise<any> {
  try {
    const response = await fetch(url);
    
    if (!response.ok) return;
    
    const data = await response.json();

    return data;
  } catch (error) {
    logger.log(`Ошибка загрузки JSON-контента: ${error}`, 'error');
  }
}