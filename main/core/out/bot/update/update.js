import { msg } from '../../api/session.js';
export function updateBotProfileData(data) {
    const version = data.profile.version;
    const password = data.profile.password;
    const proxyType = data.profile.proxyType.toUpperCase() || '-';
    const proxy = data.profile.proxy.split(':')[0] || 'Не использует';
    const status = data.profile.status;
    const load = data.profile.load;
    const ping = data.profile.ping;
    let loadColor;
    let pingColor;
    if (load <= 0) {
        loadColor = '#8f8f8fff';
    }
    else if (load > 0 && load <= 20) {
        loadColor = '#22ed17ff';
    }
    else if (load > 20 && load <= 40) {
        loadColor = '#28c305ff';
    }
    else if (load > 40 && load <= 60) {
        loadColor = '#eddf17ff';
    }
    else if (load > 60 && load <= 80) {
        loadColor = '#d1800fff';
    }
    else {
        loadColor = '#ed1717ff';
    }
    if (ping <= 60 && ping > 0) {
        pingColor = '#22ed17ff';
    }
    else if (ping > 60 && ping <= 360) {
        pingColor = '#eddf17ff';
    }
    else if (ping > 360 && ping <= 10000) {
        pingColor = '#ed1717ff';
    }
    else {
        pingColor = '#8f8f8fff';
    }
    msg('monitoring:profile-data', {
        nickname: data.nickname,
        status: status.text,
        statusColor: status.color,
        version: version,
        password: password,
        proxyType: proxyType,
        proxy: proxy,
        load: `${load}%`,
        loadColor: loadColor,
        ping: ping ? `${ping} мс` : '?',
        pingColor: pingColor
    });
}
export function updateBotChatHistory(data, maxLength) {
    msg('monitoring:chat-history', {
        nickname: data.nickname,
        type: data.type,
        text: data.text,
        maxLength: maxLength
    });
}
