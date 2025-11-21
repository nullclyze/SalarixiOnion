import mineflayer from 'mineflayer';
import { pathfinder } from 'mineflayer-pathfinder';
import fs from 'fs';

import { connection } from './connection/connection.js';
import Spoofer from './connection/spoofer.js';
import Generator from '../tools/generator.js';

import ChatController from './controllers/chat.js';
import ActionController from './controllers/action.js';
import MovementController from './controllers/movement.js';
import ImitationController from './controllers/imitation.js';
import AttackController from './controllers/attack.js';
import FlightController from './controllers/flight.js';
import SprinterController from './controllers/sprinter.js';
import GhostController from './controllers/ghost.js';

export let bots: Map<string, Bot> = new Map();
export let activeBotsObjects: Map<string, Bot> = new Map();

// Структура профиля бота
interface BotProfile {
	nickname: string;
	password: string;
	version: string;
	registered: boolean;
	reputation: number;
	rejoinProcess: 'active' | 'sleep';
	rejoinQuantity: number;
	proxyType: 'socks5' | 'socks4' | 'http' | '-';
	proxy: string;
	status: { text: 'Активен', color: '#22ed17ff' } | { text: 'Соединение...', color: '#8f8f8fff' } | { text: 'Оффлайн', color: '#ed1717ff' };
	load: number;
	ping: number;
	pingChecker: any;
	playerSaver: any;
}

// Структура задач бота
export default interface BotTasks {
	basic: { status: boolean, load: number };
	analysis: { status: boolean, load: number };
	default: { status: boolean, load: number };
	spamming: { status: boolean, load: number };
	jumping: { status: boolean, load: number };
	shifting: { status: boolean, load: number };
	movementForward: { status: boolean, load: number };
	movementBack: { status: boolean, load: number };
	movementLeft: { status: boolean, load: number };
	movementRight: { status: boolean, load: number };
	attacking: { status: boolean, load: number };
	looking: { status: boolean, load: number };
	waving: { status: boolean, load: number };
	spinning: { status: boolean, load: number };
	hybridImitation: { status: boolean, load: number };
	walkingImitation: { status: boolean, load: number };
	spoofing: { status: boolean, load: number };
	flight: { status: boolean, load: number };
	sprinter: { status: boolean, load: number };
	ghost: { status: boolean, load: number };
}

export type TasksList = 'basic' | 'analysis' | 'default' | 'spamming' | 'jumping' | 'shifting'
								| 'movementForward' | 'movementBack' | 'movementLeft' | 'movementRight'
								| 'attacking' | 'looking' | 'waving' | 'spinning' | 'hybridImitation'
								| 'walkingImitation' | 'spoofing' | 'flight' | 'sprinter' | 'ghost';

const generator = new Generator();

// Основной класс с архитектурой Minecraft бота
export class Bot {
	constructor(
		public transmitter: any,
		public address: string,
		public version: string,
		public quantity: number,
		public nickname: string,
		public password: string,
		public distance: string,
		public timeout: number,
		public skipValidation: boolean,
		public registerCommand: string,
		public registerTemplate: string,
		public registerMinDelay: number,
		public registerMaxDelay: number,
		public loginCommand: string,
		public loginTemplate: string,
		public loginMinDelay: number,
		public loginMaxDelay: number,
		public rejoinQuantity: number,
		public rejoinDelay: number,
		public proxy: string | undefined,
		public useKeepAlive: boolean,
		public usePhysics: boolean,
		public useProxy: boolean,
		public useProxyChecker: boolean,
		public useAutoRegister: boolean,
		public useAutoLogin: boolean,
		public useAutoRejoin: boolean,
		public useLogDeath: boolean,
		public useSaveChat: boolean,
		public useSavePlayers: boolean,
		public useOptimization: boolean
	) {
		this.profile = {
			nickname: this.nickname,
			password: this.password,
			version: this.version,
			registered: false,
			rejoinProcess: 'sleep',
			status: { text: 'Соединение...', color: '#8f8f8fff' },
			reputation: 100,
			rejoinQuantity: 0,
			proxyType: '-',
			proxy: '-',
			load: 0,
			ping: 0,
			pingChecker: undefined,
			playerSaver: undefined
		};

		this.tasks = {
			basic: { status: false, load: 0 },
			analysis: { status: false, load: 0 },
			default: { status: false, load: 0 },
			spamming: { status: false, load: 0 },
			jumping: { status: false, load: 0 },
			shifting: { status: false, load: 0 },
			movementForward: { status: false, load: 0 },
			movementBack: { status: false, load: 0 },
			movementLeft: { status: false, load: 0 },
			movementRight: { status: false, load: 0 },
			attacking: { status: false, load: 0 },
			looking: { status: false, load: 0 },
			waving: { status: false, load: 0 },
			spinning: { status: false, load: 0 },
			hybridImitation: { status: false, load: 0 },
			walkingImitation: { status: false, load: 0 },
			spoofing: { status: false, load: 0 },
			flight: { status: false, load: 0 },
			sprinter: { status: false, load: 0 },
			ghost: { status: false, load: 0 }
		};
	}

	public profile: BotProfile;
	public tasks: BotTasks;

	private bot: mineflayer.Bot | undefined = undefined;

	private chatController: ChatController | undefined = undefined;
	private actionController: ActionController | undefined = undefined;
	private movementController: MovementController | undefined = undefined;
	private imitationController: ImitationController | undefined = undefined;
	private attackController: AttackController | undefined = undefined;
	private flightController: FlightController | undefined = undefined;
	private sprinterController: SprinterController | undefined = undefined;

	private ghostController: GhostController | undefined = undefined;
	private chatHistory: any[] = [];

	private spoofer: Spoofer | undefined = undefined;

	private async sleep(delay: number) {
		await new Promise(resolve => setTimeout(resolve, delay));
	}
	
	private async create() {
		try {
			this.profile.status = { text: 'Соединение...', color: '#8f8f8fff' };

			let viewDistance: 'tiny' | 'short' | 'normal' | 'far';

			if (this.distance !== 'tiny' && this.distance !== 'short' && this.distance !== 'normal' && this.distance !== 'far') {
				viewDistance = 'short';
			} else {
				viewDistance = this.distance;
			}

			let options: mineflayer.BotOptions = {
				host: this.address.split(':')[0],
				username: this.nickname,
				auth: 'offline',
				port: parseInt(this.address.split(':')[1]),     
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
				let proxyType: 'socks5' | 'socks4' | 'http' | '@none';

				if (this.proxy?.startsWith('socks5://')) {
					proxyType = 'socks5';
				} else if (this.proxy?.startsWith('socks4://')) {
					proxyType = 'socks4';
				} else if (this.proxy?.startsWith('http://')) {
					proxyType = 'http';
				} else {
					proxyType = '@none';
				}

				if (proxyType === '@none') return;

				if (proxyType === 'socks5' || proxyType === 'socks4') {
					console.log(`Бот ${this.nickname} использует ${proxyType.toUpperCase()}-прокси: ${this.proxy}`);

					this.profile.proxyType = proxyType;

					const url = new URL(this.proxy);
					const host = url.hostname;
					const port = parseInt(url.port) || 80;
					const username = url.username;
					const password = url.password;

					if (!host || !port) return;

					this.profile.proxy = `${host}:${port}`;

					const socket: any = await connection({ 
						type: proxyType,
						host: host,
						port: port,
						timeout: this.timeout,
						address: this.address,
						username: username,
						password: password
					});

					const bot = mineflayer.createBot({
						...options,
						connect: (client) => {
							client.setSocket(socket);
							client.emit('connect');
						}
					});
					
					return bot;
				} else if (proxyType === 'http') {
					console.log(`Бот ${this.nickname} использует HTTP-прокси: ${this.proxy}`);

					this.profile.proxyType = 'http';

					const url = new URL(this.proxy);
          const host = url.hostname;
          const port = parseInt(url.port) || 80;
          const username = url.username;
          const password = url.password;

					if (!host || !port) return;

					this.profile.proxy = `${host}:${port}`;

					const socket: any = await connection({ 
						type: proxyType,
						host: host,
						port: port,
						timeout: this.timeout,
						address: this.address,
						username: username,
						password: password
					});

					const bot = mineflayer.createBot({
						...options,
						connect: (client) => {
							client.setSocket(socket);
							client.emit('connect');
						}
					});

					return bot;
				} 
			} else {
				this.profile.proxyType = '-';
				this.profile.proxy = 'Не использует';
				
				const bot = mineflayer.createBot(options);

				return bot;
			}
		} catch (error) {
			console.log(`Ошибка создания бота ${this.nickname}: ${error}`);
		}
	}

	public async recreate() {
		try {
			if (this.bot) {
				this.bot.end('@salarixi:disconnect');
				activeBotsObjects.delete(this.nickname);
			}

			this.clean();

			this.profile.rejoinProcess = 'active';
			this.profile.status = { text: 'Соединение...', color: '#8f8f8fff' };

			await this.sleep(generator.generateRandomNumberBetween(800, 1800));

			const operation = await this.join(true);

			if (operation?.success) {
				return {
					success: true,
					message: `Бот ${this.nickname} пересоздан`
				};
			} else {
				return {
					success: false,
					message: `Не удалось пересоздать ${this.nickname}`
				};
			}
		} catch (error) {
			return {
				success: false,
				message: `Ошибка пересоздания ${this.nickname}: ${error}`
			};
		}
	}
	
	public async join(isRecreate: boolean) {
		try {
			const bot = await this.create();

			if (!bot) {
				return {
					success: false,
					message: `Ошибка создания ${this.nickname}: Bot object damaged`
				};
			}

			this.bot = bot;

			await this.setup();
			await this.handling(isRecreate ? 'recreate' : 'join');

			return {
				success: true,
				message: `Бот ${this.nickname} успешно создан`
			};
		} catch (error) {
			this.transmitter.send({ type: 'error', data: {
      	message: `Ошибка создания ${this.nickname}: ${error}`
    	}});

			return {
				success: false,
				message: `Ошибка создания ${this.nickname}: ${error}`
			};
		}
  }   

	private async rejoin() {
		try {
			this.profile.status = { text: 'Соединение...', color: '#8f8f8fff' };
			this.profile.rejoinProcess = 'active';
			this.profile.rejoinQuantity++;

			this.clean();

			activeBotsObjects.delete(this.profile.nickname);

			await this.sleep(generator.generateRandomNumberBetween(1000, 2800));

			this.bot = await this.create();

			if (!this.bot) return false;

			await this.setup();
			await this.handling('rejoin');

			return true;
		} catch {
			return false;
		}
	}

	private async setup() {
		if (!this.bot) return;

		bots.set(this.nickname, this);

		activeBotsObjects.set(this.nickname, this);

		this.bot.loadPlugin(pathfinder);

		this.chatController = new ChatController(this.bot, this, this.tasks);
		this.actionController = new ActionController(this.bot, this, this.tasks);
		this.imitationController = new ImitationController(this.bot, this, this.tasks);
		this.movementController = new MovementController(this.bot, this, this.tasks);
		this.attackController = new AttackController(this.bot, this, this.tasks);
		this.flightController = new FlightController(this.bot, this, this.tasks);
		this.sprinterController = new SprinterController(this.bot, this, this.tasks);
		this.ghostController = new GhostController(this.bot, this, this.tasks);
		this.spoofer = new Spoofer(this.bot, this, this.tasks);

		this.profile.pingChecker = setInterval(() => {
			if (!this.bot?.player) return;
			this.profile.ping = this.bot.player.ping;
		}, 3000);

		this.profile.status = { text: 'Активен', color: '#22ed17ff' };

		if (this.useOptimization) {
			this.updateTask('basic', true, 0.8);
		} else {
			this.updateTask('basic', true, 1.3);
		}
	}

	private async auth(type: 'register' | 'login') {
		try {
			if (!this.bot) return;
			if (!this.chatController) return;

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

					await this.sleep(generator.generateRandomNumberBetween(this.registerMinDelay, this.registerMaxDelay));

					await this.chatController.send({ 
						from: this.nickname,
						message: text, 
						useMagicText: false,
						useTextMutation: false,
						useSync: false
					});

					this.profile.registered = true;

					this.transmitter.send({
						type: 'info',
						data: {
							message: `Бот ${this.nickname} зарегистрировался: ${text}`
						}
					});
				} else {
					await this.auth('login');
				}
			} else if (type === 'login') {
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

					await this.sleep(generator.generateRandomNumberBetween(this.loginMinDelay, this.loginMaxDelay));

					await this.chatController.send({ 
						from: this.nickname,
						message: text, 
						useMagicText: false,
						useTextMutation: false,
						useSync: false
					});

					this.transmitter.send({
						type: 'info',
						data: {
							message: `Бот ${this.nickname} залогинился: ${text}`
						}
					})
				} else {
					await this.auth('register');
				}
			}
		} catch (error) {
			console.log(`( ${this.nickname} / auth ) Ошибка: ${error}`);
		}
	}

	private async handling(type: 'join' | 'rejoin' | 'recreate') {
		try {
			if (!this.bot) return;

			this.bot.once('spawn', async () => {
				try {
					await this.sleep(generator.generateRandomNumberBetween(1000, 2000));

					if (type === 'join') {
						this.transmitter.send({ type: 'info', data: {
							message: `Создан новый бот: ${this.nickname}`
						}});

						if (this.useAutoRegister) await this.auth('register');
					} else if (type === 'rejoin') {
						this.transmitter.send({ type: 'info', data: {
							message: `Бот ${this.nickname} переподключился`
						}});

						this.profile.rejoinProcess = 'sleep';

						if (!this.profile.registered) {
							if (this.useAutoRegister) await this.auth('register');
						} else {
							if (this.useAutoLogin) await this.auth('login');
						}
					} else if (type === 'recreate') {
						this.profile.rejoinProcess = 'sleep';

						if (!this.profile.registered) {
							if (this.useAutoRegister) await this.auth('register');
						} else {
							if (this.useAutoLogin) await this.auth('login');
						}
					}
				} catch (error) {
					console.log(`( ${this.nickname} / spawn ) Ошибка: ${error}`);
				}
  		});

			this.bot.on('resourcePack', async (url, hash) => {
				this.bot?.emit('resourcePack', url, hash);
			});

			if (this.useLogDeath) {
				this.bot.on('death', async () => {
					this.transmitter.send({ type: 'info', data: {
						message: `Бот ${this.nickname} умер`
					}});
				});
			}

			if (this.useSaveChat) {
				this.bot.on('chat', async (username, message) => {
					if (username !== this.nickname) {
						fs.appendFileSync(`./data/chat_history_${this.nickname}.txt`, `${username}: ${message}\n`);
					}
				});
			}

			if (this.useSavePlayers) {
				this.profile.playerSaver = setInterval(() => {
					if (!this.bot) return;

					const players = Object.keys(this.bot.players);

					fs.writeFileSync(`./data/players_${this.nickname}.txt`, `---------- ${this.nickname} / ОКРУЖАЮЩИЕ ИГРОКИ ----------\n`);

					for (const player of players) {
						fs.appendFileSync(`./data/players_${this.nickname}.txt`, player + '\n');
					}
				}, 5000);
			}

			this.bot.on('message', async (message, position) => {
				try {
					const messageString = String(message).trim();
					const text = message.toHTML();
					let nickname;

					if (position === 'system') {
						nickname = 'anonymous';
					} else if (position === 'chat') {
						const nicknameMatch = messageString.match(/<(.+?)>/);
						nickname = nicknameMatch ? nicknameMatch[1] : 'anonymous';
					} else {
						nickname = 'unknown';
					}

					let isBot = false;

					activeBotsObjects.forEach((_, element) => nickname === element ? isBot = true : isBot = isBot);

					const msg = {
						type: position,
						text: isBot ? `%hb[ БОТ ]%sc ${text}` : text
					}

					if (!isBot) {
						const valid = !this.chatHistory.some(existingMsg => 
							existingMsg.type === msg.type &&
							existingMsg.text === msg.text
						);

						if (!valid) return;
					}

					this.chatHistory.push(msg);

					if (this.chatHistory.length > 6) this.chatHistory.shift();
				} catch (error) {
					if (error instanceof TypeError) return;
					console.log(`( ${this.nickname} / message ) Ошибка: ${error}`);
				}
			});

			this.bot.on('end', async (reason) => {
				try {
					if (reason === '@salarixi:disconnect') return;
					if (this.profile.rejoinProcess === 'active') return;

					if (this.useAutoRejoin && this.profile.rejoinQuantity < this.rejoinQuantity) {
						this.transmitter.send({ type: 'info', data: {
							message: `Переподключение ${this.nickname}...`
						}});

						const status = await this.rejoin();

						if (!status) {
							this.transmitter.send({ type: 'error', data: {
								message: `Бот ${this.nickname} не смог переподключиться`
							}});
						} else {
							return;
						}
					}

					this.clean();

					this.profile.status = { text: 'Оффлайн', color: '#ed1717ff' };

					this.transmitter.send({ type: 'info', data: {
						message: `${this.nickname} отключился: ${reason}`
					}});

					activeBotsObjects.delete(this.nickname);
				} catch (error) {
					console.log(`( ${this.nickname} / end ) Ошибка: ${error}`);
				}
			});

			this.bot.on('kicked', async (reason) => {
				try {
					if (reason === '@salarixi:disconnect') return;

					this.transmitter.send({ type: 'error', data: {
						message: `${this.nickname} кикнут: ${JSON.stringify(reason)}`
					}});

					this.profile.reputation = this.profile.reputation - 5;

					if (this.useAutoRejoin && this.profile.rejoinQuantity < this.rejoinQuantity && this.profile.rejoinProcess === 'sleep') {
						this.transmitter.send({ type: 'info', data: {
							message: `Переподключение ${this.nickname}...`
						}});

						const status = await this.rejoin();

						if (!status) {
							this.transmitter.send({ type: 'error', data: {
								message: `Бот ${this.nickname} не смог переподключиться`
							}});
						} else {
							return;
						}
					}

					this.clean();

					this.profile.status = { text: 'Оффлайн', color: '#ed1717ff' };

					activeBotsObjects.delete(this.nickname);
				} catch (error) {
					console.log(`( ${this.nickname} / kicked ) Ошибка: ${error}`);
				}
			});

			this.bot.on('error', async (error) => {
				try {
					activeBotsObjects.delete(this.nickname);

					this.clean();

					this.profile.status = { text: 'Оффлайн', color: '#ed1717ff' };

					this.transmitter.send({ type: 'error', data: {
						message: `Ошибка у ${this.nickname}: ${error.message || 'Invalid incoming data'}`
					}});
				} catch (error) {
					console.log(`( ${this.nickname} / error ) Ошибка: ${error}`);
				}
			});
		} catch (error) {
			console.log(`( ${this.nickname} ) Ошибка: ${error}`);
		}
	}

	public updateTask(task: TasksList, status: boolean, load?: number) {
		this.tasks[task].status = status;

		if (!status && !load) {
			this.tasks[task].load = 0;
		} else {
			this.tasks[task].load = Number(load);
		}

		let globalLoad = 0;

		for (const element in this.tasks) {
			const current = this.tasks[element as TasksList];
			globalLoad += current.load;
		}

		this.profile.load = parseFloat(globalLoad.toFixed(1));
	}

	public async disconnect() {
		if (!this.bot) return;

		this.bot.end('@salarixi:disconnect');

		activeBotsObjects.delete(this.nickname);

		this.clean();

		this.profile.status = { text: 'Оффлайн', color: '#ed1717ff' };
	}

	public clean() {
		if (!this.bot) return;

		for (const element in this.tasks) {
			let current = this.tasks[element as TasksList];
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

	public get(type: 'bot-object' | 'chat-history') {
		switch (type) {
			case 'bot-object':
				if (this.bot) return this.bot; break;
			case 'chat-history':
				if (this.bot) return this.chatHistory; break;
		}
	}

	public control(tool: 'spoofer', status: 'on' | 'off', options: any) {
		if (status === 'on') {
			switch (tool) {
				case 'spoofer':
					this.spoofer?.enableSmartSpoofing({ useSharpness: options.useSharpness, useBuffering: options.useBuffering }); break;
			}
		} else if (status === 'off') {
			switch (tool) {
				case 'spoofer':
					this.spoofer?.disableSmartSpoofing(); break;
			}
		}
	}

  public async chat(type: 'default' | 'spamming', options: any) {
		if (!this.chatController) return;

  	if (type === 'default') {
			await this.chatController.send(options);
		} else if (type === 'spamming') {
			await this.chatController.spamming(options);
		}
  } 

	public async action(state: 'start' | 'stop', action: string, options?: any) {
		if (!this.actionController) return;

		switch (action) {
			case 'jumping':
				await this.actionController.jumping(state, options); break;
			case 'shifting':
				await this.actionController.shifting(state, options); break;
			case 'waving':
				await this.actionController.waving(state, options); break;
			case 'spinning':
				await this.actionController.spinning(state, options); break;
		}
	}

	public async movement(state: 'start' | 'stop', direction: string, options?: any) {
		if (!this.movementController) return;

		await this.movementController.movement({ state: state, direction: direction as 'forward' | 'back' | 'left' | 'right', options: options });
	}

	public async imitation(state: 'start' | 'stop', type: string, options?: any) {
		if (!this.imitationController) return;

		switch (type) {
			case 'hybrid':
				await this.imitationController.hybridImitation(state, options); break;
			case 'walking':
				await this.imitationController.walkingImitation(state, options); break;
		}
	}

	public async attack(state: 'start' | 'stop', options?: any) {
		if (!this.attackController) return;

		await this.attackController.attack(state, options);
	}

	public async flight(state: 'start' | 'stop', type?: 'default' | 'jump' | 'glitch', options?: any) {
		if (!this.flightController) return;

		await this.flightController.flight(state, type, options);
	}

	public async sprinter(state: 'start' | 'stop', options?: any) {
		if (!this.sprinterController) return;

		await this.sprinterController.sprinter(state, options);
	}

	public async ghost(state: 'start' | 'stop', options?: any) {
		if (!this.ghostController) return;

		await this.ghostController.ghost(state, options);
	}
}

process.on('uncaughtException', err => {
	console.log('uncaughtException', err);
});