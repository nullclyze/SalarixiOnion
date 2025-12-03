import mineflayer from 'mineflayer';

import { Bot } from '../base.js';

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

  private options: any = undefined;

	private enabled: boolean = false;
  private sentPacketsRate: number = 0;  
  private ignoredPacketsRate: number = 0;
	private packetBuffer: any[] = [];
	private originalWrite: any = undefined;

	public enableSmartSpoofing(options: any) {
    const interval = setInterval(() => {
      if (!this.enabled) {
        this.sentPacketsRate = 0;
        this.ignoredPacketsRate = 0;
        clearInterval(interval);
        return;
      }

      this.sentPacketsRate = 0;
      this.ignoredPacketsRate = 0;
    }, 3000);

		try {
			if (this.tasks.spoofing.status) return;

			this.object.updateTask('spoofing', true, 3.2);
			this.enabled = true;

      this.options = options;

      this.packetBuffer = [];
			
			if (!this.originalWrite) {
				this.originalWrite = this.bot._client.write;
			}
      
      this.bot._client.write = (packetName, packetData) => this.packetSpoofing(packetName, packetData);
    } catch {
			this.object.updateTask('spoofing', false);
			this.enabled = false;
			this.clearPacketBuffer();
    }
	}

	public disableSmartSpoofing() {
		this.object.updateTask('spoofing', false);
		this.enabled = false;
		this.clearPacketBuffer();
	}

  private packetFrequencyInspector() {
    if ((this.sentPacketsRate - this.ignoredPacketsRate) < (this.sentPacketsRate / 1.5)) {
      return false;
    } else {
      return true;
    }
  }

  private packetSpoofing(packetName: any, packetData: any) {
    if (this.enabled && this.object.profile.ping <= 450 && this.packetFrequencyInspector()) {
      if (packetName === 'keep_alive' || packetName === 'position' || packetName === 'position_look') {
        if (Math.random() > 0.6) {
          this.originalWrite.call(this.bot._client, packetName, packetData); return;
        } else if (this.options.useBuffering) {
          this.packetBuffering(packetName, packetData); return;
        } else {
          return;
        }
      } else {
        if (Math.random() > 0.1 && this.options.useBuffering) {
          this.packetBuffering(packetName, packetData); return;
        }
      }
    } else {
      this.clearPacketBuffer();
    }

		this.originalWrite.call(this.bot._client, packetName, packetData); 
  }

	private packetBuffering(packetName: any, packetData: any) {
    this.packetBuffer.push({ name: packetName, data: packetData });
    this.ignoredPacketsRate++;
  }

	private clearPacketBuffer() {
		if (this.packetBuffer.length === 0) return;

    for (const packet of this.packetBuffer) {
      this.originalWrite.call(this.bot._client, packet.name, packet.data);
    }

    this.packetBuffer = [];
	}
}