import { activeBots } from '../bot/base.js';

async function sleep(delay: number) {
	await new Promise(resolve => setTimeout(resolve, delay));
}

class ScriptController {
  private activity: boolean = false;

  public async execute(script: string) {
    this.activity = true;

    activeBots.forEach(bot => {
      bot.profile.scriptActivity = true;
    });

    const lines = script.trim().split(';');

    for (const line of lines) {
      const [operation, method] = line.split('@');

      if (!operation || !method) return;

      const [methodName, methodOptions] = method.split(':');

      console.log(operation, methodName, methodOptions);
      
      if (operation === 'time') {
        if (!this.activity) return;

        if (methodName === 'sleep') {
          const delay = Number(methodOptions);

          await sleep(delay);
        }
      } else {
        activeBots.forEach(async bot => {
          if (!this.activity) return;
          if (!bot.profile.scriptActivity) return;
          if (!bot.object) return;

          if (operation === 'chat') {
            if (methodName === 'send') {
              const message = String(methodOptions);

              bot.object.chat(message);
            }
          } else if (operation === 'action') {
            if (methodName === 'jump') {
              bot.object.setControlState('jump', true);
              await sleep(600);
              bot.object.setControlState('jump', false);
            } else if (methodName === 'shift') {
              bot.object.setControlState('sneak', true);
              await sleep(600);
              bot.object.setControlState('sneak', false);
            }
          } else if (operation === 'client') {
            if (methodName === 'rejoin') {
              await bot.recreate();
            }
          }
        });
      }
    }

    await this.stop();
  }

  public async stop() {
    this.activity = false;

    activeBots.forEach(bot => {
      bot.profile.scriptActivity = false;
    });
  }
}

export { ScriptController };