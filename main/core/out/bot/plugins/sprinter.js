import { generateNumber, chooseRandomElementFromArray } from '../utils/generator.js';
export class SprinterController {
    constructor(bot, object, tasks) {
        this.bot = bot;
        this.object = object;
        this.tasks = tasks;
        this.bot = bot;
        this.object = object;
        this.tasks = tasks;
    }
    async sleep(delay) {
        await new Promise(resolve => setTimeout(resolve, delay));
    }
    async sprinter(state, options) {
        try {
            if (state === 'start') {
                if (!options)
                    return;
                this.object.updateTask('sprinter', true, 4.8);
                switch (options.type) {
                    case 'default':
                        await this.defaultSprinter();
                        break;
                    case 'phantom':
                        await this.defaultSprinter();
                        break;
                }
            }
            else if (state === 'stop') {
                if (!this.bot)
                    return;
                if (!this.tasks.sprinter.status)
                    return;
                this.object.updateTask('sprinter', false);
            }
        }
        catch {
            this.object.updateTask('sprinter', false);
        }
    }
    async defaultSprinter() {
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
        }, generateNumber('float', 300, 700));
    }
    async microBoost() {
        this.bot.setControlState('sprint', true);
        this.bot.setControlState('jump', true);
        if (Math.random() > 0.5) {
            this.bot.entity.position.z = this.bot.entity.position.z + chooseRandomElementFromArray([0.3, 0.6, 0.9]);
        }
        else {
            this.bot.entity.position.z = this.bot.entity.position.z + chooseRandomElementFromArray([0.05, 0.07, 0.09]);
            this.sleep(generateNumber('float', 40, 100));
            this.bot.entity.position.z = this.bot.entity.position.z + chooseRandomElementFromArray([0.3, 0.6, 0.9]);
        }
        this.bot.setControlState('jump', false);
        this.bot.setControlState('sprint', false);
    }
}
/*
const { Generator } = require('../utils/generator.js');

const generator = new Generator();

// Контроллер для управления призраком
class SprinterController {
  constructor(
        bot,
        object,
        tasks
    ) {
        this.bot = bot;
        this.object = object;
        this.tasks = tasks;
    }

  async sleep(delay) {
        await new Promise(resolve => setTimeout(resolve, delay));
    }

  async sprinter(state, options) {
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

  async defaultSprinter() {
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
    
  async microBoost() {
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

module.exports = SprinterController;
*/ 
