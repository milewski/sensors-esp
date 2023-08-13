# Capacitive Switch (HW-138)

https://github.com/milewski/sensors-esp/assets/2874967/a2bc9a32-4bb4-4a30-bed7-615bd0ee7357

Read / Write data to a SD Card module through an SPI Interface.

## Features

- Show the root directory of the SDCARD into the display.
- Use a rotary encoder to scroll the list up / down.

### How to Run

To run the example, use the following command:

```bash
cargo run -p micro-sdcard
```

### Notes

- There are some functions that can be enabled / disabled on the board by bridging some of the extra pins it provides: https://www.youtube.com/watch?v=8_GjbO8Nru0
