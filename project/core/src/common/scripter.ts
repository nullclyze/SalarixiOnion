import { activeBotsObjects } from '../bot/architecture.js';

class Scripter {
  public async execute({ script }: {
    script: string
  }) {
    let status = true;

    const lines = script.split(';');

    for (const line of lines) {
      const operation = line.split('.')[0].split(':')[1];
      const func = line.split('.')[1].split(':')[0];
      
      if (operation === 'time') {
        if (func === 'sleep') {
          const delay = Number(line.split(':')[2]);

          await new Promise(resolve => setTimeout(resolve, delay));
        }
      } else {
        for (const [_, bot] of activeBotsObjects) {
          if (operation === 'chat') {
            if (func === 'send') {
              const message = line.split(':')[2];

              bot.chat('default', {
                from: bot.nickname,
                message: message,
                useMagicText: false,
                useTextMutation: true,
                useSync: true
              });
            }
          } if (operation === 'action') {
            if (func === 'jump') {
              const object = bot.get('bot-object');

              if (object && !Array.isArray(object)) {
                object.setControlState('jump', true);
                setTimeout(() => {
                  object.setControlState('jump', false);
                }, 600);
              }
            } else if (func === 'shift') {
              const object = bot.get('bot-object');

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

export default Scripter;