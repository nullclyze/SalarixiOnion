package modules

import (
	"encoding/json"
	"os"
	"strings"
)

type ProxyStorm struct {
	path string
}

func (p ProxyStorm) checkFileExist() bool {
	_, err := os.Stat(p.path)

	return !os.IsNotExist(err)
}

func (p ProxyStorm) ReadTextFile(path string) (bool, string, []string) {
	p.path = path

	if p.checkFileExist() {
		bytes, err := os.ReadFile(p.path)

		if err != nil {
			return false, "Ошибка чтения файла", nil
		}

		data := string(bytes)

		methods := []string{
			"\n",
			";",
			",",
			" ",
		}

		spliter := "\n"

		for _, method := range methods {
			if len(strings.Split(data, method)) > 1 {
				spliter = method
				break
			}
		}

		proxies := strings.Split(data, spliter)

		return true, "Файл успешно прочитан", proxies
	} else {
		return false, "Не удалось найти файл", nil
	}
}

func (p ProxyStorm) ReadJsonFile(path string) (bool, string, map[string][]string) {
	p.path = path

	if p.checkFileExist() {
		bytes, err := os.ReadFile(p.path)

		if err != nil {
			return false, "Ошибка чтения файла", nil
		}

		var data map[string][]string

		err = json.Unmarshal(bytes, &data)

		if err != nil {
			return false, "Ошибка чтения файла", nil
		}

		if len(data["socks5"]) > 1 || len(data["socks4"]) > 1 || len(data["http"]) > 1 {
			return true, "Файл успешно прочитан", data
		}

		return true, "Некорректная структура файла", nil
	} else {
		return false, "Не удалось найти файл", nil
	}
}
