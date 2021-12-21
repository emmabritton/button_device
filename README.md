# Button Device

This project consists of a Rust library, 2 Rust desktop programs, circuit schematic and 3D models for a device with 4 buttons, 3 LEDs and a small screen that can be used with a computer.

## Technical details

The device communicates over the serial port/micro USB. It has three LEDS: green, blue and red; the blue LED is less bright than the other two. The display is 4x21 characters.

## Photos

Circuit board (Front + Back)

[<img src="media/circuit_board_front.jpg" width="256" height="192">](https://raw.githubusercontent.com/raybritton/button_device/master/media/circuit_board_front.jpg)
[<img src="media/circuit_board_back.jpg" width="256" height="192">](https://raw.githubusercontent.com/raybritton/button_device/master/media/circuit_board_back.jpg)


Board Cover

[<img src="media/circuit_board_back.jpg" width="256" height="192">](https://raw.githubusercontent.com/raybritton/button_device/master/media/circuit_board_back.jpg)

Circuit board + Cover (Front + Side)

[<img src="media/circuit_covered_front.jpg" width="256" height="192">](https://raw.githubusercontent.com/raybritton/button_device/master/media/circuit_covered_front.jpg)
[<img src="media/circuit_covered_back.jpg" width="256" height="192">](https://raw.githubusercontent.com/raybritton/button_device/master/media/circuit_covered_back.jpg)

Mount (Front + Side)

[<img src="media/mount_front.jpg" width="256" height="192">](https://raw.githubusercontent.com/raybritton/button_device/master/media/mount_front.jpg)
[<img src="media/mount_back.jpg" width="256" height="192">](https://raw.githubusercontent.com/raybritton/button_device/master/media/mount_back.jpg)

Fully assembled

(Both the cover and mount are damaged as they didn't print correctly, but the models should be correctly sized)

### Hello World Example 

**Device**

[<img src="media/demo_hw.jpg" width="256" height="192">](https://raw.githubusercontent.com/raybritton/button_device/master/media/demo_hw.jpg)

**Test Program**

[<img src="media/ss_hw.png" width="256" height="192">](https://raw.githubusercontent.com/raybritton/button_device/master/media/ss_hw.png)

When a button is pressed, the corresponding square lights up in the demo program

[<img src="media/ss_button.png" width="256" height="192">](https://raw.githubusercontent.com/raybritton/button_device/master/media/ss_button.png)

## Test Program

This can be used to check the device is assembled correctly.

The buttons (four squares at the bottom) light up when the corresponding physical button is pressed. Click the LEDs to set the state of the physical LED. Type printable characters and space to set text, backspace to remove the last letter.

