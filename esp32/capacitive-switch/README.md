# Capacitive Switch (Touch Sensor) (HW-138)

https://github.com/milewski/sensors-esp/assets/2874967/b1282f06-6cc8-4f18-97e7-f6fb12d65381

This example uses `interrupts` to detect when the capacitive switch is touched.

## Features

- Guess the random generated number by touching the capacitive switch.

### How to Run

To run the example, use the following command:

```bash
cargo run -p capacitive-switch
```

### Notes

- There are some functions that can be enabled / disabled on the board by bridging some of the extra pins it provides: https://www.youtube.com/watch?v=8_GjbO8Nru0
