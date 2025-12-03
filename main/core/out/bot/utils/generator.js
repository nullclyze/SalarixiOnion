export function generateString(type, length) {
    let chars = '';
    switch (type) {
        case 'numeric':
            chars = '0123456789';
            break;
        case 'letter':
            chars = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz';
            break;
        case 'multi':
            chars = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789';
            break;
        case 'special':
            chars = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#$%^&*()_-+=';
            break;
    }
    let result = '';
    for (let i = 0; i < length; i++) {
        const index = Math.floor(Math.random() * chars.length);
        result += chars[index];
    }
    return result;
}
export function generateNumber(type, min, max) {
    const number = Math.random() * (max - min + 1) + min;
    if (type === 'int') {
        return Math.floor(number);
    }
    else {
        return parseFloat(number.toFixed(6));
    }
}
export function chooseRandomElementFromArray(array) {
    return array[Math.floor(Math.random() * array.length)];
}
