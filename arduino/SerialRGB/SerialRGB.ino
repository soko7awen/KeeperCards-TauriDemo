#include <Adafruit_NeoPixel.h>

#define PIN        48  // GPIO pin of the built-in NeoPixel

Adafruit_NeoPixel strip = Adafruit_NeoPixel(1, PIN, NEO_GRB + NEO_KHZ800);

void setup() {
  strip.begin();
  strip.show();  // Initialize all pixels to 'off'
  Serial.begin(115200);  // Initialize serial communication at 115200 baud
  Serial.println("Ready to receive commands...");
}

void loop() {
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

      strip.setPixelColor(0, hexColor); 
      strip.show();
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
        strip.setPixelColor(0, strip.Color(r, g, b));  // Set RGB values
        strip.show();
      }
    }
  }

  delay(100);
}
