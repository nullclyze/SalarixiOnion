import { generateNumber } from '../utils/generator.js';
export class AttackController {
    constructor(bot, object, tasks) {
        this.bot = bot;
        this.object = object;
        this.tasks = tasks;
        this.bot = bot;
        this.object = object;
        this.tasks = tasks;
    }
    async aiming() {
        while (this.tasks.attacking.status) {
            const target = this.bot.nearestEntity();
            if (target) {
                if (target.type === 'mob' || target.type === 'player') {
                    if (target) {
                        this.bot.lookAt(target.position.offset(0, target.height ?? 1.6, 0), false);
                    }
                    if (target && Math.random() > 0.5) {
                        this.bot.lookAt(target.position.offset(0, target.height ?? 1.6, 0), true);
                    }
                    await new Promise(resolve => setTimeout(resolve, generateNumber('float', 100, 300)));
                }
            }
        }
    }
    hit(options) {
        const target = this.bot.nearestEntity();
        if (!target)
            return;
        if ((options.type === 'all' && target.type === 'mob' || target.type === 'player') || options.type === target.type) {
            if (this.bot.entities[target.id]) {
                if (options.useNeatness) {
                    if (this.bot.entity.position.distanceTo(target.position) <= 3.5) {
                        this.bot.lookAt(target.position.offset(0, target.height ?? 1.6, 0), true);
                        this.bot.attack(target);
                    }
                }
                else {
                    this.bot.lookAt(target.position.offset(0, target.height ?? 1.6, 0), true);
                    this.bot.attack(target);
                }
            }
        }
    }
    async attack(state, options) {
        if (state === 'start') {
            if (this.tasks.attacking.status)
                return;
            if (!options)
                return;
            try {
                this.object.updateTask('attacking', true, 4.8);
                this.aiming();
                while (this.tasks.attacking.status) {
                    if (options.useLongDelays) {
                        await new Promise(resolve => setTimeout(resolve, generateNumber('float', 400, 800)));
                    }
                    else {
                        await new Promise(resolve => setTimeout(resolve, generateNumber('float', 100, 400)));
                    }
                    if (!this.tasks.attacking.status)
                        return;
                    this.hit(options);
                    await new Promise(resolve => setTimeout(resolve, generateNumber('float', 500, 700)));
                    if (options.useImprovedStrikes) {
                        const randomChance = Math.random();
                        if (randomChance >= 0.8) {
                            this.bot.setControlState('jump', true);
                            this.hit(options);
                            await new Promise(resolve => setTimeout(resolve, generateNumber('float', 100, 150)));
                            this.bot.setControlState('jump', false);
                        }
                        else if (randomChance < 0.8 && randomChance > 0.5) {
                            this.bot.setControlState('sneak', true);
                            this.hit(options);
                            await new Promise(resolve => setTimeout(resolve, generateNumber('float', 100, 150)));
                            this.bot.setControlState('sneak', false);
                        }
                    }
                }
            }
            finally {
                this.object.updateTask('attacking', false);
                this.bot.setControlState('jump', false);
                this.bot.setControlState('sneak', false);
            }
        }
        else if (state === 'stop') {
            if (!this.tasks.attacking.status)
                return;
            this.object.updateTask('attacking', false);
            this.bot.setControlState('jump', false);
            this.bot.setControlState('sneak', false);
        }
    }
}
/*
const { Generator } = require('../utils/generator.js');

const generator = new Generator();

// Контроллер для управления движением
class AttackController {
    constructor(
        bot,
        object,
        tasks
    ) {
        this.bot = bot;
        this.object = object;
        this.tasks = tasks;
    }

    async aiming() {
        while (this.tasks.attacking.status) {
            const target = this.bot.nearestEntity();

            if (target) {
                if (target.type === 'mob' || target.type === 'player') {
                    if (target) {
                        this.bot.lookAt(target.position.offset(0, target.height ?? 1.6, 0), false);
                    }

                    if (target && Math.random() > 0.5) {
                        this.bot.lookAt(target.position.offset(0, target.height ?? 1.6, 0), true);
                    }

                    await new Promise(resolve => setTimeout(resolve, generator.generateRandomNumberBetween(100, 300)));
                }
            }
        }
    }

    hit(options) {
        const target = this.bot.nearestEntity();

        if (!target) return;

        if ((options.type === 'all' && target.type === 'mob' || target.type === 'player') || options.type === target.type) {
            if (this.bot.entities[target.id]) {
                if (options.useNeatness) {
                    if (this.bot.entity.position.distanceTo(target.position) <= 3.5) {
                        this.bot.lookAt(target.position.offset(0, target.height ?? 1.6, 0), true);
                        this.bot.attack(target);
                    }
                } else {
                    this.bot.lookAt(target.position.offset(0, target.height ?? 1.6, 0), true);
                    this.bot.attack(target);
                }
            }
        }
    }

    async attack(state, options) {
        if (state === 'start') {
            if (this.tasks.attacking.status) return;

            if (!options) return;

            try {
                this.object.updateTask('attacking', true, 4.8);

                this.aiming();

                while (this.tasks.attacking.status) {
                    if (options.useLongDelays) {
                        await new Promise(resolve => setTimeout(resolve, generator.generateRandomNumberBetween(400, 800)));
                    } else {
                        await new Promise(resolve => setTimeout(resolve, generator.generateRandomNumberBetween(100, 400)));
                    }

                    if (!this.tasks.attacking.status) return;

                    this.hit(options);

                    await new Promise(resolve => setTimeout(resolve, generator.generateRandomNumberBetween(500, 700)));

                    if (options.useImprovedStrikes) {
                        const randomChance = Math.random();

                        if (randomChance >= 0.8) {
                            this.bot.setControlState('jump', true);
                            this.hit(options);
                            await new Promise(resolve => setTimeout(resolve, generator.generateRandomNumberBetween(100, 150)));
                            this.bot.setControlState('jump', false);
                        } else if (randomChance < 0.8 && randomChance > 0.5) {
                            this.bot.setControlState('sneak', true);
                            this.hit(options);
                            await new Promise(resolve => setTimeout(resolve, generator.generateRandomNumberBetween(100, 150)));
                            this.bot.setControlState('sneak', false);
                        }
                    }
                }
            } finally {
                this.object.updateTask('attacking', false);
                this.bot.setControlState('jump', false);
                this.bot.setControlState('sneak', false);
            }
        } else if (state === 'stop') {
            if (!this.tasks.attacking.status) return;

            this.object.updateTask('attacking', false);
            this.bot.setControlState('jump', false);
            this.bot.setControlState('sneak', false);
        }
    }
}

module.exports = AttackController;
*/ 
