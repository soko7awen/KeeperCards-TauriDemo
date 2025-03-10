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

    delay(50); // Small debounce delay
}
