import { invoke } from '@tauri-apps/api/core';

import { logger } from '../utils/logger';
import { generateId } from '../utils/generator';

interface ServerInformation {
  ip_address: string;
  server_icon: string | null;
  protocol_version: number;
  server_version: string;
  description: string;
  players_online: number;
  players_max: number;
  list_of_players: Array<{ username: string; uuid: string; }>;
}

class Pinger {
  /** Метод инициализации функций, связанных с пингованием. */
  public init(): void {
    document.getElementById('ping-server')?.addEventListener('click', async () => await this.ping_server());
  }

  /** Метод пингования сервера. */
	private async ping_server(): Promise<void> {
		try {
			const address = (document.getElementById('ping-server-address') as HTMLInputElement).value;

			if (address === '') return;

			const result = await invoke('get_server_info', {
				address: address
			}) as ServerInformation;

      const pingInfo = document.getElementById('ping-info') as HTMLElement;
      
      pingInfo.innerHTML = '';
      pingInfo.style.display = 'none';

      const card = document.createElement('div');
      card.className = 'card';

      const removeBtnId = `remove-ping-card-${generateId()}`;
      
      card.innerHTML = `
        <div class="panel">
          <div class="address">${address}</div>

          <button class="btn min" id="${removeBtnId}">
            ⨉
          </button>
        </div>

        <div class="head">
          <img class="icon" src="${result.server_icon}" draggable="false">

          <div class="sep"></div>

          <div class="text">
            <p>${result.description}</p>

            <div class="sep"></div>

            <p>Игроки: ${result.players_online} / ${result.players_max}</p>
            <p>IP-адрес: ${result.ip_address}</p>
            <p>Версия протокола: ${result.protocol_version}</p>
            <p>Версия сервера: ${result.server_version}</p>
          </div>
        </div>
      `;

      const listOfPlayers = document.createElement('div');
      listOfPlayers.className = 'list';
      listOfPlayers.style.display = 'none';

      if (result.list_of_players.length > 0) {
        listOfPlayers.style.display = 'flex';

        const element = document.createElement('div');
        element.className = 'element';

        element.innerHTML = `
          <p class="username">Никнейм</p>
          <div class="sep"></div>
          <p class="uuid">UUID</p>
        `;

        listOfPlayers.appendChild(element);

        for (const player of result.list_of_players) {
          const el = document.createElement('div');
          el.className = 'element';

          el.innerHTML = `
            <p class="username">${player.username}</p>
            <div class="sep"></div>
            <p class="uuid">${player.uuid}</>
          `;

          listOfPlayers.appendChild(el);
        }

        card.appendChild(listOfPlayers);
      } else {
        const header = document.createElement('div');
        header.className = 'header';
        header.innerText = 'Не удалось получить список игроков';
        card.appendChild(header);
      }

      pingInfo.appendChild(card);

      document.getElementById(removeBtnId)?.addEventListener('click', () => {
        pingInfo.innerHTML = '';
        pingInfo.style.display = 'none';
      });

      pingInfo.style.display = 'flex';
		} catch (error) {
			logger.log(`Ошибка пингования сервера: ${error}`, 'error');
		}
	}
}

const pinger = new Pinger();

export { pinger };