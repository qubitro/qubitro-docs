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

const data = {
  key: 30,
  key2: 35
}

var sdata = JSON.stringify(data)

client.on('connect', function() {
  console.log("Connected to Qubitro!");
  console.log("Publishing data... ")
    setInterval(
        function(){
            client.publish(deviceId, sdata,function(){
                console.log("Published ", sdata)
            });
        },
    2000)
});

client.on('error', function(err) {
    console.log(err);
});