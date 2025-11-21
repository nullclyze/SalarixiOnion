import Logger from './logger';
import Functions from './functions';
import { sendDataTo } from './functions';
import Cleaner from './tools/cleaner';
import date from './tools/date';
import LineGraphicManager from './graph';

const mainPageContainer = document.getElementById('main-container') as HTMLElement;
const settingsPageContainer = document.getElementById('settings-container') as HTMLElement;
const proxyPageContainer = document.getElementById('proxy-container') as HTMLElement;
const controlSectorsPageContainer = document.getElementById('control-sectors-container') as HTMLElement;
const controlChatPageContainer = document.getElementById('control-chat-container') as HTMLElement;
const controlActionsPageContainer = document.getElementById('control-action-container') as HTMLElement;
const controlMovementPageContainer = document.getElementById('control-movement-container') as HTMLElement;
const controlImitationPageContainer = document.getElementById('control-imitation-container') as HTMLElement;
const controlAttackPageContainer = document.getElementById('control-attack-container') as HTMLElement;
const controlFlightPageContainer = document.getElementById('control-flight-container') as HTMLElement;
const controlSprinterPageContainer = document.getElementById('control-sprinter-container') as HTMLElement;
const controlGhostPageContainer = document.getElementById('control-ghost-container') as HTMLElement;
const scriptPageContainer = document.getElementById('script-container') as HTMLElement;
const graphicPageContainer = document.getElementById('graphic-container') as HTMLElement;
const monitoringPageContainer = document.getElementById('monitoring-container') as HTMLElement;
const analysisPageContainer = document.getElementById('analysis-container') as HTMLElement;
const spyPageContainer = document.getElementById('spy-container') as HTMLElement;
const logPageContainer = document.getElementById('log-container') as HTMLElement;
const aboutPageContainer = document.getElementById('about-container') as HTMLElement;

const logger = new Logger();
const functions = new Functions();
const cleaner = new Cleaner();
const lineGraphic = new LineGraphicManager();

let initialized = false;

// Структура элементов в конфиге
interface ConfigElements {
  [name: string]: {
    id: string;
    value: string | number | boolean;
  };
}

let latestConfig = {};

setInterval(async () => {
  if (!initialized) return;

  const elements = document.querySelectorAll<HTMLInputElement | HTMLTextAreaElement | HTMLSelectElement>('[conserve="true"]');

  const data: ConfigElements = {};

  for (const element of elements) {
    if (element.type === 'checkbox') {
      data[element.name] = {
        id: element.id,
        value: (element as HTMLInputElement).checked
      };
    } else {
      data[element.name] = {
        id: element.id,
        value: (element as HTMLInputElement).type === 'number' ? parseInt((element as HTMLInputElement).value) : (element as HTMLInputElement).value
      };
    }
  }

  if (JSON.stringify(data) === JSON.stringify(latestConfig)) return;

  await sendDataTo({ url: 'http://localhost:37182/salarixi/utils/config/write', method: 'POST', useHeaders: true }, { 
    key: 'salarixionion:1.0.0:ol13Rqk:config:write',
    config: data
  });

  latestConfig = data;
}, 2000);


async function loadConfig() {
  logger.log('Загрузка конфига...', 'log-system');

  const operation = await sendDataTo({ url: 'http://localhost:37182/salarixi/utils/config/read', method: 'POST', useHeaders: true }, { 
    key: 'salarixionion:1.0.0:Yi8jQ13e:config:read'
  });

  if (!operation.success) {
    logger.log(`Ошибка (load-config): ${operation.message}`, 'log-error');
    return;
  }

  if (operation.answer.invalidKey) {
    logger.log(`Ошибка (load-config): ${operation.answer.message}`, 'log-error');
    return;
  } else {
    for (const [_, element] of Object.entries(operation.answer.data)) {
      if ((element as any).value) {
        const html = document.getElementById((element as any).id) as HTMLInputElement;

        if (html) {
          if (String((element as any).id).startsWith('use')) {
            html.checked = (element as any).value;
          } else {
            if (typeof (element as any).value === 'number') {
              html.valueAsNumber = (element as any).value ? (element as any).value : 0;
            } else {
              html.value = (element as any).value;
            }
          }
        } 
      }
    }

    logger.log('Конфиг успешно загружен', 'log-system');
  }
}

const client = {
  version: '1.0.0',
  type: 'Expert',
  releaseDate: '21.11.2025'
};

const globalContainers: HTMLElement[] = [
	mainPageContainer, settingsPageContainer, proxyPageContainer,
  controlSectorsPageContainer, controlChatPageContainer, controlActionsPageContainer,
  controlMovementPageContainer, controlImitationPageContainer, controlFlightPageContainer,
  controlSprinterPageContainer, controlGhostPageContainer, 
  controlAttackPageContainer, scriptPageContainer, graphicPageContainer,
  monitoringPageContainer, analysisPageContainer, spyPageContainer,
	logPageContainer, aboutPageContainer
];

// Функция для показа определённого контейнера
function showContainer(container: HTMLElement) {
  globalContainers.forEach((element) => {
    if (element.id !== container.id) {
      element.style.display = 'none';
    }
  });

  container.style.display = 'block';

  if (container.id === 'control-sectors-container') {
    setTimeout(() => { 
      changeDisplayButtonList('tools', 'client');
      setTimeout(() => { 
        changeDisplayButtonList('cheat', 'client');
      }, 130);
    }, 100);
  }
}

// Функция для смены отображения списка кнопок
function changeDisplayButtonList(type: string, who: 'client' | 'user')  {
  let buttonListContainer = document.getElementById(`control-${type}-button-list`) as HTMLElement;

  if (who === 'client' && buttonListContainer.style.display === 'block') return;

  if (buttonListContainer.style.display === 'block') {
    buttonListContainer.classList.remove('show');
    buttonListContainer.classList.add('hide');

    setTimeout(() => {
      buttonListContainer.classList.remove('hide');
      buttonListContainer.style.display = 'none';
    }, 200);
  } else {
    buttonListContainer.classList.remove('hide');
    buttonListContainer.classList.add('show');
          
    buttonListContainer.style.display = 'block';

    const buttons = buttonListContainer.querySelectorAll('.list-btn') as NodeListOf<HTMLButtonElement>;

    buttons.forEach((button: HTMLButtonElement, index: number) => {
      setTimeout(() => {
        button.classList.add('temporary-theme');

        setTimeout(() => {
          button.classList.remove('temporary-theme');
        }, 130); 
      }, index * 70); 
    });
  }
}

// Функция для инициализации карты с информацией о клиенте
async function initializeInformationCard() {
  const clientHeaderContainer = document.getElementById('client-header') as HTMLElement;
  const clientVersionContainer = document.getElementById('client-version') as HTMLElement;
  const clientTypeContainer = document.getElementById('client-type') as HTMLElement;
  const clientReleaseDateContainer = document.getElementById('client-release-date') as HTMLElement;

  if (client.type === 'Beta') {
    clientHeaderContainer.classList.add('beta');
  } else if (client.type === 'Expert') {
    clientHeaderContainer.classList.add('expert');
  }

  const header = `${client.type}-Release`;

  clientHeaderContainer.innerText = header;
  clientVersionContainer.innerText = client.version;
  clientTypeContainer.innerText = client.type;
  clientReleaseDateContainer.innerText = client.releaseDate;

  const copyGithubLinkBtn = document.getElementById('copy-github-link-btn') as HTMLButtonElement;
  const copyTelegramLinkBtn = document.getElementById('copy-telegram-link-btn') as HTMLButtonElement;
  const copyYoutubeLinkBtn = document.getElementById('copy-youtube-link-btn') as HTMLButtonElement;

  copyGithubLinkBtn.addEventListener('click', () => navigator.clipboard.writeText('https://github.com/nullclyze/SalarixiOnion')); 
  copyTelegramLinkBtn.addEventListener('click', () => navigator.clipboard.writeText('https://t.me/salarixionion'));
  copyYoutubeLinkBtn.addEventListener('click', () => navigator.clipboard.writeText('https://www.youtube.com/@salarixionion'));
}

// Функция для инициализации панели 
async function initializePanel() {
  const panelBtns = document.querySelectorAll('.panel-btn') as NodeListOf<HTMLButtonElement>;

  panelBtns.forEach(button => {
		if (button.id === 'main') button.classList.add('selected');

  	button.addEventListener('click', () => {
    	panelBtns.forEach(btn => btn.classList.remove('selected'));
      
    	button.classList.add('selected');
  	});
	});

  const mainPageBtn = document.getElementById('main') as HTMLButtonElement;
	const settingsPageBtn = document.getElementById('settings') as HTMLButtonElement;
  const proxyPageBtn = document.getElementById('proxy') as HTMLButtonElement;
  const controlPageBtn = document.getElementById('control') as HTMLButtonElement;
  const scriptPageBtn = document.getElementById('script') as HTMLButtonElement;
  const monitoringPageBtn = document.getElementById('monitoring') as HTMLButtonElement;
  const graphicPageBtn = document.getElementById('graphic') as HTMLButtonElement;
  const analysisPageBtn = document.getElementById('analysis') as HTMLButtonElement;
  const spyPageBtn = document.getElementById('spy') as HTMLButtonElement;
	const logPageBtn = document.getElementById('log') as HTMLButtonElement;
	const	aboutPageBtn = document.getElementById('about') as HTMLButtonElement;

  mainPageBtn.addEventListener('click', () => showContainer(mainPageContainer));
	settingsPageBtn.addEventListener('click', () => showContainer(settingsPageContainer));
  proxyPageBtn.addEventListener('click', () => showContainer(proxyPageContainer));
  controlPageBtn.addEventListener('click', () => showContainer(controlSectorsPageContainer));
  scriptPageBtn.addEventListener('click', () => showContainer(scriptPageContainer));
  graphicPageBtn.addEventListener('click', () => showContainer(graphicPageContainer));
  monitoringPageBtn.addEventListener('click', () => showContainer(monitoringPageContainer));
  analysisPageBtn.addEventListener('click', () => showContainer(analysisPageContainer));
  spyPageBtn.addEventListener('click', () => showContainer(spyPageContainer));
	logPageBtn.addEventListener('click', () => showContainer(logPageContainer));
	aboutPageBtn.addEventListener('click', () => showContainer(aboutPageContainer));
}

// Функция для инициализации управляющего контейнера
async function initializeControlContainer() {
  const controlChatPageBtn = document.getElementById('control-chat') as HTMLButtonElement;
  const controlActionsPageBtn = document.getElementById('control-actions') as HTMLButtonElement;
  const controlMovementPageBtn = document.getElementById('control-movement') as HTMLButtonElement;
  const controlImitationPageBtn = document.getElementById('control-imitation') as HTMLButtonElement;
  const controlAttackPageBtn = document.getElementById('control-attack') as HTMLButtonElement;
  const controlFlightPageBtn = document.getElementById('control-flight') as HTMLButtonElement;
  const controlSprinterPageBtn = document.getElementById('control-sprinter') as HTMLButtonElement;
  const controlGhostPageBtn = document.getElementById('control-ghost') as HTMLButtonElement;
  //const controlCrasherPageBtn = document.getElementById('control-crasher') as HTMLButtonElement;

  const openToolsButtonListBtn = document.getElementById('open-tools-button-list') as HTMLButtonElement;
  const openCheatButtonListBtn = document.getElementById('open-cheat-button-list') as HTMLButtonElement;

  controlChatPageBtn.addEventListener('click', () => showContainer(controlChatPageContainer));
  controlActionsPageBtn.addEventListener('click', () => showContainer(controlActionsPageContainer));
  controlMovementPageBtn.addEventListener('click', () => showContainer(controlMovementPageContainer));
  controlImitationPageBtn.addEventListener('click', () => showContainer(controlImitationPageContainer));
  controlFlightPageBtn.addEventListener('click', () => showContainer(controlFlightPageContainer));
  controlSprinterPageBtn.addEventListener('click', () => showContainer(controlSprinterPageContainer));
  controlGhostPageBtn.addEventListener('click', () => showContainer(controlGhostPageContainer));
  controlAttackPageBtn.addEventListener('click', () => showContainer(controlAttackPageContainer));

  openToolsButtonListBtn.addEventListener('click', () => changeDisplayButtonList('tools', 'user'));
  openCheatButtonListBtn.addEventListener('click', () => changeDisplayButtonList('cheat', 'user'));

  //controlCrasherPageBtn.addEventListener('click', async () => await sendDataTo({ url: 'http://localhost:37621/salarixi/terminal/tools/start', method: 'POST', useHeaders: true, }, { tool: 'crasher' }));
}

// Функция для инициализации графика активных ботов
async function initializeLineGraphicActiveBots() {
  try {
    await lineGraphic.createGraphicActiveBots();

    const eventSource = new EventSource('http://localhost:37621/salarixi/session/graphic/line/active-bots');

    eventSource.onmessage = async (event) => {
      try {
        const data = JSON.parse(event.data);

        const activeBotsQuantity = data.activeBotsQuantity;

        await lineGraphic.addGraphicDataActiveBots(activeBotsQuantity);
      } catch (error) {
        logger.log(`Ошибка парсинга SSE-сообщения (graphic-active-bots): ${error}`, 'log-error');
      }
    }

    eventSource.onerror = async () => {
      logger.log('Ошибка (graphic-active-bots): SSE-connection was dropped', 'log-error');
      eventSource.close();
    }

    (window as any).currentLineGraphicActiveBotsEventSource = eventSource;
  } catch (error) {
    logger.log(`Ошибка инициализации графика активных ботов: ${error}`, 'log-error');
  }
}

// Функция для инициализации графика активных ботов
async function initializeLineGraphicAverageLoad() {
  try {
    await lineGraphic.createGraphicAverageLoad();

    const eventSource = new EventSource('http://localhost:37621/salarixi/session/graphic/line/average-load');

    eventSource.onmessage = async (event) => {
      try {
        const data = JSON.parse(event.data);

        const averageLoad = data.averageLoad ? data.averageLoad : 0;

        await lineGraphic.addGraphicDataAverageLoad(averageLoad);
      } catch (error) {
        logger.log(`Ошибка парсинга SSE-сообщения (graphic-average-load): ${error}`, 'log-error');
      }
    }

    eventSource.onerror = async () => {
      logger.log('Ошибка (graphic-average-load): SSE-connection was dropped', 'log-error');
      eventSource.close();
    }

    (window as any).currentLineGraphicAverageLoadEventSource = eventSource;
  } catch (error) {
    logger.log(`Ошибка инициализации графика средней нагрузки ботов: ${error}`, 'log-error');
  }
}

// Функция для инициализации мониторинга чата
async function initializeChatMonitoring() {
  try {
    let NICKNAMES_LIST: string[] = [];
    let LATEST_MESSAGES: any[] = [];

    const monitoringStatusText = document.getElementById('monitoring-chat-status-text') as HTMLElement;

    setInterval(() => {
      if (!functions.flags.IS_ACTIVE) {
        NICKNAMES_LIST.length = 0;
        LATEST_MESSAGES.length = 0;
      }
    }, 3000);

    const eventSource = new EventSource('http://localhost:37621/salarixi/session/monitoring/chat');

    eventSource.onmessage = async (event) => {
      try {
        const data = JSON.parse(event.data);

        const nickname = data.nickname;
        const type = data.type;
        const text = data.text;

        const monitoringContent = document.getElementById(`monitoring-chat-content-${nickname}`) as HTMLElement;

        if (!monitoringContent) return;

        let validMessage = true;

        LATEST_MESSAGES.forEach(element => element.nickname === nickname && element.text === text ? validMessage = false : validMessage = validMessage);

        if (validMessage) {
          LATEST_MESSAGES.push({ nickname: nickname, text: text });

          if (LATEST_MESSAGES.length > 40) LATEST_MESSAGES.shift();

          const container = document.createElement('div');

          container.className = 'monitoring-line';

          container.innerHTML = `
            <div class="monitoring-line-time">${date()}</div>
            <div class="monitoring-line-content"><span class="monitoring-type">(${type})</span> ${String(text).replace('%hb', '<span style="color: #a85fdfff; font-weight: 600;">').replace('%sc', '</span>')}</div>
          `;

          monitoringContent.appendChild(container);

          let validNickname = true;

          NICKNAMES_LIST.forEach(element => nickname === element ? validNickname = false : validNickname = validNickname);

          monitoringContent.scrollTo({
            top: monitoringContent.scrollHeight,
            behavior: 'smooth'
          });
        }
      } catch (error) {
        logger.log(`Ошибка парсинга SSE-сообщения (chat-monitoring): ${error}`, 'log-error');
      }
    }

    eventSource.onerror = async () => {
      logger.log('Ошибка (chat-monitoring): SSE-connection was dropped', 'log-error');
      eventSource.close();

      LATEST_MESSAGES.length = 0;

      for (const nickname of NICKNAMES_LIST) {
        const monitoringContent = document.getElementById(`monitoring-chat-content-${nickname}`) as HTMLElement;

        monitoringContent.innerHTML = '';
        monitoringContent.style.display = 'none';
      }

      NICKNAMES_LIST.length = 0;

      monitoringStatusText.innerText = 'Ошибка соединения с сервером\nПодробнее: SSE-connection was dropped';
      monitoringStatusText.style.color = '#e32020ff';
      monitoringStatusText.style.display = 'block';
    }

    (window as any).currentChatMonitoringEventSource = eventSource;
  } catch (error) {
    logger.log(`Ошибка инициализации мониторинга чата: ${error}`, 'log-error');
  }
}

function initializeBotCard(nickname: string) {
  const openChatBtn = document.getElementById(`open-chat-${nickname}`) as HTMLButtonElement;
  const closeChatBtn = document.getElementById(`close-chat-${nickname}`) as HTMLButtonElement;
  const recreateBotBtn = document.getElementById(`recreate-${nickname}`) as HTMLButtonElement;

  openChatBtn.addEventListener('click', () => {
    try {
      const chatContainer = document.getElementById(`chat-${nickname}`) as HTMLElement;

      chatContainer.style.display = 'flex';
    } catch (error) {
      logger.log(`Ошибка открытия чата: ${error}`, 'log-error');
    }
  });

  closeChatBtn.addEventListener('click', () => {
    try {
      const chatContainer = document.getElementById(`chat-${nickname}`) as HTMLElement;

      chatContainer.style.display = 'none';
    } catch (error) {
      logger.log(`Ошибка закрытия чата: ${error}`, 'log-error');
    }
  });

  recreateBotBtn.addEventListener('click', async () => {
    try {
      const operation = await sendDataTo({ url: 'http://localhost:37621/salarixi/advanced/recreate', method: 'POST', useHeaders: true }, { 
        nickname: nickname
      });

      logger.log(operation.answer.data.message, `log-${operation.answer.type}`);
    } catch (error) {
      logger.log(`Ошибка пересоздания бота ${nickname}: ${error}`, 'log-error');
    }
  });
}

// Функция для инициализации мониторинга чата
async function initializeBotsMonitoring() {
  try {
    let NICKNAMES_LIST: string[] = [];

    const monitoringStatusText = document.getElementById('monitoring-status-text') as HTMLElement;
    const monitoringContent = document.getElementById('bots-cards-container') as HTMLElement;
    const steveIconPngPath = document.getElementById('steve-img') as HTMLImageElement;

    setInterval(() => {
      if (!functions.flags.IS_ACTIVE) {
        NICKNAMES_LIST.length = 0;
      }
    }, 3000);

    const eventSource = new EventSource('http://localhost:37621/salarixi/session/monitoring/bots');

    eventSource.onmessage = async (event) => {
      try {
        const data = JSON.parse(event.data);

        const { 
          nickname, status, statusColor, 
          version, password, proxyType, 
          proxy, reputation, reputationColor, 
          load, loadColor, ping, pingColor
        } = data;

        if (NICKNAMES_LIST.length === 0) {
          monitoringStatusText.style.display = 'none';
        }

        if (NICKNAMES_LIST.includes(nickname)) {
          let botStatus = document.getElementById(`bot-status-${nickname}`) as HTMLElement;
          let botVersion = document.getElementById(`bot-version-${nickname}`) as HTMLElement;
          let botPassword = document.getElementById(`bot-password-${nickname}`) as HTMLElement;
          let botReputation = document.getElementById(`bot-reputation-${nickname}`) as HTMLElement;
          let botProxyType = document.getElementById(`bot-proxy-type-${nickname}`) as HTMLElement;
          let botProxy = document.getElementById(`bot-proxy-${nickname}`) as HTMLElement;
          let botLoad = document.getElementById(`bot-load-${nickname}`) as HTMLElement;
          let botPing = document.getElementById(`bot-ping-${nickname}`) as HTMLElement;

          botStatus.innerHTML = `<span style="color: ${statusColor};">${status}</span>`;
          botVersion.innerHTML = `  ${version}`;
          botPassword.innerHTML = `  ${password}`;
          botReputation.innerHTML = `<span style="color: ${reputationColor};">  ${reputation}/100</span>`;
          botProxyType.innerHTML = `  ${proxyType}`;
          botProxy.innerHTML = `  ${proxy}`;
          botLoad.innerHTML = `<span style="color: ${loadColor};">  ${load}</span>`;
          botPing.innerHTML = `<span style="color: ${pingColor};">  ${ping}</span>`;
        } else {
          const card = document.createElement('div');
          card.className = 'bot-card';

          card.innerHTML = `
            <div class="bot-card-head">
              <img src="${steveIconPngPath.src}" class="image" draggable="false">
              <div class="text">
                <div class="bot-basic-info">
                  <div class="bot-nickname" style="user-select: text; -moz-user-select: text;">${nickname}</div>
                  <div class="bot-status"><span id="bot-status-${nickname}" style="color: ${statusColor};">${status}</span></div>
                </div>
              </div>
            </div>

            <div class="bot-advanced-info">
              <p>Версия:<span id="bot-version-${nickname}">  ${version}</span></p>
              <p>Пароль:<span id="bot-password-${nickname}">  ${password}</span></p>
              <p>Репутация:<span id="bot-reputation-${nickname}"><span style="color: ${reputationColor};">  ${reputation}/100</span></span></p>
              <p>Тип прокси:<span id="bot-proxy-type-${nickname}">  ${proxyType}</span></p>
              <p>Прокси:<span id="bot-proxy-${nickname}">  ${proxy}</span></p>
              <p>Нагрузка:<span id="bot-load-${nickname}"><span style="color: ${loadColor};">  ${load}</span></span></p>
              <p>Пинг:<span id="bot-ping-${nickname}"><span style="color: ${pingColor};">  ${ping}</span></span></p>
            </div>

            <button id="open-chat-${nickname}">
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z"/>
              </svg>
              Открыть чат
            </button>

            <button id="recreate-${nickname}" style="color: #e61616ff;" class="recreate-btn">
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M23 4v6h-6M1 20v-6h6M3.51 9a9 9 0 0 1 14.85-3.36L23 10M1 14l4.64 4.36A9 9 0 0 0 20.49 15"/>
              </svg>
              Пересоздать
            </button>

            <div class="chat-container" id="chat-${nickname}">
              <div class="chat-header">
                <button class="close-button" id="close-chat-${nickname}">Закрыть</button>
              </div>
              <div class="chat-content">
                <div class="monitoring-content" id="monitoring-chat-content-${nickname}"></div>
              </div>
            </div>
          `;

          monitoringContent.appendChild(card);

          NICKNAMES_LIST.push(nickname);

          setTimeout(() => initializeBotCard(nickname), 300);
        }
      } catch (error) {
        logger.log(`Ошибка парсинга SSE-сообщения (bots-monitoring): ${error}`, 'log-error');
      }
    }

    eventSource.onerror = async () => {
      logger.log('Ошибка (bots-monitoring): SSE-connection was dropped', 'log-error');
      eventSource.close();

      monitoringContent.innerHTML = '';
      monitoringStatusText.innerText = 'Ошибка соединения с сервером\nПодробнее: SSE-connection was dropped';
      monitoringStatusText.style.color = '#e32020ff';
      monitoringStatusText.style.display = 'block';
    }

    (window as any).currentBotsMonitoringEventSource = eventSource;
  } catch (error) {
    logger.log(`Ошибка инициализации мониторинга ботов: ${error}`, 'log-error');
  }
}

async function checkUpdate() {
  try {
    const closeNoticeBtn = document.getElementById('close-notice-btn') as HTMLButtonElement;

    closeNoticeBtn.addEventListener('click', () => {
      const notice = document.getElementById('notice') as HTMLElement;
      if (!notice) return;
      notice.style.display = 'none';
    });

    const notice = document.getElementById('notice') as HTMLElement;
    const newVersion = document.getElementById('new-client-version') as HTMLElement;
    const newType = document.getElementById('new-client-type') as HTMLElement;
    const newReleaseDate = document.getElementById('new-client-release-date') as HTMLElement;
    const copyDownloadLink = document.getElementById('copy-download-link') as HTMLElement;

    const response = await fetch('https://raw.githubusercontent.com/nullclyze/SalarixiOnion/refs/heads/main/salarixi.version.json', { method: 'GET' });

    if (!response.ok) return;

    const data = await response.json();

    if (data && data.version !== client.version) {
      newVersion.innerText = data.version;
      newType.innerText = data.type;
      newReleaseDate.innerText = data.releaseDate;
      
      copyDownloadLink.addEventListener('click', () => navigator.clipboard.writeText(`https://github.com/nullclyze/SalarixiOnion/releases/tag/v${data.version}-${String(data.type).toLowerCase()}`));
      
      setTimeout(() => {
        notice.style.display = 'flex';
      }, 3000);
    }
  } catch (error) {
    logger.log(`Ошибка проверки обновлений: ${error}`, 'log-error');
  }
}

// Инициализация интерфейса
document.addEventListener('DOMContentLoaded', async () => {
  logger.log('Клиент запущен', 'log-system');

  await cleaner.purify().then(({ success, message }) => success ? logger.log(message, 'log-system') : logger.log(message, 'log-error'));

  await loadConfig();

  await initializePanel();
  await initializeInformationCard();
  await initializeControlContainer();

  await initializeLineGraphicActiveBots();
  await initializeLineGraphicAverageLoad();

  await initializeChatMonitoring();
  await initializeBotsMonitoring();

  await functions.initializeButtonFunctions();
  await functions.initializeCheckboxFunctions();
  await functions.initializeSelectFunctions();

  const loadingContainer = document.getElementById('loading-container') as HTMLElement;

  if (loadingContainer) {
    setTimeout(() => {
      loadingContainer.classList.add('hide');
      setTimeout(() => {
        loadingContainer.style.display = 'none';
      }, 590);
    }, 1000);
  }

  initialized = true;

  await checkUpdate();
});