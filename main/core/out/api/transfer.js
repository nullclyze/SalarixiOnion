export function transmit(res, type, data) {
    try {
        res.json({ type: type, data: data });
    }
    catch (error) {
        console.log(`Ошибка передачи данных: ${error}`);
    }
}
