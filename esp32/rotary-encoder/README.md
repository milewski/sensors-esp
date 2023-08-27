# Rotary Encoder

https://github.com/milewski/sensors-esp/assets/2874967/8d4bf154-145e-4edb-81a7-836eb3e7a933

This repository contains an example of how to read the rotation/direction from a rotary encoder and display the value on a 4-digit 7-segment display.

## Features

- Rotate left/right to adjust the counter value.
- Quickly press to clear the counter value.
- Press and hold to adjust the display brightness.

### How to Run

To run the example, use the following command:

```bash
cargo run -p rotary-encoder
```

### Notes

- The current implementation may be inefficient as it pulls values on every cycle, resulting in unnecessary CPU cycles being consumed. A more efficient approach would be to utilize a timer interrupt.
- Initially, I followed this [tutorial](https://lastminuteengineers.com/rotary-encoder-arduino-tutorial) and translated the original `C` implementation to `Rust`. However, this approach proved to be buggy and inaccurate, as it occasionally missed rotations and miss-detected the spin direction.
- Fortunately, I discovered a more accurate algorithm for rotary encoders [here](https://www.best-microcontroller-projects.com/rotary-encoder.html) and subsequently found a [library](https://crates.io/crates/rotary-encoder-embedded) that implements it.
