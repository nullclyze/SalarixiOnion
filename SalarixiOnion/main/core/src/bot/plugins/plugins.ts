import { ControlState } from 'mineflayer';
import { Vec3 } from 'vec3';

import { Bot, activeBots } from '../base.js';
import { generateString, generateNumber, chooseRandomElementFromArray } from '../utils/generator.js';
import { mutateText } from '../utils/mutator.js';

let sync = {
	spamming: { use: 0, delay: 0 },
  jumping: { delay: 0, use: 0 },
	shifting: { delay: 0, use: 0 },
	waving: { delay: 0, use: 0 },
	spinning: { delay: 0, use: 0 },
  moveForward: { delay: 0, use: 0 },
  moveBack: { delay: 0, use: 0 },
  moveLeft: { delay: 0, use: 0 },
  moveRight: { delay: 0, use: 0 },
  hybridImitation: { delay: 0, use: 0 },
  walkingImitation: { delay: 0, use: 0 },
  attack: { delay: 0, use: 0}
};

type SyncKeys = 'spamming' | 'jumping' | 'shifting' | 'waving' | 'spinning' | 'moveForward' | 'moveBack'
                | 'moveLeft' | 'moveRight' | 'hybridImitation' | 'walkingImitation' | 'attack';

// Пожайлуста, не трогай эту функцию
async function sleep(synchronize: boolean, options: { min: number, max: number, key?: SyncKeys }) {
  if (synchronize) {
    if (!options.key) return;

    if (sync[options.key].use >= activeBots.size) {
      sync[options.key].delay = 0;
      sync[options.key].use = 0;
    }
                
    if (!sync[options.key].delay) {
      sync[options.key].delay = generateNumber('float', options.min, options.max);
    }

    sync[options.key].use++;

    const delay = sync[options.key].delay;

    await new Promise(resolve => setTimeout(resolve, delay));
  } else {
    await new Promise(resolve => setTimeout(resolve, generateNumber('float', options.min, options.max)));
  }
}

export class Chat {
  public async flow(bot: Bot, data: any) {
    switch (data.type) {
      case 'message':
        await this.message(bot, data.options); break;
      case 'spamming':
        await this.spamming(bot, data.options); break;
    }
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

  private async message(bot: Bot, options: any) {
    try {
      if (!bot.object) return;

      bot.updateTask('message', true, 0.3);

      let text = '';

      if (options.useTextMutation) {
        const players = Object.keys(bot.object.players);
              
        text = mutateText({ 
          text: options.message, 
          advanced: true, 
          data: { players: players } 
        });
      } else {
        text = options.message;
      }

      if (!options.useSync) {
        await sleep(false, { min: 200, max: 4000 });
      }

      if (options.useMagicText) {
        text = this.createMagicText(text);
      }

      if (options.from === '@all') {
        bot.object.chat(text);
      } else {
        if (bot.nickname === options.from) {
          bot.object.chat(text);
        }
      }
    } finally {
      bot.updateTask('message', false);
    }
  }

  private async spamming(bot: Bot, options: any) {
    if (!bot.object) return;

    if (options.state === 'start') {
      bot.updateTask('spamming', true, 3.4);

      let lastTexts: string[] = [];

      while (bot.tasks.spamming.status) {
        let text = '';

        if (options.useTextMutation) {
          const players = Object.keys(bot.object.players);
              
          text = mutateText({ 
            text: options.message, 
            advanced: true, 
            data: { players: players } 
          });
        } else {
          text = options.message;
        }

        await sleep(options.useSync, { min: options.minDelay, max: options.maxDelay, key: 'spamming' });

        if (options.useMagicText) {
          text = this.createMagicText(text);
        }

        if (bot.tasks.spamming.status) {
          let valid = true;

          if (options.useAntiRepetition) {
            lastTexts.forEach(element => text === element ? valid = false : valid = valid);

            if (valid) {
              lastTexts.push(text);

              if (lastTexts.length > 5) {
                lastTexts.shift();
              }
            }
          } 
              
          if (options.useBypass) {
            text += chooseRandomElementFromArray([' : ', ' | ', ' / ']) + generateString('special', generateNumber('int', 3, 6));
          }

          if (valid) {
            if (options.from === '@all') {
              bot.object.chat(text);
            } else {
              if (bot.nickname === options.from) {
                bot.object.chat(text);
              }
            }
          }
        }
      }
    } else {
      bot.updateTask('spamming', false);
    }
  }
}

export class Action {
  private before = { pitch: 0, yaw: 0 };

  public async flow(bot: Bot, data: any) {
    switch (data.type) {
      case 'jumping':
        await this.jumping(bot, data.options); break;
      case 'shifting':
        await this.shifting(bot, data.options); break;
      case 'waving':
        await this.waving(bot, data.options); break;
      case 'spinning':
        await this.spinning(bot, data.options); break;
    }
  }

  private async jumping(bot: Bot, options: any) {
    if (options.state === 'start') {
      if (!bot.object) return;
      if (bot.tasks.jumping.status) return;

      try {
        bot.updateTask('jumping', true, 2.8);

        if (options.useImpulsiveness) {
          while (bot.tasks.jumping.status) {
            await sleep(options.useSync, { min: 1000, max: 1800, key: 'jumping' });

            if (bot.tasks.jumping.status) {
              bot.object.setControlState('jump', true);
              await sleep(options.useSync, { min: options.minDelay, max: options.maxDelay, key: 'jumping' });
              bot.object.setControlState('jump', false);

              if (Math.random() > 0.7) {
                for (let i = 0; i < generateNumber('int', 2, 4); i++) {
                  await sleep(options.useSync, { min: 400, max: 800, key: 'jumping' });
                  bot.object.setControlState('jump', true);
                  await sleep(options.useSync, { min: options.minDelay, max: options.maxDelay, key: 'jumping' });
                  bot.object.setControlState('jump', false);
                }
              }
            }
          }
        } else {
          await sleep(options.useSync, { min: 1000, max: 1800, key: 'jumping' });
          bot.object.setControlState('jump', true);
        }
      } catch {
        bot.updateTask('jumping', false);
      }
    } else if (options.state === 'stop') {
      if (!bot.object) return;
      if (!bot.tasks.jumping.status) return;

      bot.object.setControlState('jump', false);
      bot.updateTask('jumping', false);
    }
  }

  private async shifting(bot: Bot, options: any) {
    if (options.state === 'start') {
      if (!bot.object) return;
      if (bot.tasks.shifting.status) return;

      try {
        bot.updateTask('shifting', true, 2.8);

        if (options.useImpulsiveness) {
          while (bot.tasks.shifting.status) {
            await sleep(options.useSync, { min: 1000, max: 1800, key: 'shifting' });

            if (bot.tasks.shifting.status) {
              bot.object.setControlState('sneak', true);
              await sleep(options.useSync, { min: options.minDelay, max: options.maxDelay, key: 'shifting' });
              bot.object.setControlState('sneak', false);

              if (Math.random() > 0.7) {
                for (let i = 0; i < generateNumber('int', 2, 4); i++) {
                  await sleep(options.useSync, { min: 400, max: 800, key: 'shifting' });
                  bot.object.setControlState('sneak', true);
                  await sleep(options.useSync, { min: options.minDelay, max: options.maxDelay, key: 'shifting' });
                  bot.object.setControlState('sneak', false);
                }
              }
            }
          }
        } else {
          await sleep(options.useSync, { min: 1000, max: 1800, key: 'shifting' });
          bot.object.setControlState('sneak', true);
        }
      } catch {
        bot.updateTask('shifting', false);
      }
    } else if (options.state === 'stop') {
      if (!bot.object) return;
      if (!bot.tasks.shifting.status) return;

      bot.object.setControlState('sneak', false);
      bot.updateTask('shifting', false);
    }
  }

  private async waving(bot: Bot, options: any) {
    if (options.state === 'start') {
      if (!bot.object) return;
      if (bot.tasks.waving.status) return;

      try {
        bot.updateTask('waving', true, 2.2);

        if (options.useRandomizer) {
          while (bot.tasks.waving.status) {
            await sleep(options.useSync, { min: 1000, max: 3000, key: 'waving' });

            bot.object.swingArm('right');

            if (Math.random() > 0.5) {
              await sleep(options.useSync, { min: 300, max: 1000, key: 'waving' });
              bot.object.swingArm('right');
            }
          }
        } else {
          while (bot.tasks.waving.status) {
            await sleep(options.useSync, { min: 300, max: 350, key: 'waving' });
            bot.object.swingArm('right');
          }
        }
      } catch {
        bot.updateTask('waving', false);
      }
    } else if (options.state === 'stop') {
      if (!bot.tasks.waving.status) return;

      bot.updateTask('waving', false);
    }
  }

  private async spinning(bot: Bot, options: any) {
    if (options.state === 'start') {
      if (!bot.object) return;
      if (bot.tasks.spinning.status) return;

      try {
        bot.updateTask('spinning', true, 2.8);

        this.before.pitch = bot.object.entity.pitch;
        this.before.yaw = bot.object.entity.yaw;

        if (options.useRealism) {
          while (bot.tasks.spinning.status) {
            await sleep(options.useSync, { min: 300, max: 500, key: 'spinning' });

            bot.object.entity.yaw += chooseRandomElementFromArray([0.2, 0.4, 0.5, 0.6, 0.8, 0.9]);

            bot.object.entity.pitch = (generateNumber('float', -5, 5) + Math.random() * 2) / 6;

            bot.object.look(bot.object.entity.yaw, bot.object.entity.pitch, true);
          }
        } else {
          while (bot.tasks.spinning.status) {
            await sleep(options.useSync, { min: 130, max: 200, key: 'spinning' });

            bot.object.entity.yaw += chooseRandomElementFromArray([0.2, 0.4, 0.5, 0.6, 0.8]);

            bot.object.look(bot.object.entity.yaw, bot.object.entity.pitch, true);
          }
        }
      } catch {
        bot.updateTask('spinning', false);
      } finally {
        bot.object.look(this.before.yaw, this.before.pitch, false);
      }
    } else if (options.state === 'stop') {
      if (!bot.tasks.spinning.status) return;

      bot.updateTask('spinning', false);
    }
  }
}

export class Move {
  public async flow(bot: Bot, data: any) {
    await this.move(bot, data.options);
  }

  private async move(bot: Bot, options: any) {
    let name: SyncKeys | undefined;

    switch (options.direction) {
      case 'forward':
        name = 'moveForward'; break;
      case 'back':
        name = 'moveBack'; break;
      case 'left':
        name = 'moveLeft'; break;
      case 'right':
        name = 'moveRight'; break;
    }

    if (options.state === 'start') {
      if (!name) return;
      if (!bot.object) return;

      bot.updateTask(name, true, 1.8);

      try {
        if (options.useImpulsiveness) {
          while (bot.tasks[name].status) {
            await sleep(options.useSync, { min: 400, max: 2000, key: name });

            if (!bot.tasks[name].status) break;

            bot.object.setControlState(options.direction, true);
            await sleep(options.useSync, { min: 400, max: 2000, key: name });
            bot.object.setControlState(options.direction, false);
            await sleep(options.useSync, { min: 400, max: 2000, key: name });

            if (Math.random() >= 0.58) {
              if (!bot.tasks[name].status) break;

              bot.object.setControlState(options.direction, true);
              await sleep(options.useSync, { min: 400, max: 2000, key: name });
              bot.object.setControlState(options.direction, false);
              await sleep(options.useSync, { min: 400, max: 2000, key: name });
            }
          }
        } else {
          bot.object.setControlState(options.direction, true);
        }
      } catch {
        bot.object.setControlState(options.direction, false);
        bot.updateTask(name, false);
      }
    } else if (options.state === 'stop') {
      if (!name) return;
      if (!bot.object) return;
      if (!bot.tasks[name].status) return;

      bot.object.setControlState(options.direction, false);
      bot.updateTask(name, false);
    }
  }
}

export class Imitation {
  private readonly CHAIN_LENGTH: number = 15;
  private latestData = {
    hybrid: {
			chains: [] as string[],
			multitasking: [] as string[]
		},
		walking: {
			chains: [] as string[],
			multitasking: [] as string[]
		}
  };

  public async flow(bot: Bot, data: any) {
    switch (data.type) {
      case 'hybrid':
        await this.hybridImitation(bot, data.options); break;
      case 'walking':
        await this.walkingImitation(bot, data.options); break;
    }
  }

  private async generateHybridChain(multitasking: boolean) {
    let chain = '';

    const actions = [
      'sneak',
      'jump',
      'forward',
      'back',
      'left',
      'right'
    ];

    for (let i = 0; i < this.CHAIN_LENGTH; i++) {
      if (i !== 0) chain += '&';

      if (multitasking) {
        if (Math.random() >= 0.8) {
          chain += chooseRandomElementFromArray(actions);
        } else {
          let string = '';

          while (true) {
            string = '';

            let generatedActions: string[] = [];

            for (let k = 0; k < generateNumber('int', 2, 6); k++) {
              let generate = '';

              while (true) {
                generate = chooseRandomElementFromArray(actions);

                let valid = true;

                generatedActions.forEach((element) => {
                  if (generate === element) {
                    valid = false;
                  }
                });

                if (valid) break;
              }

              generatedActions.push(generate);

              if (k !== 0) string += '-';
              
              string += generate;
            }

            generatedActions = [];

            let valid = true;

            this.latestData.hybrid.multitasking.forEach(element => {
              if (string === element) {
                valid = false;
              }
            });

            if (valid) {
              this.latestData.hybrid.multitasking.push(string);

              if (this.latestData.hybrid.multitasking.length > 12) {
                this.latestData.hybrid.multitasking = [];
              }

              chain += string;

              break;
            }
          }
        }
      } else {
        chain += chooseRandomElementFromArray(actions);
      }

      if (Math.random() <= 0.1) {
        chain += `:${chooseRandomElementFromArray(['0', '1', '2'])}:${generateNumber('float', 200, 4000)}`;
      } else {
        chain += `:${chooseRandomElementFromArray(['0', '1', '2'])}:${generateNumber('float', 100, 200)}`;
      }
    }

    return chain;
  }

  private async generateWalkingChain(multitasking: boolean) {
    let chain = '';

    const actions = [
      'forward',
      'back',
      'left',
      'right'
    ];

    for (let i = 0; i < this.CHAIN_LENGTH; i++) {
      if (i !== 0) chain += '&';

      if (multitasking) {
        if (Math.random() >= 0.8) {
          chain += chooseRandomElementFromArray(actions);
        } else {
          let string = '';

          while (true) {
            string = '';

            let generatedActions: string[] = [];

            for (let k = 0; k < generateNumber('int', 2, 4); k++) {
              let generate = '';

              while (true) {
                generate = chooseRandomElementFromArray(actions);

                let valid = true;

                generatedActions.forEach(element => generate === element ? valid = false : valid = valid);
                
                if (valid) break;
              }

              generatedActions.push(generate);

              if (k !== 0) string += '-';
              
              string += generate;
            }

            generatedActions = [];

            let valid = true;

            this.latestData.walking.multitasking.forEach(element => string === element ? valid = false : valid = valid);

            if (valid) {
              this.latestData.walking.multitasking.push(string);

              if (this.latestData.walking.multitasking.length > 12) {
                this.latestData.walking.multitasking = [];
              }

              chain += string;

              break;
            }
          }
        }
      } else {
        chain += chooseRandomElementFromArray(actions);
      }

      if (Math.random() <= 0.1) {
        chain += `:${generateNumber('float', 2000, 4000)}`;
      } else {
        chain += `:${generateNumber('float', 500, 2300)}`;
      }
    }

    return chain;
  }

  private async looking(bot: Bot, useSmoothness: boolean, useLongDelays: boolean) {
    if (!bot.object) return;
    if (bot.tasks.looking.status) return;

    try {
      bot.updateTask('looking', true, 3.5);

      while (bot.tasks.looking.status) {
        if (Math.random() >= 0.8) { 
          const entity = bot.object.nearestEntity();

          if (entity && entity.type === 'player') {
            bot.object.lookAt(entity.position.offset(chooseRandomElementFromArray([Math.random() / 3, -(Math.random() / 3)]), entity.height, chooseRandomElementFromArray([Math.random() / 3, -(Math.random() / 3)])), !useSmoothness);
          }
        } else {
          //const yaw = Math.random();
          //const pitch = Math.random() / 3;

          const vector = new Vec3(bot.object.entity.position.x + generateNumber('float', -1, 1), bot.object.entity.position.y + generateNumber('float', -0.1, 0.1), bot.object.entity.position.z + generateNumber('float', -1, 1));

          bot.object.lookAt(vector, !useSmoothness);
        }

        if (useLongDelays) {
          await new Promise(resolve => setTimeout(resolve, generateNumber('float', 700, 1200)));
        } else {
          await new Promise(resolve => setTimeout(resolve, generateNumber('float', 250, 600)));
        }
      }
    } catch {
      bot.updateTask('looking', false);
    } finally {
      bot.updateTask('looking', false);
    }
  }

  private async waving(bot: Bot, useLongDelays: boolean) {
    if (!bot.object) return;
    if (bot.tasks.waving.status) return;

    try {
      bot.updateTask('waving', true, 3.2);

      while (bot.tasks.waving.status) {
        const randomChance = Math.random();

        if (randomChance >= 0.6) {
          bot.object.swingArm('right');
        } else if (randomChance < 0.6 && randomChance >= 0.3) {
          bot.object.swingArm('right');

          if (useLongDelays) {
            await new Promise(resolve => setTimeout(resolve, generateNumber('float', 250, 800)));
          } else {
            await new Promise(resolve => setTimeout(resolve, generateNumber('float', 50, 250)));
          }

          bot.object.swingArm('right');
        }

        if (useLongDelays) {
          await new Promise(resolve => setTimeout(resolve, generateNumber('float', 1800, 2500)));
        } else {
          await new Promise(resolve => setTimeout(resolve, generateNumber('float', 800, 1800)));
        }
      }
    } catch {
      bot.updateTask('waving', false);
    } finally {
      bot.updateTask('waving', false);
    }
  }

  private async hybridImitation(bot: Bot, options: any) {
    if (options.state === 'start') {
      if (!bot.object) return;
      if (bot.tasks.hybridImitation.status) return;
      if (bot.tasks.jumping.status || bot.tasks.shifting.status || bot.tasks.moveForward.status || bot.tasks.moveBack.status || bot.tasks.moveLeft.status || bot.tasks.moveRight.status) return;

      if (!options) return;

      try {
        bot.updateTask('hybridImitation', true, 4.4);
        bot.updateTask('jumping', true, 0.8);
        bot.updateTask('shifting', true, 0.8);
        bot.updateTask('moveForward', true, 0.5);
        bot.updateTask('moveBack', true, 0.5);
        bot.updateTask('moveLeft', true, 0.5);
        bot.updateTask('moveRight', true, 0.5);

        if (options.useLooking) this.looking(bot, options.useSmoothness, options.useLongDelays);
        if (options.useWaving) this.waving(bot, options.useLongDelays);

        let isFirst = true;

        while (bot.tasks.hybridImitation.status) {
          if (isFirst) {
            await sleep(options.useSync, { min: 1000, max: 2800, key: 'hybridImitation' });
            isFirst = false;
          }

          let chain = '';

          let attempts = 0;

          while (attempts < 20) {
            chain = await this.generateHybridChain(options.useMultitasking);

            let valid = true;

            this.latestData.hybrid.chains.forEach(element => chain === element ? valid = false : valid = valid);

            if (valid) break;

            attempts++;
          }

          this.latestData.hybrid.chains.push(chain);

          if (this.latestData.hybrid.chains.length > 20) {
            this.latestData.hybrid.chains.shift();
          }

          const operations = chain.split('&');
            
          for (const operation of operations) {
            const actions = String(operation.split(':')[0]).split('-');
            const mode = Number(operation.split(':')[1]);
            const delay = Number(operation.split(':')[2]);

            let usedActions: ControlState[] = [];

            for (const action of actions) {
              if (!bot.tasks.hybridImitation.status || !bot.tasks.jumping.status || !bot.tasks.shifting.status || !bot.tasks.moveForward.status || !bot.tasks.moveBack.status || !bot.tasks.moveLeft.status || !bot.tasks.moveRight.status) break;

              bot.object.setControlState(action as ControlState, true);
              usedActions.push(action as ControlState);

              if (options.useLongDelays) {
                await sleep(options.useSync, { min: 300, max: 800, key: 'hybridImitation' });
              } else {
                await sleep(options.useSync, { min: 100, max: 200, key: 'hybridImitation' });
              }
            }

            if (mode === 1) {
              if (usedActions.includes('forward')) {
                bot.object.setControlState('sprint', true);
                usedActions.push('sprint');
              }

              await sleep(options.useSync, { min: 1000, max: 4000, key: 'hybridImitation' });
            } else if (mode === 2) {
              if (usedActions.includes('forward')) {
                if (Math.random() > 0.5) {
                  bot.object.setControlState('sprint', true);
                  usedActions.push('sprint');
                }
              }

              await sleep(options.useSync, { min: 3000, max: 6000, key: 'hybridImitation' });
            } else {
              await sleep(options.useSync, { min: 500, max: 2000, key: 'hybridImitation' });
            }

            usedActions.forEach(action => {
              if (!bot.object) return;
              bot.object.setControlState(action, false);
            });

            usedActions = [];

            await sleep(options.useSync, { min: delay, max: delay, key: 'hybridImitation' });
          }
        }
      } finally {
        bot.updateTask('hybridImitation', false);
        bot.updateTask('jumping', false);
        bot.updateTask('shifting', false);
        bot.updateTask('looking', false);
        bot.updateTask('waving', false);
        bot.updateTask('moveForward', false);
        bot.updateTask('moveBack', false);
        bot.updateTask('moveLeft', false);
        bot.updateTask('moveRight', false);
      }
    } else if (options.state === 'stop') {
      if (!bot.tasks.hybridImitation.status) return;

      bot.updateTask('hybridImitation', false);
      bot.updateTask('jumping', false);
      bot.updateTask('shifting', false);
      bot.updateTask('looking', false);
      bot.updateTask('waving', false);
      bot.updateTask('moveForward', false);
      bot.updateTask('moveBack', false);
      bot.updateTask('moveLeft', false);
      bot.updateTask('moveRight', false);

      this.latestData.hybrid.chains = [];
    }
  }

  private async walkingImitation(bot: Bot, options: any) {
    if (options.state === 'start') {
      if (!bot.object) return;
      if (bot.tasks.walkingImitation.status) return;
      if (bot.tasks.moveForward.status || bot.tasks.moveBack.status || bot.tasks.moveLeft.status || bot.tasks.moveRight.status || bot.tasks.looking.status) return;

      if (!options) return;

      try {
        bot.updateTask('walkingImitation', true, 3.8);
        bot.updateTask('moveForward', true, 0.5);
        bot.updateTask('moveBack', true, 0.5);
        bot.updateTask('moveLeft', true, 0.5);
        bot.updateTask('moveRight', true, 0.5);

        this.looking(bot, options.useSmoothness, options.useLongDelays);

        let isFirst = true;

        while (bot.tasks.walkingImitation.status) {
          if (isFirst) {
            await sleep(options.useSync, { min: 1000, max: 2800, key: 'walkingImitation' });
            isFirst = false;
          }

          let chain = '';

          let attempts = 0;

          while (attempts < 20) {
            chain = await this.generateWalkingChain(options.useMultitasking);

            let valid = true;

            this.latestData.walking.chains.forEach(element => chain === element ? valid = false : valid = valid);

            if (valid) break;

            attempts++;
          }

          if (!bot.tasks.walkingImitation.status || !bot.tasks.moveForward.status || !bot.tasks.moveBack.status || !bot.tasks.moveLeft.status || !bot.tasks.moveRight.status) break;

          this.latestData.walking.chains.push(chain);

          if (this.latestData.walking.chains.length > 20) {
            this.latestData.walking.chains.shift();
          }

          const operations = chain.split('&');
              
          for (const operation of operations) {
            const actions = String(operation.split(':')[0]).split('-');
            const delay = Number(operation.split(':')[1]);

            let usedActions: ControlState[] = [];

            for (const action of actions) {
              if (!bot.tasks.walkingImitation.status || !bot.tasks.moveForward.status || !bot.tasks.moveBack.status || !bot.tasks.moveLeft.status || !bot.tasks.moveRight.status) break;

              bot.object.setControlState(action as ControlState, true);
              usedActions.push(action as ControlState);

              if (options.useLongDelays) {
                await sleep(options.useSync, { min: 300, max: 800, key: 'walkingImitation' });
              } else {
                await sleep(options.useSync, { min: 100, max: 200, key: 'walkingImitation' });
              }
            }

            if (options.useSprint) {
              if (usedActions.includes('forward') && Math.random() > 0.7) {
                bot.object.setControlState('sprint', true);
                usedActions.push('sprint');
              }
            }

            await sleep(options.useSync, { min: 1000, max: 5000, key: 'walkingImitation' });

            usedActions.forEach(action => {
              if (!bot.object) return;
              bot.object.setControlState(action, false);
            });

            usedActions = [];

            await sleep(options.useSync, { min: delay, max: delay, key: 'walkingImitation' });
          }
        }
      } finally {
        bot.updateTask('walkingImitation', false);
        bot.updateTask('looking', false);
        bot.updateTask('moveForward', false);
        bot.updateTask('moveBack', false);
        bot.updateTask('moveLeft', false);
        bot.updateTask('moveRight', false);
      }
    } else if (options.state === 'stop') {
      if (!bot.tasks.walkingImitation.status) return;

      bot.updateTask('walkingImitation', false);
      bot.updateTask('looking', false);
      bot.updateTask('moveForward', false);
      bot.updateTask('moveBack', false);
      bot.updateTask('moveLeft', false);
      bot.updateTask('moveRight', false);

      this.latestData.walking.chains = [];
    }
  }
}

export class Attack {
  private before: any = {};

  public async flow(bot: Bot, data: any) {
    await this.attack(bot, data.options);
  }

  private setup(bot: Bot) {
    if (!bot.object) return;

    this.before[bot.nickname] = {
      pitch: bot.object.entity.pitch,
      yaw: bot.object.entity.yaw
    };

    const items = bot.object.inventory.items();

    for (const item of items) {
      if (!bot.object) return;
      
      if (item.name.includes('sword') || item.name.includes('axe')) {
        if (item.slot > 8) {
          bot.object.moveSlotItem(item.slot, 0);
          bot.object.setQuickBarSlot(0);
        } else {
          bot.object.setQuickBarSlot(item.slot);
        }

        break;
      }
    }
  }

  private reset(bot: Bot) {
    if (!bot.object) return;

    bot.object.setControlState('jump', false);
    bot.object.setControlState('sneak', false);

    const current = this.before[bot.nickname];

    if (current) {
      bot.object.look(current.yaw, current.pitch, false);
    }
  }

  private async aiming(bot: Bot, type: any) {
    const interval = setInterval(() => {
      if (!bot.tasks.attack.status) {
        clearInterval(interval);
        return;
      }

      if (!bot.object) return;

      const target = bot.object.nearestEntity();

      if (target) {
        if ((type === 'all' && (target.type === 'mob' || target.type === 'player')) || type === target.type) {
          const impreciseVector = new Vec3(target.position.x + generateNumber('float', -0.6, 0.6), target.position.y + generateNumber('float', -0.1, 0.1), target.position.z + generateNumber('float', -0.6, 0.6));

          bot.object.lookAt(impreciseVector, Math.random() > 0.5 ? true : false);
        }
      }
    }, generateNumber('float', 1000, 2500));
  }

  private dodging(bot: Bot) {
    const interval = setInterval(async () => {
      if (!bot.tasks.attack.status) {
        clearInterval(interval);
        return;
      }

      if (!bot.object) return;

      const directions1: ControlState[] = ['forward', 'back'];
      const directions2: ControlState[] = ['left', 'right'];

      const direction1: ControlState = chooseRandomElementFromArray(directions1);
      const direction2: ControlState = chooseRandomElementFromArray(directions2);

      bot.object.setControlState(direction1, true);
      bot.object.setControlState(direction2, true);

      await sleep(false, { min: 150, max: 300 });

      bot.object.setControlState(direction1, false);
      bot.object.setControlState(direction2, false);
    }, generateNumber('float', 400, 600));
  }

  private async hit(bot: Bot, options: any) {
    if (!bot.object) return;

    const target = bot.object.nearestEntity();

    if (!target) return;

    if ((options.target === 'all' && (target.type === 'mob' || target.type === 'player')) || options.target === target.type) {
      if (bot.object.entities[target.id]) {
        if (options.useImitationOfMisses && Math.random() > 0.9) {
          bot.object.lookAt(target.position.offset(0, target.height ?? 1.6, 0), options.useNeatness ? false : true);
          bot.object.swingArm('right');
        } else {
          if (bot.object.entity.position.distanceTo(target.position) <= options.distance) {
            if (options.useImprovedStrikes) {
              bot.object.setControlState('back', true);
              await sleep(false, { min: 200, max: 300 });
              bot.object.setControlState('back', false);
              bot.object.setControlState('forward', true);
            }

            bot.object.lookAt(target.position.offset(0, target.height ?? 1.6, 0), options.useNeatness ? false : true);
            await sleep(false, { min: 50, max: 150 });
            bot.object.lookAt(target.position.offset(0, target.height ?? 1.6, 0), true);
            bot.object.attack(target);

            if (bot.object.getControlState('forward')) {
              bot.object.setControlState('back', false);
            }
          }
        }
      }
    }
  }

  private async attack(bot: Bot, options: any) {
    if (options.state === 'start') {
      if (!bot.object) return;
      if (bot.tasks.attack.status) return;

      if (!options) return;

      bot.updateTask('attack', true, 4.8);

      if (options.useDodging) this.dodging(bot);

      this.setup(bot);
      this.aiming(bot, options.target);

      while (bot.tasks.attack.status) {
        await sleep(false, { min: options.minDelay, max: options.maxDelay });

        if (!bot.tasks.attack.status) return;

        this.hit(bot, options);

        await sleep(false, { min: options.useLongDelays ? 700 : 500, max: options.useLongDelays ? 900 : 650 });

        if (options.useImprovedStrikes) {
          const action: ControlState = chooseRandomElementFromArray(['jump', 'sneak']);

          bot.object.setControlState(action, true);
          this.hit(bot, options);
          await sleep(false, { min: options.useLongDelays ? 500 : 300, max: options.useLongDelays ? 700 : 450 });
          bot.object.setControlState(action, false);
        }
      }
    } else if (options.state === 'stop') {
      if (!bot.object) return;
      if (!bot.tasks.attack.status) return;

      this.reset(bot);

      bot.updateTask('attack', false);
    }
  }
}

export class Flight {
  public async flow(bot: Bot, data: any) {
    await this.flight(bot, data.type, data.options);
  }

  public async flight(bot: Bot, type: 'default' | 'jump' | 'glitch', options: any) {
    try {
      if (options.state === 'start') {
        if (!type) return;
        if (bot.tasks.flight.status) return;

        bot.updateTask('flight', true, 3.8);
        
        switch (type) {
          case 'default':
            await this.defaultFlight(bot, options); break;
          case 'jump':
            await this.jumpFlight(bot, options); break;
          case 'glitch':
            await this.glitchFlight(bot, options); break;
        }
      } else if (options.state === 'stop') {
        if (!bot.tasks.flight.status) return;

        bot.updateTask('flight', false);
      }
    } catch {
      bot.updateTask('flight', false);
    }
  }

  private async defaultFlight(bot: Bot, options: any) {
    try {
      if (!bot.object) return;
      
      if (options.useSpoofing) {
        bot.control('spoofer', 'on', { useSharpness: false, useBuffering: true });
      }

      while (bot.tasks.flight.status) {
        this.boost(bot);

        for (let i = 0; i < generateNumber('int', 4, options.useHighPower ? 10 : 6); i++) {
          this.lift(bot);
          await sleep(false, { min: 5, max: 10 });
        }

        if (options.useHovering) {
          await this.hover(bot);
        } else {
          await sleep(false, { min: 100, max: 150 });
        }
      }
    } finally {
      bot.control('spoofer', 'off', undefined);
    }
  }

  private async jumpFlight(bot: Bot, options: any) {
    try {
      if (!bot.object) return;
      
      if (options.useSpoofing) {
        bot.control('spoofer', 'on', { useSharpness: false, useBuffering: true });
      }

      let isFirst = true;

      while (bot.tasks.flight.status) {
        this.boost(bot);

        bot.object.setControlState('jump', true);
        
        if (isFirst) {
          for (let i = 0; i < generateNumber('int', 4, options.useHighPower ? 10 : 6); i++) {
            this.lift(bot);
            await sleep(false, { min: 5, max: 10 });
          }

          isFirst = false;
        }

        bot.object.setControlState('jump', false);

        if (options.useHovering) {
          await this.hover(bot);
        } else {
          await sleep(false, { min: 100, max: 150 });
        }
      }
    } finally {
      bot.control('spoofer', 'off', undefined);
    }
  }

  private async glitchFlight(bot: Bot, options: any) {
    try {
      if (!bot.object) return;

      if (options.useSpoofing) {
        bot.control('spoofer', 'on', { useSharpness: false, useBuffering: true });
      }

      let isFirst = true;

      while (bot.tasks.flight.status) {
        this.boost(bot);

        if (isFirst) {
          for (let i = 0; i < generateNumber('int', 4, options.useHighPower ? 10 : 6); i++) {
            this.lift(bot);
            await sleep(false, { min: 5, max: 20 });
          }

          isFirst = false;
        }

        this.glitch(bot);

        if (options.useHovering) {
          await this.hover(bot);
        } else {
          await sleep(false, { min: 100, max: 150 });
        }
      }
    } finally {
      bot.control('spoofer', 'off', undefined);
    }
  }

  private send(bot: Bot, y: number, onGround: boolean = false) {
    if (!bot.object) return;

    bot.object._client.write('position_look', {
      x: bot.object.entity.position.x,
      y: y,
      z: bot.object.entity.position.z,
      yaw: bot.object.entity.yaw,
      pitch: bot.object.entity.pitch,
      onGround: onGround,
      time: Date.now(),
      flags: { onGround: onGround, hasHorizontalCollision: undefined }
    });
  }

  private boost(bot: Bot) {
    if (!bot.object) return;

    for (let i = 0; i < generateNumber('int', 6, 8); i++) {
      this.send(bot, bot.object.entity.position.y + generateNumber('float', 0.004, 0.0055));
    }
  }
    
  private async lift(bot: Bot) {
    if (!bot.object) return;

    if (Math.random() > 0.5) {
      this.send(bot, bot.object.entity.position.y + generateNumber('float', 0.001, 0.0015));
    } else {
      this.send(bot, bot.object.entity.position.y + generateNumber('float', 0.001, 0.0015));
      await sleep(false, { min: 30, max: 60 });
      this.send(bot, bot.object.entity.position.y + generateNumber('float', 0.001, 0.0015));
    }
  }

  private async glitch(bot: Bot) {
    if (!bot.object) return;

    const x = bot.object.entity.position.x;
    const z = bot.object.entity.position.z;

    const glitchDuration = generateNumber('float', 10, 50);

    const startTime = Date.now();
        
    while (Date.now() - startTime < glitchDuration) {
      bot.object.entity.position.x = x + generateNumber('float', -0.001, 0.001);
      bot.object.entity.position.z = z + generateNumber('float', -0.001, 0.001);

      await sleep(false, { min: 10, max: 15 });
    }

    bot.object.entity.position.x = x;
    bot.object.entity.position.z = z;
  }
  
  private async hover(bot: Bot) {
    if (!bot.object) return;

    const startTime = Date.now();
    const duration = generateNumber('float', 80, 100);
    const y = bot.object.entity.position.y;

    while (Date.now() - startTime < duration) {
      bot.object.entity.position.y = y + generateNumber('float', -0.0005, 0.001);
      await sleep(false, { min: 5, max: 10 });
    }
  }
}

export class Sprinter {
  public async flow(bot: Bot, data: any) {
    await this.sprinter(bot, data.type, data.options);
  }

  public async sprinter(bot: Bot, type: 'default' | 'phantom' | 'impulse', options: any) {
    try {
      if (options.state === 'start') {
        if (!options) return;

        bot.updateTask('sprinter', true, 4.8);
        
        switch (type) {
          case 'default':
            await this.defaultSprinter(bot, options); break;
          case 'phantom':
            await this.defaultSprinter(bot, options); break;
          case 'impulse':
            await this.defaultSprinter(bot, options); break;
        }
      } else if (options.state === 'stop') {
        if (!bot.tasks.sprinter.status) return;

        bot.updateTask('sprinter', false);
      }
    } catch {
      bot.updateTask('sprinter', false);
    }
  }

  private async defaultSprinter(bot: Bot, options: any) {
    if (!bot.object) return;

    if (options.useSpoofing) { 
      bot.control('spoofer', 'on', { useSharpness: false, useBuffering: true });
    }

    bot.object.setControlState('forward', true);

    const interval = setInterval(async () => {
      if (!bot.object) return;

      if (!bot.tasks.sprinter.status) {
        bot.control('spoofer', 'off', undefined);
        bot.object.setControlState('forward', false);
        clearInterval(interval);
        return;
      }

      if (bot.object.getControlState('forward')) {
        await this.microBoost(bot);
      }
    }, generateNumber('float', 300, 700));
  }
    
  private async microBoost(bot: Bot) {
    if (!bot.object) return;
    bot.object.setControlState('sprint', true);
    bot.object.setControlState('jump', true);

    if (Math.random() > 0.5) {
      bot.object.entity.position.z = bot.object.entity.position.z + chooseRandomElementFromArray([0.3, 0.6, 0.9]);
    } else {
      bot.object.entity.position.z = bot.object.entity.position.z + chooseRandomElementFromArray([0.05, 0.07, 0.09]);
      await sleep(false, { min: 40, max: 100 });
      bot.object.entity.position.z = bot.object.entity.position.z + chooseRandomElementFromArray([0.3, 0.6, 0.9]);
    }

    bot.object.setControlState('jump', false);
    bot.object.setControlState('sprint', false);
  }
}

export class Ghost {
  private activePacketIgnore: any = {};
  private originalWrite: any = {};
  private packetBuffer: any = {};

  public async flow(bot: Bot, data: any) {
    await this.ghost(bot, data.type, data.options);
  }

  private reset(bot: Bot) {
    this.clearPacketBuffer(bot);

    this.activePacketIgnore = {};
    this.originalWrite = {};
    this.packetBuffer = {};
  }

  private packetBuffering(bot: Bot, packetName: string, packetData: any) {
    const packetId = Date.now() + Math.random();

    this.packetBuffer[bot.nickname].set(packetId, {
      name: packetName,
      data: packetData
    });
  }

  private async clearPacketBuffer(bot: Bot) {
    if (!bot.object) return;
    if (this.packetBuffer[bot.nickname].size === 0) return;
    
    for (const [id, packet] of this.packetBuffer[bot.nickname].entries()) {
      this.originalWrite[bot.nickname].call(bot.object._client, packet.name, packet.data);
      this.packetBuffer[bot.nickname].delete(id);
    }
  }

  private async temperatePacketIgnore(bot: Bot, options: any) {
    try {
      if (!bot.object) return;

      if (!this.originalWrite[bot.nickname]) {
        this.originalWrite[bot.nickname] = bot.object._client.write;
      }

      bot.object._client.write = async (packetName, packetData) => {
        if (!bot.object) return;

        if (bot.tasks.ghost.status) {
          if (Math.random() > 0.5) {
            if (packetName === 'keep_alive' || packetName === 'position' || packetName === 'position_look' && Math.random() > 0.8) {
              if (options.useBuffering) {
                this.packetBuffering(bot, packetName, packetData);
              }

              return;
            } else {
              this.originalWrite[bot.nickname].call(bot.object._client, packetName, packetData);
            }

            if (Math.random() > 0.65) {
              this.clearPacketBuffer(bot);
            }
          } else {
            this.originalWrite[bot.nickname].call(bot.object._client, packetName, packetData);
          }
        } else {
          this.clearPacketBuffer(bot);
          this.originalWrite[bot.nickname].call(bot.object._client, packetName, packetData);
        }
      }
    } catch {
      this.reset(bot);
      bot.updateTask('ghost', false);
    }
  }

  private async normalPacketIgnore(bot: Bot, options: any) {
    try {
      if (!bot.object) return;

      if (!this.originalWrite[bot.nickname]) {
        this.originalWrite[bot.nickname] = bot.object._client.write;
      }
          
      bot.object._client.write = (packetName, packetData) => {
        if (!bot.object) return;

        if (bot.tasks.ghost.status && this.activePacketIgnore[bot.nickname]) {         
          if (packetName === 'keep_alive' || packetName === 'position' || packetName === 'position_look' && Math.random() > 0.6) {
            if (options.useBuffering) {
              this.packetBuffering(bot, packetName, packetData);
            }

            return;
          }

          if (Math.random() > 0.85) {
            this.clearPacketBuffer(bot);
          }
        } else {
          this.clearPacketBuffer(bot);
          this.originalWrite[bot.nickname].call(bot.object._client, packetName, packetData);
        }
      }
    } catch {
      this.reset(bot);
      bot.updateTask('ghost', false);
    }
  }

  private async aggressivePacketIgnore(bot: Bot, options: any) {
    try {
      if (!bot.object) return;

      if (!this.originalWrite[bot.nickname]) {
        this.originalWrite[bot.nickname] = bot.object._client.write;
      }

      bot.object._client.write = async (packetName, packetData) => {
        if (!bot.object) return;

        if (bot.tasks.ghost.status && this.activePacketIgnore[bot.nickname]) {
          if (packetName === 'keep_alive' || packetName === 'position' || packetName === 'position_look' && Math.random() > 0.3) {
            if (options.useBuffering) {
              this.packetBuffering(bot, packetName, packetData);
            }

            return;
          }

          if (Math.random() > 0.95) {
            this.clearPacketBuffer(bot);
          }
        } else {
          this.clearPacketBuffer(bot);
          this.originalWrite[bot.nickname].call(bot.object._client, packetName, packetData);
        }
      }
    } catch {
      this.reset(bot);
      bot.updateTask('ghost', false);
    }
  }

  public async ghost(bot: Bot, type: 'temperate' | 'normal' | 'aggressive', options: any) {
    if (options.state === 'start') {
      if (!bot.object) return;

      bot.updateTask('ghost', true, 3.5);

      this.activePacketIgnore[bot.nickname] = false;
      this.packetBuffer[bot.nickname] = new Map();

      if (type === 'temperate') {
        await this.temperatePacketIgnore(bot, options);
      } else  if (type === 'normal') {
        await this.normalPacketIgnore(bot, options);

        const interval = setInterval(async () => {
          if (!bot.tasks.ghost.status) {
            clearInterval(interval);
            return;
          }

          this.activePacketIgnore[bot.nickname] = true;
          await sleep(false, { min: 1000, max: 2000 });
          this.activePacketIgnore[bot.nickname] = false;
        }, generateNumber('float', 3000, 4000));
      } else if (type === 'aggressive') {
        await this.aggressivePacketIgnore(bot, options);

        const interval = setInterval(async () => {
          if (!bot.tasks.ghost.status) {
            clearInterval(interval);
            return;
          }

          this.activePacketIgnore[bot.nickname] = true;
          await sleep(false, { min: 2000, max: 2400 });
          this.activePacketIgnore[bot.nickname] = false;
        }, generateNumber('float', 2500, 3000));
       }
    } else if (options.state === 'stop') {
      if (!bot.tasks.ghost.status) return;

      this.reset(bot);

      bot.updateTask('ghost', false);
    }
  }
}