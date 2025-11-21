import fs from 'fs';

import { Bot, bots, activeBotsObjects } from '../bot/architecture.js';
import { mutateText } from '../tools/mutator.js';

// Структура конфига бота
interface BotConfig {
  address: string;
  version: string;
  quantity: number;
  delay: number;
  nickname: string;
  password: string;
  distance: string;
  timeout: number;
  skipValidation: string;
  registerCommand: string;
  registerTemplate: string;
  registerMinDelay: number;
  registerMaxDelay: number;
  loginCommand: string;
  loginTemplate: string;
  loginMinDelay: number;
  loginMaxDelay: number;
  rejoinQuantity: number;
  rejoinDelay: number;
  proxyList: string;
  useKeepAlive: boolean;
  usePhysics: boolean;
  useProxy: boolean;
  useProxyChecker: boolean;
  useAutoRegister: boolean;
  useAutoRejoin: boolean;
  useAutoLogin: boolean;
  useLogDeath: boolean;
  useSaveChat: boolean;
  useSavePlayers: boolean;
  useAiAgent: boolean;
  useDataAnalysis: boolean;
  useOptimization: boolean;
  useErrorCorrector: boolean;
}

export let ACTIVE = false;

// Класс, отвечающий за основные процессы
export default class Processor {
  public async start({ transmitter, config }: {
    transmitter: any,
    config: BotConfig
  }) {
    try {
      if (ACTIVE && activeBotsObjects.size > 0) {
        return {
          type: 'warning', 
          info: {
            success: true,
            message: 'Предупреждение (start-bots-process): Существуют активные боты, запуск невозможен'
          }
        }
      }

      changeActive(true);

      if (fs.existsSync('./data')) fs.rmSync('./data', { recursive: true, force: true });
      fs.mkdirSync('./data');

      const proxies = config.proxyList.split('\n');
      let index = 0;

      for (let i = 0; i < config.quantity; i++) {
        if (!ACTIVE) break;

        await new Promise(resolve => setTimeout(resolve, config.delay));

        const version = mutateText({ text: config.version, advanced: false, data: null });
        const nickname = mutateText({ text: config.nickname, advanced: false, data: null });
        const password = mutateText({ text: config.password, advanced: false, data: null });
        const registerCommand = mutateText({ text: config.registerCommand, advanced: false, data: null });
        const registerTemplate = mutateText({ text: config.registerTemplate, advanced: false, data: null });
        const loginCommand = mutateText({ text: config.loginCommand, advanced: false, data: null });
        const loginTemplate = mutateText({ text: config.loginTemplate, advanced: false, data: null });

        const skipValidation = config.skipValidation === 'false' ? false : true;

        let proxy;

        if (proxies.length >= i) {
          proxy = proxies[index];
        } else {
          index = 0;
          proxy = proxies[index];
        }

        index++;

        const bot = new Bot(
          transmitter,
          config.address,
          version,
          config.quantity,
          nickname,
          password,
          config.distance,
          config.timeout,
          skipValidation,
          registerCommand,
          registerTemplate,
          config.registerMinDelay,
          config.registerMaxDelay,
          loginCommand,
          loginTemplate,
          config.loginMinDelay,
          config.loginMaxDelay,
          config.rejoinQuantity,
          config.rejoinDelay,
          proxy,
          config.useKeepAlive,
          config.usePhysics,
          config.useProxy,
          config.useProxyChecker,
          config.useAutoRegister,
          config.useAutoLogin,
          config.useAutoRejoin,
          config.useLogDeath,
          config.useSaveChat,
          config.useSavePlayers,
          config.useOptimization
        );

        await bot.join(false);
      }

      await new Promise(resolve => setTimeout(resolve, config.delay));

      return {
        type: 'info', 
        info: {
          success: true,
          message: `Запуск ботов завершён`
        }
      }
    } catch (error) {
      return {
        type: 'error', 
        info: {
          success: false,
          message: `Ошибка (start-bots-process): ${error}`
        }
      }
    }
  }

  public async stop() {
    try {
      if (!activeBotsObjects) {
        return {
          type: 'error', 
          info: {
            success: false,
            message: 'Ошибка (stop-bots-process): Unable to get active bots'
          }
        }
      }

      changeActive(false);

      for (const [_, bot] of activeBotsObjects) { 
        bot.clean();
        bot.disconnect();
      }
      
      bots.clear();

      activeBotsObjects.clear();

      return {
        type: 'info', 
        info: {
          success: true,
          message: 'Боты успешно остановленны'
        }
      }
    } catch (error) {
      return {
        type: 'error', 
        info: {
          success: false,
          message: `Ошибка (stop-bots-process): ${error}`
        }
      }
    }
  }
}

// Функция для смены флага ACTIVE
export function changeActive(state: boolean) { ACTIVE = state };