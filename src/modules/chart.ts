import { invoke } from '@tauri-apps/api/core';
import { Chart } from 'chart.js';

import { log } from '../logger';
import { date } from '../helpers/date';


export class ChartManager {
  private active: boolean = false;

  private statusText: HTMLElement | null = null;

  private graphicActiveBots: HTMLElement | null = null;
  private graphicMemoryUsage: HTMLElement | null = null;

  private chartActiveBots: any = null;
  private chartMemoryUsage: any = null;

  private intervals: { activeBots: any, memoryUsage: any } = { activeBots: null, memoryUsage: null };

  public init(): void {
    try {
      this.graphicActiveBots = document.getElementById('graphic-active-bots-container');
      this.graphicMemoryUsage = document.getElementById('graphic-memory-usage-container');

      this.statusText = document.getElementById('graphic-status-text');
    } catch (error) {
      log(`Ошибка инициализации ChartManager: ${error}`, 'log-error');
    }
  }

  public enable(): void {
    try {
      this.active = true;

      this.createGraphicActiveBots();
      this.createGraphicMemoryUsage();

      this.graphicActiveBots!.style.display = 'flex';
      this.graphicMemoryUsage!.style.display = 'flex';

      this.statusText!.style.display = 'none';

      this.intervals.activeBots = setInterval(async () => {
        if (!this.active) {
          clearInterval(this.intervals.activeBots);
          return;
        }

        try {
          const data = await invoke('get_active_bots_count') as number;
          this.addGraphicDataActiveBots(data || 0);
        } catch (error) {
          log(`Ошибка ChartManager (active-bots-graphic): ${error}`, 'log-error');
        }
      }, 1800);

      this.intervals.memoryUsage = setInterval(async () => {
        if (!this.active) {
          clearInterval(this.intervals.memoryUsage);
          return;
        }

        try {
          const data = await invoke('get_memory_usage') as number;
          this.addGraphicDataMemoryUsage(parseFloat(data.toFixed(3)) || 0);
        } catch (error) {
          log(`Ошибка ChartManager (memory-usage-graphic): ${error}`, 'log-error');
        }
      }, 1800);
    } catch (error) {
      log(`Ошибка ChartManager: ${error}`, 'log-error');
    }
  }

  public disable(): void {
    this.active = false;

    if (this.chartActiveBots) {
      this.chartActiveBots.destroy();
      this.chartActiveBots = null;
    }

    if (this.chartMemoryUsage) {
      this.chartMemoryUsage.destroy();
      this.chartMemoryUsage = null;
    }
    
    if (this.intervals.activeBots) {
      clearInterval(this.intervals.activeBots);
      this.intervals.activeBots = null;
    }

    if (this.intervals.memoryUsage) {
      clearInterval(this.intervals.memoryUsage);
      this.intervals.memoryUsage = null;
    }

    this.graphicActiveBots!.style.display = 'none';
    this.graphicMemoryUsage!.style.display = 'none';

    this.statusText!.innerText = 'Данные отсутствуют';
    this.statusText!.style.display = 'flex';
  }

  private createGraphic(context: CanvasRenderingContext2D, label: string, title: string, maxY: number, tag: string): any {
    const initialLabels: string[] = [];
    const initialData: number[] = [];
    
    for (let i = 0; i < 31; i++) {
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
            borderWidth: 2,
            borderColor: '#6ff34ef1',
            backgroundColor: '#83ff3b23',
            tension: 0.1,
            pointStyle: 'line',
            pointRadius: 2
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
              callback: (value) => { return tag === 'memory-usage' ? value + ' MB' : value },
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

    return chart;
  }
  
  private createGraphicActiveBots(): void {
    const context = (document.getElementById('graphic-active-bots') as HTMLCanvasElement).getContext('2d');
    if (!context) return;
    this.chartActiveBots = this.createGraphic(context, 'Активные боты', 'График активных ботов', 500, 'active-bots');
  }

  private createGraphicMemoryUsage(): void {
    const context = (document.getElementById('graphic-memory-usage') as HTMLCanvasElement).getContext('2d');
    if (!context) return;
    this.chartMemoryUsage = this.createGraphic(context, 'Используемая память', 'График используемой памяти', 1024, 'memory-usage');
  }

  private addGraphicDataActiveBots(activeBotsQuantity: number): void {
    this.chartActiveBots.data.labels?.push(date());
    this.chartActiveBots.data.datasets[0].data.push(activeBotsQuantity);

    if (this.chartActiveBots.data.labels && this.chartActiveBots.data.labels.length > 31) {
      this.chartActiveBots.data.labels.shift();
      this.chartActiveBots.data.datasets[0].data.shift();
    }
      
    this.chartActiveBots.update(); 
  }

  private addGraphicDataMemoryUsage(memoryUsage: number): void {
    this.chartMemoryUsage.data.labels?.push(date());
    this.chartMemoryUsage.data.datasets[0].data.push(memoryUsage);

    if (this.chartMemoryUsage.data.labels && this.chartMemoryUsage.data.labels.length > 31) {
      this.chartMemoryUsage.data.labels.shift();
      this.chartMemoryUsage.data.datasets[0].data.shift();
    }
      
    this.chartMemoryUsage.update(); 
  }
}