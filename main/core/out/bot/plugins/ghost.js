import { generateNumber } from '../utils/generator.js';
export class GhostController {
    constructor(bot, object, tasks) {
        this.bot = bot;
        this.object = object;
        this.tasks = tasks;
        this.activePacketIgnore = false;
        this.originalWrite = undefined;
        this.packetBuffer = new Map();
        this.bot = bot;
        this.object = object;
        this.tasks = tasks;
    }
    async sleep(delay) {
        await new Promise(resolve => setTimeout(resolve, delay));
    }
    packetBuffering(packetName, packetData) {
        const packetId = Date.now() + Math.random();
        this.packetBuffer.set(packetId, {
            name: packetName,
            data: packetData
        });
    }
    async clearPacketBuffer() {
        if (this.packetBuffer.size === 0)
            return;
        for (const [id, packet] of this.packetBuffer.entries()) {
            this.originalWrite.call(this.bot._client, packet.name, packet.data);
            this.packetBuffer.delete(id);
        }
    }
    async temperatePacketIgnore(options) {
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
                        }
                        else {
                            this.originalWrite.call(this.bot._client, packetName, packetData);
                        }
                        if (Math.random() > 0.65) {
                            this.clearPacketBuffer();
                        }
                    }
                    else {
                        this.originalWrite.call(this.bot._client, packetName, packetData);
                    }
                }
                else {
                    this.clearPacketBuffer();
                    this.originalWrite.call(this.bot._client, packetName, packetData);
                }
            };
        }
        catch (error) {
            console.log(`Ошибка: ${error}`);
        }
    }
    async normalPacketIgnore(options) {
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
                }
                else {
                    this.clearPacketBuffer();
                    this.originalWrite.call(this.bot._client, packetName, packetData);
                }
            };
        }
        catch (error) {
            console.log(`Ошибка: ${error}`);
        }
    }
    async aggressivePacketIgnore(options) {
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
                }
                else {
                    this.clearPacketBuffer();
                    this.originalWrite.call(this.bot._client, packetName, packetData);
                }
            };
        }
        catch (error) {
            console.log(`Ошибка: ${error}`);
        }
    }
    async ghost(state, options) {
        try {
            if (state === 'start') {
                if (!options)
                    return;
                this.object.updateTask('ghost', true, 3.5);
                if (options.mode === 'temperate') {
                    await this.temperatePacketIgnore(options);
                }
                else if (options.mode === 'normal') {
                    await this.normalPacketIgnore(options);
                    const interval = setInterval(async () => {
                        if (!this.tasks.ghost.status) {
                            clearInterval(interval);
                            return;
                        }
                        this.activePacketIgnore = true;
                        await this.sleep(generateNumber('float', 1000, 2000));
                        this.activePacketIgnore = false;
                    }, generateNumber('float', 3000, 4000));
                }
                else if (options.mode === 'aggressive') {
                    await this.aggressivePacketIgnore(options);
                    const interval = setInterval(async () => {
                        if (!this.tasks.ghost.status) {
                            clearInterval(interval);
                            return;
                        }
                        this.activePacketIgnore = true;
                        await this.sleep(generateNumber('float', 2000, 2400));
                        this.activePacketIgnore = false;
                    }, generateNumber('float', 2500, 3000));
                }
            }
            else if (state === 'stop') {
                if (!this.bot)
                    return;
                if (!this.tasks.ghost.status)
                    return;
                this.clearPacketBuffer();
                this.object.updateTask('ghost', false);
            }
        }
        catch (error) {
            console.log(`Ошибка: ${error}`);
        }
    }
}
/*
const { Generator } = require('../utils/generator.js');

const generator = new Generator();

// Контроллер для управления призраком
class GhostController {
  constructor(
        bot,
        object,
        tasks
    ) {
        this.bot = bot;
        this.object = object;
        this.tasks = tasks;
    }

  activePacketIgnore = false;
  originalWrite = undefined;
  packetBuffer = new Map();

  async sleep(delay) {
    await new Promise(resolve => setTimeout(resolve, delay));
  }

  packetBuffering(packetName, packetData) {
    const packetId = Date.now() + Math.random();
    const sendTime = Date.now();

    this.packetBuffer.set(packetId, {
      packetName,
      packetData,
      sendTime,
      attempts: 0
    });
  }

  clearPacketBuffer() {
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

  async temperatePacketIgnore(options) {
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

  async normalPacketIgnore(options) {
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

  async aggressivePacketIgnore(options) {
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

  async ghost(state, options) {
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

module.exports = GhostController;
*/ 
