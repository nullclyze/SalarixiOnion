import { generateNumber } from '../utils/generator.js';
export class MovementController {
    constructor(bot, object, tasks) {
        this.bot = bot;
        this.object = object;
        this.tasks = tasks;
        this.bot = bot;
        this.object = object;
        this.tasks = tasks;
    }
    async sleep(delay) {
        await new Promise(resolve => setTimeout(resolve, delay));
    }
    async movement({ state, direction, options }) {
        let name;
        switch (direction) {
            case 'forward':
                name = 'movementForward';
                break;
            case 'back':
                name = 'movementBack';
                break;
            case 'left':
                name = 'movementLeft';
                break;
            case 'right':
                name = 'movementRight';
                break;
        }
        if (state === 'start') {
            if (!options)
                return;
            switch (direction) {
                case 'forward':
                    this.object.updateTask('movementForward', true, 1.8);
                    break;
                case 'back':
                    this.object.updateTask('movementBack', true, 1.8);
                    break;
                case 'left':
                    this.object.updateTask('movementLeft', true, 1.8);
                    break;
                case 'right':
                    this.object.updateTask('movementRight', true, 1.8);
                    break;
            }
            try {
                if (options.useImpulsiveness) {
                    const generateDelay = () => {
                        const randomChance = Math.random();
                        if (options.useSync) {
                            if (randomChance >= 0.5) {
                                return 600;
                            }
                            else {
                                return 900;
                            }
                        }
                        else {
                            if (randomChance >= 0.5) {
                                return generateNumber('float', 400, 1000);
                            }
                            else {
                                return generateNumber('float', 1000, 2000);
                            }
                        }
                    };
                    while (this.tasks[name].status) {
                        await this.sleep(generateDelay());
                        if (!this.tasks[name].status)
                            break;
                        this.bot.setControlState(direction, true);
                        await this.sleep(generateDelay());
                        this.bot.setControlState(direction, false);
                        await this.sleep(generateDelay());
                        if (Math.random() >= 0.58) {
                            if (!this.tasks[name].status)
                                break;
                            this.bot.setControlState(direction, true);
                            await this.sleep(generateDelay());
                            this.bot.setControlState(direction, false);
                            await this.sleep(generateDelay());
                        }
                    }
                }
                else {
                    this.bot.setControlState(direction, true);
                }
            }
            catch {
                this.object.updateTask(name, false);
                this.bot.setControlState(direction, false);
            }
        }
        else if (state === 'stop') {
            if (!this.tasks[name].status)
                return;
            this.object.updateTask(name, false);
            this.bot.setControlState(direction, false);
        }
    }
}
/*
const { Generator } = require('../utils/generator.js');

const generator = new Generator();

// Контроллер для управления движением
class MovementController {
    constructor(
        bot,
        object,
        tasks
    ) {
        this.bot = bot;
        this.object = object;
        this.tasks = tasks;
    }

    async sleep(delay) {
        await new Promise(resolve => setTimeout(resolve, delay));
    }

    async movement({ state, direction, options }) {
        let name;

        switch (direction) {
            case 'forward':
                name = 'movementForward'; break;
            case 'back':
                name = 'movementBack'; break;
            case 'left':
                name = 'movementLeft'; break;
            case 'right':
                name = 'movementRight'; break;
        }

        if (state === 'start') {
            if (!options) return;

            switch (direction) {
                case 'forward':
                    this.object.updateTask('movementForward', true, 1.8); break;
                case 'back':
                    this.object.updateTask('movementBack', true, 1.8); break;
                case 'left':
                    this.object.updateTask('movementLeft', true, 1.8); break;
                case 'right':
                    this.object.updateTask('movementRight', true, 1.8); break;
            }

            try {
                if (options.useImpulsiveness) {
                    const generateDelay = () => {
                        const randomChance = Math.random();

                        if (options.useSync) {
                            if (randomChance >= 0.5) {
                                return 600;
                            } else {
                                return 900;
                            }
                        } else {
                            if (randomChance >= 0.5) {
                                return generator.generateRandomNumberBetween(400, 1000);
                            } else {
                                return generator.generateRandomNumberBetween(1000, 2000);
                            }
                        }
                    }

                    while (this.tasks[name].status) {
                        await this.sleep(generateDelay());

                        if (!this.tasks[name].status) break;

                        this.bot.setControlState(direction, true);
                        await this.sleep(generateDelay());
                        this.bot.setControlState(direction, false);
                        await this.sleep(generateDelay());

                        if (Math.random() >= 0.58) {
                            if (!this.tasks[name].status) break;

                            this.bot.setControlState(direction, true);
                            await this.sleep(generateDelay());
                            this.bot.setControlState(direction, false);
                            await this.sleep(generateDelay());
                        }
                    }
                } else {
                    this.bot.setControlState(direction, true);
                }
            } catch {
                this.object.updateTask(name, false);
                this.bot.setControlState(direction, false);
            }
        } else if (state === 'stop') {
            if (!this.tasks[name].status) return;

            this.object.updateTask(name, false);
            this.bot.setControlState(direction, false);
        }
    }
}

module.exports = MovementController;
*/ 
