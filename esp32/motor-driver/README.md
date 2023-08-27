# Motor Driver 

https://github.com/milewski/sensors-esp/assets/2874967/6445422a-3ea4-4404-b6cb-2289ac1ca230

The original kit was intended to be done with an `STC 89C5RC` microcontroller, and probably written in C? Arduino? but I
have used an `ESP32` and `Rust` instead for simplicity.

### Features

Use a `Lily-Go T-Display S3` board to control a car with 2 motors and a caster wheel.
The car is controlled by a `DVR8833` motor driver, which is controlled by the `ESP32` using `PWM` signals via BLE.
The companion app is written in `Flutter` and can be found [here](./remote-control).

### Components

- Motor Driver (DVR8833)
- A RGB LED
- 2x Motors (Which I have no idea about its specs)
- A mini power supply (HW-131)
- An Lily-Go T-Display S3
- 2x wheels
- 1x caster wheel
- 1x PCB car chassis (although I'm not using the printed circuit that came with it)
- Wires

### How to Run

To run the example, use the following command:

```bash
cargo run -p motor-driver
```

### Notes

- When using a `Lily-Go T-Display S3` board the pin 15 needs high to be able to power it from the 5v pin instead of USB.
