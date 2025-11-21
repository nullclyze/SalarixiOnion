import mineflayer from 'mineflayer';

import { Bot } from '../architecture.js';
import BotTasks from '../architecture.js';
import Generator from '../../tools/generator.js';

const generator = new Generator();

interface Options {
	mode: 'temperate' | 'normal' | 'aggressive';
  useSharpness: boolean;
  useBuffering: boolean;
}

// Контроллер для управления призраком
export default class GhostController {
  constructor(
		public bot: mineflayer.Bot,
		public object: Bot,
		public tasks: BotTasks
	) {
		this.bot = bot;
		this.object = object;
		this.tasks = tasks;
	}

  private activePacketIgnore: boolean = false;
  private originalWrite: any = undefined;
  private packetBuffer: Map<number, any> = new Map();

  private async sleep(delay: number) {
    await new Promise(resolve => setTimeout(resolve, delay));
  }

  private packetBuffering(packetName: string, packetData: any) {
    const packetId = Date.now() + Math.random();
    const sendTime = Date.now();

    this.packetBuffer.set(packetId, {
      packetName,
      packetData,
      sendTime,
      attempts: 0
    });
  }

  private clearPacketBuffer() {
    if (this.packetBuffer.size === 0) return;

    const now = Date.now();
    
    for (const [packetId, packetInfo] of this.packetBuffer.entries()) {
      if (now >= packetInfo.sendTime) {
        try {
          this.originalWrite.call(this.bot._client, packetInfo.packetName, packetInfo.packetData);
          this.packetBuffer.delete(packetId);
        } catch (error) {
          packetInfo.attempts++;
          
          if (packetInfo.attempts > 3) {
            this.packetBuffer.delete(packetId);
          }
        }
      }
    }
  }

  private async temperatePacketIgnore(options: any) {
    try {
      if (!this.originalWrite) {
        this.originalWrite = this.bot._client.write;
      }

      this.bot._client.write = async (packetName, packetData) => {
        if (this.tasks.ghost.status) {
          if (Math.random() > 0.5) {
            if (packetName === 'keep_alive' || packetName === 'position' || packetName === 'position_look' && Math.random() > 0.8) {
              if (options.useBuffering) {
                this.packetBuffering(packetName, packetData);
              }

              return;
            } else {
              this.originalWrite.call(this.bot._client, packetName, packetData);
            }

            if (Math.random() > 0.65) {
              this.clearPacketBuffer();
            }
          } else {
            this.originalWrite.call(this.bot._client, packetName, packetData);
          }
        } else {
          this.clearPacketBuffer();
          this.originalWrite.call(this.bot._client, packetName, packetData);
        }
      }
    } catch (error) {
      console.log(`Ошибка: ${error}`);
    }
  }

  private async normalPacketIgnore(options: any) {
    try {
      if (!this.originalWrite) {
        this.originalWrite = this.bot._client.write;
      }
          
      this.bot._client.write = (packetName, packetData) => {
        if (this.tasks.ghost.status && this.activePacketIgnore) {         
          if (packetName === 'keep_alive' || packetName === 'position' || packetName === 'position_look' && Math.random() > 0.6) {
            if (options.useBuffering) {
              this.packetBuffering(packetName, packetData);
            }

            return;
          }

          if (Math.random() > 0.85) {
            this.clearPacketBuffer();
          }
        } else {
          this.clearPacketBuffer();
          this.originalWrite.call(this.bot._client, packetName, packetData);
        }
      }
    } catch (error) {
      console.log(`Ошибка: ${error}`);
    }
  }

  private async aggressivePacketIgnore(options: any) {
    try {
      if (!this.originalWrite) {
        this.originalWrite = this.bot._client.write;
      }

      this.bot._client.write = async (packetName, packetData) => {
        if (this.tasks.ghost.status && this.activePacketIgnore) {
          if (packetName === 'keep_alive' || packetName === 'position' || packetName === 'position_look' && Math.random() > 0.3) {
            if (options.useBuffering) {
              this.packetBuffering(packetName, packetData);
            }

            return;
          }

          if (Math.random() > 0.95) {
            this.clearPacketBuffer();
          }
        } else {
          this.clearPacketBuffer();
          this.originalWrite.call(this.bot._client, packetName, packetData);
        }
      }
    } catch (error) {
      console.log(`Ошибка: ${error}`);
    }
  }

  public async ghost(state: 'start' | 'stop', options?: Options) {
    try {
      if (state === 'start') {
        if (!options) return;

        this.object.updateTask('ghost', true, 3.5);

        if (options.mode === 'temperate') {
          await this.temperatePacketIgnore(options);
        } else  if (options.mode === 'normal') {
          await this.normalPacketIgnore(options);

          const interval = setInterval(async () => {
            if (!this.tasks.ghost.status) {
              clearInterval(interval);
              return;
            }

            this.activePacketIgnore = true;
            await this.sleep(generator.generateRandomNumberBetween(1000, 2000));
            this.activePacketIgnore = false;
          }, generator.generateRandomNumberBetween(3000, 4000));
        } else if (options.mode === 'aggressive') {
          await this.aggressivePacketIgnore(options);

          const interval = setInterval(async () => {
            if (!this.tasks.ghost.status) {
              clearInterval(interval);
              return;
            }

            this.activePacketIgnore = true;
            await this.sleep(generator.generateRandomNumberBetween(2000, 2400));
            this.activePacketIgnore = false;
          }, generator.generateRandomNumberBetween(2500, 3000));
        }
      } else if (state === 'stop') {
        if (!this.bot) return;
        if (!this.tasks.ghost.status) return;

        this.clearPacketBuffer();

        this.object.updateTask('ghost', false);
      }
    } catch (error) {
      console.log(`Ошибка: ${error}`);
    }
  }
}