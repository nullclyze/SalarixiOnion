import express from 'express';
import bodyParser from 'body-parser';
import cors from 'cors';
import path from 'path';
import { exec } from 'child_process';

import { transmit } from './transfer.js';
import { add, del, msg } from './session.js';
import { activeBots } from '../bot/base.js'
import { Processor, changeActive } from '../common/processor.js';
import { ScriptController } from '../common/scripter.js';
import { flow } from '../common/flow.js';

const processor = new Processor();
const scripter = new ScriptController();

export const app = express();

app.use(bodyParser.urlencoded({ extended: true }));
app.use(bodyParser.json());
app.use(cors());

function clear(response: any) {
  activeBots.clear();

  changeActive(false);

  transmit(response, 'system', {
    success: true,
    message: 'Данные очищены'
  });
}

async function botting(action: 'start' | 'stop', request: any, response: any) {
  try {
    if (action === 'start') {
      const options = request.body;

      transmit(response, 'system', {
        success: true,
        message: 'Начало запуска...'
      });

      del('process:botting')

      setTimeout(async () => await processor.start(options), 100);
    } else if (action === 'stop') {

      await processor.stop().then(({ type, info: { success, message } }) => {
        transmit(response, type, {
          success: success,
          message: message
        });
      });

      changeActive(false);
    }
  } catch (error) {
    transmit(response, 'error', {
      success: false,
      message: error
    });
  }
}

function stream(response: any) {
  response.setHeader('Content-Type', 'text/event-stream');
  response.setHeader('Cache-Control', 'no-cache');
  response.setHeader('Connection', 'keep-alive');
  response.setHeader('Access-Control-Allow-Origin', '*');
}

async function sessionBotting(request: any, response: any) {
  try {
    stream(response);

    add('process:botting', response);

    msg('process:botting', {
      type: 'system',
      success: true,
      message: 'SSE-соединение установлено'
    });

    const interval = setInterval(() => {
      if (!processor.checkActive()) {
        msg('process:botting', {
          type: 'system',
          success: true,
          message: 'SSE-соединение закрыто'
        });

        setTimeout(() => del('process:botting'), 2000);

        clearInterval(interval);
      }
    }, 2000);

    request.on('close', () => {
      del('process:botting');
      clearInterval(interval);
    });
  } catch {
    del('process:botting');
  }
}

async function sessionGraphicActiveBots(request: any, response: any) {
  try {
    stream(response);

    add('graphic:active-bots', response);

    const interval = setInterval(() => {
      const activeBotsQuantity = activeBots.size;

      msg('graphic:active-bots', {
        activeBotsQuantity: activeBotsQuantity
      });
    }, 2000);

    request.on('close', () => {
      del('graphic:active-bots');
      clearInterval(interval);
    });
  } catch {
    del('graphic:active-bots');
  }
}

async function sessionGraphicAverageLoad(request: any, response: any) {
  try {
    stream(response);

    add('graphic:average-load', response);

    const interval = setInterval(() => {
      let num = 0;
      let quantity = 0;

      activeBots.forEach(bot => {
        num += bot.profile.load;
        quantity += 1;
      });

      const averageLoad = parseFloat((num / quantity).toFixed(2));

      msg('graphic:average-load', {
        averageLoad: averageLoad
      });
    }, 2000);

    request.on('close', () => {
      del('graphic:average-load');
      clearInterval(interval);
    });
  } catch {
    del('graphic:average-load');
  }
}

async function executeScript(request: any, response: any) {
  try {
    const { script } = request.body;

    await scripter.execute(script).then(() => {
      transmit(response, 'info', {
        success: true,
        message: 'Скрипт успешно выполнен'
      });
    });
  } catch (error) {
    transmit(response, 'error', {
      success: false,
      message: error
    });
  }
}

async function stopScript(response: any) {
  try {
    await scripter.stop().then(() => {
      transmit(response, 'info', {
        success: true,
        message: 'Скрипт остановлен'
      });
    });
  } catch (error) {
    transmit(response, 'error', {
      success: false,
      message: error
    });
  }
}

class Executor {
  async startTerminalTool(request: any, response: any) {
    try {
      const { tool } = request.body;

      const runnerPath = path.join(path.dirname(process.execPath), 'cli', 'runner.exe');

      let toolPath = '';

      if (tool === 'crasher') {
        toolPath = path.join(path.dirname(process.execPath), 'cli', 'tools', 'crasher.exe');
      }

      exec(`cmd.exe /c start "" "${runnerPath}" "${toolPath}"`);
    } catch (error) {
      transmit(response, 'error', {
        success: false,
        message: error
      });
    }
  }

  async recreateBot(request: any, response: any) {
    try {
      const { nickname } = request.body;

      activeBots.forEach(async bot => {
        if (bot.nickname === nickname) {
          const operation = await bot.recreate();

          if (operation.success) {
            transmit(response, 'info', {
              success: true,
              message: operation.message
            });
          } else {
            transmit(response, 'error', {
              success: false,
              message: operation.message
            });
          }

          response.end();
        }
      });
    } catch (error) {
      transmit(response, 'error', {
        success: false,
        message: error
      });
    }
  }
}

const executor = new Executor();

app.get('/salarixi/session/botting', async (req, res) => await sessionBotting(req, res));
app.get('/salarixi/session/graphic/active-bots', async (req, res) => await sessionGraphicActiveBots(req, res));
app.get('/salarixi/session/graphic/average-load', async (req, res) => await sessionGraphicAverageLoad(req, res));
app.get('/salarixi/session/monitoring/profile-data', async (_, res) => add('monitoring:profile-data', res));
app.get('/salarixi/session/monitoring/chat-history', async (_, res) => add('monitoring:chat-history', res));

app.post('/salarixi/system/data/clear', (_, res) => clear(res));

app.post('/salarixi/botting/start', async (req, res) => await botting('start', req, res));
app.post('/salarixi/botting/stop', async (_, res) => await botting('stop', null, res));

app.post('/salarixi/control/chat', (req, res) => flow('chat', req.body, res));
app.post('/salarixi/control/action', (req, res) => flow('action', req.body, res));
app.post('/salarixi/control/move', (req, res) => flow('move', req.body, res));
app.post('/salarixi/control/imitation', (req, res) => flow('imitation', req.body, res));
app.post('/salarixi/control/attack', (req, res) => flow('attack', req.body, res));
app.post('/salarixi/control/flight', (req, res) => flow('flight', req.body, res));
app.post('/salarixi/control/sprinter', (req, res) => flow('sprinter', req.body, res));
app.post('/salarixi/control/ghost', (req, res) => flow('ghost', req.body, res));

app.post('/salarixi/script/execute', async (req, res) => await executeScript(req, res));
app.post('/salarixi/script/stop', async (_, res) => await stopScript(res));

app.post('/salarixi/terminal/tools/start', async (req, res) => await executor.startTerminalTool(req, res).then(() => res.end()));

app.post('/salarixi/advanced/recreate', async (req, res) => await executor.recreateBot(req, res));