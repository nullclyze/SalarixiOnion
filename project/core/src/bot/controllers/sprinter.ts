import mineflayer from 'mineflayer';

import { Bot } from '../architecture.js';
import BotTasks from '../architecture.js';
import Generator from '../../tools/generator.js';

const generator = new Generator();

interface Options {
	type: 'default' | 'phantom' | 'impulse';
}

// Контроллер для управления призраком
export default class SprinterController {
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

  public async sprinter(state: 'start' | 'stop', options?: Options) {
    try {
      if (state === 'start') {
        if (!options) return;

        this.object.updateTask('sprinter', true, 4.8);
        
        switch (options.type) {
          case 'default':
            await this.defaultSprinter(); break;
          case 'phantom':
            await this.defaultSprinter(); break;
        }
      } else if (state === 'stop') {
        if (!this.bot) return;
        if (!this.tasks.sprinter.status) return;

        this.object.updateTask('sprinter', false);
      }
    } catch {
      this.object.updateTask('sprinter', false);
    }
  }

  private async defaultSprinter() {
    this.object.control('spoofer', 'on', { useSharpness: false, useBuffering: true });

    this.bot.setControlState('forward', true);

    const interval = setInterval(async () => {
      if (!this.tasks.sprinter.status) {
        this.object.control('spoofer', 'off', undefined);
        this.bot.setControlState('forward', false);
        clearInterval(interval);
        return;
      }

      if (this.bot.getControlState('forward')) {
        await this.microBoost();
      }
    }, generator.generateRandomNumberBetween(300, 700));
  }
    
  private async microBoost() {
    this.bot.setControlState('sprint', true);
    this.bot.setControlState('jump', true);

    if (Math.random() > 0.5) {
      this.bot.entity.position.z = this.bot.entity.position.z + generator.chooseRandomValueFromArray([0.3, 0.6, 0.9]);
    } else {
      this.bot.entity.position.z = this.bot.entity.position.z + generator.chooseRandomValueFromArray([0.05, 0.07, 0.09]);
      this.sleep(generator.generateRandomNumberBetween(40, 100));
      this.bot.entity.position.z = this.bot.entity.position.z + generator.chooseRandomValueFromArray([0.3, 0.6, 0.9]);
    }

    this.bot.setControlState('jump', false);
    this.bot.setControlState('sprint', false);
  }
}