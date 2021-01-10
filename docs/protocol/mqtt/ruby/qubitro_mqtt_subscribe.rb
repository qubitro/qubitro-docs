require 'qubitro-mqtt'

deviceID = "PASTE_DEVICE_ID"
deviceToken = "PASTE_DEVICE_TOKEN"
host = "broker.qubitro.com"

MQTT::Client.connect(
  :host => host,
  :port => 8883,
  :ssl => true,
  :client_id => deviceID,
  :device_id => deviceID,
  :device_token => deviceToken,
  :ack_timeout => 15,
  :will_topic => deviceID,
  ) do |client|
    puts 'Connected to Qubitro!'
    client.get("PASTE_TOPIC") do |topic, msg|
      puts "Received data '#{msg}' on topic '#{topic}'"
    end
end 