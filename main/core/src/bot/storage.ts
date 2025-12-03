/*
import { activeBotsObjects } from './architecture.js';

interface BotScheme {
  packetBuffer: any[]
}

type Keys = 'packetBuffer';

export class SmartStorageManager {
  private performance = { level: 'maximum', storageTime: 6000 };
  private storage: Map<string, BotScheme> = new Map();

  constructor() {
    setInterval(() => {
      this.storage.forEach(element => {
        element.packetBuffer.forEach(packet => {
          if (Date.now() - packet.addedDate > this.performance.storageTime) {
            delete element.packetBuffer[packet];
          }
        });
      });
    }, 3000);
  }

  private loadInspector() {
    let generalLoad = 0;
    let quantity = 0;

    activeBotsObjects.forEach(element => {
      generalLoad += element.profile.load;
      quantity++;
    });

    const averageLoad = generalLoad / quantity;

    if (averageLoad < 30) {
      this.performance.level = 'maximum';
      this.performance.storageTime = 6000;
    } else if (averageLoad >= 30 && averageLoad < 50) {
      this.performance.level = 'average';
      this.performance.storageTime = 3500;
    } else {
      this.performance.level = 'low';
      this.performance.storageTime = 2000;
    }

    if (activeBotsObjects.size < 20) {
      this.performance.level = 'maximum';
      this.performance.storageTime = 6000;
    } else if (activeBotsObjects.size >= 20 && activeBotsObjects.size < 50) {
      this.performance.level = 'average';
      this.performance.storageTime = 3500;
    } else {
      this.performance.level = 'low';
      this.performance.storageTime = 2000;
    }
  }

  public async replenishStorage(nickname: string, key: Keys, data: any) {
    this.loadInspector();

    const
  }
}
  */