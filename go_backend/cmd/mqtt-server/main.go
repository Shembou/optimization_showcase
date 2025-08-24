package main

import (
	"log"

	mqtt "github.com/mochi-mqtt/server/v2"
	"github.com/mochi-mqtt/server/v2/hooks/auth"
	"github.com/mochi-mqtt/server/v2/listeners"
)

func main() {
	server := mqtt.New(nil)
	defer server.Close()
	messages := make(chan string, 1)

	_ = server.AddHook(new(auth.AllowHook), nil)

	tcp := listeners.NewTCP(listeners.Config{ID: "t1", Address: ":1883"})

	err := server.AddListener(tcp)
	if err != nil {
		log.Fatal(err)
	}

	go func() {
		<-messages
		err = server.Serve()
		if err != nil {
			log.Fatal(err)
		}
	}()
}
