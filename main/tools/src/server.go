package main

import (
	"encoding/json"
	"fmt"
	"net/http"
	"tools/modules"
)

type tools struct {
	configurator modules.Configurator
}

func main() {
	t := tools{
		configurator: modules.Configurator{},
	}

	http.HandleFunc("/salarixi/utils/config/write", t.writeConfigHandler)
	http.HandleFunc("/salarixi/utils/config/read", t.readConfigHandler)

	fmt.Println("\n( INFO ) Сервер запущен :: http://localhost:37182")

	fmt.Println("( INFO ) Чтение конфига :: http://localhost:37182/salarixi/utils/config/write")
	fmt.Println("( INFO ) Запись конфига :: http://localhost:37182/salarixi/utils/config/read")

	if err := http.ListenAndServe(":37182", nil); err != nil {
		fmt.Println("( INFO ) Ошибка запуска сервера: " + err.Error())
	}
}

type WriteConfigRequestBody struct {
	Key    string `json:"key"`
	Config any    `json:"config"`
}

type ReadConfigRequestBody struct {
	Key string `json:"key"`
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

	serverKey := "salarixionion:1.0.0:ol13Rqk:config:write"

	var msg ServerMessage
	var body WriteConfigRequestBody

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

	if body.Key != serverKey {
		msg = ServerMessage{
			Success:    false,
			InvalidKey: true,
			Message:    "Key is invalid",
			Data:       nil,
		}

		json.NewEncoder(w).Encode(msg)
		return
	}

	status, message := t.configurator.WriteConfig(body.Config)

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

	serverKey := "salarixionion:1.0.0:Yi8jQ13e:config:read"

	var msg ServerMessage
	var body ReadConfigRequestBody

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

	if body.Key != serverKey {
		msg = ServerMessage{
			Success:    false,
			InvalidKey: true,
			Message:    "Key is invalid",
			Data:       nil,
		}

		json.NewEncoder(w).Encode(msg)
		return
	}

	status, message, data := t.configurator.ReadConfig()

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
