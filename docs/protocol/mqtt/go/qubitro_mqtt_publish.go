package main

import (
	"encoding/json"
	"fmt"
	"time"

	mqtt "github.com/eclipse/paho.mqtt.golang"
)

var deviceID string = "PASTE_DEVICE_ID"
var deviceToken string = "PASTE_DEVICE_TOKEN"
var brokerHost string = "tls://broker.qubitro.com:8883"

var jsonData []byte

//Data is an example payload
type Data struct {
	Temperature float64
	CPUUsage    float64
}

func main() {

	data := &Data{
		Temperature: 40.5,
		CPUUsage:    78,
	}

	jsonData, err := json.Marshal(data)
	if err != nil {
		fmt.Println(err)
	}

	options := mqtt.NewClientOptions().AddBroker(brokerHost)
	options.SetClientID(deviceID)
	options.SetUsername(deviceID)
	options.SetPassword(deviceToken)
	options.AutoReconnect = true

	client := mqtt.NewClient(options)

	if token := client.Connect(); token.Wait() && token.Error() != nil {
		fmt.Println("Error: ", token.Error(), "Visit: https://docs.qubitro.com/client-guides/connect-device/mqtt")
	}

	options.OnConnect = func(c mqtt.Client) {
		fmt.Println("Connected to Qubitro")

		for {
			fmt.Println("Published: ", string(jsonData))
			client.Publish(deviceID, 0, false, jsonData)
			time.Sleep(time.Second * 2)
		}
	}
}
