/*
// Структура данных в конфиге
interface ConfigStructure {
  'input': {
    'mainContainer': {
      'address': { id: string; value: string };
      'version': { id: string; value: string };
      'quantity': { id: string; value: number };
      'delay': { id: string; value: number };
    };
    'settingsContainer': {
      'nickname': { id: string; value: string };
      'password': { id: string; value: string };
      'timeout': { id: string; value: number };
      'distance': { id: string; value: string };
      'registerCommand': { id: string; value: string };
      'registerTemplate': { id: string; value: string };
      'loginCommand': { id: string; value: string };
      'loginTemplate': { id: string; value: string };
      'rejoinQuantity': { id: string; value: number };
      'rejoinDelay': { id: string; value: number };
    };
    'proxyContainer': {
      'proxyList': { id: string; value: string };
    };
    'controlContainer': {
      'message': { id: string; value: string };
      'spammingMinDelay': { id: string; value: number };
      'spammingMaxDelay': { id: string; value: number };
    };
    'scriptContainer': {
      'script': { id: string; value: string };
    };
  };
  'checkbox': {
    'settingsContainer': {
      'useKeepAlive': { id: string; value: boolean };
      'usePhysics': { id: string; value: boolean };
      'useProxy': { id: string; value: boolean };
      'useProxyChecker': { id: string; value: boolean };
      'useAutoRegister': { id: string; value: boolean };
      'useAutoLogin': { id: string; value: boolean };
      'useAutoRejoin': { id: string; value: boolean };
      'useLogDeath': { id: string; value: boolean };
      'useSaveChat': { id: string; value: boolean };
      'useSavePlayers': { id: string; value: boolean };
      'useAiAgent': { id: string; value: boolean };
      'useDataAnalysis': { id: string; value: boolean };
      'useOptimization': { id: string; value: boolean };
      'useErrorCorrector': { id: string; value: boolean };
    };
  };
}
  */

// Класс для отправки событий на Golang-сервер, который читает и изменяет конфиг
class Configurator {
  async write(data: object): Promise<{ success: boolean; invalidKey: boolean; message: string }> {
    try {
      const response = await fetch('http://localhost:37182/salarixi/utils/config/write', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ 
          key: 'salarixionion:1.0.0:ol13Rqk:config:write',
          config: data
        })
      });

      if (!response.ok) {
        return {
          success: false,
          invalidKey: false,
          message: 'Ошибка (write-config): Incorrect server response'
        }
      }

      const info = await response.json();

      return {
        success: info.success,
        invalidKey: info.invalidKey,
        message: info.message
      }
    } catch (error) {
      if (error instanceof TypeError) {
        return {
          success: false,
          invalidKey: false,
          message: `Ошибка (write-config): Server not responding`
        }
      } else {
        return {
          success: false,
          invalidKey: false,
          message: `Ошибка (write-config): ${error}`
        }
      }
    }
  }

  async load(): Promise<{ success: boolean; invalidKey: boolean; message: string; config?: any }> {
    try {
      const response = await fetch('http://localhost:37182/salarixi/utils/config/read', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify({ 
          key: 'salarixionion:1.0.0:Yi8jQ13e:config:read'
        })
      });

      if (!response.ok) {
        return {
          success: false,
          invalidKey: false,
          message: 'Ошибка (load-config): Incorrect server response'
        }
      }

      const data = await response.json();

      if (data.invalidKey) {
        return {
          success: true,
          invalidKey: true,
          message: `Ошибка (load-config): ${data.message}`,
          config: data.data
        }
      } else {
        return {
          success: true,
          invalidKey: false,
          message: data.message,
          config: data.data
        }
      }
    } catch (error) {
      if (error instanceof TypeError) {
        return {
          success: false,
          invalidKey: false,
          message: 'Ошибка (load-config): Server not responding'
        }
      } else {
        return {
          success: false,
          invalidKey: false,
          message: `Ошибка (load-config): ${error}`
        }
      }
    }
  }
}

export default Configurator;