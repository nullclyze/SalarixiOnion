package tools

import (
	"encoding/json"
	"os"
)

func checkConfigExist() (bool, string) {
	status := false
	path := "NOT_FOUND"

	configPaths := []string{
		"./config/salarixi.config.json",
		"./salarixi.config.json",
		"./cfg/salarixi.config.json",
		"./salarixi.cfg.json",
		"./config/salarixi.cfg.json",
		"../config/salarixi.config.json",
		"../cfg/salarixi.config.json",
		"../config/salarixi.cfg.json",
	}

	for _, configPath := range configPaths {
		_, err := os.Stat(configPath)

		isNotExist := os.IsNotExist(err)

		if !isNotExist {
			status = true
			path = configPath
			break
		}
	}

	return status, path
}

func WriteConfig(config any) (bool, string) {
	data, err := json.MarshalIndent(config, "", "  ")

	if err != nil {
		return false, "Не удалось провести сериализацию JSON-данных"
	}

	status, path := checkConfigExist()

	if !status {
		return false, "Не удалось найти путь к конфигу"
	}

	err = os.WriteFile(path, data, 0644)

	if err != nil {
		return false, "Не удалось записать данные в конфиг"
	}

	return true, "Конфиг успешно изменён"
}

func ReadConfig() (bool, string, map[string]any) {
	status, path := checkConfigExist()

	if !status {
		return false, "Не удалось найти путь к конфигу", nil
	}

	data, err := os.ReadFile(path)

	if err != nil {
		return false, "Не удалось прочитать конфиг", nil
	}

	var config map[string]any

	err = json.Unmarshal(data, &config)

	if err != nil {
		return false, "Не удалось распарсить данные из конфига", nil
	}

	return true, "Конфиг успешно прочитан", config
}
