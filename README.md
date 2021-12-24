# battobs

battops is a simple application which can send mqtt or http requests to power on/off a smart plug.
It monitors the battery level of a laptop. If the configured min percentage is reached the power on 
command is send to the smart plug. If the maximum percentage is reached the power off command is sent.
The purpose is to hold the battery level between a min and max value.

the config.yml looks like:
```yaml
range:
  max: 80
  min: 40
check_interval_sec: 60
connection:
  rest:
    enabled: true
    url: http://192.168.9.91/cm?cmnd={payload}
    payload:
      off: Power Off
      on: Power On
  mqtt:
    enabled: false
    server: 192.168.9.160
    port: 1883
    channel: z2m/laptop_plug/set
    payload:
      off: "{\"state\": \"off\"}"
      on: "{\"state\": \"on\"}"
```
