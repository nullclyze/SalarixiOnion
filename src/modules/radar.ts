import { invoke } from '@tauri-apps/api/core';
import { isAbsolute } from '@tauri-apps/api/path';
import { Chart } from 'chart.js';

import { log } from '../logger';


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

export class RadarManager {
  private active: boolean = false;

  private targetCardsContainer: HTMLElement | null = null;

  private updateFrequency: number = 1500;
  private targets: Map<string, { data: any, interval: any, chart: any }> = new Map();

  public init(): void {
    this.targetCardsContainer = document.getElementById('radar-target-cards-container') as HTMLElement;

    const addTargetBtn = document.getElementById('radar-add-target') as HTMLButtonElement;
    const setUpdateFrequency = document.getElementById('radar-update-frequency') as HTMLSelectElement;
    const openSettingsBtn = document.getElementById('radar-open-settings') as HTMLButtonElement;
    const closeSettingsBtn = document.getElementById('radar-close-settings') as HTMLButtonElement;
    const removeAllTargetsBtn = document.getElementById('radar-remove-all-targets') as HTMLElement;

    addTargetBtn.addEventListener('click', () => {
      if (!this.active) return;

      const usernameInput = document.getElementById('radar-target-nickname') as HTMLInputElement;
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
          <p>Никнейм: ${username.length <= 16 ? username : username.substr(0, 16) + '...'}</p>
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
            <button class="btn min pretty" id="radar-open-route-${username}">
              <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-geo-alt-fill" viewBox="0 0 16 16">
                <path d="M8 16s6-5.686 6-10A6 6 0 0 0 2 6c0 4.314 6 10 6 10m0-7a3 3 0 1 1 0-6 3 3 0 0 1 0 6"/>
              </svg>
            </button>

            <button class="btn min pretty" id="radar-remove-target-${username}">
              <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-trash-fill" viewBox="0 0 16 16">
                <path d="M2.5 1a1 1 0 0 0-1 1v1a1 1 0 0 0 1 1H3v9a2 2 0 0 0 2 2h6a2 2 0 0 0 2-2V4h.5a1 1 0 0 0 1-1V2a1 1 0 0 0-1-1H10a1 1 0 0 0-1-1H7a1 1 0 0 0-1 1zm3 4a.5.5 0 0 1 .5.5v7a.5.5 0 0 1-1 0v-7a.5.5 0 0 1 .5-.5M8 5a.5.5 0 0 1 .5.5v7a.5.5 0 0 1-1 0v-7A.5.5 0 0 1 8 5m3 .5v7a.5.5 0 0 1-1 0v-7a.5.5 0 0 1 1 0"/>
              </svg>
            </button>
          </div>

          <div class="btn-group-flex" style="margin-top: 0;">
            <button class="btn min pretty" disabled>
              <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-crosshair" viewBox="0 0 16 16">
                <path d="M8.5.5a.5.5 0 0 0-1 0v.518A7 7 0 0 0 1.018 7.5H.5a.5.5 0 0 0 0 1h.518A7 7 0 0 0 7.5 14.982v.518a.5.5 0 0 0 1 0v-.518A7 7 0 0 0 14.982 8.5h.518a.5.5 0 0 0 0-1h-.518A7 7 0 0 0 8.5 1.018zm-6.48 7A6 6 0 0 1 7.5 2.02v.48a.5.5 0 0 0 1 0v-.48a6 6 0 0 1 5.48 5.48h-.48a.5.5 0 0 0 0 1h.48a6 6 0 0 1-5.48 5.48v-.48a.5.5 0 0 0-1 0v.48A6 6 0 0 1 2.02 8.5h.48a.5.5 0 0 0 0-1zM8 10a2 2 0 1 0 0-4 2 2 0 0 0 0 4"/>
              </svg>
            </button>

            <button class="btn min pretty" id="radar-copy-target-info-${username}">
              <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-copy" viewBox="0 0 16 16">
                <path fill-rule="evenodd" d="M4 2a2 2 0 0 1 2-2h8a2 2 0 0 1 2 2v8a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2zm2-1a1 1 0 0 0-1 1v8a1 1 0 0 0 1 1h8a1 1 0 0 0 1-1V2a1 1 0 0 0-1-1zM2 5a1 1 0 0 0-1 1v8a1 1 0 0 0 1 1h8a1 1 0 0 0 1-1v-1h1v1a2 2 0 0 1-2 2H2a2 2 0 0 1-2-2V6a2 2 0 0 1 2-2h1v1z"/>
              </svg>
            </button>
          </div>
        </div>

        <div class="cover" id="radar-route-${username}">
          <div class="panel with-header" style="margin-bottom: 30px;">
            <div class="left">
              <div class="header">Маршрут ${username}</div>
            </div>

            <div class="right">
              <button class="btn min pretty" id="radar-close-route-${username}">
                ⨉
              </button>
            </div>
          </div>

          <canvas class="radar-chart" id="radar-chart-${username}"></canvas>
        </div>
      `;

      this.targetCardsContainer!.appendChild(card);

      setTimeout(() => this.initializeTargetCard(username), 200);
    });

    setUpdateFrequency.addEventListener('change', () => {
      this.updateFrequency = setUpdateFrequency.value ? parseInt(setUpdateFrequency.value) : 1500;

      this.targets.forEach((i, n) => {
        clearInterval(i.interval);
        this.setTargetUpdateInterval(n, this.updateFrequency);
      });
    });

    openSettingsBtn.addEventListener('click', () => {
      const settings = document.getElementById('radar-settings') as HTMLElement;
      settings.style.display = 'flex';
    }); 

    closeSettingsBtn.addEventListener('click', () => {
      const settings = document.getElementById('radar-settings') as HTMLElement;
      settings.style.display = 'none';
    }); 

    removeAllTargetsBtn.addEventListener('click', () => {
      this.targets.forEach((v, n) => {
        const card = document.getElementById(`radar-target-${n}`) as HTMLElement;
        v.chart.destroy();
        card.remove();
        clearInterval(v.interval);
      });

      this.targets.clear();
    });
  }

  public enable(): void {
    this.active = true;
  }

  public disable(): void {
    this.active = false;
  }

  private initializeTargetCard(nickname: string): void {
    try {
      const openRouteBtn = document.getElementById(`radar-open-route-${nickname}`) as HTMLButtonElement;
      const closeRouteBtn = document.getElementById(`radar-close-route-${nickname}`) as HTMLButtonElement;
      const removeTargetBtn = document.getElementById(`radar-remove-target-${nickname}`) as HTMLButtonElement;
      const copyTargetInfoBtn = document.getElementById(`radar-copy-target-info-${nickname}`) as HTMLButtonElement;

      openRouteBtn.addEventListener('click', () => {
        const routeContainer = document.getElementById(`radar-route-${nickname}`) as HTMLElement;
        routeContainer.style.display = 'flex';
      });

      closeRouteBtn.addEventListener('click', () => {
        const routeContainer = document.getElementById(`radar-route-${nickname}`) as HTMLElement;
        routeContainer.style.display = 'none';
      });

      removeTargetBtn.addEventListener('click', () => {
        const card = document.getElementById(`radar-target-${nickname}`) as HTMLElement;
        this.targets.get(nickname)?.chart.destroy();
        card.remove();
        clearInterval(this.targets.get(nickname)?.interval);
        this.targets.delete(nickname);
      });

      copyTargetInfoBtn.addEventListener('click', async () => {
        try {
          const status = (document.getElementById(`radar-target-status-${nickname}`) as HTMLElement).textContent;
          const uuid = this.targets.get(nickname)?.data?.fullUUID ? this.targets.get(nickname)?.data.fullUUID : '?';
          const x = (document.getElementById(`radar-target-x-${nickname}`) as HTMLElement).textContent;
          const y = (document.getElementById(`radar-target-y-${nickname}`) as HTMLElement).textContent;
          const z = (document.getElementById(`radar-target-z-${nickname}`) as HTMLElement).textContent;

          const text = `
Никнейм: ${nickname}
Статус: ${status}
UUID: ${uuid}
Координата X: ${x}
Координата Y: ${y}
Координата Z: ${z}
          `.trim();

          await navigator.clipboard.writeText(text);
        } catch (error) {
          log(`Ошибка копирования radar-данных: ${error}`, 'error');
        }
      });

      this.targets.set(nickname, { data: null, interval: null, chart: null });

      this.createTargetChart(nickname);
      this.setTargetUpdateInterval(nickname, this.updateFrequency);
    } catch (error) {
      log(`Ошибка инициализации radar-цели: ${error}`, 'error');
    }
  }

  private setTargetUpdateInterval(nickname: string, frequency: number) {
    let lx = '';
    let ly = '';
    let lz = '';

    const target = this.targets.get(nickname);

    if (target) {
      target.interval = setInterval(async () => {
        if (!this.active) {
          clearInterval(target.interval);
          return;
        }

        try {
          const data = await invoke('get_radar_data', { target: nickname }) as RadarInfo;

          if (data) {
            const x = data.x.toFixed(3);
            const y = data.y.toFixed(3);
            const z = data.z.toFixed(3);

            (document.getElementById(`radar-target-status-${nickname}`) as HTMLElement).innerText = data.status;
            (document.getElementById(`radar-target-uuid-${nickname}`) as HTMLElement).innerText = data.uuid.substr(0, 12) + '...';
            (document.getElementById(`radar-target-x-${nickname}`) as HTMLElement).innerText = x;
            (document.getElementById(`radar-target-y-${nickname}`) as HTMLElement).innerText = y;
            (document.getElementById(`radar-target-z-${nickname}`) as HTMLElement).innerText = z;

            if (target) {
              target.data = {
                fullUUID: data.uuid
              };
            }

            if (lx !== x || ly !== y || lz !== z) {
              const card = document.getElementById(`radar-target-${nickname}`) as HTMLElement;
              card.classList.add('glow');
              setTimeout(() => card.classList.remove('glow'), 300);
            }

            lx = data.x.toFixed(3);
            ly = data.y.toFixed(3);
            lz = data.z.toFixed(3);

            this.addRoutePointToChart(nickname, parseFloat(x), parseFloat(z), data.observer);

            if ((document.getElementById('radar-auto-save') as HTMLInputElement).checked) {
              const path = (document.getElementById('radar-path') as HTMLInputElement).value;
              const filename = (document.getElementById('radar-filename') as HTMLInputElement).value;

              if (await isAbsolute(path)) {
                await invoke('save_radar_data', {
                  target: nickname,
                  path: path,
                  filename: filename || 'radar_#t',
                  x: parseFloat(x),
                  y: parseFloat(y),
                  z: parseFloat(z)
                });
              }
            }
          }
        } catch (error) {
          log(`Ошибка обновления radar-цели ${nickname}: ${error}`, 'error');
        }
      }, frequency);
    }
  }

  private createTargetChart(nickname: string) {
    const ctx = document.getElementById(`radar-chart-${nickname}`) as HTMLCanvasElement;

    const chart = new Chart(ctx, {
      type: 'scatter', 
      data: {
        datasets: [
          {
            label: ` Маршрут ${nickname}`,
            data: [], 
            backgroundColor: '#39a10fff',
            borderColor: '#0f8f0bff', 
            showLine: true, 
            fill: false,
            pointRadius: 2,
            tension: 0,
            borderWidth: 1
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

    const target = this.targets.get(nickname);

    if (target) {
      target.chart = chart;
    }
  }

  private addRoutePointToChart(nickname: string, x: number, z: number, observer: { x: number, z: number }) {
    const target = this.targets.get(nickname);

    if (target) {
      target.chart.data.datasets[0].data.push({ x: x, y: z });
      target.chart.data.datasets[1].data.push({ x: observer.x, y: observer.z });

      if (target.chart.data.datasets[0].data.length > 30) target.chart.data.datasets[0].data.shift();
      if (target.chart.data.datasets[1].data.length > 1) target.chart.data.datasets[1].data.shift();

      const xMin = x - 200;
      const xMax = x + 200;
      const zMin = z - 200;
      const zMax = z + 200;
      
      target.chart.options.scales.x.min = xMin;
      target.chart.options.scales.x.max = xMax;
      target.chart.options.scales.y.min = zMin;
      target.chart.options.scales.y.max = zMax;

      target.chart.update();
    }
  }
}