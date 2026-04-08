type DateFormat = 'common' | 'exact';

/**
 * Функция конвертации даты.
 * 
 * Если дата содержит 1 символ, то к её началу прибавится 0.
 */
const conv = (num: number): string => num.toString().padStart(2, '0');

/** Функция получения текущей даты в определённом формате. */
const date = (format: DateFormat = 'common'): string => {
  const date = new Date();

  switch (format) {
    case 'common':
      const hours = conv(date.getHours());
      const minutes = conv(date.getMinutes());
      const seconds = conv(date.getSeconds());
      return `${hours}:${minutes}:${seconds}`;
    case 'exact':
      const day = conv(date.getDate());
      const month = conv(date.getMonth() + 1);
      const year = date.getFullYear().toString();
      return `${day}.${month}.${year}`;
  }
}

export { date }