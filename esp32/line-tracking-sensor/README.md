# IR Line Tracking Sensor (TCRT5000L)

https://github.com/milewski/sensors-esp/assets/2874967/ad0a39f7-263d-4df9-998c-547a0c8fdabe

An IR line tracking sensor is a type of sensor that uses infrared (IR) light to detect the presence of a bright surface.
It does this by emitting IR light and detecting the amount of light that is reflected back to the sensor.

## Features

- Black or no obstacle = Outputs HIGH
- White or obstacle = Outputs LOW
- Detection distance: 1.5cm (tested with white paper)

### How to Run

To run the example, use the following command:

```bash
cargo run -p line-tracking-sensor
```

### Notes

- There are many different types of IR line tracking sensors. This example uses the single bit type, which only has one
  output pin.
- For better accuracy and precision, the sensor should be calibrated to the surface it is detecting. This can be done by
  adjusting the potentiometer on the sensor.
