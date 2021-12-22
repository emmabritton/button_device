//COMMS
// [command][data]
//  OUTPUT
//   Commands:
//     0x01 BUTTON PRESSED, DATA is [0 - 3] for button 1 - 4
//     0x02 BUTTON RELEASED, DATA is [0 - 3] for button 1 - 4
//  INPUT
//   Commands:
//     0x03 SET LED, DATA is [0 - 1, 0 - 1] for LED 0 blue, 1 red and state 0 off, 1 on
//     0x04 SET TEXT, DATA is [ASCII] for 84 printable ASCII chars

#include <Wire.h>
#include <Adafruit_GFX.h>
#include <Adafruit_SSD1306.h>

#define SCREEN_WIDTH 128 
#define SCREEN_HEIGHT 32 

#define OLED_RESET     -1
#define SCREEN_ADDRESS 0x3C
Adafruit_SSD1306 display(SCREEN_WIDTH, SCREEN_HEIGHT, &Wire, OLED_RESET);

const int LED_BOARD_GREEN = 2;
const int LED_BOARD_BLUE = 3;
const int LED_BOARD_RED = 4;

const int BUTTON_ONE = A7;
const int BUTTON_TWO = A6;
const int BUTTON_THREE = A3;
const int BUTTON_FOUR = A2;

const int B_RELEASED = 0;
const int B_PRESSING = 1;
const int B_PRESSED = 2;
const int B_RELEASING = 3;

const byte COMMAND_BUTTON_PRESSED = 0x01;
const byte COMMAND_BUTTON_RELEASED = 0x02;
const byte COMMAND_SET_LED = 0x03;
const byte COMMAND_SET_TEXT = 0x04;
const byte COMMAND_SYNC = 0x05;

const byte COMMAND_LED_BLUE = 0;
const byte COMMAND_LED_RED = 1;
const byte COMMAND_LED_GREEN = 2;
const byte COMMAND_LED_OFF = 0;
const byte COMMAND_LED_ON = 1;

int buttons[] = {B_RELEASED, B_RELEASED, B_RELEASED, B_RELEASED};

void setup() {
  Serial.begin(9600);
  
  pinMode(LED_BOARD_GREEN, OUTPUT);
  pinMode(LED_BOARD_BLUE, OUTPUT);
  pinMode(LED_BOARD_RED, OUTPUT);
  pinMode(BUTTON_ONE, INPUT);
  pinMode(BUTTON_TWO, INPUT);
  pinMode(BUTTON_THREE, INPUT);
  pinMode(BUTTON_FOUR, INPUT);

  digitalWrite(LED_BOARD_RED, HIGH);

  if (!display.begin(SSD1306_SWITCHCAPVCC, SCREEN_ADDRESS)) {
    Serial.println("Display failed to start");
    return;
  }

  display.setRotation(2);
  display.clearDisplay();
  display.setTextSize(1);
  display.setTextColor(SSD1306_WHITE);
  display.setCursor(0,0);
  display.println("Ready...");
  display.display();

  digitalWrite(LED_BOARD_RED, LOW);
  digitalWrite(LED_BOARD_GREEN, HIGH);
}

void loop() {
  handleButton(BUTTON_ONE, 0);
  handleButton(BUTTON_TWO, 1);
  handleButton(BUTTON_THREE, 2);
  handleButton(BUTTON_FOUR, 3);

  for (byte i = 0; i < 4; i++) {
    byte num = i;
    byte data[] = {0, num};
    switch (buttons[i]) {
      case B_PRESSING:
        data[0] = COMMAND_BUTTON_PRESSED;
        Serial.write(data, 2);
        buttons[i] = B_PRESSED;
        break;
      case B_RELEASING:
        data[0] = COMMAND_BUTTON_RELEASED;
        Serial.write(data, 2);
        buttons[i] = B_RELEASED;
        break;
    }
  }

  if (Serial.available() > 0) {
    switch (Serial.read()) {
      case COMMAND_SET_LED: {
        int led = Serial.read();
        int state = Serial.read() == 1 ? HIGH : LOW;
        switch (led) {
          case COMMAND_LED_BLUE:
            digitalWrite(LED_BOARD_BLUE, state);
            break;
          case COMMAND_LED_GREEN:
            digitalWrite(LED_BOARD_GREEN, state);
            break;
          case COMMAND_LED_RED:
            digitalWrite(LED_BOARD_RED, state);
            break;
        }
        break;
      }
      case COMMAND_SET_TEXT: {
        char text[85];
        for (int i = 0; i < 84; i++) {
          text[i] = Serial.read();
        }
        display.clearDisplay();
        display.setCursor(0,0);
        display.println(text);
        display.display();
        break;
      }        
    }
  }

  delay(1);
}

void handleButton(int pin, int idx) {
  // HIGH, RELEASED -> PRESSING
  // HIGH, PRESSING //not possible
  // HIGH, PRESSED //do nothing
  // HIGH, RELEASING //not possible
  // LOW, RELEASED //do nothing 
  // LOW, PRESSING //not possible
  // LOW, PRESSED -> RELEASING
  // LOW, RELEASING //not possible
  if (digitalRead(pin) == HIGH && buttons[idx] == B_RELEASED) {
    buttons[idx] = B_PRESSING;
  } else if (digitalRead(pin) == LOW && buttons[idx] == B_PRESSED) {
    buttons[idx] = B_RELEASING;
  }
}
