import mineflayer from 'mineflayer';

import { Bot } from '../base.js';
import { generateNumber } from '../utils/generator.js';

export class Spoofer {
	constructor(
		public bot: mineflayer.Bot,
		public object: Bot,
		public tasks: any
	) {
		this.bot = bot;
		this.object = object;
		this.tasks = tasks;
	}

  public enabled: boolean = false;

  private status: 'active' | 'sleep' = 'sleep';
  private sentPacketsRate: number = 0;  
  private bufferedPacketsRate: number = 0;
	private packetBuffer: Map<number, any> = new Map();
	private originalWrite: any = undefined;

	public enableSmartSpoofing(options: any) {
    const packetRateInterval = setInterval(() => {
      if (!this.enabled) {
        this.sentPacketsRate = 0;
        this.bufferedPacketsRate = 0;
        clearInterval(packetRateInterval);
        return;
      }

      this.sentPacketsRate = 0;
      this.bufferedPacketsRate = 0;
    }, 3000);

    const packetInspectorInterval = setInterval(async () => {
      if (!this.enabled) {
        clearInterval(packetInspectorInterval);
        return;
      }

      if ((this.sentPacketsRate - this.bufferedPacketsRate) < (this.sentPacketsRate / 1.2) || this.bot.player.ping > 300) {
        this.status = 'sleep';
      } else {
        this.status = 'active';
      }
    }, 1500);

		try {
			if (this.tasks.spoofing.status) return;

			this.object.updateTask('spoofing', true, 3.2);
			this.enabled = true;
      this.status = 'active';

      this.packetBuffer.clear();
			
			if (!this.originalWrite) {
				this.originalWrite = this.bot._client.write;
			}
      
      this.bot._client.write = (packetName, packetData) => {
        if (this.enabled && this.status) {         
          if (packetName === 'keep_alive' || packetName === 'position' || packetName === 'position_look' && Math.random() > 0.6) {
            if (options.useBuffering) {
              this.packetBuffering(packetName, packetData);
            }

            return;
          }

          if (Math.random() > 0.9) {
            this.clearPacketBuffer();
          }

          if (Math.random() > 0.4) {
            this.originalWrite.call(this.bot._client, packetName, packetData);
          }
        } else {
          this.clearPacketBuffer();
          this.originalWrite.call(this.bot._client, packetName, packetData);
        }
      }
    } catch {
      this.disableSmartSpoofing();
    }
	}

	public disableSmartSpoofing() {
		this.object.updateTask('spoofing', false);
		this.enabled = false;
    this.status = 'sleep';
		this.clearPacketBuffer();
	}

	private packetBuffering(packetName: any, packetData: any) {
    const id = Date.now() + generateNumber('int', 10000, 99999);

    this.packetBuffer.set(id, { name: packetName, data: packetData });
    this.bufferedPacketsRate++;
  }

	private clearPacketBuffer() {
		if (this.packetBuffer.size === 0) return;

    for (const packet of this.packetBuffer) {
      this.originalWrite.call(this.bot._client, packet[1].name, packet[1].data);
      this.packetBuffer.delete(packet[0]);
    }

    this.packetBuffer.clear();
	}
}