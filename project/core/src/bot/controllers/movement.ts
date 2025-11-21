import mineflayer from 'mineflayer';

import { Bot, TasksList } from '../architecture.js';
import Generator from '../../tools/generator.js';
import BotTasks from '../architecture.js';

interface Options {
  useSync: boolean;
  useAntiDetect: boolean;
	useImpulsiveness: boolean;
}

const generator = new Generator();

// Контроллер для управления движением
export default class MovementController {
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

	public async movement({ state, direction, options }: {
		state: 'start' | 'stop';
		direction: 'forward' | 'back' | 'left' | 'right';
		options?: Options;
	}) {
		let name: TasksList;

		switch (direction) {
			case 'forward':
				name = 'movementForward'; break;
			case 'back':
				name = 'movementBack'; break;
			case 'left':
				name = 'movementLeft'; break;
			case 'right':
				name = 'movementRight'; break;
		}

		if (state === 'start') {
			if (!options) return;

			switch (direction) {
				case 'forward':
					this.object.updateTask('movementForward', true, 1.8); break;
				case 'back':
					this.object.updateTask('movementBack', true, 1.8); break;
				case 'left':
					this.object.updateTask('movementLeft', true, 1.8); break;
				case 'right':
					this.object.updateTask('movementRight', true, 1.8); break;
			}

			try {
				if (options.useImpulsiveness) {
					const generateDelay = () => {
						const randomChance = Math.random();

						if (options.useSync) {
							if (randomChance >= 0.5) {
								return 600;
							} else {
								return 900;
							}
						} else {
							if (randomChance >= 0.5) {
								return generator.generateRandomNumberBetween(400, 1000);
							} else {
								return generator.generateRandomNumberBetween(1000, 2000);
							}
						}
					}

					while (this.tasks[name].status) {
						await this.sleep(generateDelay());

						if (!this.tasks[name].status) break;

						this.bot.setControlState(direction, true);
						await this.sleep(generateDelay());
						this.bot.setControlState(direction, false);
						await this.sleep(generateDelay());

						if (Math.random() >= 0.58) {
							if (!this.tasks[name].status) break;

							this.bot.setControlState(direction, true);
							await this.sleep(generateDelay());
							this.bot.setControlState(direction, false);
							await this.sleep(generateDelay());
						}
					}
				} else {
					this.bot.setControlState(direction, true);
				}
			} catch {
				this.object.updateTask(name, false);
				this.bot.setControlState(direction, false);
			}
		} else if (state === 'stop') {
			if (!this.tasks[name].status) return;

			this.object.updateTask(name, false);
			this.bot.setControlState(direction, false);
		}
	}
}