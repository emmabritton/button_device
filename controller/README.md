# Controller

This program is used with the button device can be used to run scripts when buttons are pressed and the LEDs and display can be used to show the result of automatically run scripts

## Usage

* macOS/Linux
`./controller <config>`
* Windows
`controller.exe <config>`

`<config>` can be omitted if the file is named `config.json` and is in the same directory as `controller` 

## Config

The config is stored as JSON

#### Format
* `device_name` (string, optional)
* `output` (string)
* `display` (object, optional)
  * `freq_amount` (number)
  * `freq_unit` (number)
  * `script` (string)
  * `args` (array(string), optional)
* `leds` (object, optional)
  * `red`, `green`, `blue` (object, optional)
    * `freq_amount` (number)
    * `freq_unit` (number)
    * `script` (string)
    * `args` (array(string), optional)
* `buttons` (array, optional)
  * `script` (string)
  * `args` (array(string), optional)


#### Fields

| Name                    |     | Description                                                               | Values                            | Default   |
|-------------------------|:----|---------------------------------------------------------------------------|-----------------------------------|-----------|
| `device_name`           |     | Serial port name, if blank program will guess, see **Device name**        |                                   |           |
| `output`                |     | Level of info to print to terminal                                        | `none`, `debug` or `all`          | `all`     |
| `freq_amount`           |     | Number in `freq_unit` to execute script                                   | 1 - 59                            | `1`       |
| `freq_unit`             |     | Unit for number in `freq_amount`                                          | `minutes` or `seconds`            | `minutes` |
| `script`                |     | Path to executable to run                                                 |                                   |           |
| `args`                  |     | Arguments to pass to script                                               |                                   |           |

#### Example

```json
{
  "display": {
    "freq_amount": 30,
    "freq_unit": "seconds",
    "script": "./memory_usage.sh"
  },
  "leds": {
    "red": {
      "freq_amount": 5,
      "freq_unit": "minutes",
      "script": "program",
      "args": [
        "$PATH"
      ]
    }
  },
  "buttons": [
    {
      "script": "button_0.sh"
    },
    {
      "script": "button_1.sh",
      "args": ["true"]
    },
    {
      "script": "button_2.sh",
      "args": [
        "$JAVA_HOME"
      ]
    },
    {
      "script": "button_3.sh"
    }
  ]
}
```

## Device Name

This program uses the serial port for the device name, on mac and linux is normally `/dev/tty.usbmodemXXXXX` and on windows `COMX`.

If not included the program will pick the first port that matches the expected format. 

