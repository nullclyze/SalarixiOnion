// Функция для получения локального времени в заданном формате
const date = (format: string = 'H:M:S'): string => {
  const date = new Date();

  const hours = date.getHours().toString().padStart(2, '0');
  const minutes = date.getMinutes().toString().padStart(2, '0');
  const seconds = date.getSeconds().toString().padStart(2, '0');

  if (format === 'H:M:S') {
    return `${hours}:${minutes}:${seconds}`;
  } else if (format === 'H:M') {
    return `${hours}:${minutes}`;
  } else if (format === 'M:S') {
    return `${minutes}:${seconds}`;
  } else {
    return `${hours}:${minutes}:${seconds}`;
  }
}

export default date;