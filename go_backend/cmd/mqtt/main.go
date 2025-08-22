package main

import (
	"log"

	mqtt "github.com/mochi-co/mqtt/server"
	"github.com/mochi-co/mqtt/server/listeners"
)

func main() {
	server := mqtt.NewServer(nil)

	tcp := listeners.NewTCP("t1", ":1833")

	err := server.AddListener(tcp, nil)
	if err != nil {
		log.Fatal(err)
	}

	err = server.Serve()
	if err != nil {
		log.Fatal(err)
	}
}
