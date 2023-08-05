# IR Line Tracking Sensor (TCRT5000L) 

This sensor can be used to detect the presence of a line or obstacle, it goes LOW when it detects a line or obstacle and HIGH when it does not.

## Features

- Black or no Obstacle = Logic HIGH
- White or obstacle = Logic LOW
- Detection distance: 1.5cm (tested with white paper)

### How to Run

To run the example, use the following command:

```bash
cargo run -p line-tracking-sensor
```

### Notes

- There are many different types of IR line tracking sensors. This example uses the single bit type, which only has one output pin.
