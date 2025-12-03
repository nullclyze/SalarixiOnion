import express from 'express';
import bodyParser from 'body-parser';
import cors from 'cors';
import path from 'path';
import { exec } from 'child_process';
import { transmit, SessionManager } from './transfer.js';
import { activeBots, bots } from '../bot/base.js';
import { Processor, changeActive } from '../common/processor.js';
import { Scripter } from '../common/scripter.js';
import { flow } from '../common/flow.js';
const session = new SessionManager();
const processor = new Processor();
const scripter = new Scripter();
const app = express();
app.use(bodyParser.urlencoded({ extended: true }));
app.use(bodyParser.json());
app.use(cors());
class Server {
    setup() {
        try {
            const port = 37621;
            app.listen(port);
        }
        catch (error) {
            console.log(`Критичная ошибка: ${error}`);
        }
    }
}
class Executor {
    async clear(response) {
        bots.clear();
        activeBots.clear();
        changeActive(false);
        transmit(response, 'system', {
            success: true,
            message: 'Данные очищены'
        });
    }
    async startProcess(request, response) {
        try {
            const options = request.body;
            transmit(response, 'system', {
                success: true,
                message: ''
            });
            setTimeout(async () => {
                const operation = await processor.start(session, options);
                session.msg('process', {
                    type: operation.type,
                    success: operation.info.success,
                    message: operation.info.message
                });
            }, 100);
        }
        catch (error) {
            transmit(response, 'error', {
                success: false,
                message: error
            });
        }
    }
    async stopProcess(response) {
        try {
            await processor.stop().then(({ type, info: { success, message } }) => {
                transmit(response, type, {
                    success: success,
                    message: message
                });
            });
            changeActive(false);
        }
        catch (error) {
            transmit(response, 'error', {
                success: false,
                message: error
            });
        }
    }
    async sessionProcess(request, response) {
        try {
            response.setHeader('Content-Type', 'text/event-stream');
            response.setHeader('Cache-Control', 'no-cache');
            response.setHeader('Connection', 'keep-alive');
            response.setHeader('Access-Control-Allow-Origin', '*');
            session.add('process:botting', response);
            session.msg('process:botting', {
                type: 'system',
                success: true,
                message: 'SSE-соединение установлено'
            });
            const interval = setInterval(() => {
                if (!processor.checkActive()) {
                    session.msg('process:botting', {
                        type: 'system',
                        success: true,
                        message: 'SSE-соединение закрыто'
                    });
                    setTimeout(() => session.del('process:botting'), 2000);
                    clearInterval(interval);
                }
            }, 2000);
            request.on('close', () => {
                session.del('process:botting');
                clearInterval(interval);
            });
        }
        catch {
            session.del('process:botting');
        }
    }
    async sessionLineGraphicActiveBots(request, response) {
        try {
            response.setHeader('Content-Type', 'text/event-stream');
            response.setHeader('Cache-Control', 'no-cache');
            response.setHeader('Connection', 'keep-alive');
            response.setHeader('Access-Control-Allow-Origin', '*');
            session.add('graphic:active-bots', response);
            const interval = setInterval(() => {
                const activeBotsQuantity = activeBots.size;
                session.msg('graphic:active-bots', {
                    activeBotsQuantity: activeBotsQuantity
                });
            }, 2000);
            request.on('close', () => {
                session.del('graphic:active-bots');
                clearInterval(interval);
            });
        }
        catch {
            session.del('graphic:active-bots');
        }
    }
    async sessionLineGraphicAverageLoad(request, response) {
        try {
            response.setHeader('Content-Type', 'text/event-stream');
            response.setHeader('Cache-Control', 'no-cache');
            response.setHeader('Connection', 'keep-alive');
            response.setHeader('Access-Control-Allow-Origin', '*');
            session.add('graphic:average-load', response);
            const interval = setInterval(() => {
                let num = 0;
                let quantity = 0;
                activeBots.forEach(bot => {
                    num += bot.profile.load;
                    quantity += 1;
                });
                const averageLoad = parseFloat((num / quantity).toFixed(2));
                session.msg('graphic:average-load', {
                    averageLoad: averageLoad
                });
            }, 2000);
            request.on('close', () => {
                session.del('graphic:average-load');
                clearInterval(interval);
            });
        }
        catch {
            session.del('graphic:average-load');
        }
    }
    async sessionChatHistoryMonitoring(request, response) {
        try {
            response.setHeader('Content-Type', 'text/event-stream');
            response.setHeader('Cache-Control', 'no-cache');
            response.setHeader('Connection', 'keep-alive');
            response.setHeader('Access-Control-Allow-Origin', '*');
            session.add('monitoring:chat-history', response);
            request.on('close', () => {
                session.del('monitoring:chat-history');
            });
        }
        catch {
            session.del('monitoring:chat-history');
        }
    }
    async sessionProfileDataMonitoring(request, response) {
        try {
            response.setHeader('Content-Type', 'text/event-stream');
            response.setHeader('Cache-Control', 'no-cache');
            response.setHeader('Connection', 'keep-alive');
            response.setHeader('Access-Control-Allow-Origin', '*');
            session.add('monitoring:profile-data', response);
            request.on('close', () => {
                session.del('monitoring:profile-data');
            });
        }
        catch {
            session.del('monitoring:profile-data');
        }
    }
    async executeScript(request, response) {
        try {
            const { script } = request.body;
            console.log(`( INFO : Script / Execute ) Полученные данные: ${JSON.stringify(request.body)}`);
            await scripter.execute(script)
                .then((status) => {
                if (status) {
                    transmit(response, 'info', {
                        success: true,
                        message: 'Скрипт успешно выполнен'
                    });
                }
                else {
                    transmit(response, 'error', {
                        success: false,
                        message: 'Не удалось выполнить скрипт'
                    });
                }
            });
        }
        catch (error) {
            transmit(response, 'error', {
                success: false,
                message: error
            });
        }
    }
    async startTerminalTool(request, response) {
        try {
            const { tool } = request.body;
            console.log(`( INFO : Terminal ) Полученные данные: ${JSON.stringify(request.body)}`);
            const runnerPath = path.join(path.dirname(process.execPath), 'cli', 'runner.exe');
            let toolPath = '';
            if (tool === 'crasher') {
                toolPath = path.join(path.dirname(process.execPath), 'cli', 'tools', 'crasher.exe');
            }
            exec(`cmd.exe /c start "" "${runnerPath}" "${toolPath}"`);
        }
        catch (error) {
            transmit(response, 'error', {
                success: false,
                message: error
            });
        }
    }
    async recreateBot(request, response) {
        try {
            const { nickname } = request.body;
            console.log(`( INFO : Recreate ) Полученные данные: ${JSON.stringify(request.body)}`);
            bots.forEach(async (bot) => {
                if (bot.nickname === nickname) {
                    const operation = await bot.recreate();
                    if (operation.success) {
                        transmit(response, 'info', {
                            success: true,
                            message: operation.message
                        });
                    }
                    else {
                        transmit(response, 'error', {
                            success: false,
                            message: operation.message
                        });
                    }
                    response.end();
                }
            });
        }
        catch (error) {
            transmit(response, 'error', {
                success: false,
                message: error
            });
        }
    }
}
const server = new Server();
const executor = new Executor();
app.post('/salarixi/system/data/clear', async (_, res) => await executor.clear(res));
app.post('/salarixi/process/start', async (req, res) => await executor.startProcess(req, res));
app.post('/salarixi/process/stop', async (_, res) => await executor.stopProcess(res).then(() => res.end()));
app.get('/salarixi/session/process', async (req, res) => await executor.sessionProcess(req, res));
app.get('/salarixi/session/graphic/line/active-bots', async (req, res) => await executor.sessionLineGraphicActiveBots(req, res));
app.get('/salarixi/session/graphic/line/average-load', async (req, res) => await executor.sessionLineGraphicAverageLoad(req, res));
app.get('/salarixi/session/monitoring/chat', async (req, res) => await executor.sessionChatHistoryMonitoring(req, res));
app.get('/salarixi/session/monitoring/bots', async (req, res) => await executor.sessionProfileDataMonitoring(req, res));
app.post('/salarixi/control/chat', async (req, res) => flow('chat', req.body, res));
app.post('/salarixi/control/action', async (req, res) => flow('action', req.body, res));
app.post('/salarixi/control/move', async (req, res) => flow('move', req.body, res));
app.post('/salarixi/control/imitation', async (req, res) => flow('imitation', req.body, res));
app.post('/salarixi/control/attack', async (req, res) => flow('attack', req.body, res));
app.post('/salarixi/control/flight', async (req, res) => flow('flight', req.body, res));
app.post('/salarixi/control/sprinter', async (req, res) => flow('sprinter', req.body, res));
app.post('/salarixi/control/ghost', async (req, res) => flow('ghost', req.body, res));
app.post('/salarixi/script/execute', async (req, res) => await executor.executeScript(req, res));
app.post('/salarixi/terminal/tools/start', async (req, res) => await executor.startTerminalTool(req, res).then(() => res.end()));
app.post('/salarixi/advanced/recreate', async (req, res) => await executor.recreateBot(req, res));
export { session, server };
