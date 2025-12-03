let sessions = {};
export function add(name, res) {
    if (!Object.keys(sessions).includes(name)) {
        res.setHeader('Content-Type', 'text/event-stream');
        res.setHeader('Cache-Control', 'no-cache');
        res.setHeader('Connection', 'keep-alive');
        res.setHeader('Access-Control-Allow-Origin', '*');
        sessions[name] = res;
        res.on('close', () => {
            delete sessions[name];
        });
    }
}
export function del(name) {
    if (Object.keys(sessions).includes(name) && typeof sessions[name].end === 'function') {
        sessions[name].end();
        delete sessions[name];
    }
}
export function msg(name, data) {
    const session = sessions[name];
    if (session && !session.writableEnded) {
        session.write(`data: ${JSON.stringify(data)}\n\n`);
    }
}
