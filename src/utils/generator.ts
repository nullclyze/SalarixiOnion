/** Функция генерации случайного ID из 6 цифр. */
const generateId = (): string => {
  let chars = '0123456789';
  let id = '';
  for (let i = 0; i < 6; i++) id += chars[Math.floor(Math.random() * chars.length)];
  return id;
}

export { generateId }