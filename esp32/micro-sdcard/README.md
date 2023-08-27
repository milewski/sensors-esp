# Micro SD Card (HW-125)

https://github.com/milewski/sensors-esp/assets/2874967/3a201961-1b17-4acc-9759-398c52dc3896

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

- Current libraries only supports FAT16 and FAT32 file systems.
