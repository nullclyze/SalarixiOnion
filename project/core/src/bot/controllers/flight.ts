import mineflayer from 'mineflayer';

import { Bot } from '../architecture.js';
import BotTasks from '../architecture.js';
import Generator from '../../tools/generator.js';

const generator = new Generator();

interface Options {
  useSpoofing: boolean;
}

// Контроллер для управления призраком
export default class FlightController {
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

  public async flight(state: 'start' | 'stop', type?: 'default' | 'jump' | 'glitch', options?: Options) {
    try {
      if (state === 'start') {
        if (!type) return;
        if (!options) return;

        this.object.updateTask('flight', true, 3.8);
        
        switch (type) {
          case 'default':
            await this.defaultFlight(options); break;
          case 'jump':
            await this.jumpFlight(options); break;
          case 'glitch':
            await this.glitchFlight(options); break;
        }
      } else if (state === 'stop') {
        if (!this.bot) return;
        if (!this.tasks.flight.status) return;

        this.object.updateTask('flight', false);
      }
    } catch {
      this.object.updateTask('flight', false);
    }
  }

  private async defaultFlight(options: Options) {
    if (options.useSpoofing) {
      this.object.control('spoofer', 'on', { useSharpness: false, useBuffering: true });
    }

    const interval = setInterval(async () => {
      if (!this.tasks.flight.status) {
        this.object.control('spoofer', 'off', undefined);
        clearInterval(interval);
        return;
      }

      for (let i = 0; i < generator.generateRandomNumberBetween(2, 6); i++) {
        await this.smoothLift();
      }
    }, generator.generateRandomNumberBetween(100, 400));
  }

  private async jumpFlight(options: Options) {
    try {
      if (options.useSpoofing) {
        this.object.control('spoofer', 'on', { useSharpness: false, useBuffering: true });
      }

      while (this.tasks.flight.status) {
        this.bot.setControlState('jump', true);

        for (let i = 0; i < generator.generateRandomNumberBetween(2, 4); i++) {
          await this.smoothLift();
          await this.sleep(generator.generateRandomNumberBetween(5, 20));
        }

        this.bot.setControlState('jump', false);

        for (let i = 0; i < generator.generateRandomNumberBetween(2, 4); i++) {
          await this.smoothLift();
          await this.sleep(generator.generateRandomNumberBetween(5, 20));
        }

        await this.hover();

        this.sendFakePosition(this.bot.entity.position.y, true);

        await this.sleep(generator.generateRandomNumberBetween(200, 400));
      }
    } finally {
      this.object.control('spoofer', 'off', undefined);
    }
  }

  private async glitchFlight(options: Options) {
    try {
      if (options.useSpoofing) {
        this.object.control('spoofer', 'on', { useSharpness: false, useBuffering: true });
      }

      while (this.tasks.flight.status) {
        await this.glitch();

        for (let i = 0; i < generator.generateRandomNumberBetween(2, 4); i++) {
          await this.smoothLift();
        }

        await this.glitch();

        for (let i = 0; i < generator.generateRandomNumberBetween(2, 6); i++) {
          await this.smoothLift();
        }

        await this.hover();

        this.sendFakePosition(this.bot.entity.position.y, true);

        await this.sleep(generator.generateRandomNumberBetween(200, 400));
      }
    } finally {
      this.object.control('spoofer', 'off', undefined);
    }
  }

  private sendFakePosition(y: number, onGround: boolean = false) {
    this.bot._client.write('position_look', {
      x: this.bot.entity.position.x,
      y: y,
      z: this.bot.entity.position.z,
      yaw: this.bot.entity.yaw,
      pitch: this.bot.entity.pitch,
      onGround: onGround,
      time: Date.now(),
      flags: { onGround: onGround, hasHorizontalCollision: undefined }
    });
  }
    
  private async smoothLift() {
    if (Math.random() > 0.5) {
      this.sendFakePosition(this.bot.entity.position.y + (0.5 + Math.random()) * 1.5);
    } else {
      this.sendFakePosition(this.bot.entity.position.y + (0.5 + Math.random()) * 0.3);
      this.sleep(generator.generateRandomNumberBetween(30, 60));
      this.sendFakePosition(this.bot.entity.position.y + (0.5 + Math.random()) * 0.3);
    }
  }

  private async glitch() {
    const x = this.bot.entity.position.x;
    const y = this.bot.entity.position.y;
    const z = this.bot.entity.position.z;

    const glitchDuration = generator.generateRandomNumberBetween(10, 50);

    const startTime = Date.now();
        
    while (Date.now() - startTime < glitchDuration) {
      this.bot.entity.position.x = x + ((Math.random() - 0.5) * 0.1) + (Math.random() * generator.chooseRandomValueFromArray([0.03, 0.05, 0.07]));
      this.bot.entity.position.y = y + ((Math.random() - 0.5) * 0.1) + (Math.random() * generator.chooseRandomValueFromArray([0.1, 0.15, 0.2]));
      this.bot.entity.position.z = z + ((Math.random() - 0.5) * 0.1) + (Math.random() * generator.chooseRandomValueFromArray([0.03, 0.05, 0.07]));

      await this.sleep(5);
    }
  }
  
  private async hover() {
    const hoverY = this.bot.entity.position.y;
    const hoverDuration = 10 + Math.random() * 20;
    const startTime = Date.now();
        
    while (Date.now() - startTime < hoverDuration) {
      this.bot.entity.position.y = hoverY + (Math.random() - 0.5) * 0.1;
      await this.sleep(5);
    }
  }
}