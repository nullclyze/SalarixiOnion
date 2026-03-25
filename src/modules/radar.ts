import { invoke } from '@tauri-apps/api/core';
import { isAbsolute } from '@tauri-apps/api/path';
import { Chart } from 'chart.js';

import { logger } from '../utils/logger';
import { messages } from '../utils/message';

interface RadarInfo {
  status: string;
  uuid: string;
  x: number;
  y: number;
  z: number;
  observer: {
    x: number;
    z: number;
  };
}

class Radar {
  private active: boolean = false;

  private targetCardsContainer: HTMLElement | null = null;
  private targetWrappersContainer: HTMLElement | null = null;

  private updateFrequency: number = 1500;
  private targets: Map<string, { data: any, interval: any, chart: any }> = new Map();

  /** Метод инициализации функций, связанных с радаром. */
  public init(): void {
    this.targetCardsContainer = document.getElementById('radar-target-cards-container') as HTMLElement;
    this.targetWrappersContainer = document.getElementById('radar-target-wrappers-container') as HTMLElement;

    const addTargetBtn = document.getElementById('radar-add-target') as HTMLButtonElement;
    const openSettingsBtn = document.getElementById('radar-open-settings') as HTMLButtonElement;
    const closeSettingsBtn = document.getElementById('radar-close-settings') as HTMLButtonElement;
    const removeAllTargetsBtn = document.getElementById('radar-remove-all-targets') as HTMLElement;
    const updateFrequency = document.getElementById('radar_select_update-frequency') as HTMLSelectElement;

    addTargetBtn.addEventListener('click', () => {
      if (!this.active) return;

      const usernameInput = document.getElementById('radar-target-username') as HTMLInputElement;
      const username = usernameInput.value;

      if (this.targets.has(username)) return;

      usernameInput.value = '';

      this.targets.set(username, { data: null, interval: null, chart: null });

      const card = document.createElement('div');
      card.className = 'radar-target';
      card.id = `radar-target-${username}`;

      card.innerHTML = `
        <svg class="icon" xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-person-fill" viewBox="0 0 16 16">
          <path d="M3 14s-1 0-1-1 1-4 6-4 6 3 6 4-1 1-1 1zm5-6a3 3 0 1 0 0-6 3 3 0 0 0 0 6"/>
        </svg>

        <div class="sep"></div>

        <div class="info" style="min-width: 220px; max-width: 220px;">
          <p>Никнейм: <span>${username.length <= 16 ? username : username.substr(0, 16) + '...'}</span></p>
          <p>Статус: <span id="radar-target-status-${username}">Не найден</span></p>
          <p>UUID: <span id="radar-target-uuid-${username}">?</span></p>
        </div>

        <div class="sep"></div>

        <div class="info" style="min-width: 150px; max-width: 150px;">
          <p>X: <span id="radar-target-x-${username}">?</span></p>
          <p>Y: <span id="radar-target-y-${username}">?</span></p>
          <p>Z: <span id="radar-target-z-${username}">?</span></p>
        </div>

        <div class="sep"></div>

        <div class="btn-group">
          <div class="btn-group-flex" style="margin-top: 0;">
            <button class="btn min" id="radar-open-route-${username}">
              <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-geo-alt-fill" viewBox="0 0 16 16">
                <path d="M8 16s6-5.686 6-10A6 6 0 0 0 2 6c0 4.314 6 10 6 10m0-7a3 3 0 1 1 0-6 3 3 0 0 1 0 6"/>
              </svg>
            </button>

            <button class="btn min" id="radar-remove-target-${username}">
              <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-trash-fill" viewBox="0 0 16 16">
                <path d="M2.5 1a1 1 0 0 0-1 1v1a1 1 0 0 0 1 1H3v9a2 2 0 0 0 2 2h6a2 2 0 0 0 2-2V4h.5a1 1 0 0 0 1-1V2a1 1 0 0 0-1-1H10a1 1 0 0 0-1-1H7a1 1 0 0 0-1 1zm3 4a.5.5 0 0 1 .5.5v7a.5.5 0 0 1-1 0v-7a.5.5 0 0 1 .5-.5M8 5a.5.5 0 0 1 .5.5v7a.5.5 0 0 1-1 0v-7A.5.5 0 0 1 8 5m3 .5v7a.5.5 0 0 1-1 0v-7a.5.5 0 0 1 1 0"/>
              </svg>
            </button>
          </div>

          <div class="btn-group-flex" style="margin-top: 0;">
            <button class="btn min" id="radar-follow-target-${username}">
              <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-crosshair" viewBox="0 0 16 16">
                <path d="M8.5.5a.5.5 0 0 0-1 0v.518A7 7 0 0 0 1.018 7.5H.5a.5.5 0 0 0 0 1h.518A7 7 0 0 0 7.5 14.982v.518a.5.5 0 0 0 1 0v-.518A7 7 0 0 0 14.982 8.5h.518a.5.5 0 0 0 0-1h-.518A7 7 0 0 0 8.5 1.018zm-6.48 7A6 6 0 0 1 7.5 2.02v.48a.5.5 0 0 0 1 0v-.48a6 6 0 0 1 5.48 5.48h-.48a.5.5 0 0 0 0 1h.48a6 6 0 0 1-5.48 5.48v-.48a.5.5 0 0 0-1 0v.48A6 6 0 0 1 2.02 8.5h.48a.5.5 0 0 0 0-1zM8 10a2 2 0 1 0 0-4 2 2 0 0 0 0 4"/>
              </svg>
            </button>

            <button class="btn min" id="radar-copy-target-info-${username}">
              <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-copy" viewBox="0 0 16 16">
                <path fill-rule="evenodd" d="M4 2a2 2 0 0 1 2-2h8a2 2 0 0 1 2 2v8a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2zm2-1a1 1 0 0 0-1 1v8a1 1 0 0 0 1 1h8a1 1 0 0 0 1-1V2a1 1 0 0 0-1-1zM2 5a1 1 0 0 0-1 1v8a1 1 0 0 0 1 1h8a1 1 0 0 0 1-1v-1h1v1a2 2 0 0 1-2 2H2a2 2 0 0 1-2-2V6a2 2 0 0 1 2-2h1v1z"/>
              </svg>
            </button>
          </div>
        </div>
      `;

      const routeWrapper = document.createElement('div');
      routeWrapper.className = 'cover';
      routeWrapper.id = `radar-route-${username}`;

      routeWrapper.innerHTML = `
        <div class="panel with-header" style="margin-bottom: 40px;">
          <div class="left">
            <div class="header">Маршрут ${username}</div>
          </div>

          <div class="right">
            <button class="btn min" id="radar-close-route-${username}">
              ⨉
            </button>
          </div>
        </div>

        <canvas class="radar-chart" id="radar-chart-${username}"></canvas>
      `;

      this.targetCardsContainer?.appendChild(card);
      this.targetWrappersContainer?.appendChild(routeWrapper);

      this.initializeTargetCard(username);
    });

    openSettingsBtn.addEventListener('click', () => (document.getElementById('radar-settings') as HTMLElement).style.display = 'flex'); 
    closeSettingsBtn.addEventListener('click', () => (document.getElementById('radar-settings') as HTMLElement).style.display = 'none'); 

    removeAllTargetsBtn.addEventListener('click', () => {
      this.targets.forEach((v, n) => {
        const card = document.getElementById(`radar-target-${n}`) as HTMLElement;
        v.chart.destroy();
        card.remove();
        clearInterval(v.interval);
      });

      this.targets.clear();
    });

    updateFrequency.addEventListener('change', () => {
      this.updateFrequency = updateFrequency.value ? parseInt(updateFrequency.value) : 1500;

      this.targets.forEach((i, n) => {
        clearInterval(i.interval);
        this.setTargetUpdateInterval(n, this.updateFrequency);
      });
    });
  }

  /** Метод активации радара. */
  public enable(): void {
    this.active = true;
  }

  /** Метод выключения радара. */
  public disable(): void {
    this.active = false;
  }

  /** Метод инициализации карточки цели. */
  private initializeTargetCard(username: string): void {
    try {
      const openRouteBtn = document.getElementById(`radar-open-route-${username}`) as HTMLButtonElement;
      const closeRouteBtn = document.getElementById(`radar-close-route-${username}`) as HTMLButtonElement;
      const removeTargetBtn = document.getElementById(`radar-remove-target-${username}`) as HTMLButtonElement;
      const copyTargetInfoBtn = document.getElementById(`radar-copy-target-info-${username}`) as HTMLButtonElement;
      const followTargetBtn = document.getElementById(`radar-follow-target-${username}`) as HTMLButtonElement;

      openRouteBtn.addEventListener('click', () => (document.getElementById(`radar-route-${username}`) as HTMLElement).style.display = 'flex');
      closeRouteBtn.addEventListener('click', () => (document.getElementById(`radar-route-${username}`) as HTMLElement).style.display = 'none');

      removeTargetBtn.addEventListener('click', () => {
        const card = document.getElementById(`radar-target-${username}`) as HTMLElement;
        this.targets.get(username)?.chart.destroy();
        card.remove();
        clearInterval(this.targets.get(username)?.interval);
        this.targets.delete(username);
      });

      copyTargetInfoBtn.addEventListener('click', async () => {
        try {
          const status = (document.getElementById(`radar-target-status-${username}`) as HTMLElement).textContent;
          const uuid = this.targets.get(username)?.data?.fullUUID ? this.targets.get(username)?.data.fullUUID : '?';
          const x = (document.getElementById(`radar-target-x-${username}`) as HTMLElement).textContent;
          const y = (document.getElementById(`radar-target-y-${username}`) as HTMLElement).textContent;
          const z = (document.getElementById(`radar-target-z-${username}`) as HTMLElement).textContent;

          const text = `
Никнейм: ${username}
Статус: ${status}
UUID: ${uuid}
Координата X: ${x}
Координата Y: ${y}
Координата Z: ${z}
          `.trim();

          await navigator.clipboard.writeText(text);

          messages.message('Радар', `Данные игрока ${username} успешно скопированы в буфер обмена`);
        } catch (error) {
          logger.log(`Ошибка копирования данных радара: ${error}`, 'error');
        }
      });

      followTargetBtn.addEventListener('click', async () => {
        try {
          const x = parseInt((document.getElementById(`radar-target-x-${username}`) as HTMLElement).textContent);
          const z = parseInt((document.getElementById(`radar-target-z-${username}`) as HTMLElement).textContent);

          await invoke('follow_radar_target', {
            x: x,
            z: z
          });

          messages.message('Радар', `Боты начали преследовать игрока ${username} (X: ${x}, Z: ${z})`);
        } catch (error) {
          logger.log(`Ошибка преследования цели радара: ${error}`, 'error');
        }
      });

      this.targets.set(username, { data: null, interval: null, chart: null });

      this.createTargetChart(username);
      this.setTargetUpdateInterval(username, this.updateFrequency);
    } catch (error) {
      logger.log(`Ошибка инициализации цели радара: ${error}`, 'error');
    }
  }

  /** Метод установки частоты обновления. */
  private setTargetUpdateInterval(username: string, frequency: number) {
    let lx = '';
    let ly = '';
    let lz = '';

    const target = this.targets.get(username);

    if (target) {
      target.interval = setInterval(async () => {
        if (!this.active) {
          clearInterval(target.interval);
          return;
        }

        try {
          const data = await invoke('get_radar_data', { target: username }) as RadarInfo;

          if (!data) return;

          const x = data.x.toFixed(3);
          const y = data.y.toFixed(3);
          const z = data.z.toFixed(3);

          (document.getElementById(`radar-target-status-${username}`) as HTMLElement).innerText = data.status;
          (document.getElementById(`radar-target-uuid-${username}`) as HTMLElement).innerText = data.uuid.substr(0, 12) + '...';
          (document.getElementById(`radar-target-x-${username}`) as HTMLElement).innerText = x;
          (document.getElementById(`radar-target-y-${username}`) as HTMLElement).innerText = y;
          (document.getElementById(`radar-target-z-${username}`) as HTMLElement).innerText = z;

          if (target) {
            target.data = {
              fullUUID: data.uuid
            };
          }

          if (lx !== x || ly !== y || lz !== z) {
            const card = document.getElementById(`radar-target-${username}`) as HTMLElement;
            card.classList.add('glow');
            setTimeout(() => card.classList.remove('glow'), 300);
          }

          lx = data.x.toFixed(3);
          ly = data.y.toFixed(3);
          lz = data.z.toFixed(3);

          this.addRoutePointToChart(username, parseFloat(x), parseFloat(z), data.observer);

          if ((document.getElementById('radar_chbx_auto-save') as HTMLInputElement).checked) {
            const path = (document.getElementById('radar_option_path') as HTMLInputElement).value;
            const filename = (document.getElementById('radar_option_filename') as HTMLInputElement).value;

            if (await isAbsolute(path)) {
              await invoke('save_radar_data', {
                target: username,
                path: path,
                filename: filename || 'radar_#t',
                x: parseFloat(x),
                y: parseFloat(y),
                z: parseFloat(z)
              });
            }
          }
        } catch (error) {
          logger.log(`Ошибка обновления цели радара ${username}: ${error}`, 'error');
        }
      }, frequency);
    }
  }

  /** Метод создания обёртки маршрута цели. */
  private createTargetChart(username: string) {
    const ctx = document.getElementById(`radar-chart-${username}`) as HTMLCanvasElement;

    const chart = new Chart(ctx, {
      type: 'scatter', 
      data: {
        datasets: [
          {
            label: ` Маршрут ${username}`,
            data: [], 
            backgroundColor: '#39a10fff',
            borderColor: '#0f8f0bff', 
            showLine: true, 
            fill: false,
            pointRadius: 2,
            tension: 0,
            borderWidth: 2
          },
          {
            label: ` Метка наблюдателя`,
            data: [], 
            backgroundColor: '#d31212ff',
            borderColor: '#800c0cff', 
            showLine: false, 
            fill: false,
            pointRadius: 3,
            tension: 0,
            borderWidth: 1
          }
        ]
      },
      options: {
        responsive: true,
        animation: {
          duration: 300
        },
        scales: {
          x: {
            type: 'linear',
            position: 'bottom',
            title: {
              display: true,
              text: 'X'
            },
            min: -200,
            max: 200, 
            grid: { 
              color: '#30303086'
            },
            ticks: {
              stepSize: 50
            }
          },
          y: {
            type: 'linear',
            position: 'left',
            title: {
              display: true,
              text: 'Z'
            },
            min: -200,
            max: 200, 
            grid: { 
              color: '#30303086' 
            },
            ticks: {
              stepSize: 50
            }
          }
        },
        plugins: {
          title: {
            display: false,
          },
          legend: {
            display: false 
          },
          tooltip: {
            enabled: false 
          }
        }
      }
    });

    const target = this.targets.get(username);

    if (target) target.chart = chart;
  }

  /** Метод добавления поинта цели (её текущей позиции) на чарт маршрута. */
  private addRoutePointToChart(username: string, x: number, z: number, observer: { x: number, z: number }) {
    const target = this.targets.get(username);

    if (!target) return;

    target.chart.data.datasets[0].data.push({ x: x, y: z });
    target.chart.data.datasets[1].data.push({ x: observer.x, y: observer.z });

    if (target.chart.data.datasets[0].data.length > 30) target.chart.data.datasets[0].data.shift();
    if (target.chart.data.datasets[1].data.length > 1) target.chart.data.datasets[1].data.shift();

    const xMin = Number(x.toFixed(1)) - 200;
    const xMax = Number(x.toFixed(1)) + 200;
    const zMin = Number(z.toFixed(1)) - 200;
    const zMax = Number(z.toFixed(1)) + 200;
    
    target.chart.options.scales.x.min = xMin;
    target.chart.options.scales.x.max = xMax;
    target.chart.options.scales.y.min = zMin;
    target.chart.options.scales.y.max = zMax;

    target.chart.update();
  }
}

const radar = new Radar();

export { radar }