import { Generator } from '../utils/generator.js';
const generator = new Generator();
let sync = {
    jumping: { delay: 0, use: 0 },
    shifting: { delay: 0, use: 0 },
    waving: { delay: 0, use: 0 },
    spinning: { delay: 0, use: 0 }
};
export class ActionController {
    constructor(bot, object, tasks) {
        this.bot = bot;
        this.object = object;
        this.tasks = tasks;
        this.primaryInformation = { pitch: 0, yaw: 0 };
        this.bot = bot;
        this.object = object;
        this.tasks = tasks;
    }
    async sleep(delay) {
        await new Promise(resolve => setTimeout(resolve, delay));
    }
    generateDelay(useSync, sector, min, max) {
        if (useSync) {
            if (sync[sector].use >= this.object.quantity) {
                sync[sector].delay = 0;
                sync[sector].use = 0;
            }
            if (!sync[sector].delay) {
                sync[sector].delay = generator.generateRandomNumberBetween(min, max);
            }
            sync[sector].use++;
            return sync[sector].delay;
        }
        else {
            return generator.generateRandomNumberBetween(min, max);
        }
    }
    async jumping(state, options) {
        if (state === 'start') {
            if (!this.bot)
                return;
            if (this.tasks.jumping.status)
                return;
            if (!options)
                return;
            try {
                this.object.updateTask('jumping', true, 2.8);
                if (options.useImpulsiveness) {
                    while (this.tasks.jumping.status) {
                        await this.sleep(this.generateDelay(options.useSync, 'jumping', 1000, 1800));
                        if (this.tasks.jumping.status) {
                            this.bot.setControlState('jump', true);
                            await this.sleep(this.generateDelay(options.useSync, 'jumping', options.minDelay, options.maxDelay));
                            this.bot.setControlState('jump', false);
                            if (options.useAntiDetect) {
                                if (Math.random() > 0.5) {
                                    this.bot.setControlState('jump', true);
                                    await this.sleep(this.generateDelay(options.useSync, 'jumping', options.minDelay, options.maxDelay));
                                    this.bot.setControlState('jump', false);
                                }
                                else {
                                    this.bot.setControlState('jump', true);
                                    await this.sleep(this.generateDelay(options.useSync, 'jumping', options.minDelay, options.maxDelay));
                                    this.bot.setControlState('jump', false);
                                    this.bot.setControlState('jump', true);
                                    await this.sleep(this.generateDelay(options.useSync, 'jumping', options.minDelay, options.maxDelay));
                                    this.bot.setControlState('jump', false);
                                }
                            }
                        }
                    }
                }
                else {
                    await this.sleep(this.generateDelay(options.useSync, 'jumping', 1000, 1800));
                    this.bot.setControlState('jump', true);
                }
            }
            catch {
                this.object.updateTask('jumping', false);
            }
        }
        else if (state === 'stop') {
            if (!this.tasks.jumping.status)
                return;
            this.bot.setControlState('jump', false);
            this.object.updateTask('jumping', false);
        }
    }
    async shifting(state, options) {
        if (state === 'start') {
            if (!this.bot)
                return;
            if (this.tasks.shifting.status)
                return;
            if (!options)
                return;
            try {
                this.object.updateTask('shifting', true, 2.8);
                if (options.useImpulsiveness) {
                    while (this.tasks.shifting.status) {
                        await this.sleep(this.generateDelay(options.useSync, 'shifting', 1000, 1800));
                        if (this.tasks.shifting.status) {
                            this.bot.setControlState('sneak', true);
                            await this.sleep(this.generateDelay(options.useSync, 'shifting', options.minDelay, options.maxDelay));
                            this.bot.setControlState('sneak', false);
                            if (options.useAntiDetect) {
                                if (Math.random() > 0.5) {
                                    this.bot.setControlState('sneak', true);
                                    await this.sleep(this.generateDelay(options.useSync, 'shifting', options.minDelay, options.maxDelay));
                                    this.bot.setControlState('sneak', false);
                                }
                                else {
                                    this.bot.setControlState('sneak', true);
                                    await this.sleep(this.generateDelay(options.useSync, 'shifting', options.minDelay, options.maxDelay));
                                    this.bot.setControlState('sneak', false);
                                    this.bot.setControlState('sneak', true);
                                    await this.sleep(this.generateDelay(options.useSync, 'shifting', options.minDelay, options.maxDelay));
                                    this.bot.setControlState('sneak', false);
                                }
                            }
                        }
                    }
                }
                else {
                    await this.sleep(this.generateDelay(options.useSync, 'jumping', 1000, 1800));
                    this.bot.setControlState('sneak', true);
                }
            }
            catch {
                this.object.updateTask('shifting', false);
            }
        }
        else if (state === 'stop') {
            if (!this.tasks.shifting.status)
                return;
            this.bot.setControlState('sneak', false);
            this.object.updateTask('shifting', false);
        }
    }
    async waving(state, options) {
        if (state === 'start') {
            if (!this.bot)
                return;
            if (this.tasks.waving.status)
                return;
            if (!options)
                return;
            try {
                this.object.updateTask('waving', true, 2.2);
                if (options.useRandomizer) {
                    while (this.tasks.waving.status) {
                        await this.sleep(this.generateDelay(options.useSync, 'waving', 1000, 3000));
                        this.bot.swingArm('right');
                        if (Math.random() > 0.5) {
                            await this.sleep(this.generateDelay(options.useSync, 'waving', 300, 1000));
                            this.bot.swingArm('right');
                        }
                    }
                }
                else {
                    while (this.tasks.waving.status) {
                        await this.sleep(this.generateDelay(options.useSync, 'jumping', 300, 300));
                        this.bot.swingArm('right');
                    }
                }
            }
            catch {
                this.object.updateTask('waving', false);
            }
        }
        else if (state === 'stop') {
            if (!this.tasks.waving.status)
                return;
            this.object.updateTask('waving', false);
        }
    }
    async spinning(state, options) {
        if (state === 'start') {
            if (!this.bot)
                return;
            if (this.tasks.spinning.status)
                return;
            if (!options)
                return;
            try {
                this.object.updateTask('spinning', true, 2.8);
                this.primaryInformation.pitch = this.bot.entity.pitch;
                this.primaryInformation.yaw = this.bot.entity.yaw;
                if (options.useRealism) {
                    while (this.tasks.spinning.status) {
                        await this.sleep(this.generateDelay(options.useSync, 'spinning', 300, 500));
                        this.bot.entity.yaw += generator.chooseRandomValueFromArray([0.2, 0.4, 0.5, 0.6, 0.8, 0.9]);
                        this.bot.entity.pitch = (generator.generateRandomNumberBetween(-5, 5) + Math.random() * 2) / 6;
                        this.bot.look(this.bot.entity.yaw, this.bot.entity.pitch, true);
                    }
                }
                else {
                    while (this.tasks.spinning.status) {
                        await this.sleep(this.generateDelay(options.useSync, 'spinning', 130, 200));
                        this.bot.entity.yaw += generator.chooseRandomValueFromArray([0.2, 0.4, 0.5, 0.6, 0.8]);
                        this.bot.look(this.bot.entity.yaw, this.bot.entity.pitch, true);
                    }
                }
            }
            catch {
                this.object.updateTask('spinning', false);
            }
            finally {
                this.bot.look(this.primaryInformation.yaw, this.primaryInformation.pitch, false);
            }
        }
        else if (state === 'stop') {
            if (!this.tasks.spinning.status)
                return;
            this.object.updateTask('spinning', false);
        }
    }
}
/*
 { Generator } = require('../utils/generator.js');
const { activeBotsObjects } = require('../architecture.js');

const generator = new Generator();

let sync = {
    jumping: { delay: 0, use: 0 },
    shifting: { delay: 0, use: 0 },
    waving: { delay: 0, use: 0 },
    spinning: { delay: 0, use: 0 }
};

class ActionController {
    primaryInformation = { pitch: 0, yaw: 0 };

    async sleep(delay) {
        await new Promise(resolve => setTimeout(resolve, delay));
    }

    generateDelay(useSync, sector, min, max) {
        if (useSync) {
            if (sync[sector].use >= this.object.quantity) {
                sync[sector].delay = 0;
                sync[sector].use = 0;
            }
                                
            if (!sync[sector].delay) {
                sync[sector].delay = generator.generateRandomNumberBetween(min, max);
            }

            sync[sector].use++;

            return sync[sector].delay;
        } else {
            return generator.generateRandomNumberBetween(min, max);
        }
    }

    async jumping(state, options) {
        if (state === 'start') {
            if (!this.bot) return;
            if (this.tasks.jumping.status) return;

            if (!options) return;

            try {
                this.object.updateTask('jumping', true, 2.8);

                if (options.useImpulsiveness) {
                    while (this.tasks.jumping.status) {
                        await this.sleep(this.generateDelay(options.useSync, 'jumping', 1000, 1800));

                        if (this.tasks.jumping.status) {
                            this.bot.setControlState('jump', true);
                            await this.sleep(this.generateDelay(options.useSync, 'jumping', options.minDelay, options.maxDelay));
                            this.bot.setControlState('jump', false);

                            if (options.useAntiDetect) {
                                if (Math.random() > 0.5) {
                                    this.bot.setControlState('jump', true);
                                    await this.sleep(this.generateDelay(options.useSync, 'jumping', options.minDelay, options.maxDelay));
                                    this.bot.setControlState('jump', false);
                                } else {
                                    this.bot.setControlState('jump', true);
                                    await this.sleep(this.generateDelay(options.useSync, 'jumping', options.minDelay, options.maxDelay));
                                    this.bot.setControlState('jump', false);

                                    this.bot.setControlState('jump', true);
                                    await this.sleep(this.generateDelay(options.useSync, 'jumping', options.minDelay, options.maxDelay));
                                    this.bot.setControlState('jump', false);
                                }
                            }
                        }
                    }
                } else {
                    await this.sleep(this.generateDelay(options.useSync, 'jumping', 1000, 1800));
                    this.bot.setControlState('jump', true);
                }
            } catch {
                this.object.updateTask('jumping', false);
            }
        } else if (state === 'stop') {
            if (!this.tasks.jumping.status) return;

            this.bot.setControlState('jump', false);
            this.object.updateTask('jumping', false);
        }
    }

    async shifting(state, options) {
        if (state === 'start') {
            if (!this.bot) return;
            if (this.tasks.shifting.status) return;

            if (!options) return;

            try {
                this.object.updateTask('shifting', true, 2.8);

                if (options.useImpulsiveness) {
                    while (this.tasks.shifting.status) {
                        await this.sleep(this.generateDelay(options.useSync, 'shifting', 1000, 1800));

                        if (this.tasks.shifting.status) {
                            this.bot.setControlState('sneak', true);
                            await this.sleep(this.generateDelay(options.useSync, 'shifting', options.minDelay, options.maxDelay));
                            this.bot.setControlState('sneak', false);

                            if (options.useAntiDetect) {
                                if (Math.random() > 0.5) {
                                    this.bot.setControlState('sneak', true);
                                    await this.sleep(this.generateDelay(options.useSync, 'shifting', options.minDelay, options.maxDelay));
                                    this.bot.setControlState('sneak', false);
                                } else {
                                    this.bot.setControlState('sneak', true);
                                    await this.sleep(this.generateDelay(options.useSync, 'shifting', options.minDelay, options.maxDelay));
                                    this.bot.setControlState('sneak', false);

                                    this.bot.setControlState('sneak', true);
                                    await this.sleep(this.generateDelay(options.useSync, 'shifting', options.minDelay, options.maxDelay));
                                    this.bot.setControlState('sneak', false);
                                }
                            }
                        }
                    }
                } else {
                    await this.sleep(this.generateDelay(options.useSync, 'jumping', 1000, 1800));
                    this.bot.setControlState('sneak', true);
                }
            } catch {
                this.object.updateTask('shifting', false);
            }
        } else if (state === 'stop') {
            if (!this.tasks.shifting.status) return;

            this.bot.setControlState('sneak', false);
            this.object.updateTask('shifting', false);
        }
    }

    async waving(state, options) {
        if (state === 'start') {
            if (!this.bot) return;
            if (this.tasks.waving.status) return;

            if (!options) return;

            try {
                this.object.updateTask('waving', true, 2.2);

                if (options.useRandomizer) {
                    while (this.tasks.waving.status) {
                        await this.sleep(this.generateDelay(options.useSync, 'waving', 1000, 3000));

                        this.bot.swingArm('right');

                        if (Math.random() > 0.5) {
                            await this.sleep(this.generateDelay(options.useSync, 'waving', 300, 1000));
                            this.bot.swingArm('right');
                        }
                    }
                } else {
                    while (this.tasks.waving.status) {
                        await this.sleep(this.generateDelay(options.useSync, 'jumping', 300, 300));
                        this.bot.swingArm('right');
                    }
                }
            } catch {
                this.object.updateTask('waving', false);
            }
        } else if (state === 'stop') {
            if (!this.tasks.waving.status) return;

            this.object.updateTask('waving', false);
        }
    }

    async spinning(state, options) {
        if (state === 'start') {
            if (!this.bot) return;
            if (this.tasks.spinning.status) return;

            if (!options) return;

            try {
                this.object.updateTask('spinning', true, 2.8);

                this.primaryInformation.pitch = this.bot.entity.pitch;
                this.primaryInformation.yaw = this.bot.entity.yaw;

                if (options.useRealism) {
                    while (this.tasks.spinning.status) {
                        await this.sleep(this.generateDelay(options.useSync, 'spinning', 300, 500));

                        this.bot.entity.yaw += generator.chooseRandomValueFromArray([0.2, 0.4, 0.5, 0.6, 0.8, 0.9]);

                        this.bot.entity.pitch = (generator.generateRandomNumberBetween(-5, 5) + Math.random() * 2) / 6;

                        this.bot.look(this.bot.entity.yaw, this.bot.entity.pitch, true);
                    }
                } else {
                    while (this.tasks.spinning.status) {
                        await this.sleep(this.generateDelay(options.useSync, 'spinning', 130, 200));

                        this.bot.entity.yaw += generator.chooseRandomValueFromArray([0.2, 0.4, 0.5, 0.6, 0.8]);

                        this.bot.look(this.bot.entity.yaw, this.bot.entity.pitch, true);
                    }
                }
            } catch {
                this.object.updateTask('spinning', false);
            } finally {
                this.bot.look(this.primaryInformation.yaw, this.primaryInformation.pitch, false);
            }
        } else if (state === 'stop') {
            if (!this.tasks.spinning.status) return;

            this.object.updateTask('spinning', false);
        }
    }
}

module.exports = ActionController;
*/ 
