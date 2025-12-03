import { activeBots } from '../bot/base.js';

class Scripter {
  async execute(script: string) {
    let status = true;

    const lines = script.split(';');

    for (const line of lines) {
      const operation = line.split('.')[0]?.split(':')[1];
      const func = line.split('.')[1]?.split(':')[0];
      
      if (operation === 'time') {
        if (func === 'sleep') {
          const delay = Number(line.split(':')[2]);

          await new Promise(resolve => setTimeout(resolve, delay));
        }
      } else {
        for (const [_, bot] of activeBots) {
          if (operation === 'chat') {
            if (func === 'send') {
              const message = String(line.split(':')[2]);

              bot.object?.chat(message);
            }
          } if (operation === 'action') {
            const object = bot.object;

            if (func === 'jump') {
              if (object && !Array.isArray(object)) {
                object.setControlState('jump', true);
                setTimeout(() => {
                  object.setControlState('jump', false);
                }, 600);
              }
            } else if (func === 'shift') {
              if (object && !Array.isArray(object)) {
                object.setControlState('sneak', true);
                setTimeout(() => {
                  object.setControlState('sneak', false);
                }, 600);
              }
            }
          }
        }
      }
    }

    return status;
  }
}

export { Scripter };