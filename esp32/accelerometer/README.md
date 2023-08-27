# Accelerometer (ADXL345)

https://github.com/milewski/sensors-esp/assets/2874967/de9e5b50-550e-41a6-889d-e5e6d7d1fa5b

This accelerometer is a 3-axis accelerometer and can be wired via I2C or SPI, this example uses I2C.

There was some extra functions provided by the hardware like single tap, double tap, activity, inactivity, free fall
detection etc. which were not implemented in this example.

## Features

### How to Run

To run the example, use the following command:

```bash
cargo run -p accelerometer
```

### Notes

- Datasheet: https://www.analog.com/media/en/technical-documentation/data-sheets/adxl345.pdf
- Sample driver implementation in https://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git/tree/drivers/input/misc/adxl34x.c?id=HEAD#n256