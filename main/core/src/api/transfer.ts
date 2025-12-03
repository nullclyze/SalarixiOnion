export function transmit(res: any, type: string, data: any) {
  try {
    res.json({ type: type, data: data });
  } catch (error) {
    console.log(`Ошибка передачи данных: ${error}`);
  }
}