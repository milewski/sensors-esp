# Passive Buzzer

https://github.com/milewski/sensors-esp/assets/2874967/ad7b49f8-3256-4350-b537-90daa4b9aa3c

This example uses a RMT transmitter to generate pulses at a specific frequency to play a song on a passive buzzer.

Part of the code/songs was translated from C to Rust from this [repository](https://github.com/robsoncouto/arduino-songs).

## Features

- Rotate left/right to change the song.
- Display the index of the song on the 4-digit 7-segment display.
- Push the rotary encoder button to stop the song.

### How to Run

To run the example, use the following command:

```bash
cargo run -p passive-buzzer
```

### Notes

- Active buzzer is louder than passive buzzer.
- 