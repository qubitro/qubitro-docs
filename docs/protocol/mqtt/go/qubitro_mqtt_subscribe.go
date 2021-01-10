package main

import (
	"fmt"

	mqtt "github.com/eclipse/paho.mqtt.golang"
)

var brokerHost string = "tls://broker.qubitro.com:8883"
var deviceID string = "PASTE_DEVICE_ID"
var deviceToken string = "PASTE_DEVICE_TOKEN"
var topic string = "PASTE_TOPIC"

func main() {

	var messageHandler mqtt.MessageHandler = func(client mqtt.Client, msg mqtt.Message) {
		fmt.Printf("Received, message: %s\n", string(msg.Payload()))
	}

	options := mqtt.NewClientOptions().AddBroker(brokerHost)
	options.SetClientID(deviceID)
	options.SetUsername(deviceID)
	options.SetPassword(deviceToken)
	options.SetDefaultPublishHandler(messageHandler) // To receive message, add handler
	options.AutoReconnect = true

	options.OnConnect = func(c mqtt.Client) {
		fmt.Println("Connected to Qubitro")

	}
	client := mqtt.NewClient(options)

	forever := make(chan bool)

	go func(topic string) {

		if token := client.Connect(); token.Wait() && token.Error() != nil {
			fmt.Println("Error: ", token.Error(), "Visit: https://docs.qubitro.com/client-guides/connect-device/mqtt")
		}

		if token := client.Subscribe(topic, 0, nil); token.Wait() {
			fmt.Println("Subscribed to topic: ", topic)
			if token.Error() != nil {
				fmt.Println(token.Error())
			}
		}
	}(topic)

	<-forever

}
