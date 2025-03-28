#include <Adafruit_NeoPixel.h>

#define NEO_PIN        48  // GPIO pin of the built-in NeoPixel

Adafruit_NeoPixel neo_strip = Adafruit_NeoPixel(1, NEO_PIN, NEO_GRB + NEO_KHZ800);

const int monitoredPins[] = {1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 15, 16, 17, 35, 36, 37, 38, 39, 41, 46, 47}; // Usable GPIOs on ESP32-S3
const int numPins = sizeof(monitoredPins) / sizeof(monitoredPins[0]);
bool lastStates[sizeof(monitoredPins) / sizeof(monitoredPins[0])];

void setup() {
    Serial.begin(115200);
    
    for (int i = 0; i < numPins; i++) {
        pinMode(monitoredPins[i], INPUT_PULLUP); // Enable pull-up resistor
        lastStates[i] = HIGH; // Assume initially unconnected
    }
}

void loop() {
  checkGPIO();
  checkRGB();
  delay(50); // Small debounce delay
}

void checkGPIO() {
  for (int i = 0; i < numPins; i++) {
      int pin = monitoredPins[i];
      bool currentState = digitalRead(pin);

      if (currentState != lastStates[i]) {
          if (currentState == LOW) {
              Serial.printf("{\"state\":\"connected\", \"pin\":%d}\n", pin);
          } else {
              Serial.printf("{\"state\":\"disconnected\", \"pin\":%d}\n", pin);
          }
          lastStates[i] = currentState;
      }
  }
}

void checkRGB() {
  if (Serial.available() > 0) {
    String input = Serial.readStringUntil('\n');  // Read the entire line

    // Check for hex
    if (input.startsWith("#")) {
      uint32_t hexColor = 0;

      if (input.length() == 7) {  // Full hex
        hexColor = strtoul(input.substring(1).c_str(), NULL, 16);
      } 
      else if (input.length() == 4) {  // Shorthand hex
        char expandedHex[7];
        for (int i = 0; i < 3; i++) {
          expandedHex[i * 2] = input.charAt(i + 1);
          expandedHex[i * 2 + 1] = input.charAt(i + 1);
        }
        expandedHex[6] = '\0';  // Null terminator
        
        hexColor = strtoul(expandedHex, NULL, 16);
      }

      neo_strip.setPixelColor(0, hexColor); 
      neo_strip.show();
    }
    // If the input is in rgb(X,X,X) format
    else if (input.startsWith("rgb(") && input.endsWith(")")) {
      input = input.substring(4, input.length() - 1);  // Remove "rgb(" and ")"
      int r, g, b;
      if (sscanf(input.c_str(), "%d,%d,%d", &r, &g, &b) == 3) {
        Serial.print("RGB Color - R: ");
        Serial.print(r);
        Serial.print(" G: ");
        Serial.print(g);
        Serial.print(" B: ");
        Serial.println(b);
        neo_strip.setPixelColor(0, neo_strip.Color(r, g, b));  // Set RGB values
        neo_strip.show();
      }
    }
  }
}
