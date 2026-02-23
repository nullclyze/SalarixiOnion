export function generateId(): string {
  let chars = '0123456789';

  let id = '';

  for (let i = 0; i < 6; i++) {
    const index = Math.floor(Math.random() * chars.length);
    id += chars[index];
  }

  return id;
}