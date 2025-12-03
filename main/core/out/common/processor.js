import fs from 'fs';
import { Bot, activeBots } from '../bot/base.js';
import { mutateText } from '../bot/utils/mutator.js';
import { chooseRandomElementFromArray } from '../bot/utils/generator.js';
export let ACTIVE = false;
export class Processor {
    async start(options) {
        try {
            if (ACTIVE && activeBots.size > 0) {
                return { type: 'warning', info: {
                        success: false,
                        message: 'Предупреждение (start-bots-process): Существуют активные боты, запуск невозможен'
                    } };
            }
            changeActive(true);
            if (fs.existsSync('./data'))
                fs.rmSync('./data', { recursive: true, force: true });
            fs.mkdirSync('./data');
            let proxies = [];
            if (options.proxyList) {
                proxies = String(options.proxyList).split('\n');
            }
            let index = 0;
            for (let i = 0; i < options.quantity; i++) {
                if (!ACTIVE)
                    break;
                await new Promise(resolve => setTimeout(resolve, options.delay));
                const version = mutateText({ text: options.version, advanced: false, data: null });
                const nickname = mutateText({ text: options.nickname, advanced: false, data: null });
                const password = mutateText({ text: options.password, advanced: false, data: null });
                const registerCommand = mutateText({ text: options.registerCommand, advanced: false, data: null });
                const registerTemplate = mutateText({ text: options.registerTemplate, advanced: false, data: null });
                const loginCommand = mutateText({ text: options.loginCommand, advanced: false, data: null });
                const loginTemplate = mutateText({ text: options.loginTemplate, advanced: false, data: null });
                const skipValidation = options.skipValidation === 'false' ? false : true;
                let proxy;
                if (proxies.length >= i) {
                    proxy = proxies[index];
                }
                else {
                    proxy = chooseRandomElementFromArray(proxies);
                }
                index++;
                const bot = new Bot(options.address, version, options.quantity, nickname, password, options.distance, options.timeout, skipValidation, registerCommand, registerTemplate, options.registerMinDelay, options.registerMaxDelay, loginCommand, loginTemplate, options.loginMinDelay, options.loginMaxDelay, options.rejoinQuantity, options.rejoinDelay, options.dataUpdateFrequency, options.chatHistoryLength, proxy, options.useKeepAlive, options.usePhysics, options.useProxy, options.useProxyChecker, options.useAutoRegister, options.useAutoLogin, options.useAutoRejoin, options.useLogDeath, options.useSaveChat, options.useSavePlayers, options.useOptimization);
                await bot.join(false);
            }
            await new Promise(resolve => setTimeout(resolve, options.delay));
            return { type: 'info', info: {
                    success: true,
                    message: `Запуск ботов завершён`
                } };
        }
        catch (error) {
            return { type: 'error', info: {
                    success: false,
                    message: `Ошибка (start-bots-process): ${error}`
                } };
        }
    }
    async stop() {
        try {
            if (!activeBots) {
                return { type: 'error', info: {
                        success: false,
                        message: 'Ошибка (stop-bots-process): Unable to get active bots'
                    } };
            }
            changeActive(false);
            activeBots.forEach(async (bot) => {
                await bot.disconnect();
            });
            activeBots.clear();
            return { type: 'info', info: {
                    success: true,
                    message: 'Боты успешно остановленны'
                } };
        }
        catch (error) {
            return { type: 'error', info: {
                    success: false,
                    message: `Ошибка (stop-bots-process): ${error}`
                } };
        }
    }
    checkActive() {
        if (ACTIVE || activeBots.size > 0) {
            return true;
        }
        else {
            return false;
        }
    }
}
export function changeActive(state) { ACTIVE = state; }
;
