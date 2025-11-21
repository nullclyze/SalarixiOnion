import mineflayer from 'mineflayer';

import { Bot } from '../architecture.js';
import BotTasks from '../architecture.js';
import Generator from '../../tools/generator.js';

const generator = new Generator();

interface Options {
  useSharpness: boolean;
  useBuffering: boolean;
}

export default class Spoofer {
	constructor(
		public bot: mineflayer.Bot,
		public object: Bot,
		public tasks: BotTasks
	) {
		this.bot = bot;
		this.object = object;
		this.tasks = tasks;
	}

	private enabled: boolean = false;
	private packetBuffer: Array<{ name: string, data: any }> = [];
	private originalWrite: any = undefined;
	private latestDate: number = 0;

	public enableSmartSpoofing(options: Options) {
		try {
			if (this.tasks.spoofing.status) return;

			this.object.updateTask('spoofing', true, 3.2);
			this.enabled = true;

      this.packetBuffer = [];
      this.latestDate = Date.now();
			
			if (!this.originalWrite) {
				this.originalWrite = this.bot._client.write;
			}

      const ignoreDelay = generator.generateRandomNumberBetween(2000, 3500);

      let latestDate = 0;
      let pauseDelay = generator.generateRandomNumberBetween(2000, 4000);
        
      this.bot._client.write = (packetName, packetData) => {
        if (this.enabled && this.object.profile.ping < 650 && this.object.profile.load < 65) {
					if (this.packetBuffer.length > 20 && options.useBuffering) {
						this.clearPacketBuffer();
					} 

          if (Date.now() - this.latestDate < ignoreDelay) {
            latestDate = Date.now();

            if (packetName === 'keep_alive' || packetName === 'position' || packetName === 'position_look') {
              if (Math.random() > 0.6) {
                this.originalWrite.call(this.bot._client, packetName, packetData); return;
              } else if (options.useBuffering) {
                this.packetBuffering(packetName, packetData); return;
              } else {
                return;
              }
            } else {
              if (Math.random() < 0.9 && options.useBuffering) {
                this.packetBuffering(packetName, packetData); return;
              }
            }
          } else {
            if (Date.now() - latestDate > pauseDelay) {
              this.latestDate = Date.now();
            } else {
              this.clearPacketBuffer();
            }
          }
        } else {
          this.clearPacketBuffer();
        }

				this.originalWrite.call(this.bot._client, packetName, packetData); 
      }
    } catch (error) {
			this.object.updateTask('spoofing', false);
			this.enabled = false;
			this.clearPacketBuffer();
      console.log(`Ошибка: ${error}`);
    }
	}

	public disableSmartSpoofing() {
		this.object.updateTask('spoofing', false);
		this.enabled = false;
		this.clearPacketBuffer();
	}

	private packetBuffering(packetName: string, packetData: any) {
    this.packetBuffer.push({ name: packetName, data: packetData });
  }

	private clearPacketBuffer() {
		if (this.packetBuffer.length === 0) return;

		let count = 0;

    for (const packet of this.packetBuffer) {
      if (count % generator.generateRandomNumberBetween(5, 8) !== 0) {
        this.originalWrite.call(this.bot._client, packet.name, packet.data);
      } 

      count++;
    }

    this.packetBuffer = [];
	}
}