function transmit(res, type, data) {
    try {
        res.json({ type: type, data: data });
    }
    catch (error) {
        console.log(`Ошибка передачи данных: ${error}`);
    }
}
class SessionManager {
    constructor() {
        this.sessions = {};
    }
    add(name, res) {
        if (!Object.keys(this.sessions).includes(name)) {
            this.sessions[name] = res;
            res.on('close', () => {
                delete this.sessions[name];
            });
        }
    }
    del(name) {
        if (Object.keys(this.sessions).includes(name) && typeof this.sessions[name].end === 'function') {
            this.sessions[name].end();
            delete this.sessions[name];
        }
    }
    msg(name, data) {
        const session = this.sessions[name];
        if (session && !session.writableEnded) {
            session.write(`data: ${JSON.stringify(data)}\n\n`);
        }
    }
}
export { transmit, SessionManager };
