import date from './tools/date';
import { Chart, registerables } from 'chart.js';

Chart.register(...registerables);

// Класс для отрисовки line-графика в реальном времени
class LineGraphicManager {
  private chartActiveBots: any = undefined;
  private chartAverageLoad: any = undefined;

  public async createGraphicActiveBots(): Promise<void> {
    const context = (document.getElementById('line-graphic-active-bots') as HTMLCanvasElement).getContext('2d');

    if (!context) return;

    const initialLabels: string[] = [];
    const initialDataActive: number[] = [];
    
    for (let i = 0; i < 28; i++) {
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
            borderColor: '#3ba7ffff',
            backgroundColor: '#3ba7ff46',
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
            border: { color: '#50505086' },
            grid: { color: '#50505086' }
          },
          y: {
            min: 0,
            max: 100, 
            ticks: { color: '#a3a3a3ff' },
            border: { color: '#50505086' },
            grid: { color: '#50505086' }
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

  public async createGraphicAverageLoad(): Promise<void> {
    const context = (document.getElementById('line-graphic-average-load') as HTMLCanvasElement).getContext('2d');

    if (!context) return;

    const initialLabels: string[] = [];
    const initialDataLoad: number[] = [];
    
    for (let i = 0; i < 28; i++) {
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
            borderColor: '#3ba7ffff',
            backgroundColor: '#3ba7ff46',
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
            border: { color: '#50505086' },
            grid: { color: '#50505086' }
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
            border: { color: '#50505086' },
            grid: { color: '#50505086' }
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

  public async addGraphicDataActiveBots(activeBotsQuantity: number): Promise<void> {
    this.chartActiveBots.data.labels?.push(date());
    this.chartActiveBots.data.datasets[0].data.push(activeBotsQuantity);

    if (this.chartActiveBots.data.labels && this.chartActiveBots.data.labels.length > 28) {
      this.chartActiveBots.data.labels.shift();
      this.chartActiveBots.data.datasets[0].data.shift();
    }
      
    this.chartActiveBots.update(); 
  }

  public async addGraphicDataAverageLoad(averageLoad: number): Promise<void> {
    this.chartAverageLoad.data.labels?.push(date());
    this.chartAverageLoad.data.datasets[0].data.push(averageLoad);

    if (this.chartAverageLoad.data.labels && this.chartAverageLoad.data.labels.length > 28) {
      this.chartAverageLoad.data.labels.shift();
      this.chartAverageLoad.data.datasets[0].data.shift();
    }
      
    this.chartAverageLoad.update(); 
  }
}

export default LineGraphicManager;