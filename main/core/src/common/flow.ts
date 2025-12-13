import { transmit } from '../api/transfer.js';
import { activeBots } from '../bot/base.js';
import { Chat, Action, Move, Imitation, Attack, Flight, Sprinter, Ghost } from '../bot/plugins/plugins.js';

const chat = new Chat();
const action = new Action();
const move = new Move();
const imitation = new Imitation();
const attack = new Attack();
const flight = new Flight();
const sprinter = new Sprinter();
const ghost = new Ghost();

export function flow(type: string, body: any, response: any) {
  try {
    let plugin: Chat | Action | Move | Imitation | Attack | Flight | Sprinter | Ghost | undefined = undefined;
    let name = '';

    switch (type) {
      case 'chat':
        name = 'Чат';
        plugin = chat;
        break;
      case 'action':
        name = 'Действия';
        plugin = action; 
        break;
      case 'move':
        name = 'Движение';
        plugin = move;
        break;
      case 'imitation':
        name = 'Имитация';
        plugin = imitation;
        break;
      case 'attack':
        name = 'Атака';
        plugin = attack;
        break;
      case 'flight':
        name = 'Полёт';
        plugin = flight;
        break;
      case 'flight':
        name = 'Полёт';
        plugin = flight;
        break;
      case 'sprinter':
        name = 'Спринтер';
        plugin = sprinter;
        break;
      case 'ghost':
        name = 'Призрак';
        plugin = ghost;
        break;
    }

    if (plugin) {
      for (const [_, bot] of activeBots) {
        plugin.flow(bot, body);
      }

      transmit(response, 'info', {
        success: true,
        message: `Управление / ${name} ==> Команда принята (входящие данные: ${JSON.stringify(body)})`
      });
    } else {
      transmit(response, 'error', {
        success: false,
        message: `Управление / ${name} ==> Не удалось принять команду (входящие данные: ${JSON.stringify(body)})`
      });
    }
  } catch (error) {
    transmit(response, 'error', {
      success: false,
      message: error
    });
  }
}