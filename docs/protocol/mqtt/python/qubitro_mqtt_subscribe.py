import paho.mqtt.client as mqtt
import json
import time
import ssl
broker_host = "broker.qubitro.com"
broker_port = 8883
device_id = "PASTE_DEVICE_ID"
device_token = "PASTE_DEVICE_TOKEN"

def on_connect(client, userdata, flags, rc):
    if rc == 0:
        print("Connected to Qubitro!")
        client.on_subscribe = on_subscribe
        client.on_message = on_message
    else:
        print("Failed to connect, visit: https://docs.qubitro.com/client-guides/connect-device/mqtt\n return code:", rc)

def on_subscribe(mqttc, obj, mid, granted_qos):
    print("Subscribed, " + "qos: " + str(granted_qos))

def on_message(client, userdata, message):
    print("received message =",str(message.payload.decode("utf-8")))

client = mqtt.Client(client_id=device_id)
context = ssl.SSLContext(ssl.PROTOCOL_TLSv1)
client.tls_set_context(context)
client.username_pw_set(username=device_id, password=device_token)
client.connect(broker_host, broker_port, 60)
client.on_connect = on_connect
client.subscribe("PASTE_TOPIC_TO_SUBSCRIBE", 0)
client.loop_start()

while True:
    if client.is_connected:
        time.sleep(1)