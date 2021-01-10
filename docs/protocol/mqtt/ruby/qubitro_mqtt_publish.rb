require 'qubitro-mqtt'

deviceID = "PASTE_DEVICE_ID"
deviceToken = "PASTE_DEVICE_TOKEN"
host = "broker.qubitro.com"

payload = ''

MQTT::Client.connect(
  :host => host,
  :port => 8883,
  :ssl => true,
  :client_id => deviceID,
  :device_id => deviceID,
  :device_token => deviceToken,
  :ack_timeout => 15,
  :will_topic => deviceID,
  :will_payload => payload,
  ) do |client|
    puts 'Connected to Qubitro!'
    loop do
      random = Random.rand(35...38).to_s
      random2 = Random.rand(20...30).to_s
      payload = '{"Temperature":'+random2+',"Value":'+random+'}'
      client.publish(deviceID, payload)
      puts 'published: ' + payload
      sleep(2)
  end
end 