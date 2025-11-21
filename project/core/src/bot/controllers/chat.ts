import mineflayer from 'mineflayer';

import { Bot } from '../architecture.js';
import Generator from '../../tools/generator.js';
import { mutateText } from '../../tools/mutator.js';
import BotTasks from '../architecture.js';

let sync = {
	spamming: { use: 0, delay: 0 }
};

const generator = new Generator();

interface Options {
	'default': {
		from: string;
		message: string;
    useMagicText: boolean;
		useTextMutation: boolean;
    useSync: boolean;
	};
	'spamming': {
		state: 'start' | 'stop';
		from: string;
    message: string;
    minDelay: number;
		maxDelay: number;
    useMagicText: boolean;
    useTextMutation: boolean;
    useSync: boolean;
    useAntiRepetition: boolean;
	};
}

// Контроллер для управления чатом
export default class ChatController {
	constructor(
		public bot: mineflayer.Bot,
		public object: Bot,
		public tasks: BotTasks
	) {
		this.bot = bot;
		this.object = object;
		this.tasks = tasks;
	}

	private async sleep(delay: number) {
		await new Promise(resolve => setTimeout(resolve, delay));
	}

	private createMagicText(text: string) {
		let magicText = '';

		const words = text.split(' ');

		for (const word of words) {
			const randomChance = Math.random();

			magicText += ' ';

			if (word.includes('http://') || word.includes('https://')) {
				magicText += word;
			} else {
				if (randomChance >= 0.9) {
					magicText += word.toLowerCase();
				} else if (randomChance < 0.9 && randomChance > 0.75) {
					magicText += word.toUpperCase();
				} else {
					for (const char of word) {
						const randomChance = Math.random();

						if (randomChance >= 0.70) {
							magicText += char.toLowerCase()
								.replace(/o/g, () => Math.random() > 0.5 ? '0' : '@')
								.replace(/о/g, () => Math.random() > 0.5 ? '0' : '@')
								.replace(/a/g, '4')
								.replace(/а/g, '4')
								.replace(/z/g, '3')
								.replace(/з/g, '3')
								.replace(/e/g, '3')
								.replace(/е/g, '3')
								.replace(/i/g, () => Math.random() > 0.5 ? '1' : '!')
								.replace(/l/g, () => Math.random() > 0.5 ? '1' : '!')
								.replace(/л/g, () => Math.random() > 0.5 ? '1' : '!')
								.replace(/и/g, () => Math.random() > 0.5 ? '1' : '!')
								.replace(/п/g, '5')
								.replace(/p/g, '5')
								.replace(/v/g, () => Math.random() > 0.5 ? '8' : '&')
								.replace(/в/g, () => Math.random() > 0.5 ? '8' : '&')
								.replace(/б/g, '6')
								.replace(/b/g, '6')
								.replace(/с/g, '$')
								.replace(/s/g, '$');
						} else if (randomChance < 0.70 && randomChance >= 0.5) {
							magicText += char.toUpperCase()
						} else {
							magicText += char;
						}
					}
				}
			}
		}

		return magicText;
	} 

	public async send(options: Options['default']) {
		if (!this.bot) return;

		try {
			this.object.updateTask('default', true, 0.3);

			let text = '';

			if (options.useTextMutation) {
				const players = Object.keys(this.bot.players);
						
				text = mutateText({ 
					text: options.message, 
					advanced: true, 
					data: { players: players } 
				});
			} else {
				text = options.message;
			}

			if (!options.useSync) {
				await this.sleep(generator.generateRandomNumberBetween(200, 4000));
			}

			if (options.useMagicText) text = this.createMagicText(text);

			if (options.from === '@all') {
				this.bot.chat(text);
			} else {
				if (this.object.nickname === options.from) this.bot.chat(text);
			}
		} catch {
			this.object.updateTask('default', false);
		} finally {
			this.object.updateTask('default', false);
		}
	}

	public async spamming(options: Options['spamming']) {
		if (options.state === 'start') {
			if (!this.bot) return;
			if (this.tasks.spamming.status) return;

			try {
				this.object.updateTask('spamming', true, 3.4);

				let lastTexts: string[] = [];

				while (this.tasks.spamming.status) {
					let text = '';

					if (options.useTextMutation) {
						const players = Object.keys(this.bot.players);
						
						text = mutateText({ 
							text: options.message, 
							advanced: true, 
							data: { players: players } 
						});
					} else {
						text = options.message;
					}

					if (!options.useSync) {
						await this.sleep(generator.generateRandomNumberBetween(options.minDelay, options.maxDelay));
					} else {
						if (sync.spamming.use >= this.object.quantity) {
							sync.spamming.delay = 0;
							sync.spamming.use = 0;
						}
						
						if (!sync.spamming.delay) {
							sync.spamming.delay = generator.generateRandomNumberBetween(options.minDelay, options.maxDelay);
						}

						sync.spamming.use++;

						await this.sleep(sync.spamming.delay);
					}
			
					if (options.useMagicText) text = this.createMagicText(text);

					if (this.tasks.spamming.status) {
						let valid = true;

						if (options.useAntiRepetition) {
							lastTexts.forEach(element => text === element ? valid = false : valid = valid);

							if (valid) {
								lastTexts.push(text);
								if (lastTexts.length > 5) lastTexts.shift();
							}
						} 

						if (valid) {
							if (options.from === '@all') {
								this.bot.chat(text);
							} else {
								if (this.object.nickname === options.from) this.bot.chat(text);
							}
						}
					}
				}
			} catch {
				this.object.updateTask('spamming', false);
			}
		} else if (options.state === 'stop') {
			if (!this.tasks.spamming.status) return;

			this.object.updateTask('spamming', false);
		}
	}
}