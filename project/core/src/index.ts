import express from 'express';
import bodyParser from 'body-parser';
import cors from 'cors';
import path from 'path';
import { exec } from 'child_process';

import Validator from './tools/validator.js';
import { DisposableTransmitter, ReusableTransmitter } from './transmittor.js';
import { activeBotsObjects, bots } from './bot/architecture.js';
import { TasksList } from './bot/architecture.js';
import Processor from './common/processor.js';
import { ACTIVE, changeActive } from './common/processor.js';
import Scripter from './common/scripter.js';

const validator = new Validator();
const disposableTransmitter = new DisposableTransmitter();
const reusableTransmitter = new ReusableTransmitter();
const processor = new Processor();
const scripter = new Scripter();

const app = express();

app.use(bodyParser.urlencoded({ extended: true }));
app.use(bodyParser.json());
app.use(cors());

function setup() {
  try {
    const port = 37621;
    app.listen(port);
    console.log(`Сервер успешно запущен на http://localhost:${port}`);
  } catch (error) {
    console.log(`КРИТИЧНАЯ ОШИБКА: ${error}`);
  }
}

// Основной класс для выполнения операций
class Executor {
  public async clean() {
    bots.clear();
    
    activeBotsObjects.clear();

    changeActive(false);
  }

  public async startProcess(request: any, response: any) {
    try {
      const { 
        address, version, quantity, delay, 
        nickname, password, distance, timeout, skipValidation,
        registerCommand, registerTemplate, 
        registerMinDelay, registerMaxDelay,
        loginMinDelay, loginMaxDelay, loginCommand, 
        loginTemplate, rejoinQuantity, rejoinDelay, 
        proxyList, useKeepAlive, usePhysics, 
        useProxy, useProxyChecker, useAutoRegister, 
        useAutoLogin, useAutoRejoin, useLogDeath, 
        useSaveChat, useSavePlayers, useAiAgent, 
        useDataAnalysis, useOptimization, useErrorCorrector
      } = request.body;

      console.log(`( INFO : Process / Start ) Полученные данные: ${JSON.stringify(request.body)}`);

      const validation = validator.validateBotSettings({
        address: address,
        version: version,
        quantity: quantity,
        nickname: nickname
      });

      if (!validation.status) {
        disposableTransmitter.send(response, 'error', {
          success: false,
          message: validation.message
        }); return;
      } else {
        disposableTransmitter.send(response, 'system', {
          success: true,
          message: validation.message
        });
      }

      reusableTransmitter.add(response);

      setTimeout(async () => {
        const operation = await processor.start({
          transmitter: reusableTransmitter,
          config: {
            address: address, 
            version: version,
            quantity: quantity,
            delay: delay,
            nickname: nickname,
            password: password,
            distance: distance,
            timeout: timeout,
            skipValidation: skipValidation,
            registerCommand: registerCommand,
            registerTemplate: registerTemplate,
            registerMinDelay: registerMinDelay, 
            registerMaxDelay: registerMaxDelay,
            loginCommand: loginCommand,
            loginTemplate: loginTemplate,
            loginMinDelay: loginMinDelay, 
            loginMaxDelay: loginMaxDelay,
            rejoinQuantity: rejoinQuantity,
            rejoinDelay: rejoinDelay,
            proxyList: proxyList,
            useKeepAlive: useKeepAlive,
            usePhysics: usePhysics,
            useProxy: useProxy,
            useProxyChecker: useProxyChecker,
            useAutoRegister: useAutoRegister,
            useAutoRejoin: useAutoRejoin,
            useAutoLogin: useAutoLogin,
            useLogDeath: useLogDeath,
            useSaveChat: useSaveChat,
            useSavePlayers: useSavePlayers,
            useAiAgent: useAiAgent,
            useDataAnalysis: useDataAnalysis,
            useOptimization: useOptimization,
            useErrorCorrector: useErrorCorrector
          }
        });

        reusableTransmitter.send({ type: operation.type, data: {
          success: operation.info.success,
          message: operation.info.message
        }});
      }, 100);
    } catch (error) {
      disposableTransmitter.send(response, 'error', {
        success: false,
        message: error
      });
    }
  }

  public async stopProcess(response: any) {
    try {
      await processor.stop().then(({ type, info: { success, message } }) => {
        disposableTransmitter.send(response, type, {
          success: success,
          message: message
        });
      });

      changeActive(false);
    } catch (error) {
      disposableTransmitter.send(response, 'error', {
        success: false,
        message: error
      });
    }
  }

  public async sessionProcess(request: any, response: any) {
    try {
      console.log(`( INFO : Session / Process ) Запущена новая сессия`);
    
      response.setHeader('Content-Type', 'text/event-stream');
      response.setHeader('Cache-Control', 'no-cache');
      response.setHeader('Connection', 'keep-alive');
      response.setHeader('Access-Control-Allow-Origin', '*');

      reusableTransmitter.add(response);

      reusableTransmitter.send({ type: 'system', data: {
        success: true,
        message: 'SSE-соединение установлено'
      }});

      const interval = setInterval(async () => {
        if (!ACTIVE) {
          reusableTransmitter.send({ type: 'system', data: {
            success: true,
            message: 'SSE-соединение закрыто'
          }});

          response.end();
          response.socket?.destroy;
          clearInterval(interval);
        }
      }, 3000);

      request.on('close', () => {
        response.end();
        response.socket?.destroy;
        clearInterval(interval);
      });
    } catch (error) {
      console.log(`Ошибка сессии (session-process): ${error}`);
      response.end();
      response.socket?.destroy;
    }
  }

  public async sessionLineGraphicActiveBots(request: any, response: any) {
    try {
      console.log(`( INFO : Session / Graphic / Line / Active Bots ) Запущена новая сессия`);

      response.setHeader('Content-Type', 'text/event-stream');
      response.setHeader('Cache-Control', 'no-cache');
      response.setHeader('Connection', 'keep-alive');
      response.setHeader('Access-Control-Allow-Origin', '*');

      const interval = setInterval(async () => {
        const activeBotsQuantity = activeBotsObjects.size;

        response.write(`data: ${JSON.stringify({
          activeBotsQuantity: activeBotsQuantity
        })}\n\n`);
      }, 4000);

      request.on('close', () => {
        response.end();
        response.socket?.destroy;
        clearInterval(interval);
      });
    } catch (error) {
      console.log(`Ошибка сессии (session-line-graphic-active-bots): ${error}`);
      response.end();
      response.socket?.destroy;
    }
  }

  public async sessionLineGraphicAverageLoad(request: any, response: any) {
    try {
      console.log(`( INFO : Session / Graphic / Line / Average Load ) Запущена новая сессия`);

      response.setHeader('Content-Type', 'text/event-stream');
      response.setHeader('Cache-Control', 'no-cache');
      response.setHeader('Connection', 'keep-alive');
      response.setHeader('Access-Control-Allow-Origin', '*');

      const interval = setInterval(async () => {
        let num = 0;
        let quantity = 0;

        activeBotsObjects.forEach(bot => {
          num += bot.profile.load;
          quantity += 1;
        });

        const averageLoad = parseFloat((num / quantity).toFixed(2));

        response.write(`data: ${JSON.stringify({
          averageLoad: averageLoad
        })}\n\n`);
      }, 4000);

      request.on('close', () => {
        response.end();
        response.socket?.destroy;
        clearInterval(interval);
      });
    } catch (error) {
      console.log(`Ошибка сессии (session-line-graphic-average-load): ${error}`);
      response.end();
      response.socket?.destroy;
    }
  }

  public async sessionChatMonitoring(request: any, response: any) {
    try {
      console.log(`( INFO : Session / Monitoring / Chat ) Запущена новая сессия`);

      response.setHeader('Content-Type', 'text/event-stream');
      response.setHeader('Cache-Control', 'no-cache');
      response.setHeader('Connection', 'keep-alive');
      response.setHeader('Access-Control-Allow-Origin', '*');

      const interval = setInterval(async () => {
        if (!ACTIVE) return;

        for (const [nickname, bot] of activeBotsObjects) {
          const chatHistory = bot.get('chat-history');

          if (chatHistory && Array.isArray(chatHistory)) {
            for (const message of chatHistory) {
              response.write(`data: ${JSON.stringify({
                nickname: nickname,
                type: message.type,
                text: message.text
              })}\n\n`);
            }
          }
        }
      }, 2000);

      request.on('close', () => {
        response.end();
        response.socket?.destroy;
        clearInterval(interval);
      });
    } catch (error) {
      console.log(`Ошибка сессии (session-chat-monitoring): ${error}`);
      response.end();
      response.socket?.destroy;
    }
  }

  public async sessionBotsMonitoring(request: any, response: any) {
    try {
      console.log(`( INFO : Session / Monitoring / Bots ) Запущена новая сессия`);

      response.setHeader('Content-Type', 'text/event-stream');
      response.setHeader('Cache-Control', 'no-cache');
      response.setHeader('Connection', 'keep-alive');
      response.setHeader('Access-Control-Allow-Origin', '*');

      const interval = setInterval(async () => {
        if (!ACTIVE) return;

        for (const [nickname, bot] of bots) {
          const version = bot.profile.version;
          const password = bot.profile.password;
          const proxyType = bot.profile.proxyType.toUpperCase() || '-';
          const proxy = bot.profile.proxy.split(':')[0] || 'Не использует';
          const status = bot.profile.status;
          const reputation = bot.profile.reputation;
          const load = bot.profile.load;
          const ping = bot.profile.ping;

          let reputationColor: string;
          let loadColor: string;
          let pingColor: string;

          if (reputation <= 20) {
            reputationColor = '#ed1717ff';
          } else if (reputation > 20 && reputation <= 60) {
            reputationColor = '#eddf17ff';
          } else {
            reputationColor = '#22ed17ff';
          }

          if (load <= 0) {
            loadColor = '#8f8f8fff';
          } else if (load > 0 && load <= 20) {
            loadColor = '#22ed17ff';
          } else if (load > 20 && load <= 40) {
            loadColor = '#28c305ff';
          } else if (load > 40 && load <= 60) {
            loadColor = '#eddf17ff';
          } else if (load > 60 && load <= 80) {
            loadColor = '#d1800fff';
          } else {
            loadColor = '#ed1717ff';
          }

          if (ping <= 60 && ping > 0) {
            pingColor = '#22ed17ff';
          } else if (ping > 60 && ping <= 360) {
            pingColor = '#eddf17ff';
          } else if (ping > 360 && ping <= 10000) {
            pingColor = '#ed1717ff';
          } else {
            pingColor = '#8f8f8fff';
          }

          response.write(`data: ${JSON.stringify({
            nickname: nickname,
            status: status.text,
            statusColor: status.color,
            version: version,
            password: password,
            proxyType: proxyType,
            proxy: proxy,
            reputation: reputation,
            reputationColor: reputationColor,
            load: `${load}%`,
            loadColor: loadColor,
            ping: ping ? `${ping} мс` : '?',
            pingColor: pingColor
          })}\n\n`);
        }
      }, 1500);

      request.on('close', () => {
        response.end();
        response.socket?.destroy;
        clearInterval(interval);
      });
    } catch (error) {
      console.log(`Ошибка сессии (session-bots-monitoring): ${error}`);
      response.end();
      response.socket?.destroy;
    }
  }

  public async controlChat(request: any, response: any) {
    try {
      const { type } = request.body;

      console.log(`( INFO : Control / Chat ) Полученные данные: ${JSON.stringify(request.body)}`);

      if (type === 'default') {
        const { data } = request.body;

        for (const [_, bot] of activeBotsObjects) {
          bot.chat('default', data);
        }

        disposableTransmitter.send(response, 'info', {
          success: true,
          message: `Боты отправили сообщение «${data.message}» в чат`
        }); return;
      } else if (type === 'spamming') {
        const { data } = request.body;
        const state: 'start' | 'stop' = data.state;

        let table = '';

        if (state === 'start') {
          const { data } = request.body;

          for (const [nickname, bot] of activeBotsObjects) {
            const flag = bot.tasks.spamming.status;

            if (!flag) {
              table += `\n[ БОТ ]\t${nickname} ~ %hcgStart%sc`;
            } else {
              table += `\n[ БОТ ]\t${nickname} ~ %hcyIgnore%sc`;
            }

            bot.chat('spamming', data);
          }

          disposableTransmitter.send(response, 'info', {
            success: true,
            message: `Спамминг ==> Команда ${(state as string).toUpperCase()}${table}`
          }); return;
        } else if (state === 'stop') {
          for (const [nickname, bot] of activeBotsObjects) {
            const flag = bot.tasks.spamming.status;

            if (flag) {
              table += `\n[ БОТ ]\t${nickname} ~ %hcrStop%sc`;
            } else {
              table += `\n[ БОТ ]\t${nickname} ~ %hcyIgnore%sc`;
            }

            bot.chat('spamming', { state: 'stop' });
          }

          disposableTransmitter.send(response, 'info', {
            success: true,
            message: `Спамминг ==> Команда ${(state as string).toUpperCase()}${table}`
          }); return;
        }
      }
    } catch (error) {
      disposableTransmitter.send(response, 'error', {
        success: false,
        message: error
      });
    }
  }

  public async controlAction(request: any, response: any) {
    try {
      const { state, action } = request.body;

      console.log(`( INFO : Control / Action ) Полученные данные: ${JSON.stringify(request.body)}`);

      let name = '';

      switch (action) {
        case 'jumping':
          name = 'Джампинг'; break;
        case 'shifting':
          name = 'Шифтинг'; break;
        case 'waving':
          name = 'Махание рукой'; break;
        case 'looking':
          name = 'Осмотр'; break;
        case 'spinning':
          name = 'Спиннинг'; break;
      }

      let table = '';

      if (state === 'start') {
        const { data } = request.body;

        for (const [nickname, bot] of activeBotsObjects) {
          let flag = false;

          switch (action) {
            case 'jumping':
              flag = bot.tasks.jumping.status; break;
            case 'shifting':
              flag = bot.tasks.shifting.status; break;
            case 'waving':
              flag = bot.tasks.waving.status; break;
            case 'looking':
              flag = bot.tasks.looking.status; break;
            case 'spinning':
              flag = bot.tasks.spinning.status; break;
          }

          if (!flag) {
            table += `\n[ БОТ ]\t${nickname} ~ %hcgStart%sc`;
          } else {
            table += `\n[ БОТ ]\t${nickname} ~ %hcyIgnore%sc`;
          }

          bot.action('start', action, data);
        }

        disposableTransmitter.send(response, 'info', {
          success: true,
          message: `${name} ==> Команда ${(state as string).toUpperCase()}${table}`
        }); return;
      } else if (state === 'stop') {
        for (const [nickname, bot] of activeBotsObjects) {
          let flag = true;

          switch (action) {
            case 'jumping':
              flag = bot.tasks.jumping.status; break;
            case 'shifting':
              flag = bot.tasks.shifting.status; break;
            case 'waving':
              flag = bot.tasks.waving.status; break;
            case 'looking':
              flag = bot.tasks.looking.status; break;
            case 'spinning':
              flag = bot.tasks.spinning.status; break;
          }

          if (flag) {
            table += `\n[ БОТ ]\t${nickname} ~ %hcrStop%sc`;
          } else {
            table += `\n[ БОТ ]\t${nickname} ~ %hcyIgnore%sc`;
          }
          
          bot.action('stop', action, undefined);
        }

        disposableTransmitter.send(response, 'info', {
          success: true,
          message: `${name} ==> Команда ${(state as string).toUpperCase()}${table}`
        }); return;
      }
    } catch (error) {
      disposableTransmitter.send(response, 'error', {
        success: false,
        message: error
      });
    }
  }

  public async controlMovement(request: any, response: any) {
    try {
      const { state, direction } = request.body;

      console.log(`( INFO : Control / Movement ) Полученные данные: ${JSON.stringify(request.body)}`);

      let info: { name: string, flag: TasksList | undefined } = { name: '', flag: undefined };

      switch (direction) {
        case 'forward':
          info = { name: 'Движение вперёд', flag: 'movementForward' }; break;
        case 'back':
          info = { name: 'Движение назад', flag: 'movementBack' }; break;
        case 'left':
          info = { name: 'Движение влево', flag: 'movementLeft' }; break;
        case 'right':
          info = { name: 'Движение вправо', flag: 'movementRight' }; break;
      }

      if (!info.flag) return;

      let table = '';

      if (state === 'start') {
        const { data } = request.body;

        for (const [nickname, bot] of activeBotsObjects) {
          let flag = bot.tasks[info.flag].status;

          if (!flag) {
            table += `\n[ БОТ ]\t${nickname} ~ %hcgStart%sc`;
          } else {
            table += `\n[ БОТ ]\t${nickname} ~ %hcyIgnore%sc`;
          }
          
          bot.movement('start', direction, data);
        }

        disposableTransmitter.send(response, 'info', {
          success: true,
          message: `${info.name} ==> Команда ${(state as string).toUpperCase()}${table}`
        }); return;
      } else if (state === 'stop') {
        for (const [nickname, bot] of activeBotsObjects) {
          let flag = bot.tasks[info.flag].status;

          if (flag) {
            table += `\n[ БОТ ]\t${nickname} ~ %hcrStop%sc`;
          } else {
            table += `\n[ БОТ ]\t${nickname} ~ %hcyIgnore%sc`;
          }

          bot.movement('stop', direction, undefined);
        }

        disposableTransmitter.send(response, 'info', {
          success: true,
          message: `${info.name} ==> Команда ${(state as string).toUpperCase()}${table}`
        }); return;
      }
    } catch (error) {
      disposableTransmitter.send(response, 'error', {
        success: false,
        message: error
      });
    }
  }

  public async controlImitation(request: any, response: any) {
    try {
      const { state, type } = request.body;

      console.log(`( INFO : Control / Imitation ) Полученные данные: ${JSON.stringify(request.body)}`);

      let name = '';

      switch (type) {
        case 'hybrid':
          name = 'Гибридная имитация'; break;
        case 'walking':
          name = 'Имитация хотьбы'; break;
        case 'looking':
          name = 'Имитация осмотра'; break;
        case 'speaking':
          name = 'Имитация общения'; break;
      }

      let table = '';

      if (state === 'start') {
        const { data } = request.body;

        for (const [nickname, bot] of activeBotsObjects) {
          let flag = false;

          switch (type) {
            case 'hybrid':
              flag = bot.tasks.hybridImitation.status; break;
            case 'walking':
              flag = bot.tasks.walkingImitation.status; break;
          }

          if (!flag) {
            table += `\n[ БОТ ]\t${nickname} ~ %hcgStart%sc`;
          } else {
            table += `\n[ БОТ ]\t${nickname} ~ %hcyIgnore%sc`;
          }
          
          bot.imitation('start', type, data);
        }

        disposableTransmitter.send(response, 'info', {
          success: true,
          message: `${name} ==> Команда ${(state as string).toUpperCase()}${table}`
        }); return;
      } else if (state === 'stop') {
        for (const [nickname, bot] of activeBotsObjects) {
          let flag = true;

          switch (type) {
            case 'hybrid':
              flag = bot.tasks.hybridImitation.status; break;
            case 'walking':
              flag = bot.tasks.walkingImitation.status; break;
          }

          if (flag) {
            table += `\n[ БОТ ]\t${nickname} ~ %hcrStop%sc`;
          } else {
            table += `\n[ БОТ ]\t${nickname} ~ %hcyIgnore%sc`;
          }

          bot.imitation('stop', type, undefined);
        }

        disposableTransmitter.send(response, 'info', {
          success: true,
          message: `${name} ==> Команда ${(state as string).toUpperCase()}${table}`
        }); return;
      }
    } catch (error) {
      disposableTransmitter.send(response, 'error', {
        success: false,
        message: error
      });
    }
  }

  public async controlAttack(request: any, response: any) {
    try {
      const { state } = request.body;

      console.log(`( INFO : Control / Attack ) Полученные данные: ${JSON.stringify(request.body)}`);

      let table = '';

      if (state === 'start') {
        const { data } = request.body;

        for (const [nickname, bot] of activeBotsObjects) {
          const flag = bot.tasks.attacking.status;

          if (!flag) {
            table += `\n[ БОТ ]\t${nickname} ~ %hcgStart%sc`;
          } else {
            table += `\n[ БОТ ]\t${nickname} ~ %hcyIgnore%sc`;
          }
          
          bot.attack('start', data);
        }

        disposableTransmitter.send(response, 'info', {
          success: true,
          message: `Атака ==> Команда ${(state as string).toUpperCase()}${table}`
        }); return;
      } else if (state === 'stop') {
        for (const [nickname, bot] of activeBotsObjects) {
          const flag = bot.tasks.attacking.status;

          if (flag) {
            table += `\n[ БОТ ]\t${nickname} ~ %hcrStop%sc`;
          } else {
            table += `\n[ БОТ ]\t${nickname} ~ %hcyIgnore%sc`;
          }

          bot.attack('stop', undefined);
        }

        disposableTransmitter.send(response, 'info', {
          success: true,
          message: `Атака ==> Команда ${(state as string).toUpperCase()}${table}`
        }); return;
      }
    } catch (error) {
      disposableTransmitter.send(response, 'error', {
        success: false,
        message: error
      });
    }
  }

  public async controlFlight(request: any, response: any) {
    try {
      const { state, type } = request.body;

      console.log(`( INFO : Control / Flight ) Полученные данные: ${JSON.stringify(request.body)}`);

      let table = '';

      if (state === 'start') {
        const { options } = request.body;

        for (const [nickname, bot] of activeBotsObjects) {
          const flag = bot.tasks.flight.status;

          if (!flag) {
            table += `\n[ БОТ ]\t${nickname} ~ %hcgStart%sc`;
          } else {
            table += `\n[ БОТ ]\t${nickname} ~ %hcyIgnore%sc`;
          }
          
          bot.flight('start', type, options);
        }

        disposableTransmitter.send(response, 'info', {
          success: true,
          message: `Полёт ==> Команда ${(state as string).toUpperCase()}${table}`
        }); return;
      } else if (state === 'stop') {
        for (const [nickname, bot] of activeBotsObjects) {
          const flag = bot.tasks.flight.status;

          if (flag) {
            table += `\n[ БОТ ]\t${nickname} ~ %hcrStop%sc`;
          } else {
            table += `\n[ БОТ ]\t${nickname} ~ %hcyIgnore%sc`;
          }

          bot.flight('stop', type, undefined);
        }

        disposableTransmitter.send(response, 'info', {
          success: true,
          message: `Полёт ==> Команда ${(state as string).toUpperCase()}${table}`
        }); return;
      }
    } catch (error) {
      disposableTransmitter.send(response, 'error', {
        success: false,
        message: error
      });
    }
  }

  public async controlSprinter(request: any, response: any) {
    try {
      const { state } = request.body;

      console.log(`( INFO : Control / Sprinter ) Полученные данные: ${JSON.stringify(request.body)}`);

      let table = '';

      if (state === 'start') {
        const { data } = request.body;

        for (const [nickname, bot] of activeBotsObjects) {
          const flag = bot.tasks.sprinter.status;

          if (!flag) {
            table += `\n[ БОТ ]\t${nickname} ~ %hcgStart%sc`;
          } else {
            table += `\n[ БОТ ]\t${nickname} ~ %hcyIgnore%sc`;
          }
          
          bot.sprinter('start', data);
        }

        disposableTransmitter.send(response, 'info', {
          success: true,
          message: `Спринтер ==> Команда ${(state as string).toUpperCase()}${table}`
        }); return;
      } else if (state === 'stop') {
        for (const [nickname, bot] of activeBotsObjects) {
          const flag = bot.tasks.sprinter.status;

          if (flag) {
            table += `\n[ БОТ ]\t${nickname} ~ %hcrStop%sc`;
          } else {
            table += `\n[ БОТ ]\t${nickname} ~ %hcyIgnore%sc`;
          }

          bot.sprinter('stop', undefined);
        }

        disposableTransmitter.send(response, 'info', {
          success: true,
          message: `Спринтер ==> Команда ${(state as string).toUpperCase()}${table}`
        }); return;
      }
    } catch (error) {
      disposableTransmitter.send(response, 'error', {
        success: false,
        message: error
      });
    }
  }

  public async controlGhost(request: any, response: any) {
    try {
      const { state } = request.body;

      console.log(`( INFO : Control / Ghost ) Полученные данные: ${JSON.stringify(request.body)}`);

      let table = '';

      if (state === 'start') {
        const { data } = request.body;

        for (const [nickname, bot] of activeBotsObjects) {
          const flag = bot.tasks.ghost.status;

          if (!flag) {
            table += `\n[ БОТ ]\t${nickname} ~ %hcgStart%sc`;
          } else {
            table += `\n[ БОТ ]\t${nickname} ~ %hcyIgnore%sc`;
          }
          
          bot.ghost('start', data);
        }

        disposableTransmitter.send(response, 'info', {
          success: true,
          message: `Призрак ==> Команда ${(state as string).toUpperCase()}${table}`
        }); return;
      } else if (state === 'stop') {
        for (const [nickname, bot] of activeBotsObjects) {
          const flag = bot.tasks.ghost.status;

          if (flag) {
            table += `\n[ БОТ ]\t${nickname} ~ %hcrStop%sc`;
          } else {
            table += `\n[ БОТ ]\t${nickname} ~ %hcyIgnore%sc`;
          }

          bot.ghost('stop', undefined);
        }

        disposableTransmitter.send(response, 'info', {
          success: true,
          message: `Призрак ==> Команда ${(state as string).toUpperCase()}${table}`
        }); return;
      }
    } catch (error) {
      disposableTransmitter.send(response, 'error', {
        success: false,
        message: error
      });
    }
  }

  public async executeScript(request: any, response: any) {
    try {
      const { script } = request.body;

      console.log(`( INFO : Script / Execute ) Полученные данные: ${JSON.stringify(request.body)}`);

      await scripter.execute({ script: script })
        .then((status) => {
          if (status) {
            disposableTransmitter.send(response, 'info', {
              success: true,
              message: 'Скрипт успешно выполнен'
            }); return;
          } else {
            disposableTransmitter.send(response, 'error', {
              success: false,
              message: 'Не удалось выполнить скрипт'
            }); return;
          }
        });
    } catch (error) {
      disposableTransmitter.send(response, 'error', {
        success: false,
        message: error
      });
    }
  }

  public async startTerminalTool(request: any, response: any) {
    try {
      const { tool } = request.body;

      console.log(`( INFO : Terminal ) Полученные данные: ${JSON.stringify(request.body)}`);

      const runnerPath = path.join(path.dirname(process.execPath), 'cli', 'runner.exe');

      let toolPath = '';

      if (tool === 'crasher') {
        toolPath = path.join(path.dirname(process.execPath), 'cli', 'tools', 'crasher.exe');
      }

      exec(`cmd.exe /c start "" "${runnerPath}" "${toolPath}"`);
    } catch (error) {
      disposableTransmitter.send(response, 'error', {
        success: false,
        message: error
      });
    }
  }

  public async recreateBot(request: any, response: any) {
    try {
      const { nickname } = request.body;

      console.log(`( INFO : Recreate ) Полученные данные: ${JSON.stringify(request.body)}`);

      bots.forEach(async bot => {
        if (bot.nickname === nickname) {
          const operation = await bot.recreate();

          if (operation.success) {
            disposableTransmitter.send(response, 'info', {
              success: true,
              message: operation.message
            });
          } else {
            disposableTransmitter.send(response, 'error', {
              success: false,
              message: operation.message
            });
          }

          response.end();
        }
      });
    } catch (error) {
      disposableTransmitter.send(response, 'error', {
        success: false,
        message: error
      });
    }
  }
}

const executor = new Executor();

app.post('/salarixi/system/data/clean', async (_, res) => await executor.clean().then(() => res.end()));

app.post('/salarixi/process/start', async (req, res) => await executor.startProcess(req, res));
app.post('/salarixi/process/stop', async (_, res) => await executor.stopProcess(res).then(() => res.end()));

app.get('/salarixi/session/process', async (req, res) => await executor.sessionProcess(req, res));
app.get('/salarixi/session/graphic/line/active-bots', async (req, res) => await executor.sessionLineGraphicActiveBots(req, res));
app.get('/salarixi/session/graphic/line/average-load', async (req, res) => await executor.sessionLineGraphicAverageLoad(req, res));
app.get('/salarixi/session/monitoring/chat', async (req, res) => await executor.sessionChatMonitoring(req, res));
app.get('/salarixi/session/monitoring/bots', async (req, res) => await executor.sessionBotsMonitoring(req, res));

app.post('/salarixi/control/chat', async (req, res) => await executor.controlChat(req, res));
app.post('/salarixi/control/action', async (req, res) => await executor.controlAction(req, res));
app.post('/salarixi/control/movement', async (req, res) => await executor.controlMovement(req, res));
app.post('/salarixi/control/imitation', async (req, res) => await executor.controlImitation(req, res));
app.post('/salarixi/control/attack', async (req, res) => await executor.controlAttack(req, res));
app.post('/salarixi/control/flight', async (req, res) => await executor.controlFlight(req, res));
app.post('/salarixi/control/sprinter', async (req, res) => await executor.controlSprinter(req, res));
app.post('/salarixi/control/ghost', async (req, res) => await executor.controlGhost(req, res));

app.post('/salarixi/script/execute', async (req, res) => await executor.executeScript(req, res));

app.post('/salarixi/terminal/tools/start', async (req, res) => await executor.startTerminalTool(req, res).then(() => res.end()));

app.post('/salarixi/advanced/recreate', async (req, res) => await executor.recreateBot(req, res));

setup();