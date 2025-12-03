export class Spoofer {
    constructor(bot, object, tasks) {
        this.bot = bot;
        this.object = object;
        this.tasks = tasks;
        this.options = undefined;
        this.enabled = false;
        this.sentPacketsRate = 0;
        this.ignoredPacketsRate = 0;
        this.packetBuffer = [];
        this.originalWrite = undefined;
        this.bot = bot;
        this.object = object;
        this.tasks = tasks;
    }
    enableSmartSpoofing(options) {
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
            if (this.tasks.spoofing.status)
                return;
            this.object.updateTask('spoofing', true, 3.2);
            this.enabled = true;
            this.options = options;
            this.packetBuffer = [];
            if (!this.originalWrite) {
                this.originalWrite = this.bot._client.write;
            }
            this.bot._client.write = (packetName, packetData) => this.packetSpoofing(packetName, packetData);
        }
        catch {
            this.object.updateTask('spoofing', false);
            this.enabled = false;
            this.clearPacketBuffer();
        }
    }
    disableSmartSpoofing() {
        this.object.updateTask('spoofing', false);
        this.enabled = false;
        this.clearPacketBuffer();
    }
    packetFrequencyInspector() {
        if ((this.sentPacketsRate - this.ignoredPacketsRate) < (this.sentPacketsRate / 1.5)) {
            return false;
        }
        else {
            return true;
        }
    }
    packetSpoofing(packetName, packetData) {
        if (this.enabled && this.object.profile.ping <= 450 && this.packetFrequencyInspector()) {
            if (packetName === 'keep_alive' || packetName === 'position' || packetName === 'position_look') {
                if (Math.random() > 0.6) {
                    this.originalWrite.call(this.bot._client, packetName, packetData);
                    return;
                }
                else if (this.options.useBuffering) {
                    this.packetBuffering(packetName, packetData);
                    return;
                }
                else {
                    return;
                }
            }
            else {
                if (Math.random() > 0.1 && this.options.useBuffering) {
                    this.packetBuffering(packetName, packetData);
                    return;
                }
            }
        }
        else {
            this.clearPacketBuffer();
        }
        this.originalWrite.call(this.bot._client, packetName, packetData);
    }
    packetBuffering(packetName, packetData) {
        this.packetBuffer.push({ name: packetName, data: packetData });
        this.ignoredPacketsRate++;
    }
    clearPacketBuffer() {
        if (this.packetBuffer.length === 0)
            return;
        for (const packet of this.packetBuffer) {
            this.originalWrite.call(this.bot._client, packet.name, packet.data);
        }
        this.packetBuffer = [];
    }
}
