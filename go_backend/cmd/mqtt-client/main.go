package main

import (
	"fmt"
	"log"

	mqtt "github.com/eclipse/paho.mqtt.golang"
)

func main() {
	opts := mqtt.NewClientOptions()
	opts.AddBroker("tcp://localhost:1883")
	opts.SetClientID("go-client")
	opts.OnConnect = func(c mqtt.Client) {
		fmt.Println("Client connected")

		// Subscribe to topic
		if token := c.Subscribe("test/topic", 0, func(_ mqtt.Client, msg mqtt.Message) {
			fmt.Printf("Received message on [%s]: %s\n", msg.Topic(), msg.Payload())
		}); token.Wait() && token.Error() != nil {
			log.Println("Subscribe error:", token.Error())
		}

		// Publish to topic
		token := c.Publish("test/topic", 0, false, "Hello from client")
		token.Wait()
		fmt.Println("Message published")
	}

	client := mqtt.NewClient(opts)
	defer client.Disconnect(250)
	if token := client.Connect(); token.Wait() && token.Error() != nil {
		log.Fatal(token.Error())
	}
}
