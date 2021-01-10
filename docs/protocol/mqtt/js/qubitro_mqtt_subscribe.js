var mqtt = require('mqtt')

var broker = 'mqtts://broker.qubitro.com'
var deviceId  = 'PASTE_DEVICE_ID'
var deviceToken  = 'PASTE_DEVICE_TOKEN'

var options = {
  keepalive: 60,
  port: 8883,
  username: deviceId,
  password: deviceToken,
  clientId: deviceId,
  clean: false,
  secureProtocol: 'TLSv1_method',
};

var client = mqtt.connect(broker, options);

client.on('connect', function() {
    console.log("Connected to Qubitro!");
    client.subscribe("TOPIC");
    client.on('message', function (topic, message) {
    console.log("Received '" + message + "' on '" + topic + "'");
    });
});

client.on('error', function(err) {
    console.log(err);
});