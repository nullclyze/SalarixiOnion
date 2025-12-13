package modules

import (
	"encoding/json"
	"os"
)

type ConfigManager struct {
	path string
}

func (c ConfigManager) GetConfigPath() (bool, string) {
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

func (c ConfigManager) checkConfigExist() bool {
	_, err := os.Stat(c.path)

	return !os.IsNotExist(err)
}

func (c ConfigManager) WriteConfig(config any) (bool, string) {
	data, err := json.MarshalIndent(config, "", "  ")

	if err != nil {
		return false, "Не удалось провести сериализацию JSON-данных"
	}

	if c.path == "NOT_FOUND" || !c.checkConfigExist() {
		status, path := c.GetConfigPath()

		if !status {
			c.path = "NOT_FOUND"
			return false, "Не удалось найти путь к конфигу"
		}

		c.path = path
	}

	err = os.WriteFile(c.path, data, 0644)

	if err != nil {
		return false, "Не удалось записать данные в конфиг"
	}

	return true, "Конфиг успешно изменён"
}

func (c ConfigManager) ReadConfig() (bool, string, map[string]any) {
	if c.path == "NOT_FOUND" || !c.checkConfigExist() {
		status, path := c.GetConfigPath()

		if !status {
			c.path = "NOT_FOUND"
			return false, "Не удалось найти путь к конфигу", nil
		}

		c.path = path
	}

	data, err := os.ReadFile(c.path)

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
