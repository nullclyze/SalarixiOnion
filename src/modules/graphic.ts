import { invoke } from '@tauri-apps/api/core';
import { Chart } from 'chart.js';

import { logger } from '../utils/logger';
import { date } from '../utils/date';

class Graphic {
  private active: boolean = false;

  private statusText: HTMLElement | null = null;

  private objects: Record<string, { chart: Chart | null, interval: any }> = {};

  private graphicActiveBots: HTMLElement | null = null;
  private graphicMemoryUsage: HTMLElement | null = null;

  /** Метод ининициализации функций, связанных с графиком. */
  public init(): void {
    try {
      this.graphicActiveBots = document.getElementById('graphic-active-bots-container');
      this.graphicMemoryUsage = document.getElementById('graphic-memory-usage-container');
      this.statusText = document.getElementById('graphic-status-text');
    } catch (error) {
      logger.log(`Ошибка инициализации графиков: ${error}`, 'error');
    }
  }

  /** Метод активации графиков. */
  public enable(): void {
    try {
      this.active = true;

      this.statusText!.style.display = 'none';
      
      this.createChartActiveBots();
      this.createChartMemoryUsage();
      
      this.setDisplay('flex');
    } catch (error) {
      logger.log(`Ошибка графика: ${error}`, 'error');
    }
  }

  /** Метод очистки и выключения графиков. */
  public disable(): void {
    this.active = false;

    this.destroyChart('active-bots');
    this.destroyChart('memory-usage');

    this.setDisplay('none');

    this.statusText!.innerText = 'Данные отсутствуют';
    this.statusText!.style.display = 'flex';
  }

  /** Метод установки отображения графиков. */
  private setDisplay(display: string): void {
    if (!this.graphicActiveBots || !this.graphicMemoryUsage) return;

    this.graphicActiveBots.style.display = display;
    this.graphicMemoryUsage.style.display = display;
  }

  /** Метод уничтожения чарта и его интервала. */
  private destroyChart(name: string): void {
    const object = this.objects[name];

    object.chart?.destroy();
    clearInterval(object.interval);

    object.chart = null;
    object.interval = null;
  }

  /** Метод создания чарта. */
  private createChart(context: CanvasRenderingContext2D, label: string, title: string, maxY: number, ps: string): Chart {
    const initialLabels: string[] = [];
    const initialData: number[] = [];
    
    for (let i = 0; i < 10; i++) {
      initialLabels.push(date());
      initialData.push(0);
    }

    const chart = new Chart(context, {
      type: 'line',
      data: {
        labels: initialLabels,
        datasets: [
          {
            label: ` ${label}`,
            data: initialData,
            fill: true,
            borderWidth: 1,
            borderColor: 'rgb(65, 226, 25)',
            backgroundColor: 'rgba(131, 255, 59, 0.05)',
            tension: 0,
            pointStyle: 'circle',
            pointRadius: 2,
            pointBackgroundColor: 'rgb(65, 226, 25)'
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
            max: maxY, 
            ticks: { 
              callback: (value) => value + ` ${ps}`,
              color: '#a3a3a3ff' 
            },
            border: { color: '#383838ab' },
            grid: { color: '#383838ab' }
          }
        },
        plugins: {
          title: {
            text: title,
            display: true,
            color: '#a2a2a2ff'
          },
          legend: {
            display: false,
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

    return chart;
  }
  
  /** Метод создания графика активных ботов. */
  private createChartActiveBots(): void {
    const name = 'active-bots';
    const context = (document.getElementById('graphic-active-bots') as HTMLCanvasElement).getContext('2d');

    if (!context) return;

    let interval = setInterval(async () => {
      if (!this.active) {
        clearInterval(this.objects[name].interval);
        return;
      }

      try {
        const data = await invoke('get_active_bots_count') as number;
        this.updateChart(name, data || 0);
      } catch (error) {
        logger.log(`Ошибка графика (${name}-graphic): ${error}`, 'error');
      }
    }, 2000);

    this.objects[name] = {
      chart: this.createChart(context, 'Активные боты', 'График активных ботов', 500, 'шт'),
      interval: interval
    };
  }

  /** Метод создания графика используемой памяти. */
  private createChartMemoryUsage(): void {
    const name = 'memory-usage';
    const context = (document.getElementById('graphic-memory-usage') as HTMLCanvasElement).getContext('2d');

    if (!context) return;

    let interval = setInterval(async () => {
      if (!this.active) {
        clearInterval(this.objects[name].interval);
        return;
      }

      try {
        const data = await invoke('get_memory_usage') as number;
        this.updateChart(name, parseFloat(data.toFixed(3)) || 0);
      } catch (error) {
        logger.log(`Ошибка графика (${name}-graphic): ${error}`, 'error');
      }
    }, 2000);

    this.objects[name] = {
      chart: this.createChart(context, 'Используется', 'График используемой памяти', 1024, 'MB'),
      interval: interval
    };
  }

  /** Метод обновления чарта. */
  private updateChart(name: string, data: number): void {
    const chart = this.objects[name].chart;

    if (!chart) return;
    
    chart.data.labels?.push(date());
    chart.data.datasets[0].data.push(data);

    if (chart.data.labels && chart.data.labels.length > 10) {
      chart.data.labels.shift();
      chart.data.datasets[0].data.shift();
    }

    chart.update(); 
  }
}

const graphic = new Graphic();

export { graphic }