export function date(format: string = 'common'): string {
  const date = new Date();

  const hours = date.getHours().toString().padStart(2, '0');
  const minutes = date.getMinutes().toString().padStart(2, '0');
  const seconds = date.getSeconds().toString().padStart(2, '0');

  if (format === 'common') {
    return `${hours}:${minutes}:${seconds}`;
  } else if (format === 'exact') {
    const day = date.getDate().toString().padStart(2, '0');
    const month = (date.getMonth() + 1).toString().padStart(2, '0');
    const year = date.getFullYear().toString();

    return `${day}.${month}.${year}`;
  } else {
    return `${hours}:${minutes}`;
  }
}