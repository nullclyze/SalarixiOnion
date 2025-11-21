interface BotSettings {
	address: string;
	version: string;
	quantity: number;
	nickname: string;
}

class Validator {
	public validateBotSettings(data: BotSettings) {
		if (!data.address.split(':')[0] || !data.address.split(':')[1]) {
			return {
				status: false,
				message: 'Поле "Адрес сервера" содержит ошибки'
			}
		}

		if ((!data.version.split('.')[0] || !data.version.split('.')[1]) && data.version !== 'auto') {
			return {
				status: false,
				message: 'Поле "Версия Minecraft" содержит ошибки'
			}
		}

		if (!data.quantity || data.quantity < 1) {
			return {
				status: false,
				message: 'Поле "Кол-во ботов" содержит ошибки'
			}
		}

		if (!data.nickname) {
			return {
				status: false,
				message: 'Поле "Никнейм" пустое'
			}
		}

		return {
			status: true,
			message: 'Валидация данных успешно пройдена'
		}
	}
}

export default Validator;