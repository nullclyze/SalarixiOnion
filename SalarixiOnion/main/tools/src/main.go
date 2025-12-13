package main

import (
	"encoding/json"
	"fmt"
	"net/http"
	"tools/modules"
)

type tools struct {
	key           string
	configManager modules.ConfigManager
	proxyStorm    modules.ProxyStorm
}

func main() {
	t := tools{
		key:           "salarixionion:j3l14rFj",
		configManager: modules.ConfigManager{},
		proxyStorm:    modules.ProxyStorm{},
	}

	http.HandleFunc("/salarixi/system/config/write", t.writeConfigHandler)
	http.HandleFunc("/salarixi/system/config/read", t.readConfigHandler)

	http.HandleFunc("/salarixi/system/proxy/text", t.readTextFile)
	http.HandleFunc("/salarixi/system/proxy/json", t.readJsonFile)

	fmt.Println("Сервер запущен")

	if err := http.ListenAndServe(":37182", nil); err != nil {
		fmt.Println("Ошибка запуска сервера: " + err.Error())
	}
}

type ConfiguratorStruct struct {
	Config any `json:"config"`
}

type ReaderStruct struct {
	Path string `json:"path"`
}

type RequestBodyForConfigManager struct {
	Key  string             `json:"key"`
	Data ConfiguratorStruct `json:"data"`
}

type RequestBodyForProxyStorm struct {
	Key  string       `json:"key"`
	Data ReaderStruct `json:"data"`
}

type ServerMessage struct {
	Success    bool   `json:"success"`
	InvalidKey bool   `json:"invalidKey"`
	Message    string `json:"message"`
	Data       any    `json:"data"`
}

func (t tools) writeConfigHandler(w http.ResponseWriter, r *http.Request) {
	w.Header().Set("Access-Control-Allow-Origin", "*")
	w.Header().Set("Access-Control-Allow-Methods", "GET, POST, OPTIONS")
	w.Header().Set("Access-Control-Allow-Headers", "Content-Type")
	w.Header().Set("Content-Type", "application/json")

	var msg ServerMessage
	var body RequestBodyForConfigManager

	err := json.NewDecoder(r.Body).Decode(&body)

	if err != nil {
		msg = ServerMessage{
			Success:    false,
			InvalidKey: false,
			Message:    "Ошибка парсинга Body: " + err.Error(),
			Data:       nil,
		}

		json.NewEncoder(w).Encode(msg)
		return
	}

	if body.Key != t.key {
		msg = ServerMessage{
			Success:    false,
			InvalidKey: true,
			Message:    "Key is invalid",
			Data:       nil,
		}

		json.NewEncoder(w).Encode(msg)
		return
	}

	status, message := t.configManager.WriteConfig(body.Data.Config)

	if !status {
		msg = ServerMessage{
			Success:    false,
			InvalidKey: false,
			Message:    message,
			Data:       nil,
		}
	} else {
		msg = ServerMessage{
			Success:    true,
			InvalidKey: false,
			Message:    message,
			Data:       nil,
		}
	}

	json.NewEncoder(w).Encode(msg)
}

func (t tools) readConfigHandler(w http.ResponseWriter, r *http.Request) {
	w.Header().Set("Access-Control-Allow-Origin", "*")
	w.Header().Set("Access-Control-Allow-Methods", "GET, POST, OPTIONS")
	w.Header().Set("Access-Control-Allow-Headers", "Content-Type")
	w.Header().Set("Content-Type", "application/json")

	var msg ServerMessage
	var body RequestBodyForConfigManager

	err := json.NewDecoder(r.Body).Decode(&body)

	if err != nil {
		msg = ServerMessage{
			Success:    false,
			InvalidKey: false,
			Message:    "Ошибка парсинга Body: " + err.Error(),
			Data:       nil,
		}

		json.NewEncoder(w).Encode(msg)
		return
	}

	if body.Key != t.key {
		msg = ServerMessage{
			Success:    false,
			InvalidKey: true,
			Message:    "Key is invalid",
			Data:       nil,
		}

		json.NewEncoder(w).Encode(msg)
		return
	}

	status, message, data := t.configManager.ReadConfig()

	if !status {
		msg = ServerMessage{
			Success:    false,
			InvalidKey: false,
			Message:    message,
			Data:       nil,
		}
	} else {
		msg = ServerMessage{
			Success:    true,
			InvalidKey: false,
			Message:    message,
			Data:       data,
		}
	}

	json.NewEncoder(w).Encode(msg)
}

func (t tools) readTextFile(w http.ResponseWriter, r *http.Request) {
	w.Header().Set("Access-Control-Allow-Origin", "*")
	w.Header().Set("Access-Control-Allow-Methods", "GET, POST, OPTIONS")
	w.Header().Set("Access-Control-Allow-Headers", "Content-Type")
	w.Header().Set("Content-Type", "application/json")

	var msg ServerMessage
	var body RequestBodyForProxyStorm

	err := json.NewDecoder(r.Body).Decode(&body)

	if err != nil {
		msg = ServerMessage{
			Success:    false,
			InvalidKey: false,
			Message:    "Ошибка парсинга Body: " + err.Error(),
			Data:       nil,
		}

		json.NewEncoder(w).Encode(msg)
		return
	}

	if body.Key != t.key {
		msg = ServerMessage{
			Success:    false,
			InvalidKey: true,
			Message:    "Key is invalid",
			Data:       nil,
		}

		json.NewEncoder(w).Encode(msg)
		return
	}

	status, message, data := t.proxyStorm.ReadTextFile(body.Data.Path)

	if !status {
		msg = ServerMessage{
			Success:    false,
			InvalidKey: false,
			Message:    message,
			Data:       nil,
		}
	} else {
		msg = ServerMessage{
			Success:    true,
			InvalidKey: false,
			Message:    message,
			Data:       data,
		}
	}

	json.NewEncoder(w).Encode(msg)
}

func (t tools) readJsonFile(w http.ResponseWriter, r *http.Request) {
	w.Header().Set("Access-Control-Allow-Origin", "*")
	w.Header().Set("Access-Control-Allow-Methods", "GET, POST, OPTIONS")
	w.Header().Set("Access-Control-Allow-Headers", "Content-Type")
	w.Header().Set("Content-Type", "application/json")

	var msg ServerMessage
	var body RequestBodyForProxyStorm

	err := json.NewDecoder(r.Body).Decode(&body)

	if err != nil {
		msg = ServerMessage{
			Success:    false,
			InvalidKey: false,
			Message:    "Ошибка парсинга Body: " + err.Error(),
			Data:       nil,
		}

		json.NewEncoder(w).Encode(msg)
		return
	}

	if body.Key != t.key {
		msg = ServerMessage{
			Success:    false,
			InvalidKey: true,
			Message:    "Key is invalid",
			Data:       nil,
		}

		json.NewEncoder(w).Encode(msg)
		return
	}

	status, message, data := t.proxyStorm.ReadJsonFile(body.Data.Path)

	if !status {
		msg = ServerMessage{
			Success:    false,
			InvalidKey: false,
			Message:    message,
			Data:       nil,
		}
	} else {
		msg = ServerMessage{
			Success:    true,
			InvalidKey: false,
			Message:    message,
			Data:       data,
		}
	}

	json.NewEncoder(w).Encode(msg)
}
