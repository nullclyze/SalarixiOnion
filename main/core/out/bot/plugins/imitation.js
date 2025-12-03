import { generateNumber, chooseRandomElementFromArray } from '../utils/generator.js';
let LATEST_DATA = {
    imitation: {
        hybrid: {
            chains: [],
            multitaskingAlgorithms: []
        },
        walking: {
            chains: [],
            multitaskingAlgorithms: []
        }
    }
};
export class ImitationController {
    constructor(bot, object, tasks) {
        this.bot = bot;
        this.object = object;
        this.tasks = tasks;
        this.bot = bot;
        this.object = object;
        this.tasks = tasks;
    }
    async generateHybridChain(multitasking) {
        let chain = '';
        const actions = [
            'sneak',
            'jump',
            'forward',
            'back',
            'left',
            'right'
        ];
        for (let i = 0; i < 15; i++) {
            if (i !== 0)
                chain += '&';
            if (multitasking) {
                if (Math.random() >= 0.8) {
                    chain += chooseRandomElementFromArray(actions);
                }
                else {
                    let string = '';
                    while (true) {
                        let generatedActions = [];
                        for (let k = 0; k < generateNumber('int', 2, 6); k++) {
                            let generate = '';
                            while (true) {
                                generate = chooseRandomElementFromArray(actions);
                                let valid = true;
                                generatedActions.forEach((element) => {
                                    if (generate === element) {
                                        valid = false;
                                    }
                                });
                                if (valid)
                                    break;
                            }
                            generatedActions.push(generate);
                            if (k !== 0)
                                string += '-';
                            string += generate;
                        }
                        generatedActions.length = 0;
                        let valid = true;
                        LATEST_DATA.imitation.hybrid.multitaskingAlgorithms.forEach(element => {
                            if (string === element) {
                                valid = false;
                            }
                        });
                        if (valid) {
                            LATEST_DATA.imitation.hybrid.multitaskingAlgorithms.push(string);
                            if (LATEST_DATA.imitation.hybrid.multitaskingAlgorithms.length > 8) {
                                LATEST_DATA.imitation.hybrid.multitaskingAlgorithms.length = 0;
                            }
                            chain += string;
                            break;
                        }
                    }
                }
            }
            else {
                chain += chooseRandomElementFromArray(actions);
            }
            if (Math.random() <= 0.1) {
                chain += `:${chooseRandomElementFromArray(['0', '1', '2'])}:${generateNumber('float', 200, 4000)}`;
            }
            else {
                chain += `:${chooseRandomElementFromArray(['0', '1', '2'])}:${generateNumber('float', 100, 200)}`;
            }
        }
        return chain;
    }
    async generateWalkingChain(multitasking) {
        let chain = '';
        const actions = [
            'forward',
            'back',
            'left',
            'right'
        ];
        for (let i = 0; i < 15; i++) {
            if (i !== 0)
                chain += '&';
            if (multitasking) {
                if (Math.random() >= 0.8) {
                    chain += chooseRandomElementFromArray(actions);
                }
                else {
                    let string = '';
                    while (true) {
                        let generatedActions = [];
                        for (let k = 0; k < generateNumber('int', 2, 4); k++) {
                            let generate = '';
                            while (true) {
                                generate = chooseRandomElementFromArray(actions);
                                let valid = true;
                                generatedActions.forEach(element => generate === element ? valid = false : valid = valid);
                                if (valid)
                                    break;
                            }
                            generatedActions.push(generate);
                            if (k !== 0)
                                string += '-';
                            string += generate;
                        }
                        generatedActions.length = 0;
                        let valid = true;
                        LATEST_DATA.imitation.walking.multitaskingAlgorithms.forEach(element => string === element ? valid = false : valid = valid);
                        if (valid) {
                            LATEST_DATA.imitation.walking.multitaskingAlgorithms.push(string);
                            if (LATEST_DATA.imitation.walking.multitaskingAlgorithms.length > 8) {
                                LATEST_DATA.imitation.walking.multitaskingAlgorithms.length = 0;
                            }
                            chain += string;
                            break;
                        }
                    }
                }
            }
            else {
                chain += chooseRandomElementFromArray(actions);
            }
            if (Math.random() <= 0.1) {
                chain += `:${generateNumber('float', 2000, 4000)}`;
            }
            else {
                chain += `:${generateNumber('float', 500, 2300)}`;
            }
        }
        return chain;
    }
    async looking(useSmoothness, useLongDelays) {
        if (!this.bot)
            return;
        if (this.tasks.looking.status)
            return;
        try {
            this.object.updateTask('looking', true, 4.5);
            while (this.tasks.looking.status) {
                if (Math.random() >= 0.8) {
                    const players = Object.values(this.bot.players);
                    let closestPlayer = null;
                    let closestDistance = 24;
                    for (const player of players) {
                        if (player.entity) {
                            const distance = this.bot.entity.position.distanceTo(player.entity.position);
                            if (distance < closestDistance) {
                                closestDistance = distance;
                                closestPlayer = player;
                            }
                        }
                    }
                    if (closestPlayer) {
                        const target = closestPlayer.entity;
                        this.bot.lookAt(target.position.offset(chooseRandomElementFromArray([Math.random() / 3, -(Math.random() / 3)]), target.height, chooseRandomElementFromArray([Math.random() / 3, -(Math.random() / 3)])), !useSmoothness);
                    }
                }
                else {
                    const yaw = Math.random();
                    const pitch = Math.random() / 3;
                    if (Math.random() >= 0.5) {
                        this.bot.look(yaw, pitch, !useSmoothness);
                    }
                    else {
                        this.bot.look(-yaw, -pitch, !useSmoothness);
                    }
                }
                if (useLongDelays) {
                    await new Promise(resolve => setTimeout(resolve, generateNumber('float', 1000, 2200)));
                }
                else {
                    await new Promise(resolve => setTimeout(resolve, generateNumber('float', 250, 1000)));
                }
            }
        }
        catch {
            this.object.updateTask('looking', false);
        }
        finally {
            this.object.updateTask('looking', false);
        }
    }
    async waving(useLongDelays) {
        if (!this.bot)
            return;
        if (this.tasks.waving.status)
            return;
        try {
            this.object.updateTask('waving', true, 3.2);
            while (this.tasks.waving.status) {
                const randomChance = Math.random();
                if (randomChance >= 0.6) {
                    this.bot.swingArm('right');
                }
                else if (randomChance < 0.6 && randomChance >= 0.3) {
                    this.bot.swingArm('right');
                    if (useLongDelays) {
                        await new Promise(resolve => setTimeout(resolve, generateNumber('float', 250, 800)));
                    }
                    else {
                        await new Promise(resolve => setTimeout(resolve, generateNumber('float', 50, 250)));
                    }
                    this.bot.swingArm('right');
                }
                if (useLongDelays) {
                    await new Promise(resolve => setTimeout(resolve, generateNumber('float', 1800, 2500)));
                }
                else {
                    await new Promise(resolve => setTimeout(resolve, generateNumber('float', 800, 1800)));
                }
            }
        }
        catch {
            this.object.updateTask('waving', false);
        }
        finally {
            this.object.updateTask('waving', false);
        }
    }
    async hybridImitation(state, options) {
        if (!this.bot)
            return;
        if (state === 'start') {
            if (this.tasks.hybridImitation.status)
                return;
            if (this.tasks.jumping.status || this.tasks.shifting.status || this.tasks.movementForward.status || this.tasks.movementBack.status || this.tasks.movementLeft.status || this.tasks.movementRight.status)
                return;
            if (!options)
                return;
            try {
                this.object.updateTask('hybridImitation', true, 4.4);
                this.object.updateTask('jumping', true, 0.8);
                this.object.updateTask('shifting', true, 0.8);
                this.object.updateTask('movementForward', true, 0.5);
                this.object.updateTask('movementBack', true, 0.5);
                this.object.updateTask('movementLeft', true, 0.5);
                this.object.updateTask('movementRight', true, 0.5);
                if (options.useLooking)
                    this.looking(options.useSmoothness, options.useLongDelays);
                if (options.useWaving)
                    this.waving(options.useLongDelays);
                while (this.tasks.hybridImitation.status) {
                    await new Promise(resolve => setTimeout(resolve, generateNumber('float', 1000, 2800)));
                    let chain = '';
                    while (true) {
                        chain = await this.generateHybridChain(options.useMultitasking);
                        let valid = true;
                        LATEST_DATA.imitation.hybrid.chains.forEach(element => chain === element ? valid = false : valid = valid);
                        if (valid)
                            break;
                    }
                    LATEST_DATA.imitation.hybrid.chains.push(chain);
                    if (LATEST_DATA.imitation.hybrid.chains.length > 20) {
                        LATEST_DATA.imitation.hybrid.chains.shift();
                    }
                    const operations = chain.split('&');
                    for (const operation of operations) {
                        const actions = String(operation.split(':')[0]?.split('-'));
                        const mode = Number(operation.split(':')[1]);
                        const delay = Number(operation.split(':')[2]);
                        let usedActions = [];
                        for (const action of actions) {
                            if (!this.tasks.hybridImitation.status || !this.tasks.jumping.status || !this.tasks.shifting.status || !this.tasks.movementForward.status || !this.tasks.movementBack.status || !this.tasks.movementLeft.status || !this.tasks.movementRight.status)
                                break;
                            this.bot.setControlState(action, true);
                            usedActions.push(action);
                            if (options.useLongDelays) {
                                await new Promise(resolve => setTimeout(resolve, generateNumber('float', 200, 800)));
                            }
                            else {
                                await new Promise(resolve => setTimeout(resolve, generateNumber('float', 100, 200)));
                            }
                        }
                        if (mode === 1) {
                            if (usedActions.includes('forward')) {
                                this.bot.setControlState('sprint', true);
                                usedActions.push('sprint');
                            }
                            await new Promise(resolve => setTimeout(resolve, generateNumber('float', 1000, 4000)));
                        }
                        else if (mode === 2) {
                            if (usedActions.includes('forward')) {
                                if (Math.random() > 0.5) {
                                    this.bot.setControlState('sprint', true);
                                    usedActions.push('sprint');
                                }
                            }
                            await new Promise(resolve => setTimeout(resolve, generateNumber('float', 3000, 6000)));
                        }
                        else {
                            await new Promise(resolve => setTimeout(resolve, generateNumber('float', 500, 2000)));
                        }
                        usedActions.forEach(action => this.bot.setControlState(action, false));
                        usedActions = [];
                        await new Promise(resolve => setTimeout(resolve, delay));
                    }
                }
            }
            finally {
                this.object.updateTask('hybridImitation', false);
                this.object.updateTask('jumping', false);
                this.object.updateTask('shifting', false);
                this.object.updateTask('looking', false);
                this.object.updateTask('waving', false);
                this.object.updateTask('movementForward', false);
                this.object.updateTask('movementBack', false);
                this.object.updateTask('movementLeft', false);
                this.object.updateTask('movementRight', false);
            }
        }
        else if (state === 'stop') {
            if (!this.tasks.hybridImitation.status)
                return;
            this.object.updateTask('hybridImitation', false);
            this.object.updateTask('jumping', false);
            this.object.updateTask('shifting', false);
            this.object.updateTask('looking', false);
            this.object.updateTask('waving', false);
            this.object.updateTask('movementForward', false);
            this.object.updateTask('movementBack', false);
            this.object.updateTask('movementLeft', false);
            this.object.updateTask('movementRight', false);
            LATEST_DATA.imitation.hybrid.chains = [];
        }
    }
    async walkingImitation(state, options) {
        if (!this.bot)
            return;
        if (state === 'start') {
            if (this.tasks.walkingImitation.status)
                return;
            if (this.tasks.movementForward.status || this.tasks.movementBack.status || this.tasks.movementLeft.status || this.tasks.movementRight.status)
                return;
            if (!options)
                return;
            try {
                this.object.updateTask('walkingImitation', true, 3.8);
                this.object.updateTask('movementForward', true, 0.5);
                this.object.updateTask('movementBack', true, 0.5);
                this.object.updateTask('movementLeft', true, 0.5);
                this.object.updateTask('movementRight', true, 0.5);
                this.looking(options.useSmoothness, options.useLongDelays);
                while (this.tasks.walkingImitation.status) {
                    await new Promise(resolve => setTimeout(resolve, generateNumber('float', 1000, 2800)));
                    let chain = '';
                    while (true) {
                        chain = await this.generateWalkingChain(options.useMultitasking);
                        let valid = true;
                        LATEST_DATA.imitation.walking.chains.forEach(element => chain === element ? valid = false : valid = valid);
                        if (valid)
                            break;
                    }
                    if (!this.tasks.walkingImitation.status || !this.tasks.movementForward.status || !this.tasks.movementBack.status || !this.tasks.movementLeft.status || !this.tasks.movementRight.status)
                        break;
                    LATEST_DATA.imitation.walking.chains.push(chain);
                    if (LATEST_DATA.imitation.walking.chains.length > 20) {
                        LATEST_DATA.imitation.walking.chains.shift();
                    }
                    const operations = chain.split('&');
                    for (const operation of operations) {
                        const actions = String(operation.split(':')[0]?.split('-'));
                        const delay = Number(operation.split(':')[1]);
                        let usedActions = [];
                        for (const action of actions) {
                            if (!this.tasks.walkingImitation.status || !this.tasks.movementForward.status || !this.tasks.movementBack.status || !this.tasks.movementLeft.status || !this.tasks.movementRight.status)
                                break;
                            this.bot.setControlState(action, true);
                            usedActions.push(action);
                            if (options.useLongDelays) {
                                await new Promise(resolve => setTimeout(resolve, generateNumber('float', 200, 800)));
                            }
                            else {
                                await new Promise(resolve => setTimeout(resolve, generateNumber('float', 100, 200)));
                            }
                        }
                        if (options.useSprint) {
                            if (usedActions.includes('forward') && Math.random() > 0.7) {
                                this.bot.setControlState('sprint', true);
                                usedActions.push('sprint');
                            }
                        }
                        await new Promise(resolve => setTimeout(resolve, generateNumber('float', 1000, 5000)));
                        usedActions.forEach(action => this.bot.setControlState(action, false));
                        usedActions = [];
                        await new Promise(resolve => setTimeout(resolve, delay));
                    }
                }
            }
            finally {
                this.object.updateTask('walkingImitation', false);
                this.object.updateTask('looking', false);
                this.object.updateTask('movementForward', false);
                this.object.updateTask('movementBack', false);
                this.object.updateTask('movementLeft', false);
                this.object.updateTask('movementRight', false);
            }
        }
        else if (state === 'stop') {
            if (!this.tasks.walkingImitation.status)
                return;
            this.object.updateTask('walkingImitation', false);
            this.object.updateTask('looking', false);
            this.object.updateTask('movementForward', false);
            this.object.updateTask('movementBack', false);
            this.object.updateTask('movementLeft', false);
            this.object.updateTask('movementRight', false);
            LATEST_DATA.imitation.walking.chains = [];
        }
    }
}
/*
const { Generator } = require('../utils/generator.js');

const generator = new Generator();

let LATEST_DATA = {
    imitation: {
        hybrid: {
            chains: [],
            multitaskingAlgorithms: []
        },
        walking: {
            chains: [],
            multitaskingAlgorithms: []
        }
    }
}


// Контроллер для управления имитацией
class ImitationController {
    constructor(
        bot,
        object,
        tasks
    ) {
        this.bot = bot;
        this.object = object;
        this.tasks = tasks;
    }

    async generateHybridChain(multitasking) {
        let chain = '';

        const actions = [
            'sneak',
            'jump',
            'forward',
            'back',
            'left',
            'right'
        ];

        for (let i = 0; i < 15; i++) {
            if (i !== 0) chain += '&';

            if (multitasking) {
                if (Math.random() >= 0.8) {
                    chain += generator.chooseRandomValueFromArray(actions);
                } else {
                    let string = '';

                    while (true) {
                        let generatedActions = [];

                        for (let k = 0; k < generator.generateRandomNumberBetween(2, 6); k++) {
                            let generate = '';

                            while (true) {
                                generate = generator.chooseRandomValueFromArray(actions);

                                let valid = true;

                                generatedActions.forEach((element) => {
                                    if (generate === element) {
                                        valid = false;
                                    }
                                })

                                if (valid) break;
                            }

                            generatedActions.push(generate);

                            if (k !== 0) string += '-';
                            
                            string += generate;
                        }

                        generatedActions.length = 0;

                        let valid = true;

                        LATEST_DATA.imitation.hybrid.multitaskingAlgorithms.forEach(element => {
                            if (string === element) {
                                valid = false;
                            }
                        })

                        if (valid) {
                            LATEST_DATA.imitation.hybrid.multitaskingAlgorithms.push(string);

                            if (LATEST_DATA.imitation.hybrid.multitaskingAlgorithms.length > 8) {
                                LATEST_DATA.imitation.hybrid.multitaskingAlgorithms.length = 0;
                            }

                            chain += string;

                            break;
                        }
                    }
                }
            } else {
                chain += generator.chooseRandomValueFromArray(actions);
            }

            if (Math.random() <= 0.1) {
                chain += `:${generator.chooseRandomValueFromArray(['0', '1', '2'])}:${generator.generateRandomNumberBetween(200, 4000)}`;
            } else {
                chain += `:${generator.chooseRandomValueFromArray(['0', '1', '2'])}:${generator.generateRandomNumberBetween(100, 200)}`;
            }
        }

        return chain;
    }

    async generateWalkingChain(multitasking) {
        let chain = '';

        const actions = [
            'forward',
            'back',
            'left',
            'right'
        ];

        for (let i = 0; i < 15; i++) {
            if (i !== 0) chain += '&';

            if (multitasking) {
                if (Math.random() >= 0.8) {
                    chain += generator.chooseRandomValueFromArray(actions);
                } else {
                    let string = '';

                    while (true) {
                        let generatedActions = [];

                        for (let k = 0; k < generator.generateRandomNumberBetween(2, 4); k++) {
                            let generate = '';

                            while (true) {
                                generate = generator.chooseRandomValueFromArray(actions);

                                let valid = true;

                                generatedActions.forEach(element => generate === element ? valid = false : valid = valid);
                                
                                if (valid) break;
                            }

                            generatedActions.push(generate);

                            if (k !== 0) string += '-';
                            
                            string += generate;
                        }

                        generatedActions.length = 0;

                        let valid = true;

                        LATEST_DATA.imitation.walking.multitaskingAlgorithms.forEach(element => string === element ? valid = false : valid = valid);

                        if (valid) {
                            LATEST_DATA.imitation.walking.multitaskingAlgorithms.push(string);

                            if (LATEST_DATA.imitation.walking.multitaskingAlgorithms.length > 8) {
                                LATEST_DATA.imitation.walking.multitaskingAlgorithms.length = 0;
                            }

                            chain += string;

                            break;
                        }
                    }
                }
            } else {
                chain += generator.chooseRandomValueFromArray(actions);
            }

            if (Math.random() <= 0.1) {
                chain += `:${generator.generateRandomNumberBetween(2000, 4000)}`;
            } else {
                chain += `:${generator.generateRandomNumberBetween(500, 2300)}`;
            }
        }

        return chain;
    }

    async looking(useSmoothness, useLongDelays) {
        if (!this.bot) return;
        if (this.tasks.looking.status) return;

        try {
            this.object.updateTask('looking', true, 4.5);

            while (this.tasks.looking.status) {
                if (Math.random() >= 0.8) {
                    const players = Object.values(this.bot.players);
                    let closestPlayer = null;
                    let closestDistance = 24;

                    for (const player of players) {
                        if (player.entity) {
                            const distance = this.bot.entity.position.distanceTo(player.entity.position);
                            if (distance < closestDistance) {
                                closestDistance = distance;
                                closestPlayer = player;
                            }
                        }
                    }

                    if (closestPlayer) {
                        const target = closestPlayer.entity;
                        this.bot.lookAt(target.position.offset(Number(generator.chooseRandomValueFromArray([Math.random() / 3, -(Math.random() / 3)])), target.height, Number(generator.chooseRandomValueFromArray([Math.random() / 3, -(Math.random() / 3)]))), !useSmoothness);
                    }
                } else {
                    const yaw = Math.random();
                    const pitch = Math.random() / 3;

                    if (Math.random() >= 0.5) {
                        this.bot.look(yaw, pitch, !useSmoothness);
                    } else {
                        this.bot.look(-yaw, -pitch, !useSmoothness);
                    }
                }

                if (useLongDelays) {
                    await new Promise(resolve => setTimeout(resolve, generator.generateRandomNumberBetween(1000, 2200)));
                } else {
                    await new Promise(resolve => setTimeout(resolve, generator.generateRandomNumberBetween(250, 1000)));
                }
            }
        } catch {
            this.object.updateTask('looking', false);
        } finally {
            this.object.updateTask('looking', false);
        }
    }

    async waving(useLongDelays) {
        if (!this.bot) return;
        if (this.tasks.waving.status) return;

        try {
            this.object.updateTask('waving', true, 3.2);

            while (this.tasks.waving.status) {
                const randomChance = Math.random();

                if (randomChance >= 0.6) {
                    this.bot.swingArm('right');
                } else if (randomChance < 0.6 && randomChance >= 0.3) {
                    this.bot.swingArm('right');

                    if (useLongDelays) {
                        await new Promise(resolve => setTimeout(resolve, generator.generateRandomNumberBetween(250, 800)));
                    } else {
                        await new Promise(resolve => setTimeout(resolve, generator.generateRandomNumberBetween(50, 250)));
                    }

                    this.bot.swingArm('right');
                }

                if (useLongDelays) {
                    await new Promise(resolve => setTimeout(resolve, generator.generateRandomNumberBetween(1800, 2500)));
                } else {
                    await new Promise(resolve => setTimeout(resolve, generator.generateRandomNumberBetween(800, 1800)));
                }
            }
        } catch {
            this.object.updateTask('waving', false);
        } finally {
            this.object.updateTask('waving', false);
        }
    }

    async hybridImitation(state, options) {
        if (!this.bot) return;

        if (state === 'start') {
            if (this.tasks.hybridImitation.status) return;
            if (this.tasks.jumping.status || this.tasks.shifting.status || this.tasks.movementForward.status || this.tasks.movementBack.status || this.tasks.movementLeft.status || this.tasks.movementRight.status) return;

            if (!options) return;

            try {
                this.object.updateTask('hybridImitation', true, 4.4);
                this.object.updateTask('jumping', true, 0.8);
                this.object.updateTask('shifting', true, 0.8);
                this.object.updateTask('movementForward', true, 0.5);
                this.object.updateTask('movementBack', true, 0.5);
                this.object.updateTask('movementLeft', true, 0.5);
                this.object.updateTask('movementRight', true, 0.5);

                if (options.useLooking) this.looking(options.useSmoothness, options.useLongDelays);
                if (options.useWaving) this.waving(options.useLongDelays);

                while (this.tasks.hybridImitation.status) {
                    await new Promise(resolve => setTimeout(resolve, generator.generateRandomNumberBetween(1000, 2800)));

                    let chain = '';

                    while (true) {
                        chain = await this.generateHybridChain(options.useMultitasking);

                        let valid = true;

                        LATEST_DATA.imitation.hybrid.chains.forEach(element => chain === element ? valid = false : valid = valid);

                        if (valid) break;
                    }

                    LATEST_DATA.imitation.hybrid.chains.push(chain);

                    if (LATEST_DATA.imitation.hybrid.chains.length > 20) {
                        LATEST_DATA.imitation.hybrid.chains.shift();
                    }

                    const operations = chain.split('&');
                        
                    for (const operation of operations) {
                        const actions = operation.split(':')[0].split('-');
                        const mode = Number(operation.split(':')[1]);
                        const delay = Number(operation.split(':')[2]);

                        let usedActions = [];

                        for (const action of actions) {
                            if (!this.tasks.hybridImitation.status || !this.tasks.jumping.status || !this.tasks.shifting.status || !this.tasks.movementForward.status || !this.tasks.movementBack.status || !this.tasks.movementLeft.status || !this.tasks.movementRight.status) break;

                            this.bot.setControlState(action, true);
                            usedActions.push(action);

                            if (options.useLongDelays) {
                                await new Promise(resolve => setTimeout(resolve, generator.generateRandomNumberBetween(200, 800)));
                            } else {
                                await new Promise(resolve => setTimeout(resolve, generator.generateRandomNumberBetween(100, 200)));
                            }
                        }

                        if (mode === 1) {
                            if (usedActions.includes('forward')) {
                                this.bot.setControlState('sprint', true);
                                usedActions.push('sprint');
                            }

                            await new Promise(resolve => setTimeout(resolve, generator.generateRandomNumberBetween(1000, 4000)));
                        } else if (mode === 2) {
                            if (usedActions.includes('forward')) {
                                if (Math.random() > 0.5) {
                                    this.bot.setControlState('sprint', true);
                                    usedActions.push('sprint');
                                }
                            }

                            await new Promise(resolve => setTimeout(resolve, generator.generateRandomNumberBetween(3000, 6000)));
                        } else {
                            await new Promise(resolve => setTimeout(resolve, generator.generateRandomNumberBetween(500, 2000)));
                        }

                        usedActions.forEach(action => this.bot.setControlState(action, false));
                        usedActions = [];

                        await new Promise(resolve => setTimeout(resolve, delay));
                    }
                }
            } finally {
                this.object.updateTask('hybridImitation', false);
                this.object.updateTask('jumping', false);
                this.object.updateTask('shifting', false);
                this.object.updateTask('looking', false);
                this.object.updateTask('waving', false);
                this.object.updateTask('movementForward', false);
                this.object.updateTask('movementBack', false);
                this.object.updateTask('movementLeft', false);
                this.object.updateTask('movementRight', false);
            }
        } else if (state === 'stop') {
            if (!this.tasks.hybridImitation.status) return;

            this.object.updateTask('hybridImitation', false);
            this.object.updateTask('jumping', false);
            this.object.updateTask('shifting', false);
            this.object.updateTask('looking', false);
            this.object.updateTask('waving', false);
            this.object.updateTask('movementForward', false);
            this.object.updateTask('movementBack', false);
            this.object.updateTask('movementLeft', false);
            this.object.updateTask('movementRight', false);

            LATEST_DATA.imitation.hybrid.chains = [];
        }
    }

    async walkingImitation(state, options) {
        if (!this.bot) return;
        
        if (state === 'start') {
            if (this.tasks.walkingImitation.status) return;
            if (this.tasks.movementForward.status || this.tasks.movementBack.status || this.tasks.movementLeft.status || this.tasks.movementRight.status) return;

            if (!options) return;

            try {
                this.object.updateTask('walkingImitation', true, 3.8);
                this.object.updateTask('movementForward', true, 0.5);
                this.object.updateTask('movementBack', true, 0.5);
                this.object.updateTask('movementLeft', true, 0.5);
                this.object.updateTask('movementRight', true, 0.5);

                this.looking(options.useSmoothness, options.useLongDelays);

                while (this.tasks.walkingImitation.status) {
                    await new Promise(resolve => setTimeout(resolve, generator.generateRandomNumberBetween(1000, 2800)));

                    let chain = '';

                    while (true) {
                        chain = await this.generateWalkingChain(options.useMultitasking);

                        let valid = true;

                        LATEST_DATA.imitation.walking.chains.forEach(element => chain === element ? valid = false : valid = valid);

                        if (valid) break;
                    }

                    if (!this.tasks.walkingImitation.status || !this.tasks.movementForward.status || !this.tasks.movementBack.status || !this.tasks.movementLeft.status || !this.tasks.movementRight.status) break;

                    LATEST_DATA.imitation.walking.chains.push(chain);

                    if (LATEST_DATA.imitation.walking.chains.length > 20) {
                        LATEST_DATA.imitation.walking.chains.shift();
                    }

                    const operations = chain.split('&');
                            
                    for (const operation of operations) {
                        const actions = operation.split(':')[0].split('-');
                        const delay = Number(operation.split(':')[1]);

                        let usedActions = [];

                        for (const action of actions) {
                            if (!this.tasks.walkingImitation.status || !this.tasks.movementForward.status || !this.tasks.movementBack.status || !this.tasks.movementLeft.status || !this.tasks.movementRight.status) break;

                            this.bot.setControlState(action, true);
                            usedActions.push(action);

                            if (options.useLongDelays) {
                                await new Promise(resolve => setTimeout(resolve, generator.generateRandomNumberBetween(200, 800)));
                            } else {
                                await new Promise(resolve => setTimeout(resolve, generator.generateRandomNumberBetween(100, 200)));
                            }
                        }

                        if (options.useSprint) {
                            if (usedActions.includes('forward') && Math.random() > 0.7) {
                                this.bot.setControlState('sprint', true);
                                usedActions.push('sprint');
                            }
                        }

                        await new Promise(resolve => setTimeout(resolve, generator.generateRandomNumberBetween(1000, 5000)));

                        usedActions.forEach(action => this.bot.setControlState(action, false));
                        usedActions = [];

                        await new Promise(resolve => setTimeout(resolve, delay));
                    }
                }
            } finally {
                this.object.updateTask('walkingImitation', false);
                this.object.updateTask('looking', false);
                this.object.updateTask('movementForward', false);
                this.object.updateTask('movementBack', false);
                this.object.updateTask('movementLeft', false);
                this.object.updateTask('movementRight', false);
            }
        } else if (state === 'stop') {
            if (!this.tasks.walkingImitation.status) return;

            this.object.updateTask('walkingImitation', false);
            this.object.updateTask('looking', false);
            this.object.updateTask('movementForward', false);
            this.object.updateTask('movementBack', false);
            this.object.updateTask('movementLeft', false);
            this.object.updateTask('movementRight', false);

            LATEST_DATA.imitation.walking.chains = [];
        }
    }
}

module.exports = ImitationController;
*/ 
