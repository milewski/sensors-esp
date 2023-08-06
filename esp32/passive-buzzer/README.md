# Passive Buzzer

https://github.com/milewski/sensors-esp/assets/2874967/4fd229d9-babe-495e-8abf-6bdeab3a12ad

This example uses an RMT transmitter to generate pulses at a specific frequency to play a song on a passive buzzer.

> Part of the code/songs was translated from C to Rust from this [repository](https://github.com/robsoncouto/arduino-songs).

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

- Active buzzer is louder than passive one.
- These buzzers have 3 pins, but I have not figured out what is the middle pin used for.
