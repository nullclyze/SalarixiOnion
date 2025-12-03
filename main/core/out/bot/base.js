import mineflayer from 'mineflayer';
import fs from 'fs';
import { msg } from '../api/session.js';
import { connection } from './connection/connection.js';
import { Spoofer } from './connection/spoofer.js';
import { generateNumber } from './utils/generator.js';
import { updateBotProfileData, updateBotChatHistory } from './update/update.js';
export let activeBots = new Map();
export class Bot {
    constructor(address, version, quantity, nickname, password, distance, timeout, skipValidation, registerCommand, registerTemplate, registerMinDelay, registerMaxDelay, loginCommand, loginTemplate, loginMinDelay, loginMaxDelay, rejoinQuantity, rejoinDelay, dataUpdateFrequency, chatHistoryLength, proxy, useKeepAlive, usePhysics, useProxy, useProxyChecker, useAutoRegister, useAutoLogin, useAutoRejoin, useLogDeath, useSaveChat, useSavePlayers, useOptimization) {
        this.address = address;
        this.version = version;
        this.quantity = quantity;
        this.nickname = nickname;
        this.password = password;
        this.distance = distance;
        this.timeout = timeout;
        this.skipValidation = skipValidation;
        this.registerCommand = registerCommand;
        this.registerTemplate = registerTemplate;
        this.registerMinDelay = registerMinDelay;
        this.registerMaxDelay = registerMaxDelay;
        this.loginCommand = loginCommand;
        this.loginTemplate = loginTemplate;
        this.loginMinDelay = loginMinDelay;
        this.loginMaxDelay = loginMaxDelay;
        this.rejoinQuantity = rejoinQuantity;
        this.rejoinDelay = rejoinDelay;
        this.dataUpdateFrequency = dataUpdateFrequency;
        this.chatHistoryLength = chatHistoryLength;
        this.proxy = proxy;
        this.useKeepAlive = useKeepAlive;
        this.usePhysics = usePhysics;
        this.useProxy = useProxy;
        this.useProxyChecker = useProxyChecker;
        this.useAutoRegister = useAutoRegister;
        this.useAutoLogin = useAutoLogin;
        this.useAutoRejoin = useAutoRejoin;
        this.useLogDeath = useLogDeath;
        this.useSaveChat = useSaveChat;
        this.useSavePlayers = useSavePlayers;
        this.useOptimization = useOptimization;
        this.object = undefined;
        this.destroyed = false;
        this.spoofer = undefined;
        this.address = address;
        this.version = version;
        this.quantity = quantity;
        this.nickname = nickname;
        this.password = password;
        this.distance = distance;
        this.timeout = timeout;
        this.skipValidation = skipValidation;
        this.registerCommand = registerCommand;
        this.registerTemplate = registerTemplate;
        this.registerMinDelay = registerMinDelay;
        this.registerMaxDelay = registerMaxDelay;
        this.loginCommand = loginCommand;
        this.loginTemplate = loginTemplate;
        this.loginMinDelay = loginMinDelay;
        this.loginMaxDelay = loginMaxDelay;
        this.rejoinQuantity = rejoinQuantity;
        this.rejoinDelay = rejoinDelay;
        this.proxy = proxy;
        this.useKeepAlive = useKeepAlive;
        this.usePhysics = usePhysics;
        this.useProxy = useProxy;
        this.useProxyChecker = useProxyChecker;
        this.useAutoRegister = useAutoRegister;
        this.useAutoLogin = useAutoLogin;
        this.useAutoRejoin = useAutoRejoin;
        this.useLogDeath = useLogDeath;
        this.useSaveChat = useSaveChat;
        this.useSavePlayers = useSavePlayers;
        this.useOptimization = useOptimization;
        this.profile = {
            nickname: this.nickname,
            password: this.password,
            version: this.version,
            registered: false,
            rejoinProcess: 'sleep',
            status: { text: 'Соединение...', color: '#8f8f8fff' },
            rejoinQuantity: 0,
            proxyType: '-',
            proxy: '-',
            load: 0,
            ping: 0,
            pingChecker: undefined,
            playerSaver: undefined,
            updateProfile: undefined
        };
        this.tasks = {
            basic: { status: false, load: 0 },
            analysis: { status: false, load: 0 },
            message: { status: false, load: 0 },
            spamming: { status: false, load: 0 },
            jumping: { status: false, load: 0 },
            shifting: { status: false, load: 0 },
            waving: { status: false, load: 0 },
            spinning: { status: false, load: 0 },
            looking: { status: false, load: 0 },
            moveForward: { status: false, load: 0 },
            moveBack: { status: false, load: 0 },
            moveLeft: { status: false, load: 0 },
            moveRight: { status: false, load: 0 },
            hybridImitation: { status: false, load: 0 },
            walkingImitation: { status: false, load: 0 },
            attack: { status: false, load: 0 },
            spoofing: { status: false, load: 0 },
            flight: { status: false, load: 0 },
            sprinter: { status: false, load: 0 },
            ghost: { status: false, load: 0 }
        };
        const interval = setInterval(() => {
            if (this.destroyed) {
                clearInterval(interval);
                return;
            }
            updateBotProfileData({ nickname: this.nickname, profile: this.profile });
        }, this.dataUpdateFrequency);
    }
    async sleep(delay) {
        await new Promise(resolve => setTimeout(resolve, delay));
    }
    async create() {
        try {
            let object = null;
            this.profile.status = { text: 'Соединение...', color: '#8f8f8fff' };
            let viewDistance = 'tiny';
            if (this.distance !== 'tiny' && this.distance !== 'short' && this.distance !== 'normal' && this.distance !== 'far') {
                viewDistance = 'short';
            }
            else {
                viewDistance = this.distance;
            }
            let options = {
                host: String(this.address.split(':')[0]),
                username: this.nickname,
                auth: 'offline',
                port: Number(this.address.split(':')[1]),
                version: this.version,
                viewDistance: viewDistance,
                keepAlive: this.useKeepAlive,
                physicsEnabled: this.usePhysics,
                closeTimeout: this.timeout,
                hideErrors: true,
                skipValidation: this.skipValidation
            };
            if (this.useOptimization) {
                options.plugins = {
                    anvil: false,
                    book: false,
                    boss_bar: false,
                    breath: false,
                    conversions: false,
                    digging: false,
                    enchantment_table: false,
                    explosion: false,
                    fishing: false,
                    furnace: false,
                    generic_place: false,
                    loader: false,
                    painting: false,
                    particle: false,
                    rain: false,
                    ray_trace: false,
                    scoreboard: false,
                    sound: false,
                    team: false,
                    title: false,
                    villager: false
                };
            }
            if (this.useProxy && this.proxy) {
                let proxyType;
                if (this.proxy?.startsWith('socks5://')) {
                    proxyType = 'socks5';
                }
                else if (this.proxy?.startsWith('socks4://')) {
                    proxyType = 'socks4';
                }
                else if (this.proxy?.startsWith('http://')) {
                    proxyType = 'http';
                }
                else {
                    proxyType = '@none';
                }
                if (proxyType === '@none')
                    return;
                if (proxyType === 'socks5' || proxyType === 'socks4') {
                    this.profile.proxyType = proxyType;
                    const url = new URL(this.proxy);
                    const host = url.hostname;
                    const port = parseInt(url.port) || 80;
                    const username = url.username;
                    const password = url.password;
                    if (!host || !port)
                        return;
                    this.profile.proxy = `${host}:${port}`;
                    const socket = await connection({
                        type: proxyType,
                        host: host,
                        port: port,
                        timeout: this.timeout,
                        address: this.address,
                        username: username,
                        password: password
                    });
                    object = mineflayer.createBot({
                        ...options,
                        connect: (client) => {
                            client.setSocket(socket);
                            client.emit('connect');
                        }
                    });
                }
                else {
                    this.profile.proxyType = 'http';
                    const url = new URL(this.proxy);
                    const host = url.hostname;
                    const port = parseInt(url.port) || 80;
                    const username = url.username;
                    const password = url.password;
                    if (!host || !port)
                        return;
                    this.profile.proxy = `${host}:${port}`;
                    const socket = await connection({
                        type: proxyType,
                        host: host,
                        port: port,
                        timeout: this.timeout,
                        address: this.address,
                        username: username,
                        password: password
                    });
                    object = mineflayer.createBot({
                        ...options,
                        connect: (client) => {
                            client.setSocket(socket);
                            client.emit('connect');
                        }
                    });
                }
            }
            else {
                this.profile.proxyType = '-';
                this.profile.proxy = 'Не использует';
                object = mineflayer.createBot(options);
            }
            return object;
        }
        catch {
            return null;
        }
    }
    async recreate() {
        try {
            if (this.object) {
                this.object.end('@salarixi:disconnect');
                activeBots.delete(this.nickname);
            }
            this.clean();
            this.profile.rejoinProcess = 'active';
            this.profile.status = { text: 'Соединение...', color: '#8f8f8fff' };
            await this.sleep(generateNumber('float', 800, 1800));
            const operation = await this.join(true);
            if (operation?.success) {
                return {
                    success: true,
                    message: `Бот ${this.nickname} пересоздан`
                };
            }
            else {
                return {
                    success: false,
                    message: `Не удалось пересоздать ${this.nickname}`
                };
            }
        }
        catch (error) {
            return {
                success: false,
                message: `Ошибка пересоздания ${this.nickname}: ${error}`
            };
        }
    }
    async join(isRecreate) {
        try {
            const object = await this.create();
            if (!object) {
                return {
                    success: false,
                    message: `Ошибка создания ${this.nickname}: Bot object damaged`
                };
            }
            this.object = object;
            await this.setup();
            await this.handling(isRecreate ? 'recreate' : 'join');
            return {
                success: true,
                message: `Бот ${this.nickname} успешно создан`
            };
        }
        catch (error) {
            msg('process:botting', {
                type: 'error',
                message: `Ошибка создания ${this.nickname}: ${error}`
            });
            return {
                success: false,
                message: `Ошибка создания ${this.nickname}: ${error}`
            };
        }
    }
    async rejoin() {
        try {
            this.profile.status = { text: 'Соединение...', color: '#8f8f8fff' };
            this.profile.rejoinProcess = 'active';
            this.profile.rejoinQuantity++;
            this.clean();
            activeBots.delete(this.profile.nickname);
            await this.sleep(generateNumber('float', 1000, 2800));
            this.object = await this.create();
            if (!this.object)
                return false;
            await this.setup();
            await this.handling('rejoin');
            return true;
        }
        catch {
            return false;
        }
    }
    async setup() {
        if (!this.object)
            return;
        activeBots.set(this.nickname, this);
        this.spoofer = new Spoofer(this.object, this, this.tasks);
        this.profile.pingChecker = setInterval(() => {
            if (!this.object?.player)
                return;
            this.profile.ping = this.object.player.ping;
        }, this.dataUpdateFrequency);
        this.profile.status = { text: 'Активен', color: '#22ed17ff' };
        if (this.useOptimization) {
            this.updateTask('basic', true, 0.8);
        }
        else {
            this.updateTask('basic', true, 1.3);
        }
    }
    async auth(type) {
        try {
            if (!this.object)
                return;
            if (type === 'register') {
                if (!this.profile.registered) {
                    const text = this.registerTemplate
                        .replace(/@command/g, this.registerCommand)
                        .replace(/@cmd/g, this.registerCommand)
                        .replace(/@c/g, this.registerCommand)
                        .replace(/@register/g, this.registerCommand)
                        .replace(/@reg/g, this.registerCommand)
                        .replace(/@password/g, this.password)
                        .replace(/@pass/g, this.password)
                        .replace(/@p/g, this.password);
                    await this.sleep(generateNumber('float', this.registerMinDelay, this.registerMaxDelay));
                    this.object.chat(text);
                    this.profile.registered = true;
                    msg('process:botting', {
                        type: 'info',
                        message: `Бот ${this.nickname} зарегистрировался: ${text}`
                    });
                }
                else {
                    await this.auth('login');
                }
            }
            else if (type === 'login') {
                if (this.profile.registered) {
                    const text = this.loginTemplate
                        .replace(/@command/g, this.loginCommand)
                        .replace(/@cmd/g, this.loginCommand)
                        .replace(/@c/g, this.loginCommand)
                        .replace(/@login/g, this.loginCommand)
                        .replace(/@l/g, this.loginCommand)
                        .replace(/@password/g, this.password)
                        .replace(/@pass/g, this.password)
                        .replace(/@p/g, this.password);
                    await this.sleep(generateNumber('float', this.loginMinDelay, this.loginMaxDelay));
                    this.object.chat(text);
                    msg('process:botting', {
                        type: 'info',
                        message: `Бот ${this.nickname} залогинился: ${text}`
                    });
                }
                else {
                    await this.auth('register');
                }
            }
        }
        catch (error) {
            console.log(`( ${this.nickname} / auth ) Ошибка: ${error}`);
        }
    }
    async handling(type) {
        try {
            if (!this.object)
                return;
            this.object.once('spawn', async () => {
                try {
                    await this.sleep(generateNumber('float', 1000, 2000));
                    if (type === 'join') {
                        msg('process:botting', {
                            type: 'info',
                            message: `Создан новый бот: ${this.nickname}`
                        });
                        if (this.useAutoRegister)
                            await this.auth('register');
                    }
                    else if (type === 'rejoin') {
                        msg('process:botting', {
                            type: 'info',
                            message: `Бот ${this.nickname} переподключился`
                        });
                        this.profile.rejoinProcess = 'sleep';
                        if (!this.profile.registered) {
                            if (this.useAutoRegister)
                                await this.auth('register');
                        }
                        else {
                            if (this.useAutoLogin)
                                await this.auth('login');
                        }
                    }
                    else if (type === 'recreate') {
                        this.profile.rejoinProcess = 'sleep';
                        if (!this.profile.registered) {
                            if (this.useAutoRegister)
                                await this.auth('register');
                        }
                        else {
                            if (this.useAutoLogin)
                                await this.auth('login');
                        }
                    }
                }
                catch (error) {
                    console.log(`( ${this.nickname} / spawn ) Ошибка: ${error}`);
                }
            });
            this.object.on('resourcePack', async (url, hash) => {
                this.object?.emit('resourcePack', url, hash);
            });
            if (this.useLogDeath) {
                this.object.on('death', async () => {
                    msg('process:botting', {
                        type: 'info',
                        message: `Бот ${this.nickname} умер`
                    });
                });
            }
            if (this.useSaveChat) {
                this.object.on('chat', async (username, message) => {
                    if (username !== this.nickname) {
                        fs.appendFileSync(`./data/chat_history_${this.nickname}.txt`, `${username}: ${message}\n`);
                    }
                });
            }
            if (this.useSavePlayers) {
                this.profile.playerSaver = setInterval(() => {
                    if (!this.object)
                        return;
                    const players = Object.keys(this.object.players);
                    fs.writeFileSync(`./data/players_${this.nickname}.txt`, `---------- ${this.nickname} / ОКРУЖАЮЩИЕ ИГРОКИ ----------\n`);
                    for (const player of players) {
                        fs.appendFileSync(`./data/players_${this.nickname}.txt`, player + '\n');
                    }
                }, 5000);
            }
            this.object.on('message', async (message, position) => {
                try {
                    const messageString = String(message).trim();
                    const text = message.toHTML();
                    let nickname;
                    if (position === 'system') {
                        nickname = 'anonymous';
                    }
                    else if (position === 'chat') {
                        const nicknameMatch = messageString.match(/<(.+?)>/);
                        nickname = nicknameMatch ? nicknameMatch[1] : 'anonymous';
                    }
                    else {
                        nickname = 'unknown';
                    }
                    let isBot = false;
                    activeBots.forEach((_, element) => nickname === element ? isBot = true : isBot = isBot);
                    const msg = {
                        nickname: this.nickname,
                        type: position,
                        text: isBot ? `%hb[ БОТ ]%sc ${text}` : text
                    };
                    updateBotChatHistory(msg, this.chatHistoryLength);
                }
                catch (error) {
                    if (error instanceof TypeError)
                        return;
                    console.log(`( ${this.nickname} / message ) Ошибка: ${error}`);
                }
            });
            this.object.on('end', async (reason) => {
                try {
                    if (reason === '@salarixi:disconnect')
                        return;
                    if (this.profile.rejoinProcess === 'active')
                        return;
                    if (this.useAutoRejoin && this.profile.rejoinQuantity < this.rejoinQuantity) {
                        msg('process:botting', {
                            type: 'info',
                            message: `Переподключение ${this.nickname}...`
                        });
                        const status = await this.rejoin();
                        if (!status) {
                            msg('process:botting', {
                                type: 'error',
                                message: `Бот ${this.nickname} не смог переподключиться`
                            });
                        }
                        else {
                            return;
                        }
                    }
                    this.clean();
                    this.profile.status = { text: 'Оффлайн', color: '#ed1717ff' };
                    msg('process:botting', {
                        type: 'info',
                        message: `${this.nickname} отключился: ${reason}`
                    });
                    activeBots.delete(this.nickname);
                }
                catch (error) {
                    console.log(`( ${this.nickname} / end ) Ошибка: ${error}`);
                }
            });
            this.object.on('kicked', async (reason) => {
                try {
                    if (reason === '@salarixi:disconnect')
                        return;
                    msg('process:botting', {
                        type: 'error',
                        message: `${this.nickname} кикнут: ${JSON.stringify(reason)}`
                    });
                    if (this.useAutoRejoin && this.profile.rejoinQuantity < this.rejoinQuantity && this.profile.rejoinProcess === 'sleep') {
                        msg('process:botting', {
                            type: 'info',
                            message: `Переподключение ${this.nickname}...`
                        });
                        const status = await this.rejoin();
                        if (!status) {
                            msg('process:botting', {
                                type: 'error',
                                message: `Бот ${this.nickname} не смог переподключиться`
                            });
                        }
                        else {
                            return;
                        }
                    }
                    this.clean();
                    this.profile.status = { text: 'Оффлайн', color: '#ed1717ff' };
                    activeBots.delete(this.nickname);
                }
                catch (error) {
                    console.log(`( ${this.nickname} / kicked ) Ошибка: ${error}`);
                }
            });
            this.object.on('error', async (error) => {
                try {
                    activeBots.delete(this.nickname);
                    this.clean();
                    this.profile.status = { text: 'Оффлайн', color: '#ed1717ff' };
                    msg('process:botting', {
                        type: 'error',
                        message: `Ошибка у ${this.nickname}: ${error.message || 'Invalid incoming data'}`
                    });
                }
                catch (error) {
                    console.log(`( ${this.nickname} / error ) Ошибка: ${error}`);
                }
            });
        }
        catch (error) {
            console.log(`( ${this.nickname} ) Ошибка: ${error}`);
        }
    }
    updateTask(task, status, load) {
        this.tasks[task].status = status;
        if (!status && !load) {
            this.tasks[task].load = 0;
        }
        else {
            this.tasks[task].load = Number(load);
        }
        let globalLoad = 0;
        for (const element in this.tasks) {
            const current = this.tasks[element];
            globalLoad += current.load;
        }
        this.profile.load = parseFloat(globalLoad.toFixed(1));
    }
    async disconnect() {
        if (!this.object)
            return;
        this.object.end('@salarixi:disconnect');
        activeBots.delete(this.nickname);
        this.clean();
        this.destroyed = true;
        this.profile.status = { text: 'Оффлайн', color: '#ed1717ff' };
    }
    clean() {
        if (!this.object)
            return;
        for (const element in this.tasks) {
            let current = this.tasks[element];
            current.status = false;
            current.load = 0;
        }
        this.profile.load = 0;
        this.profile.ping = 0;
        clearInterval(this.profile.pingChecker);
        clearInterval(this.profile.playerSaver);
        this.profile.pingChecker = undefined;
        this.profile.playerSaver = undefined;
    }
    control(tool, status, options) {
        if (status === 'on') {
            switch (tool) {
                case 'spoofer':
                    this.spoofer?.enableSmartSpoofing({ useSharpness: options.useSharpness, useBuffering: options.useBuffering });
                    break;
            }
        }
        else if (status === 'off') {
            switch (tool) {
                case 'spoofer':
                    this.spoofer?.disableSmartSpoofing();
                    break;
            }
        }
    }
}
process.on('uncaughtException', err => {
    console.log('uncaughtException', err);
});
