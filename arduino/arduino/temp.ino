#define END_BIT 255
#define DELAY 500
#define START_PIN 2
#define END_PIN 6

void setup() {
    Serial.begin(9600);
    for (int i = START_PIN; i <= END_PIN; i++) pinMode(i, INPUT);
}

void send_signal(int number) {
    Serial.write(number);
    Serial.write(END_BIT);
    Serial.flush();
    delay(DELAY);
}

void check_state(int pin) {
    int button_state = digitalRead(pin);

    if (button_state == HIGH) send_signal(pin);
}

void loop() {
    for (int i = START_PIN; i <= END_PIN; i++) {
        check_state(i);
    }
}

