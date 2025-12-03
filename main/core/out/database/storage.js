import fs from 'fs';
import { JSONFile } from 'lowdb/node';
import { Low } from 'lowdb';
class StorageManager {
    constructor() {
        this.worked = false;
        this.adapter = undefined;
        this.db = undefined;
        this.createStorageFile();
        const possiblePaths = [
            './salarixi.db.json',
            '../salarixi.db.json',
            '../../salarixi.db.json',
            './salarixi.database.json',
            '../../../salarixi.db.json',
            '../salarixi.database.json'
        ];
        for (const possiblePath of possiblePaths) {
            if (fs.existsSync(possiblePath)) {
                this.adapter = new JSONFile(possiblePath);
                this.worked = true;
                break;
            }
        }
        if (this.worked && this.adapter) {
            this.db = new Low(this.adapter, { bots: {} });
        }
    }
    createStorageFile() {
        const initialData = { bots: {} };
        fs.writeFileSync('./salarixi.db.json', JSON.stringify(initialData, null, 2));
    }
    async createBotStorage(nickname) {
        if (this.worked && this.db) {
            await this.db.read();
            this.db.data.bots[nickname] = {
                packetBuffer: []
            };
            await this.db.write();
        }
    }
    async write(nickname, key, data) {
        if (this.worked && this.db) {
            await this.db.read();
            const bot = this.db.data.bots[nickname];
            if (bot) {
                bot[key].push(data);
                await this.db.write();
            }
        }
    }
    async read(nickname) {
        if (this.worked && this.db) {
            const data = this.db.data.bots[nickname];
            if (data) {
                return { success: true, data: data };
            }
        }
        return { success: false, data: null };
    }
    async remove(nickname, key, id) {
        if (this.worked && this.db) {
            await this.db.read();
            const current = this.db.data.bots[nickname];
            if (current) {
                current[key].forEach((element, index) => {
                    if (element.id === id) {
                        current[key].splice(index, 1);
                    }
                });
            }
            await this.db.write();
        }
    }
}
export const storage = new StorageManager();
