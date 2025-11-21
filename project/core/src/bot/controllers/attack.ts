import mineflayer from 'mineflayer';

import { Bot } from '../architecture.js';
import BotTasks from '../architecture.js';
import Generator from '../../tools/generator.js';

const generator = new Generator();

interface Options {
	type: 'mob' | 'player' | 'all';
  useLongDelays: boolean;
	useNeatness: boolean;
  useAntiDetect: boolean;
	useImprovedStrikes: boolean;
}

// Контроллер для управления движением
export default class AttackController {
 	constructor(
		public bot: mineflayer.Bot,
		public object: Bot,
		public tasks: BotTasks
	) {
		this.bot = bot;
		this.object = object;
		this.tasks = tasks;
	}

	private async aiming() {
		while (this.tasks.attacking.status) {
			const target = this.bot.nearestEntity();

			if (target) {
				if (target.type === 'mob' || target.type === 'player') {
					if (target) {
						this.bot.lookAt(target.position.offset(0, target.height ?? 1.6, 0), false);
					}

					if (target && Math.random() > 0.5) {
						this.bot.lookAt(target.position.offset(0, target.height ?? 1.6, 0), true);
					}

					await new Promise(resolve => setTimeout(resolve, generator.generateRandomNumberBetween(100, 300)));
				}
			}
		}
	}

	private hit(options: Options) {
		const target = this.bot.nearestEntity();

		if (!target) return;

		if ((options.type === 'all' && target.type === 'mob' || target.type === 'player') || options.type === target.type) {
			if (this.bot.entities[target.id]) {
				if (options.useNeatness) {
					if (this.bot.entity.position.distanceTo(target.position) <= 3.5) {
						this.bot.lookAt(target.position.offset(0, target.height ?? 1.6, 0), true);
						this.bot.attack(target);
					}
				} else {
					this.bot.lookAt(target.position.offset(0, target.height ?? 1.6, 0), true);
					this.bot.attack(target);
				}
			}
		}
	}

	public async attack(state: 'start' | 'stop', options?: Options) {
		if (state === 'start') {
			if (this.tasks.attacking.status) return;

			if (!options) return;

			try {
				this.object.updateTask('attacking', true, 4.8);

				this.aiming();

				while (this.tasks.attacking.status) {
					if (options.useLongDelays) {
						await new Promise(resolve => setTimeout(resolve, generator.generateRandomNumberBetween(400, 800)));
					} else {
						await new Promise(resolve => setTimeout(resolve, generator.generateRandomNumberBetween(100, 400)));
					}

					if (!this.tasks.attacking.status) return;

					this.hit(options);

					await new Promise(resolve => setTimeout(resolve, generator.generateRandomNumberBetween(500, 700)));

					if (options.useImprovedStrikes) {
						const randomChance = Math.random();

						if (randomChance >= 0.8) {
							this.bot.setControlState('jump', true);
							this.hit(options);
							await new Promise(resolve => setTimeout(resolve, generator.generateRandomNumberBetween(100, 150)));
							this.bot.setControlState('jump', false);
						} else if (randomChance < 0.8 && randomChance > 0.5) {
							this.bot.setControlState('sneak', true);
							this.hit(options);
							await new Promise(resolve => setTimeout(resolve, generator.generateRandomNumberBetween(100, 150)));
							this.bot.setControlState('sneak', false);
						}
					}
				}
			} finally {
				this.object.updateTask('attacking', false);
				this.bot.setControlState('jump', false);
				this.bot.setControlState('sneak', false);
			}
		} else if (state === 'stop') {
			if (!this.tasks.attacking.status) return;

			this.object.updateTask('attacking', false);
			this.bot.setControlState('jump', false);
			this.bot.setControlState('sneak', false);
		}
	}
}