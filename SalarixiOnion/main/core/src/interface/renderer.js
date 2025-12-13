let statistics = {
  isInitialized: false,
  logIndex: 0,
  showSystemLogs: true,
  latestConfig: {},
  nicknamesList: []
};

let activity = {
  botting: false,
  profileDataMonitoring: false,
  chatHistoryMonitoring: false
};

let localhost;

let mainPageContainer;
let settingsPageContainer;
let proxyPageContainer;
let controlSectorsPageContainer;
let controlChatPageContainer;
let controlActionsPageContainer;
let controlMovePageContainer;
let controlImitationPageContainer;
let controlAttackPageContainer;
//let controlInventoryPageContainer;
let controlFlightPageContainer;
let controlSprinterPageContainer;
let controlGhostPageContainer;
let scriptPageContainer;
let graphicPageContainer;
let monitoringPageContainer;
let analysisPageContainer;
let spyPageContainer;
let logPageContainer;
let aboutPageContainer;

let globalContainers = [];

function date(format = 'H:M:S') {
  const date = new Date();

  const hours = date.getHours().toString().padStart(2, '0');
  const minutes = date.getMinutes().toString().padStart(2, '0');
  const seconds = date.getSeconds().toString().padStart(2, '0');

  if (format === 'H:M:S') {
    return `${hours}:${minutes}:${seconds}`;
  } else if (format === 'H:M') {
    return `${hours}:${minutes}`;
  } else if (format === 'M:S') {
    return `${minutes}:${seconds}`;
  } else {
    return `${hours}:${minutes}:${seconds}`;
  }
}

function log(text, type) {
  const logContent = document.getElementById('log-content');

  if (!logContent) return;

  if (statistics.logIndex >= 500) {
    statistics.logIndex = 0;
    logContent.innerHTML = '';
  }

  statistics.logIndex++;

  const container = document.createElement('div');

  container.className = 'log-line';

  if (type === 'log-system') {
    container.className += ' log-line-system';
  }

  text = text
    .replace(/%hcg/g, '<span style="color: #21d618ba;">')
    .replace(/%hcy/g, '<span style="color: #d6d018b6;">')
    .replace(/%hcr/g, '<span style="color: #d61b1893;">')
    .replace(/%sc/g, '</span>');

  container.innerHTML = `
    <div class="log-line-date">${date()}</div>
    <div class="log-line-content ${type}">${text}</div>
  `;

  if (!statistics.showSystemLogs) {
    container.style.display = 'none';
  }

  logContent.appendChild(container);
}

async function sendDataTo(options, data) {
  try {
    let structure;

    if (options.useHeaders) {
      structure = {
        method: options.method,
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(data)
      };
    } else {
      structure = { 
        method: options.method,
        body: JSON.stringify(data)
      };
    }

    const response = await fetch(options.url, structure);

    if (!response.ok) {
      return { success: false, message: 'Server not available', answer: undefined };
    }

    const answer = await response.json();

    if (!answer) {
      return { success: false, message: 'Answer corrupted', answer: undefined };
    }

    return { success: true, message: `Operation is successful`, answer: answer };
  } catch (error) {
    return { success: false, message: error, answer: undefined };
  }
}

async function clear() {
  await fetch(`${localhost}/system/data/clear`, { method: 'POST', headers: { 'Content-Type': 'application/json' } });
}

class Functions {
  async initializeButtonFunctions() {
    try {
      const initButtonsInMainContainer = async () => {
        const startBotsProcessBtn = document.getElementById('start');
        const stopBotsProcessBtn = document.getElementById('stop');
        const setRandomValuesBtn =  document.getElementById('random');
        const cleanInputValuesBtn = document.getElementById('clean');

        startBotsProcessBtn.addEventListener('click', async () => {
          try {
            const address = (document.getElementById('address')).value;
            const version = (document.getElementById('version')).value;
            const quantity = parseInt((document.getElementById('quantity')).value);
            const delay = parseInt((document.getElementById('delay')).value);

            const nickname = (document.getElementById('nickname')).value;
            const password = (document.getElementById('password')).value;
            const distance = String((document.getElementById('distance')).value).toLowerCase();
            const timeout = parseInt((document.getElementById('timeout')).value);
            const skipValidation = ((document.getElementById('skip-validation')).value).toLowerCase();
            const registerCommand = (document.getElementById('register-command')).value;
            const registerTemplate = (document.getElementById('register-template')).value;
            const registerMinDelay = parseInt((document.getElementById('register-min-delay')).value);
            const registerMaxDelay = parseInt((document.getElementById('register-max-delay')).value);
            const loginCommand = (document.getElementById('login-command')).value;
            const loginTemplate = (document.getElementById('login-template')).value;
            const loginMinDelay = parseInt((document.getElementById('login-min-delay')).value);
            const loginMaxDelay = parseInt((document.getElementById('login-max-delay')).value);
            const rejoinQuantity = parseInt((document.getElementById('rejoin-quantity')).value);
            const rejoinDelay = parseInt((document.getElementById('rejoin-delay')).value);
            const dataUpdateFrequency = parseInt(document.getElementById('data-update-frequency').value);
            const chatHistoryLength = parseInt(document.getElementById('chat-history-length').value);

            const proxyList = String((document.getElementById('proxy-list')).value).toLowerCase();

            const useKeepAlive = (document.getElementById('use-keep-alive')).checked;
            const usePhysics = (document.getElementById('use-physics')).checked;
            const useProxy = (document.getElementById('use-proxy')).checked;
            const useAutoRegister = (document.getElementById('use-auto-register')).checked;
            const useAutoRejoin = (document.getElementById('use-auto-rejoin')).checked;
            const useAutoLogin = (document.getElementById('use-auto-login')).checked;
            const useSaveChat = (document.getElementById('use-save-chat')).checked;
            const useSavePlayers = (document.getElementById('use-save-players')).checked;
            const useAiAgent = (document.getElementById('use-ai-agent')).checked;
            const useDataAnalysis = (document.getElementById('use-data-analysis')).checked;
            const useOptimization = (document.getElementById('use-optimization')).checked;
            const useErrorCorrector = (document.getElementById('use-error-corrector')).checked;
            const useExtendedLogs = (document.getElementById('use-extended-logs')).checked;

            log(`Запуск ботов на сервер...`, 'log-info');

            if (activity.botting) {
              log('Запуск невозможен, есть активные боты', 'log-warning'); return;
            }

            monitoring.maxChatHistoryLength = chatHistoryLength;
        
            const response = await fetch(`${localhost}/botting/start`, {
              method: 'POST',
              headers: { 'Content-Type': 'application/json' },
              body: JSON.stringify({
                address: address,
                version: version || 'auto',
                quantity: quantity,
                delay: delay || 1000,
                nickname: nickname,
                password: password,
                distance: distance,
                timeout: timeout || 10000,
                skipValidation: skipValidation,
                registerCommand: registerCommand,
                registerTemplate: registerTemplate,
                registerMinDelay: registerMinDelay || 2000,
                registerMaxDelay: registerMaxDelay || 5000,
                rejoinQuantity: rejoinQuantity,
                rejoinDelay: rejoinDelay || 3000,
                loginCommand: loginCommand,
                loginTemplate: loginTemplate,
                loginMinDelay: loginMinDelay || 2000,
                loginMaxDelay: loginMaxDelay || 3000,
                dataUpdateFrequency: dataUpdateFrequency,
                proxyList: proxyList,
                useKeepAlive: useKeepAlive,
                usePhysics: usePhysics,
                useProxy: useProxy,
                useAutoRegister: useAutoRegister,
                useAutoRejoin: useAutoRejoin,
                useAutoLogin: useAutoLogin,
                useSaveChat: useSaveChat,
                useSavePlayers: useSavePlayers,
                useAiAgent: useAiAgent,
                useDataAnalysis: useDataAnalysis,
                useOptimization: useOptimization,
                useErrorCorrector: useErrorCorrector,
                useExtendedLogs: useExtendedLogs
              })
            });

            if (!response.ok) {
              log('Ошибка сервера, перезапустите клиент', 'log-error'); return;
            }

            activity.botting = true;

            log(`Включение мониторинга...`, 'log-system');

            await monitoring.wait();

            log(`Мониторинг включён`, 'log-system');

            log(`Установка SSE-соединения...`, 'log-system');

            const eventSource = new EventSource(`${localhost}/session/botting`);

            eventSource.onmessage = async (event) => {
              try {
                const data = JSON.parse(event.data);

                log(data.message, `log-${data.type}`);

                if (data.message === 'SSE-соединение закрыто') {
                  eventSource.close();
                }
              } catch (error) {
                log(`Ошибка (start-bots-process): ${error}`, 'log-error');
              }
            }

            eventSource.onerror = async () => {
              log('Ошибка (start-bots-process): SSE-connection was dropped', 'log-error');
              eventSource.close();
            }
          } catch (error) {
            log(`Ошибка (start-bots-process): ${error}`, 'log-error');
          }
        });

        stopBotsProcessBtn.addEventListener('click', async () => {
          try {
            log('Остановка ботов...', 'log-info');

            const response = await fetch(`${localhost}/botting/stop`, {
              method: 'POST',
              headers: { 'Content-Type': 'application/json' }
            });

            if (!response.ok) {
              log('Ошибка сервера, перезапустите клиент', 'log-error'); return;
            }

            const data = await response.json();

            log(data.data.message, `log-${data.type}`);

            log(`Выключение мониторинга...`, 'log-system');

            await monitoring.clear();

            log(`Мониторинг выключен`, 'log-system');

            statistics.nicknamesList = []; 

            activity.botting = false;

            await clear();
          } catch (error) {
            log(`Ошибка (stop-bots-process): ${error}`, 'log-error');
          }
        });

        setRandomValuesBtn.addEventListener('click', async () => {
          try {
            const setRandomValue = (element) => {
              let current = document.getElementById(element); 

              const addresses = [
                'org.mc-complex.com',
                'join.insanitycraft.net',
                'play.MysticMC.co',
                'play.mellowcraft.org',
                'hub.opblocks.com',
                'org.earthmc.net',
                'hub.manacube.com',
                'ms.rainyday.gg',
                'ms.blossomcraft.org',
                'mc.masedworld.net',
                'mc.mineblaze.net',
                'mc.dexland.org',
                'mc.hypemc.pro',
                'mc.hypemc.ru'
              ];

              const versions = [
                '1.8.9',
                '1.12.2',
                '1.12',
                '1.14',
                '1.16.4',
                '1.16.5',
                '1.19',
                '1.20.1',
                '1.20.3',
                '1.21',
                '1.21.1',
                '1.21.3',
                '1.21.6',
                '1.21.5'
              ];

              switch (element) {
                case 'address':
                  const randomAddress = addresses[Math.floor(Math.random() * addresses.length)] + ':25565';
                  current.value = randomAddress; break;
                case 'version':
                  const randomVersion = String(versions[Math.floor(Math.random() * versions.length)]);
                  current.value = randomVersion; break;
                case 'quantity':
                  const randomQuantity = Math.floor(Math.random() * (30 - 5 + 1) + 5);
                  current.valueAsNumber = randomQuantity; break;
                case 'delay':
                  const randomDelay = Math.floor(Math.random() * (7000 - 1000 + 1) + 1000);
                  current.valueAsNumber = randomDelay; break;
              }
            }

            setRandomValue('address');
            setRandomValue('version');
            setRandomValue('quantity');
            setRandomValue('delay');

            log('Установлены случайные значения элементов', 'log-system');
          } catch (error) {
            log('Не удалось установить случайные значения элементов', 'log-error');
          }
        });

        cleanInputValuesBtn.addEventListener('click', async () => {
          try {
            let address = document.getElementById('address');
            let version = document.getElementById('version');
            let quantity = document.getElementById('quantity');
            let delay = document.getElementById('delay');

            address.value = '';
            version.value = '';
            quantity.value = '';
            delay.value = '';

            log('Значения элементов очищены', 'log-system');
          } catch (error) {
            log('Не удалось очистить значения элементов', 'log-error');
          }
        });
      }

      const initButtonsInControlContainer = async () => {
        try {
          const sendMessageBtn = document.getElementById('chat-send-message');
          const startSpammingBtn = document.getElementById('chat-start-spamming');
          const stopSpammingBtn = document.getElementById('chat-stop-spamming');
          const startAnyActionBtn = document.getElementById('start-any-action');
          const stopAnyActionBtn = document.getElementById('stop-any-action');
          const startAnyMove = document.getElementById('start-any-move');
          const stopAnyMove = document.getElementById('stop-any-move');
          const startAnyImitationBtn = document.getElementById('start-any-imitation');
          const stopAnyImitationBtn = document.getElementById('stop-any-imitation');
          const startAnyAttackBtn = document.getElementById('start-any-attack');
          const stopAnyAttackBtn = document.getElementById('stop-any-attack');
          const startFlightBtn = document.getElementById('start-flight');
          const stopFlightBtn = document.getElementById('stop-flight');
          const startSprinterBtn = document.getElementById('start-sprinter');
          const stopSprinterBtn = document.getElementById('stop-sprinter');
          const startGhostBtn = document.getElementById('start-ghost');
          const stopGhostBtn = document.getElementById('stop-ghost');

          sendMessageBtn.addEventListener('click', async () => {
            if (!activity.botting) {
              log('Активных ботов не существует, действие невозможно', 'log-warning'); return;
            }

            try {
              const from = (document.getElementById('chat-from')).value;
              const message = (document.getElementById('chat-message')).value;

              const useMagicText = (document.getElementById('use-chat-magic-text')).checked;
              const useTextMutation = (document.getElementById('use-chat-text-mutation')).checked;
              const useSync = (document.getElementById('use-chat-sync')).checked;

              const response = await fetch(`${localhost}/control/chat`, {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({
                  type: 'message',
                  options: {
                    from: from,
                    message: message,
                    useMagicText: useMagicText,
                    useTextMutation: useTextMutation,
                    useSync: useSync
                  }
                })
              });

              const data = await response.json();

              log(data.data.message, `log-${data.type}`);
            } catch (error) {
              log(`Ошибка отправки сообщения: ${error}`, 'log-error');
            }
          });

          startSpammingBtn.addEventListener('click', async () => {
            if (!activity.botting) {
              log('Активных ботов не существует, действие невозможно', 'log-warning'); return;
            }

            try {
              const from = (document.getElementById('chat-from')).value;
              const message = (document.getElementById('chat-message')).value;
              const minDelay = Number((document.getElementById('chat-spamming-min-delay')).value) | 2000;
              const maxDelay = Number((document.getElementById('chat-spamming-max-delay')).value) | 4000;

              const useMagicText = (document.getElementById('use-chat-magic-text')).checked;
              const useTextMutation = (document.getElementById('use-chat-text-mutation')).checked;
              const useSync = (document.getElementById('use-chat-sync')).checked;
              const useAntiRepetition = (document.getElementById('use-chat-spamming-anti-repetition')).checked;
              const useBypass = (document.getElementById('use-chat-spamming-bypass')).checked;

              const response = await fetch(`${localhost}/control/chat`, {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({
                  type: 'spamming',
                  options: {
                    state: 'start',
                    from: from,
                    message: message,
                    minDelay: minDelay,
                    maxDelay: maxDelay,
                    useMagicText: useMagicText,
                    useTextMutation: useTextMutation,
                    useSync: useSync,
                    useAntiRepetition: useAntiRepetition,
                    useBypass: useBypass
                  }
                })
              });

              const data = await response.json();

              log(data.data.message, `log-${data.type}`);
            } catch (error) {
              log(`Ошибка старта «Спамминг»: ${error}`, 'log-error');
            }
          });

          stopSpammingBtn.addEventListener('click', async () => {
            if (!activity.botting) {
              log('Активных ботов не существует, действие невозможно', 'log-warning'); return;
            }

            try {
              const response = await fetch(`${localhost}/control/chat`, {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({
                  type: 'spamming',
                  options: {
                    state: 'stop'
                  }
                })
              });

              const data = await response.json();

              log(data.data.message, `log-${data.type}`);
            } catch (error) {
              log(`Ошибка остановки «Спамминг»: ${error}`, 'log-error');
            }
          });

          startAnyActionBtn.addEventListener('click', async () => {
            if (!activity.botting) {
              log('Активных ботов не существует, действие невозможно', 'log-warning'); return;
            }

            const action = (document.getElementById('select-action')).value;

            let actionName = '';

            if (action === 'jumping') {
              actionName = 'Джампинг';
            } else if (action === 'shifting') {
              actionName = 'Шифтинг';
            } else if (action === 'waving') {
              actionName = 'Махание рукой';
            } else if (action === 'looking') {
              actionName = 'Осмотр';
            } else if (action === 'spinning') {
              actionName = 'Спиннинг';
            }

            try {
              const useSync = (document.getElementById('use-action-sync')).checked;
              const useAntiDetect = (document.getElementById('use-action-anti-detect')).checked;
              const useImpulsiveness = (document.getElementById('use-action-impulsiveness')).checked;
              const useRandomizer = (document.getElementById('use-action-randomizer')).checked;
              const useRealism = (document.getElementById('use-action-realism')).checked;

              const minDelay = parseInt((document.getElementById('action-min-delay')).value);
              const maxDelay = parseInt((document.getElementById('action-max-delay')).value);

              let options;

              switch (action) {
                case 'jumping':
                  options = {
                    state: 'start',
                    useSync: useSync,
                    useAntiDetect: useAntiDetect,
                    useImpulsiveness: useImpulsiveness,
                    minDelay: minDelay,
                    maxDelay: maxDelay
                  }; break;
                case 'shifting':
                  options = {
                    state: 'start',
                    useSync: useSync,
                    useAntiDetect: useAntiDetect,
                    useImpulsiveness: useImpulsiveness,
                    minDelay: minDelay,
                    maxDelay: maxDelay
                  }; break;
                case 'waving':
                  options = {
                    state: 'start',
                    useSync: useSync,
                    useAntiDetect: useAntiDetect,
                    useRandomizer: useRandomizer
                  }; break;
                case 'spinning':
                  options = {
                    state: 'start',
                    useSync: useSync,
                    useAntiDetect: useAntiDetect,
                    useRealism: useRealism
                  }; break;
              }

              const response = await fetch(`${localhost}/control/action`, {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({
                  type: action,
                  options: options
                })
              });

              const data = await response.json();

              log(data.data.message, `log-${data.type}`);
            } catch (error) {
              log(`Ошибка старта «${actionName}»: ${error}`, 'log-error');
            }
          });

          stopAnyActionBtn.addEventListener('click', async () => {
            if (!activity.botting) {
              log('Активных ботов не существует, действие невозможно', 'log-warning'); return;
            }

            const action = (document.getElementById('select-action')).value;

            let actionName = '';

            if (action === 'jumping') {
              actionName = 'Джампинг';
            } else if (action === 'shifting') {
              actionName = 'Шифтинг';
            } else if (action === 'waving') {
              actionName = 'Махание рукой';
            } else if (action === 'looking') {
              actionName = 'Осмотр';
            } else if (action === 'spinning') {
              actionName = 'Спиннинг';
            }

            try {
              const response = await fetch(`${localhost}/control/action`, {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({
                  type: action,
                  options: {
                    state: 'stop'
                  }
                })
              });

              const data = await response.json();

              log(data.data.message, `log-${data.type}`);
            } catch (error) {
              log(`Ошибка остановки «${actionName}»: ${error}`, 'log-error');
            }
          });

          startAnyMove.addEventListener('click', async () => {
            if (!activity.botting) {
              log('Активных ботов не существует, действие невозможно', 'log-warning'); return;
            }

            const direction = (document.getElementById('select-move-direction')).value;

            let directionName = '';

            if (direction === 'forward') {
              directionName = 'Движение вперёд';
            } else if (direction === 'back') {
              directionName = 'Движение назад';
            } else if (direction === 'left') {
              directionName = 'Движение влево';
            } else if (direction === 'right') {
              directionName = 'Движение вправо';
            }

            try {
              const useSync = (document.getElementById('use-move-sync')).checked;
              const useAntiDetect = (document.getElementById('use-move-anti-detect')).checked;
              const useImpulsiveness = (document.getElementById('use-move-impulsiveness')).checked;

              const response = await fetch(`${localhost}/control/move`, {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({
                  options: {
                    state: 'start',
                    direction: direction,
                    useSync: useSync,
                    useAntiDetect: useAntiDetect,
                    useImpulsiveness: useImpulsiveness
                  }
                })
              });

              const data = await response.json();

              log(data.data.message, `log-${data.type}`);
            } catch (error) {
              log(`Ошибка остановки «${directionName}»: ${error}`, 'log-error');
            }
          });

          stopAnyMove.addEventListener('click', async () => {
            if (!activity.botting) {
              log('Активных ботов не существует, действие невозможно', 'log-warning'); return;
            }

            const direction = (document.getElementById('select-move-direction')).value;

            let directionName = '';

            if (direction === 'forward') {
              directionName = 'Движение вперёд';
            } else if (direction === 'back') {
              directionName = 'Движение назад';
            } else if (direction === 'left') {
              directionName = 'Движение влево';
            } else if (direction === 'right') {
              directionName = 'Движение вправо';
            }

            try {
              const response = await fetch(`${localhost}/control/move`, {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({
                  options: {
                    state: 'stop',
                    direction: direction
                  }
                })
              });

              const data = await response.json();

              log(data.data.message, `log-${data.type}`);
            } catch (error) {
              log(`Ошибка остановки «${directionName}»: ${error}`, 'log-error');
            }
          });

          startAnyImitationBtn.addEventListener('click', async () => {
            if (!activity.botting) {
              log('Активных ботов не существует, действие невозможно', 'log-warning'); return;
            }

            const imitation = (document.getElementById('select-imitation')).value;

            let imitationName = '';

            if (imitation === 'walking') {
              imitationName = 'Имитация хотьбы';
            } else if (imitation === 'chating') {
              imitationName = 'Имитация чата';
            } else if (imitation === 'looking') {
              imitationName = 'Имитация осмотра';
            } else if (imitation === 'hybrid') {
              imitationName = 'Гибридная имитация';
            }

            try {
              const useSmoothness = (document.getElementById('use-imitation-smoothness')).checked;
              const useLongDelays = (document.getElementById('use-imitation-long-delays')).checked;
              const useLooking = (document.getElementById('use-imitation-looking')).checked;
              const useWaving = (document.getElementById('use-imitation-waving')).checked;
              const useMultitasking = (document.getElementById('use-imitation-multitasking')).checked;
              const useSmartRoutes = (document.getElementById('use-imitation-smart-routes')).checked;
              const useSprint = (document.getElementById('use-imitation-sprint')).checked;

              let options = {};

              if (imitation === 'hybrid') {
                options = {
                  state: 'start',
                  useSmoothness: useSmoothness,
                  useLongDelays: useLongDelays,
                  useLooking: useLooking,
                  useWaving: useWaving,
                  useMultitasking: useMultitasking
                }
              } else if (imitation === 'walking') {
                options = {
                  state: 'start',
                  useSmoothness: useSmoothness,
                  useLongDelays: useLongDelays,
                  useSmartRoutes: useSmartRoutes,
                  useSprint: useSprint,
                  useMultitasking: useMultitasking
                }
              }

              const response = await fetch(`${localhost}/control/imitation`, {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({
                  type: imitation,
                  options: options
                })
              });

              const data = await response.json();

              log(data.data.message, `log-${data.type}`);
            } catch (error) {
              log(`Ошибка старта «${imitationName}»: ${error}`, 'log-error');
            }
          });

          stopAnyImitationBtn.addEventListener('click', async () => {
            if (!activity.botting) {
              log('Активных ботов не существует, действие невозможно', 'log-warning'); return;
            }

            const imitation = (document.getElementById('select-imitation')).value;

            let imitationName = '';

            if (imitation === 'walking') {
              imitationName = 'Имитация хотьбы';
            } else if (imitation === 'chating') {
              imitationName = 'Имитация чата';
            } else if (imitation === 'looking') {
              imitationName = 'Имитация осмотра';
            } else if (imitation === 'hybrid') {
              imitationName = 'Гибридная имитация';
            }

            try {
              const response = await fetch(`${localhost}/control/imitation`, {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({
                  type: imitation,
                  options: {
                    state: 'stop'
                  }
                })
              });

              const data = await response.json();

              log(data.data.message, `log-${data.type}`);
            } catch (error) {
              log(`Ошибка остановки «${imitationName}»: ${error}`, 'log-error');
            }
          });

          startAnyAttackBtn.addEventListener('click', async () => {
            if (!activity.botting) {
              log('Активных ботов не существует, действие невозможно', 'log-warning'); return;
            }

            try {
              const target = document.getElementById('select-attack-target').value;

              const distance = parseFloat(document.getElementById('attack-distance').value);
              const minDelay = Number(document.getElementById('attack-min-delay').value);
              const maxDelay = Number(document.getElementById('attack-max-delay').value);

              const useLongDelays = document.getElementById('use-attack-long-delays').checked;
              const useNeatness = document.getElementById('use-attack-neatness').checked;
              const useAntiDetect = document.getElementById('use-attack-anti-detect').checked;
              const useImprovedStrikes = document.getElementById('use-attack-improved-strikes').checked;
              const useImitationOfMisses = document.getElementById('use-attack-imitation-of-misses').checked;
              const useDodging = document.getElementById('use-attack-dodging').checked;

              const response = await fetch(`${localhost}/control/attack`, {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({
                  options: {
                    state: 'start',
                    target: target,
                    distance: distance,
                    minDelay: minDelay,
                    maxDelay: maxDelay,
                    useLongDelays: useLongDelays,
                    useNeatness: useNeatness,
                    useAntiDetect: useAntiDetect,
                    useImprovedStrikes: useImprovedStrikes,
                    useImitationOfMisses: useImitationOfMisses,
                    useDodging: useDodging
                  }
                })
              });

              const data = await response.json();

              log(data.data.message, `log-${data.type}`);
            } catch (error) {
              log(`Ошибка старта «Атака»: ${error}`, 'log-error');
            }
          });

          stopAnyAttackBtn.addEventListener('click', async () => {
            if (!activity.botting) {
              log('Активных ботов не существует, действие невозможно', 'log-warning'); return;
            }

            try {
              const response = await fetch(`${localhost}/control/attack`, {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({
                  options: {
                    state: 'stop'
                  }
                })
              });

              const data = await response.json();

              log(data.data.message, `log-${data.type}`);
            } catch (error) {
              log(`Ошибка остановки «Атака»: ${error}`, 'log-error');
            }
          });

          startFlightBtn.addEventListener('click', async () => {
            if (!activity.botting) {
              log('Активных ботов не существует, действие невозможно', 'log-warning'); return;
            }

            try {
              const type = document.getElementById('select-flight-type').value;

              const useAntiKick = document.getElementById('use-flight-anti-kick').checked;
              const useSpoofing = document.getElementById('use-flight-spoofing').checked;
              const useHighPower = document.getElementById('use-flight-high-power').checked;
              const useHovering = document.getElementById('use-flight-hovering').checked;

              const response = await fetch(`${localhost}/control/flight`, {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({
                  type: type,
                  options: {
                    state: 'start',
                    useAntiKick: useAntiKick,
                    useSpoofing: useSpoofing,
                    useHighPower: useHighPower,
                    useHovering: useHovering
                  }
                })
              });

              const data = await response.json();

              log(data.data.message, `log-${data.type}`);
            } catch (error) {
              log(`Ошибка старта «Полёт»: ${error}`, 'log-error');
            }
          });

          stopFlightBtn.addEventListener('click', async () => {
            if (!activity.botting) {
              log('Активных ботов не существует, действие невозможно', 'log-warning'); return;
            }

            try {
              const type = (document.getElementById('select-flight-type')).value;

              const response = await fetch(`${localhost}/control/flight`, {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({
                  type: type,
                  options: {
                    state: 'stop'
                  }
                })
              });

              const data = await response.json();

              log(data.data.message, `log-${data.type}`);
            } catch (error) {
              log(`Ошибка остановки «Полёт»: ${error}`, 'log-error');
            }
          });

          startSprinterBtn.addEventListener('click', async () => {
            if (!activity.botting) {
              log('Активных ботов не существует, действие невозможно', 'log-warning'); return;
            }

            try {
              const type = document.getElementById('select-sprinter-type').value;

              const useSpoofing = document.getElementById('use-sprinter-spoofing').checked;

              const response = await fetch(`${localhost}/control/sprinter`, {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({
                  type: type,
                  options: {
                    state: 'start',
                    useSpoofing: useSpoofing
                  }
                })
              });

              const data = await response.json();

              log(data.data.message, `log-${data.type}`);
            } catch (error) {
              log(`Ошибка старта «Спринтер»: ${error}`, 'log-error');
            }
          });

          stopSprinterBtn.addEventListener('click', async () => {
            if (!activity.botting) {
              log('Активных ботов не существует, действие невозможно', 'log-warning'); return;
            }

            try {
              const type = document.getElementById('select-sprinter-type').value;

              const response = await fetch(`${localhost}/control/sprinter`, {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({
                  type: type,
                  options: {
                    state: 'stop'
                  }
                })
              });

              const data = await response.json();

              log(data.data.message, `log-${data.type}`);
            } catch (error) {
              log(`Ошибка остановки «Спринтер»: ${error}`, 'log-error');
            }
          });

          startGhostBtn.addEventListener('click', async () => {
            if (!activity.botting) {
              log('Активных ботов не существует, действие невозможно', 'log-warning'); return;
            }

            try {
              const type = document.getElementById('select-ghost-mode').value;
              const useSharpness = document.getElementById('use-ghost-sharpness').checked;
              const useBuffering = document.getElementById('use-ghost-buffering').checked;

              const response = await fetch(`${localhost}/control/ghost`, {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({
                  type: type,
                  options: {
                    state: 'start',
                    useSharpness: useSharpness,
                    useBuffering: useBuffering
                  }
                })
              });

              const data = await response.json();

              log(data.data.message, `log-${data.type}`);
            } catch (error) {
              log(`Ошибка старта «Призрак»: ${error}`, 'log-error');
            }
          });

          stopGhostBtn.addEventListener('click', async () => {
            if (!activity.botting) {
              log('Активных ботов не существует, действие невозможно', 'log-warning'); return;
            }

            try {
              const type = document.getElementById('select-ghost-mode').value;
              
              const response = await fetch(`${localhost}/control/ghost`, {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({
                  type: type,
                  options: {
                    state: 'stop'
                  }
                })
              });

              const data = await response.json();

              log(data.data.message, `log-${data.type}`);
            } catch (error) {
              log(`Ошибка остановки «Призрак»: ${error}`, 'log-error');
            }
          });
        } catch (error) {
          log(`Ошибка операции initButtonsInControlContainer: ${error}`, 'log-error');
        }
      }

      const initButtonsInScriptContainer = async () => {
        try {
          const executeScriptBtn = document.getElementById('execute-script');
          const stopScriptBtn = document.getElementById('stop-script');

          executeScriptBtn.addEventListener('click', async () => {
            try {
              const script = document.getElementById('script').value;

              const response = await fetch(`${localhost}/script/execute`, {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({
                  script: script
                })
              });

              const data = await response.json();

              log(data.data.message, `log-${data.type}`);
            } catch (error) {
              log(`Ошибка выполнения скрипта: ${error}`, 'log-error');
            }
          });

          stopScriptBtn.addEventListener('click', async () => {
            try {
              const response = await fetch(`${localhost}/script/stop`, {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' }
              });

              const data = await response.json();

              log(data.data.message, `log-${data.type}`);
            } catch (error) {
              log(`Ошибка остановки скрипта: ${error}`, 'log-error');
            }
          });
        } catch (error) {
          log(`Ошибка операции initButtonsInScriptContainer: ${error}`, 'log-error');
        }
      }

      await initButtonsInMainContainer();
      await initButtonsInControlContainer();
      await initButtonsInScriptContainer();
    } catch (error) {
      log(`Ошибка операции initializeButtonFunctions: ${error}`, 'log-error');
    }
  }

  async initializeCheckboxFunctions() {
    try {
      const initCheckboxesInSettingsContainer = async () => {
        try {
          const autoRegisterChbx = document.getElementById('use-auto-register');
          const autoRejoinChbx = document.getElementById('use-auto-rejoin');
          const autoLoginChbx = document.getElementById('use-auto-login');

          const autoRegisterInputContainer = document.getElementById('auto-register-input-container');
          const autoRejoinInputContainer = document.getElementById('auto-rejoin-input-container');
          const autoLoginInputContainer = document.getElementById('auto-login-input-container');

          const checkRegisterChbx = () => {
            if (autoRegisterChbx.checked) {
              autoRegisterInputContainer.style.display = 'block';
            } else {
              autoRegisterInputContainer.style.display = 'none';
            }
          }

          const checkRejoinChbx = () => {
            if (autoRejoinChbx.checked) {
              autoRejoinInputContainer.style.display = 'block';
            } else {
              autoRejoinInputContainer.style.display = 'none';
            }
          }

          const checkLoginChbx = () => {
            if (autoLoginChbx.checked) {
              autoLoginInputContainer.style.display = 'block';
            } else {
              autoLoginInputContainer.style.display = 'none';
            }
          }

          checkRegisterChbx();
          checkRejoinChbx();
          checkLoginChbx();

          autoRegisterChbx.addEventListener('change', () => checkRegisterChbx());
          autoRejoinChbx.addEventListener('change', () => checkRejoinChbx());
          autoLoginChbx.addEventListener('change', () => checkLoginChbx());
        } catch (error) {
          log(`Ошибка операции initCheckboxesInSettingsContainer: ${error}`, 'log-error');
        }
      }

      const initCheckboxesInControlContainer = async () => {
        try {
          const chatSpammingChbx = document.getElementById('use-chat-spamming');
          const actionImpulsivenessChbx = document.getElementById('use-action-impulsiveness');
          const inventoryDelayChbx = document.getElementById('use-inventory-delay');

          chatSpammingChbx.addEventListener('change', async () => {
            const chatSpammingChbxContainer = document.getElementById('chat-spamming-chbx-container');
            const chatSpammingInputContainer = document.getElementById('chat-spamming-input-container');
            
            const chatDefaultBtnsContainer = document.getElementById('chat-default-btns-container');
            const chatSpammingBtnsContainer = document.getElementById('chat-spamming-btns-container');

            if (chatSpammingChbx.checked) {
              chatSpammingChbxContainer.style.display = 'block';
              chatSpammingInputContainer.style.display = 'block';
              chatSpammingBtnsContainer.style.display = 'flex';

              chatDefaultBtnsContainer.style.display = 'none';
            } else {
              chatDefaultBtnsContainer.style.display = 'flex';

              chatSpammingChbxContainer.style.display = 'none';
              chatSpammingInputContainer.style.display = 'none';
              chatSpammingBtnsContainer.style.display = 'none';
            }
          });

          actionImpulsivenessChbx.addEventListener('change', async () => {
            const actionDelayInputContainer = document.getElementById('action-jumping-and-shifting-input-container');

            if (actionImpulsivenessChbx.checked) {
              actionDelayInputContainer.style.display = 'block';
            } else {
              actionDelayInputContainer.style.display = 'none';
            }
          });

          inventoryDelayChbx.addEventListener('change', async () => {
            const inventoryDelayContainer = document.getElementById('inventory-delay-container');

            if (inventoryDelayChbx.checked) {
              inventoryDelayContainer.style.display = 'block';
            } else {
              inventoryDelayContainer.style.display = 'none';
            }
          });
        } catch (error) {
          log(`Ошибка операции initCheckboxesInControlContainer: ${error}`, 'log-error');
        }
      }

      const initCheckboxesInLogContainer = async () => {
        try {
          const showSystemLogChbx = document.getElementById('show-system-log');

          showSystemLogChbx.addEventListener('change', () => {
            const systemLogs = document.querySelectorAll('.log-line-system');

            if (showSystemLogChbx.checked) {
              statistics.showSystemLogs = true;
              systemLogs.forEach(element => {
                element.style.display = 'flex';
              });
            } else {
              statistics.showSystemLogs = false;
              systemLogs.forEach(element => {
                element.style.display = 'none';
              });
            }
          });
        } catch (error) {
          log(`Ошибка операции initCheckboxesInLogContainer: ${error}`, 'log-error');
        }
      }

      await initCheckboxesInSettingsContainer();
      await initCheckboxesInControlContainer();
      await initCheckboxesInLogContainer();
    } catch (error) {
      log(`Ошибка операции initializeCheckboxFunctions: ${error}`, 'log-error');
    }
  }

  async initializeSelectFunctions() {
    try {
      const initSelectControlContainer = async () => {
        try {
          const actionSelect = document.getElementById('select-action');
          const imitationSelect = document.getElementById('select-imitation');
          const inventoryTypeSelect = document.getElementById('invenotry-type');

          actionSelect.addEventListener('change', () => {
            const actionJumpingAndShiftingInputContainer = document.getElementById('action-jumping-and-shifting-input-container');
            const actionJumpingAndShiftingChbxContainer = document.getElementById('action-jumping-and-shifting-chbx-container');
            const actionWavingChbxContainer = document.getElementById('action-waving-chbx-container');
            const actionSpinningChbxContainer = document.getElementById('action-spinning-chbx-container');

            if (actionSelect.value === 'jumping' || actionSelect.value === 'shifting') {
              actionJumpingAndShiftingInputContainer.style.display = 'block';
              actionJumpingAndShiftingChbxContainer.style.display = 'block';
              actionWavingChbxContainer.style.display = 'none';
              actionSpinningChbxContainer.style.display = 'none';
            } else if (actionSelect.value === 'waving') {
              actionJumpingAndShiftingInputContainer.style.display = 'none';
              actionJumpingAndShiftingChbxContainer.style.display = 'none';
              actionWavingChbxContainer.style.display = 'block';
              actionSpinningChbxContainer.style.display = 'none';
            } else if (actionSelect.value === 'looking') {
              actionJumpingAndShiftingInputContainer.style.display = 'none';
              actionJumpingAndShiftingChbxContainer.style.display = 'none';
              actionWavingChbxContainer.style.display = 'none';
              actionSpinningChbxContainer.style.display = 'none';
            } else if (actionSelect.value === 'spinning') {
              actionJumpingAndShiftingInputContainer.style.display = 'none';
              actionJumpingAndShiftingChbxContainer.style.display = 'none';
              actionWavingChbxContainer.style.display = 'none';
              actionSpinningChbxContainer.style.display = 'block';
            }
          });

          imitationSelect.addEventListener('change', () => {
            const hybridImitationChbxContainer = document.getElementById('imitation-hybrid-chbx-container');
            const walkingImitationChbxContainer = document.getElementById('imitation-walking-chbx-container');
            const lookingImitationChbxContainer = document.getElementById('imitation-looking-chbx-container');

            if (imitationSelect.value === 'hybrid') {
              hybridImitationChbxContainer.style.display = 'block';
              walkingImitationChbxContainer.style.display = 'none';
              lookingImitationChbxContainer.style.display = 'none';
            } else if (imitationSelect.value === 'walking') {
              hybridImitationChbxContainer.style.display = 'none';
              walkingImitationChbxContainer.style.display = 'block';
              lookingImitationChbxContainer.style.display = 'none';
            } else if (imitationSelect.value === 'looking') {
              hybridImitationChbxContainer.style.display = 'none';
              walkingImitationChbxContainer.style.display = 'none';
              lookingImitationChbxContainer.style.display = 'block';
            }
          });

          inventoryTypeSelect.addEventListener('change', () => {
            const inventoryHotbarContainer = document.getElementById('inventory-hotbar-container');
            const inventoryOtherContainer = document.getElementById('inventory-other-container');

            if (inventoryTypeSelect.value === 'hotbar') {
              inventoryHotbarContainer.style.display = 'block';
              inventoryOtherContainer.style.display = 'none';
            } else if (inventoryTypeSelect.value === 'other') {
              inventoryHotbarContainer.style.display = 'none';
              inventoryOtherContainer.style.display = 'block';
            }
          });
        } catch (error) {
          log(`Ошибка операции initSelectControlContainer: ${error}`, 'log-error');
        }
      }

      await initSelectControlContainer();
    } catch (error) {
      log(`Ошибка операции initializeSelectFunctions: ${error}`, 'log-error');
    }
  }
}

class GraphicManager {
  chartActiveBots = undefined;
  chartAverageLoad = undefined;
  
  async createGraphicActiveBots() {
    const context = document.getElementById('line-graphic-active-bots').getContext('2d');

    if (!context) return;

    const initialLabels = [];
    const initialDataActive = [];
    
    for (let i = 0; i < 31; i++) {
      initialLabels.push(date());
      initialDataActive.push(0);
    }

    this.chartActiveBots = new Chart(context, {
      type: 'line',
      data: {
        labels: initialLabels,
        datasets: [
          {
            label: ' Активные боты',
            data: initialDataActive,
            pointStyle: false,
            fill: true,
            borderWidth: 2,
            borderColor: '#3bffd5ff',
            backgroundColor: '#3bffd521',
            tension: 0.1
          }
        ]
      },
      options: {
        responsive: true,
        animation: { duration: 400 },
        scales: {
          x: {
            ticks: { color: '#858585ff' },
            border: { color: '#383838ab' },
            grid: { color: '#383838ab' }
          },
          y: {
            min: 0,
            max: 500, 
            ticks: { color: '#a3a3a3ff' },
            border: { color: '#383838ab' },
            grid: { color: '#383838ab' }
          }
        },
        plugins: {
          title: {
            text: 'График активных ботов',
            display: true,
            color: '#a2a2a2ff'
          },
          legend: {
            display: false,
            position: 'top',
            labels: {
              color: '#979797ff',
              font: { size: 12 },
              usePointStyle: true
            }
          },
          tooltip: {
            mode: 'index',
            intersect: false,
            backgroundColor: 'rgba(10, 10, 10, 0.8)',
            titleColor: '#ffffff',
            bodyColor: '#ffffff'
          }
        }
      }
    });
  }

  async createGraphicAverageLoad() {
    const context = document.getElementById('line-graphic-average-load').getContext('2d');

    if (!context) return;

    const initialLabels = [];
    const initialDataLoad = [];
    
    for (let i = 0; i < 31; i++) {
      initialLabels.push(date());
      initialDataLoad.push(0);
    }

    this.chartAverageLoad = new Chart(context, {
      type: 'line',
      data: {
        labels: initialLabels,
        datasets: [
          {
            label: ' Средняя нагрузка',
            data: initialDataLoad,
            pointStyle: false,
            fill: true,
            borderWidth: 2,
            borderColor: '#3bffd5ff',
            backgroundColor: '#3bffd521',
            tension: 0.1
          }
        ]
      },
      options: {
        responsive: true,
        animation: { duration: 400 },
        scales: {
          x: {
            ticks: { color: '#858585ff' },
            border: { color: '#383838ab' },
            grid: { color: '#383838ab' }
          },
          y: {
            min: 0,
            max: 100, 
            ticks: {
              callback: (value) => { return value + '%' },
              color: '#a3a3a3ff',   
              font: { size: 12 },   
              maxRotation: 0,
              minRotation: 0
            },
            border: { color: '#383838ab' },
            grid: { color: '#383838ab' }
          }
        },
        plugins: {
          title: {
            text: 'График средней нагрузки ботов',
            display: true,
            color: '#a2a2a2ff'
          },
          legend: {
            display: false,
            position: 'top',
            labels: {
              color: '#979797ff',
              font: { size: 12 },
              usePointStyle: true
            }
          },
          tooltip: {
            mode: 'index',
            intersect: false,
            backgroundColor: 'rgba(10, 10, 10, 0.8)',
            titleColor: '#ffffff',
            bodyColor: '#ffffff'
          }
        }
      }
    });
  }

  async addGraphicDataActiveBots(activeBotsQuantity) {
    this.chartActiveBots.data.labels?.push(date());
    this.chartActiveBots.data.datasets[0].data.push(activeBotsQuantity);

    if (this.chartActiveBots.data.labels && this.chartActiveBots.data.labels.length > 31) {
      this.chartActiveBots.data.labels.shift();
      this.chartActiveBots.data.datasets[0].data.shift();
    }
      
    this.chartActiveBots.update(); 
  }

  async addGraphicDataAverageLoad(averageLoad) {
    this.chartAverageLoad.data.labels?.push(date());
    this.chartAverageLoad.data.datasets[0].data.push(averageLoad);

    if (this.chartAverageLoad.data.labels && this.chartAverageLoad.data.labels.length > 31) {
      this.chartAverageLoad.data.labels.shift();
      this.chartAverageLoad.data.datasets[0].data.shift();
    }
      
    this.chartAverageLoad.update(); 
  }
}

class MonitoringManager {
  statusText = null;
  botCardsContainer = null;

  maxChatHistoryLength = null;
  chatMessageCounter = {};
  chatHistoryFilters = {};

  async init() {
    this.statusText = document.getElementById('monitoring-status-text');
    this.botCardsContainer = document.getElementById('bot-cards-container');

    this.statusText.innerText = 'Объекты ботов отсутствуют';
    this.statusText.style.color = '#646464f7';
    this.statusText.style.display = 'block';
  }

  async wait() {
    this.statusText.innerText = 'Ожидание активных ботов...';
    this.statusText.style.color = '#646464f7';

    this.botCardsContainer.innerHTML = '';
    this.botCardsContainer.style.display = 'grid';
  }

  async clear() {
    const cards = document.querySelectorAll('bot-card');

    this.chatMessageCounter = {};
    this.chatHistoryFilters = {};

    cards.forEach(card => card.remove());

    this.statusText.innerText = 'Объекты ботов отсутствуют';
    this.statusText.style.color = '#646464f7';
    this.statusText.style.display = 'block';

    this.botCardsContainer.innerHTML = '';
    this.botCardsContainer.style.display = 'none';
  }

  initializeBotCard(nickname) {
    const openChatBtn = document.getElementById(`open-chat-${nickname}`);
    const recreateBotBtn = document.getElementById(`recreate-${nickname}`);

    const filterChatBtn = document.getElementById(`filter-chat-${nickname}`);
    const clearChatBtn = document.getElementById(`clear-chat-${nickname}`);
    const closeChatBtn = document.getElementById(`close-chat-${nickname}`);

    openChatBtn.addEventListener('click', () => {
      const chat = document.getElementById(`chat-${nickname}`);
      chat.style.display = 'flex';
    });

    recreateBotBtn.addEventListener('click', async () => {
      try {
        const operation = await sendDataTo({ url: `${localhost}/advanced/recreate`, method: 'POST', useHeaders: true }, { 
          nickname: nickname
        });

        log(operation.answer.data.message, `log-${operation.answer.type}`);
      } catch (error) {
        log(`Ошибка пересоздания бота ${nickname}: ${error}`, 'log-error');
      }
    });

    filterChatBtn.addEventListener('click', () => {
      try {
        const chat = document.getElementById(`monitoring-chat-content-${nickname}`);
        const type = document.getElementById(`select-chat-filter-${nickname}`);

        const messages = document.querySelectorAll(`#monitoring-message-${nickname}`);

        switch (type.value) {
          case 'all':
            chat.innerHTML = '';

            this.chatHistoryFilters[nickname] = 'all';

            messages.forEach(msg => chat.appendChild(msg)); break;
          case 'bans':
            chat.innerHTML = '';

            this.chatHistoryFilters[nickname] = 'bans';

            messages.forEach(async msg => {
              if (this.filterMessage('bans', msg.textContent)) {
                chat.appendChild(msg);
              }
            }); break;
          case 'mentions':
            chat.innerHTML = '';

            this.chatHistoryFilters[nickname] = 'mentions';

            messages.forEach(async msg => {
              if (this.filterMessage('mentions', msg.textContent)) {
                chat.appendChild(msg);
              }
            }); break;
          case 'links':
            chat.innerHTML = '';

            this.chatHistoryFilters[nickname] = 'links';

            messages.forEach(async msg => {
              if (this.filterMessage('links', msg.textContent)) {
                chat.appendChild(msg);
              }
            }); break;
        }
      } catch (error) {
        log(`Ошибка фильтровки чата: ${error}`, 'log-error');
      }
    });

    clearChatBtn.addEventListener('click', () => {
      const messages = document.querySelectorAll(`#monitoring-message-${nickname}`);
      messages.forEach(msg => msg.remove());
      this.chatMessageCounter[nickname] = 0;
    });

    closeChatBtn.addEventListener('click', () => {
      const chat = document.getElementById(`chat-${nickname}`);
      chat.style.display = 'none';
    });
  }

  async profileDataMonitoring() {
    try {
      const steveIconPath = document.getElementById('steve-img');

      const eventSource = new EventSource(`${localhost}/session/monitoring/profile-data`);

      eventSource.onmessage = async (event) => {
        try {
          if (!activity.botting) return;

          const data = JSON.parse(event.data);

          const { 
            nickname, status, statusColor, 
            version, password, proxyType, 
            proxy, scriptActivity, load, 
            loadColor, ping, pingColor
          } = data;

          if (statistics.nicknamesList.length === 0) {
            this.statusText.style.display = 'none';
          }

          if (statistics.nicknamesList.includes(nickname)) {
            document.getElementById(`bot-status-${nickname}`).innerHTML = `<span style="color: ${statusColor};">• ${status}</span>`;
            document.getElementById(`bot-version-${nickname}`).innerHTML = `  ${version}`;
            document.getElementById(`bot-password-${nickname}`).innerHTML = `  ${password}`;
            document.getElementById(`bot-proxy-${nickname}`).innerHTML = `  ${proxy}`;
            document.getElementById(`bot-proxy-type-${nickname}`).innerHTML = `  ${proxyType}`;
            document.getElementById(`bot-script-activity-${nickname}`).innerHTML = `  ${scriptActivity ? 'Активен' : 'Неактивен'}`;
            document.getElementById(`bot-load-${nickname}`).innerHTML = `<span style="color: ${loadColor};">  ${load}</span>`;
            document.getElementById(`bot-ping-${nickname}`).innerHTML = `<span style="color: ${pingColor};">  ${ping}</span>`;
          } else {
            const card = document.createElement('div');
            card.className = 'bot-card';

            card.innerHTML = `
              <div class="bot-card-head">
                <img src="${steveIconPath.src}" class="image" draggable="false">
                <div class="text">
                  <div class="bot-basic-info">
                    <div class="bot-nickname" style="user-select: text; -moz-user-select: text;">${nickname}</div>
                    <div class="bot-status"><span id="bot-status-${nickname}" style="color: ${statusColor};">• ${status}</span></div>
                  </div>
                </div>
              </div>

              <div class="bot-advanced-info">
                <p>Версия:<span id="bot-version-${nickname}">  ${version}</span></p>
                <p>Пароль:<span id="bot-password-${nickname}">  ${password}</span></p>
                <p>Прокси:<span id="bot-proxy-${nickname}">  ${proxy}</span></p>
                <p>Тип прокси:<span id="bot-proxy-type-${nickname}">  ${proxyType}</span></p>
                <p>Скрипт:<span id="bot-script-activity-${nickname}">  ${scriptActivity ? 'Активен' : 'Неактивен'}</span></p>
                <p>Нагрузка:<span id="bot-load-${nickname}"><span style="color: ${loadColor};">  ${load}</span></span></p>
                <p>Пинг:<span id="bot-ping-${nickname}"><span style="color: ${pingColor};">  ${ping}</span></span></p>
              </div>

              <button id="open-chat-${nickname}">
                <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-chat-text" viewBox="0 0 16 16">
                  <path d="M2.678 11.894a1 1 0 0 1 .287.801 11 11 0 0 1-.398 2c1.395-.323 2.247-.697 2.634-.893a1 1 0 0 1 .71-.074A8 8 0 0 0 8 14c3.996 0 7-2.807 7-6s-3.004-6-7-6-7 2.808-7 6c0 1.468.617 2.83 1.678 3.894m-.493 3.905a22 22 0 0 1-.713.129c-.2.032-.352-.176-.273-.362a10 10 0 0 0 .244-.637l.003-.01c.248-.72.45-1.548.524-2.319C.743 11.37 0 9.76 0 8c0-3.866 3.582-7 8-7s8 3.134 8 7-3.582 7-8 7a9 9 0 0 1-2.347-.306c-.52.263-1.639.742-3.468 1.105"/>
                  <path d="M4 5.5a.5.5 0 0 1 .5-.5h7a.5.5 0 0 1 0 1h-7a.5.5 0 0 1-.5-.5M4 8a.5.5 0 0 1 .5-.5h7a.5.5 0 0 1 0 1h-7A.5.5 0 0 1 4 8m0 2.5a.5.5 0 0 1 .5-.5h4a.5.5 0 0 1 0 1h-4a.5.5 0 0 1-.5-.5"/>
                </svg>
                Открыть чат
              </button>

              <button class="recreate-btn" id="recreate-${nickname}" style="color: #f10e0eff; margin-bottom: 12px;">
                <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-arrow-repeat" viewBox="0 0 16 16">
                  <path d="M11.534 7h3.932a.25.25 0 0 1 .192.41l-1.966 2.36a.25.25 0 0 1-.384 0l-1.966-2.36a.25.25 0 0 1 .192-.41m-11 2h3.932a.25.25 0 0 0 .192-.41L2.692 6.23a.25.25 0 0 0-.384 0L.342 8.59A.25.25 0 0 0 .534 9"/>
                  <path fill-rule="evenodd" d="M8 3c-1.552 0-2.94.707-3.857 1.818a.5.5 0 1 1-.771-.636A6.002 6.002 0 0 1 13.917 7H12.9A5 5 0 0 0 8 3M3.1 9a5.002 5.002 0 0 0 8.757 2.182.5.5 0 1 1 .771.636A6.002 6.002 0 0 1 2.083 9z"/>
                </svg>
                Пересоздать
              </button>

              <div class="chat-container" id="chat-${nickname}">
                <div class="chat-element-group">
                  <div class="left-group">
                    <div class="custom-select" style="margin-top: 5px;">
                      <select id="select-chat-filter-${nickname}" style="background: #292828b6;">
                        <option value="all">Все сообщения</option>
                        <option value="bans">Блокировки</option>
                        <option value="mentions">Упоминания</option>
                        <option value="links">Ссылки</option>
                      </select>
                    </div>

                    <button id="filter-chat-${nickname}">
                      <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-funnel" viewBox="0 0 16 16">
                        <path d="M1.5 1.5A.5.5 0 0 1 2 1h12a.5.5 0 0 1 .5.5v2a.5.5 0 0 1-.128.334L10 8.692V13.5a.5.5 0 0 1-.342.474l-3 1A.5.5 0 0 1 6 14.5V8.692L1.628 3.834A.5.5 0 0 1 1.5 3.5zm1 .5v1.308l4.372 4.858A.5.5 0 0 1 7 8.5v5.306l2-.666V8.5a.5.5 0 0 1 .128-.334L13.5 3.308V2z"/>
                      </svg>
                      Фильтровать
                    </button>
                  </div>

                  <div class="right-group">
                    <button id="clear-chat-${nickname}" style="color: #ff0000e7;">
                      <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-trash-fill" viewBox="0 0 16 16">
                        <path d="M2.5 1a1 1 0 0 0-1 1v1a1 1 0 0 0 1 1H3v9a2 2 0 0 0 2 2h6a2 2 0 0 0 2-2V4h.5a1 1 0 0 0 1-1V2a1 1 0 0 0-1-1H10a1 1 0 0 0-1-1H7a1 1 0 0 0-1 1zm3 4a.5.5 0 0 1 .5.5v7a.5.5 0 0 1-1 0v-7a.5.5 0 0 1 .5-.5M8 5a.5.5 0 0 1 .5.5v7a.5.5 0 0 1-1 0v-7A.5.5 0 0 1 8 5m3 .5v7a.5.5 0 0 1-1 0v-7a.5.5 0 0 1 1 0"/>
                      </svg>
                      Очистить
                    </button>

                    <button id="close-chat-${nickname}" style="margin-left: 1px;">
                      <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-x-lg" viewBox="0 0 16 16">
                        <path d="M2.146 2.854a.5.5 0 1 1 .708-.708L8 7.293l5.146-5.147a.5.5 0 0 1 .708.708L8.707 8l5.147 5.146a.5.5 0 0 1-.708.708L8 8.707l-5.146 5.147a.5.5 0 0 1-.708-.708L7.293 8z"/>
                      </svg>
                      Закрыть
                    </button>
                  </div>
                </div>

                <div class="chat-content">
                  <div class="monitoring-content" id="monitoring-chat-content-${nickname}"></div>
                </div>

                <p class="signature">Чат бота ${nickname}</p>
              </div>
            `;

            this.botCardsContainer.appendChild(card);
            this.chatHistoryFilters[nickname] = 'all';

            statistics.nicknamesList.push(nickname);

            setTimeout(() => this.initializeBotCard(nickname), 200);
          }
        } catch (error) {
          log(`Ошибка парсинга SSE-сообщения (bots-monitoring): ${error}`, 'log-error');
        }
      }

      eventSource.onerror = async () => {
        log('Ошибка (bots-monitoring): SSE-connection was dropped', 'log-error');
        eventSource.close();

        this.botCardsContainer.innerHTML = '';

        this.statusText.innerText = 'Ошибка соединения с сервером\nПодробнее: SSE-connection was dropped';
        this.statusText.style.color = '#e32020ff';
        this.statusText.style.display = 'block';
      }
    } catch (error) {
      log(`Ошибка инициализации мониторинга ботов: ${error}`, 'log-error');
    }
  }

  filterMessage(type, message) {
    switch (type) {
      case 'all':
        return true;
      case 'bans':
        const blockingPatterns = [
          'banned', 'ban', 'banned IP', 'IP banned',
          'заблокирован', 'account ban', 'блокировка',
          'IP заблокирован', 'заблокирован по IP', 
          'забанен', 'IP забанен', 'забанили',
          'blocked', 'IP blocked', 'account blocked'
        ];

        let valid = false;

        blockingPatterns.forEach(word => {
          if (message.toLowerCase().includes(word)) {
            valid = true;
          } 
        });

        return valid;
      case 'mentions':
        if (message.toLowerCase().includes('@')) return true;
      case 'links':
        if (message.toLowerCase().includes('http://') || message.toLowerCase().includes('https://')) return true;
    }

    return false;
  }

  async chatHistoryMonitoring() {
    try {
      const eventSource = new EventSource(`${localhost}/session/monitoring/chat-history`);

      eventSource.onmessage = async (event) => {
        try {
          const data = JSON.parse(event.data);

          const nickname = data.nickname;
          const type = data.type;
          const text = data.text;

          if (!this.filterMessage(this.chatHistoryFilters[nickname], String(text))) return;

          const chat = document.getElementById(`monitoring-chat-content-${nickname}`);

          if (!chat) return;

          const container = document.createElement('div');

          container.className = 'monitoring-line';
          container.id = `monitoring-message-${nickname}`;

          container.innerHTML = `
            <div class="monitoring-line-time">${date()}</div>
            <div class="monitoring-line-content"><span class="monitoring-type">(${type})</span> ${String(text).replace('%hb', '<span style="color: #a85fdfff; font-weight: 600;">').replace('%sc', '</span>')}</div>
          `;

          chat.appendChild(container);

          if (!this.chatMessageCounter[nickname]) {
            this.chatMessageCounter[nickname] = 1;
          } else {
            this.chatMessageCounter[nickname] = this.chatMessageCounter[nickname] + 1;

            if (this.chatMessageCounter[nickname] > this.maxChatHistoryLength) {
              this.chatMessageCounter[nickname] = this.chatMessageCounter[nickname] - 1;
              chat.firstChild.remove();
            }
          }

          chat.scrollTo({
            top: chat.scrollHeight,
            behavior: 'smooth'
          });
        } catch (error) {
          log(`Ошибка парсинга SSE-сообщения (chat-monitoring): ${error}`, 'log-error');
        }
      }

      eventSource.onerror = async () => {
        log('Ошибка (chat-monitoring): SSE-connection was dropped', 'log-error');
        eventSource.close();

        this.chatMessageCounter = {};

        for (const nickname of statistics.nicknamesList) {
          const chat = document.getElementById(`monitoring-chat-content-${nickname}`);

          chat.innerHTML = '';
          chat.style.display = 'none';
          chat.remove();
        }
      }
    } catch (error) {
      log(`Ошибка инициализации мониторинга чата: ${error}`, 'log-error');
    }
  }
}

class ProxyManager {
  proxyList = null;
  proxyCount = { socks5: null, socks4: null, http: null, total: null };

  proxyFinderStatus = null;

  init() {
    this.proxyList = document.getElementById('proxy-list');

    this.proxyFinderStatus = document.getElementById('proxy-finder-status');

    this.proxyCount.socks5 = document.getElementById('socks5-proxy-count');
    this.proxyCount.socks4 = document.getElementById('socks4-proxy-count');
    this.proxyCount.http = document.getElementById('http-proxy-count');
    this.proxyCount.total = document.getElementById('total-proxy-count');

    this.proxyList.addEventListener('input', () => this.updateCount());

    document.getElementById('load-proxy-file').addEventListener('click', async () => {
      const path = await client.openFile();
      const splitPath = String(path).split('.');

      if (splitPath[splitPath.length - 1] === 'txt') {
        const operation = await sendDataTo({ url: `http://localhost:37182/salarixi/system/proxy/text`, method: 'POST', useHeaders: true }, {
          key: 'salarixionion:j3l14rFj',
          data: { path: path }
        });

        if (operation.success) {
          const proxyList = document.getElementById('proxy-list');

          let isFirst = true;

          for (const element of String(operation.answer.data).split(',')) {
            isFirst ? isFirst = false : proxyList.value += '\n';
            proxyList.value += element;
          }
        } else {
          log(`Ошибка загрузки файла: ${operation.message}`, 'log-error');
        }
      } else if (splitPath[splitPath.length - 1] === 'json') {
        const operation = await sendDataTo({ url: `http://localhost:37182/salarixi/system/proxy/json`, method: 'POST', useHeaders: true }, {
          key: 'salarixionion:j3l14rFj',
          data: { path: path }
        });

        if (operation.success) {
          const proxyList = document.getElementById('proxy-list');
          
          const data = operation.answer.data;

          let isFirst = true;

          const protocols = ['http', 'socks4', 'socks5'];

          for (const protocol of protocols) {
            if (data[protocol]) {
              for (const proxy of data[protocol]) {
                isFirst ? isFirst = false : proxyList.value += '\n';
                proxyList.value += `${protocol}://${proxy}`;
              }
            }
          }
    
          log(JSON.stringify(data), 'log-system');
        } else {
          log(`Ошибка загрузки файла: ${operation.message}`, 'log-error');
        }
      }

      proxy.updateCount();
    });

    document.getElementById('clear-proxy-list').addEventListener('click', () => {
      this.proxyList.value = '';
      this.updateCount();
    });

    document.getElementById('find-proxy').addEventListener('click', () => this.collectProxy());

    this.updateCount();
  }

  updateCount() {
    let socks5 = 0;
    let socks4 = 0;
    let http = 0;

    String(this.proxyList.value).split('\n').forEach(element => {
      if (element.startsWith('socks5://')) socks5++;
      if (element.startsWith('socks4://')) socks4++;
      if (element.startsWith('http://')) http++;
    });

    this.proxyCount.socks5.innerText = socks5;
    this.proxyCount.socks4.innerText = socks4;
    this.proxyCount.http.innerText = http;
    this.proxyCount.total.innerText = socks5 + socks4 + http;
  }
  
  async collectProxy() {
    try {
      const algorithm = document.getElementById('proxy-finder-algorithm').value;
      const protocol = document.getElementById('proxy-finder-protocol').value;
      const country = document.getElementById('proxy-finder-country').value;
      const quantity = document.getElementById('proxy-finder-quantity').value;

      this.proxyFinderStatus.innerText = 'Поиск прокси...';

      const response = await fetch('http://localhost:37475/salarixi/web/collector/proxy', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          algorithm: algorithm,
          protocol: protocol,
          country: country,
          quantity: quantity
        })
      });

      this.proxyFinderStatus.innerText = 'Парсинг ответа...';
      
      const data = await response.json();

      if (data.success) {
        this.proxyFinderStatus.style.color = '#0cd212ff';
        this.proxyFinderStatus.innerText = 'Поиск окончен';
        this.proxyList.value = Array.from(data.list).filter(e => e && e.trim() !== '').join('\n');
      } else {
        this.proxyFinderStatus.style.color = '#cc1d1dff';
        this.proxyFinderStatus.innerText = 'Ошибка поиска';

        log(`Ошибка поиска прокси: ${data.error}`, 'log-error');
      }
    } catch (error) {
      this.proxyFinderStatus.style.color = '#cc1d1dff';
      this.proxyFinderStatus.innerText = 'Ошибка поиска';

      log(`Ошибка поиска прокси: ${error}`, 'log-error');
    } finally {
      this.updateCount();
      setTimeout(() => {
        this.proxyFinderStatus.style.color = '#848080';
        this.proxyFinderStatus.innerText = 'Поиск неактивен';
      }, 2000);
    }
  }
}

const functions = new Functions();
const graphic = new GraphicManager();
const monitoring = new MonitoringManager();
const proxy = new ProxyManager();

setInterval(async () => {
  if (!statistics.isInitialized) return;

  const elements = document.querySelectorAll('[conserve="true"]');

  const config = {};

  for (const element of elements) {
    if (element.type === 'checkbox') {
      config[element.name] = {
        id: element.id,
        value: element.checked
      };
    } else {
      config[element.name] = {
        id: element.id,
        value: element.type === 'number' ? parseInt(element.value) : element.value
      };
    }
  }

  if (JSON.stringify(config) === JSON.stringify(statistics.latestConfig)) return;

  await sendDataTo({ url: 'http://localhost:37182/salarixi/system/config/write', method: 'POST', useHeaders: true }, { 
    key: 'salarixionion:j3l14rFj',
    data: { config: config }
  });

  statistics.latestConfig = config;
}, 1400);

async function loadConfig() {
  log('Загрузка конфига...', 'log-system');

  const operation = await sendDataTo({ url: 'http://localhost:37182/salarixi/system/config/read', method: 'POST', useHeaders: true }, { 
    key: 'salarixionion:j3l14rFj'
  });

  if (!operation.success) {
    log(`Ошибка (load-config): ${operation.message}`, 'log-error');
    return;
  }

  if (operation.answer.invalidKey) {
    log(`Ошибка (load-config): ${operation.answer.message}`, 'log-error');
    return;
  } else {
    for (const [_, element] of Object.entries(operation.answer.data)) {
      if (element.value) {
        const html = document.getElementById(element.id);

        if (html) {
          if (String(element.id).startsWith('use')) {
            html.checked = element.value;
          } else {
            if (typeof element.value === 'number') {
              html.valueAsNumber = element.value ? element.value : 0;
            } else {
              html.value = element.value;
            }
          }
        } 
      }
    }

    log('Конфиг успешно загружен', 'log-system');
  }
}

function showContainer(container) {
  globalContainers.forEach(element => {
    if (!element) return;

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

function changeDisplayButtonList(type, who)  {
  let buttonListContainer = document.getElementById(`control-${type}-button-list`);

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

    const buttons = buttonListContainer.querySelectorAll('.list-btn');

    buttons.forEach((button, index) => {
      setTimeout(() => {
        button.classList.add('temporary-theme');

        setTimeout(() => {
          button.classList.remove('temporary-theme');
        }, 130); 
      }, index * 70); 
    });
  }
}

async function initializeInformationCard() {
  const clientHeaderContainer = document.getElementById('client-header');
  const clientVersionContainer = document.getElementById('client-version');
  const clientTypeContainer = document.getElementById('client-type');
  const clientReleaseDateContainer = document.getElementById('client-release-date');

  const info = await client.getInfo();

  if (info.type === 'Beta') {
    clientHeaderContainer.classList.add('beta');
  } else if (info.type === 'Expert') {
    clientHeaderContainer.classList.add('expert');
  }

  const header = `${info.type} Release`;

  clientHeaderContainer.innerText = header;
  clientVersionContainer.innerText = info.version;
  clientTypeContainer.innerText = info.type;
  clientReleaseDateContainer.innerText = info.releaseDate;
}

async function initializeSocialButtons() {
  const telegram = document.getElementById('telegram');
  const github = document.getElementById('github');
  const youtube = document.getElementById('youtube');

  telegram.addEventListener('click', async () => await client.openUrl('https://t.me/salarixionion')); 
  github.addEventListener('click', async () => await client.openUrl('https://github.com/nullclyze/SalarixiOnion'));
  youtube.addEventListener('click', async () => await client.openUrl('https://www.youtube.com/@salarixionion'));
}

async function initializePanel() {
  const panelBtns = document.querySelectorAll('.panel-btn');

  panelBtns.forEach(button => {
		if (button.id === 'main') button.classList.add('selected');

  	button.addEventListener('click', () => {
    	panelBtns.forEach(btn => btn.classList.remove('selected'));
      
    	button.classList.add('selected');
  	});
	});

  const mainPageBtn = document.getElementById('main');
	const settingsPageBtn = document.getElementById('settings');
  const proxyPageBtn = document.getElementById('proxy');
  const controlPageBtn = document.getElementById('control');
  const scriptPageBtn = document.getElementById('script');
  const monitoringPageBtn = document.getElementById('monitoring');
  const graphicPageBtn = document.getElementById('graphic');
  const analysisPageBtn = document.getElementById('analysis');
  const spyPageBtn = document.getElementById('spy');
	const logPageBtn = document.getElementById('log');
	const	aboutPageBtn = document.getElementById('about');

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

async function initializeControlContainer() {
  const controlChatPageBtn = document.getElementById('control-chat');
  const controlActionsPageBtn = document.getElementById('control-actions');
  const controlMovePageBtn = document.getElementById('control-move');
  const controlImitationPageBtn = document.getElementById('control-imitation');
  const controlAttackPageBtn = document.getElementById('control-attack');
  //const controlInvenotryPageBtn = document.getElementById('control-inventory');
  const controlFlightPageBtn = document.getElementById('control-flight');
  const controlSprinterPageBtn = document.getElementById('control-sprinter');
  const controlGhostPageBtn = document.getElementById('control-ghost');

  const openToolsButtonListBtn = document.getElementById('open-tools-button-list');
  const openCheatButtonListBtn = document.getElementById('open-cheat-button-list');

  controlChatPageBtn.addEventListener('click', () => showContainer(controlChatPageContainer));
  controlActionsPageBtn.addEventListener('click', () => showContainer(controlActionsPageContainer));
  controlMovePageBtn.addEventListener('click', () => showContainer(controlMovePageContainer));
  controlImitationPageBtn.addEventListener('click', () => showContainer(controlImitationPageContainer));
  controlAttackPageBtn.addEventListener('click', () => showContainer(controlAttackPageContainer));
  //controlInvenotryPageBtn.addEventListener('click', () => showContainer(controlInventoryPageContainer));
  controlFlightPageBtn.addEventListener('click', () => showContainer(controlFlightPageContainer));
  controlSprinterPageBtn.addEventListener('click', () => showContainer(controlSprinterPageContainer));
  controlGhostPageBtn.addEventListener('click', () => showContainer(controlGhostPageContainer));

  openToolsButtonListBtn.addEventListener('click', () => changeDisplayButtonList('tools', 'user'));
  openCheatButtonListBtn.addEventListener('click', () => changeDisplayButtonList('cheat', 'user'));
}

async function initializeGraphicActiveBots() {
  try {
    await graphic.createGraphicActiveBots();

    const eventSource = new EventSource(`${localhost}/session/graphic/active-bots`);

    eventSource.onmessage = async (event) => {
      try {
        const data = JSON.parse(event.data);

        const activeBotsQuantity = data.activeBotsQuantity;

        await graphic.addGraphicDataActiveBots(activeBotsQuantity);
      } catch (error) {
        log(`Ошибка парсинга SSE-сообщения (graphic-active-bots): ${error}`, 'log-error');
      }
    }

    eventSource.onerror = async () => {
      log('Ошибка (graphic-active-bots): SSE-connection was dropped', 'log-error');
      eventSource.close();
    }
  } catch (error) {
    log(`Ошибка инициализации графика активных ботов: ${error}`, 'log-error');
  }
}

async function initializeGraphicAverageLoad() {
  try {
    await graphic.createGraphicAverageLoad();

    const eventSource = new EventSource(`${localhost}/session/graphic/average-load`);

    eventSource.onmessage = async (event) => {
      try {
        const data = JSON.parse(event.data);

        const averageLoad = data.averageLoad ? data.averageLoad : 0;

        await graphic.addGraphicDataAverageLoad(averageLoad);
      } catch (error) {
        log(`Ошибка парсинга SSE-сообщения (graphic-average-load): ${error}`, 'log-error');
      }
    }

    eventSource.onerror = async () => {
      log('Ошибка (graphic-average-load): SSE-connection was dropped', 'log-error');
      eventSource.close();
    }
  } catch (error) {
    log(`Ошибка инициализации графика средней нагрузки ботов: ${error}`, 'log-error');
  }
}

async function checkUpdate() {
  try {
    const closeNoticeBtn = document.getElementById('close-notice-btn');

    closeNoticeBtn.addEventListener('click', () => {
      const notice = document.getElementById('notice');
      if (!notice) return;
      notice.style.display = 'none';
    });

    const notice = document.getElementById('notice');
    const newVersion = document.getElementById('new-client-version');
    const newType = document.getElementById('new-client-type');
    const newTag = document.getElementById('new-client-tag');
    const newReleaseDate = document.getElementById('new-client-release-date');
    const openClientRelease = document.getElementById('open-client-release');

    const response = await fetch('https://raw.githubusercontent.com/nullclyze/SalarixiOnion/refs/heads/main/salarixi.version.json', { method: 'GET' });

    if (!response.ok) return;

    const data = await response.json();
    const info = await client.getInfo();

    if (data && data.version !== info.version) {
      tag = `v${data.version}-${String(data.type).toLowerCase()}`;

      newVersion.innerText = data.version;
      newType.innerText = data.type;
      newTag.innerText = tag;
      newReleaseDate.innerText = data.releaseDate;
      
      openClientRelease.addEventListener('click', async () => await client.openUrl(`https://github.com/nullclyze/SalarixiOnion/releases/tag/${tag}`));
      
      setTimeout(() => {
        notice.style.display = 'flex';
      }, 3500);
    }
  } catch (error) {
    log(`Ошибка проверки обновлений: ${error}`, 'log-error');
  }
}

document.addEventListener('DOMContentLoaded', async () => {
  log('Клиент запущен', 'log-info');

  localhost = `http://localhost:${await client.port()}/salarixi`;

  document.getElementById('window-minimize').addEventListener('click', () => client.window('minimize'));
  document.getElementById('window-close').addEventListener('click', () => client.window('close'));

  mainPageContainer = document.getElementById('main-container');
  settingsPageContainer = document.getElementById('settings-container');
  proxyPageContainer = document.getElementById('proxy-container');
  controlSectorsPageContainer = document.getElementById('control-sectors-container');
  controlChatPageContainer = document.getElementById('control-chat-container');
  controlActionsPageContainer = document.getElementById('control-action-container');
  controlMovePageContainer = document.getElementById('control-move-container');
  controlImitationPageContainer = document.getElementById('control-imitation-container');
  controlAttackPageContainer = document.getElementById('control-attack-container');
  controlInventoryPageContainer = document.getElementById('control-inventory-container');
  controlFlightPageContainer = document.getElementById('control-flight-container');
  controlSprinterPageContainer = document.getElementById('control-sprinter-container');
  controlGhostPageContainer = document.getElementById('control-ghost-container');
  scriptPageContainer = document.getElementById('script-container');
  graphicPageContainer = document.getElementById('graphic-container');
  monitoringPageContainer = document.getElementById('monitoring-container');
  analysisPageContainer = document.getElementById('analysis-container');
  spyPageContainer = document.getElementById('spy-container');
  logPageContainer = document.getElementById('log-container');
  aboutPageContainer = document.getElementById('about-container');

  globalContainers = [
    mainPageContainer, settingsPageContainer, proxyPageContainer,
    controlSectorsPageContainer, controlChatPageContainer, controlActionsPageContainer,
    controlMovePageContainer, controlImitationPageContainer, controlFlightPageContainer,
    controlSprinterPageContainer, controlGhostPageContainer, controlAttackPageContainer, 
    controlInventoryPageContainer, scriptPageContainer, graphicPageContainer,
    monitoringPageContainer, analysisPageContainer, spyPageContainer,
    logPageContainer, aboutPageContainer
  ];

  await clear();

  await loadConfig();

  await initializePanel();
  await initializeInformationCard();
  await initializeControlContainer();

  await initializeSocialButtons();

  proxy.init();

  await initializeGraphicActiveBots();
  await initializeGraphicAverageLoad();

  await monitoring.init().then(async () => {
    await monitoring.profileDataMonitoring();
    await monitoring.chatHistoryMonitoring();
  });
  
  await functions.initializeButtonFunctions();
  await functions.initializeCheckboxFunctions();
  await functions.initializeSelectFunctions();

  statistics.isInitialized = true;

  const loadingContainer = document.getElementById('loading-container');

  if (loadingContainer) {
    setTimeout(() => {
      loadingContainer.classList.add('hide');
      setTimeout(() => {
        loadingContainer.style.display = 'none';
      }, 590);
    }, 1000);
  }

  await checkUpdate();
});