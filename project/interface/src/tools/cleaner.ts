// Класс для очистки старой информации
class Cleaner {
  async purify(): Promise<{ success: boolean, message: string }> {
    try {
      const response = await fetch('http://localhost:37621/salarixi/system/data/clean', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' }
      });

      if (!response.ok) {
        return {
          success: false,
          message: 'Ошибка (clean-old-data): Incorrect server response'
        }
      }

      return {
        success: true,
        message: 'Старые данные успешно очищены'
      }
    } catch (error) {
      if (error instanceof TypeError) {
        return {
          success: false,
          message: `Ошибка (clean-old-data): Server not responding`
        }
      } else {
        return {
          success: false,
          message: `Ошибка (clean-old-data): ${error}`
        }
      }
    }
  }
}

export default Cleaner;