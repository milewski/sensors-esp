# ADC/DAC Analog Digital Converter (PCF8591)

https://github.com/milewski/sensors-esp/assets/2874967/7f22f0f1-59ac-43df-bbaf-7cc244e60b11

The esp32 s3 board already includes many built-in DAC pins, making this module somewhat redundant for use with it.
However, using it as a learning exercise can be valuable. One advantage I noticed is that I could connect 2 joysticks to
a single I2C interface. I could potentially also connect the display to the same I2C interface since it has different
addresses. This means I could achieve this demo using just 2 pins (plus power and ground).

You might have noticed a lot of jumper wires in the demo above. This was because the modules I used were not
breadboard-friendly, so I used the wires to keep them in place for recording.

One downside of this module is its low resolution, which is only 8 bits. This may or may not be an issue for your
project.

The board also features an Analog OUT. According to the datasheet, you can write a digital value and have it converted
into an analog signal. I found a [video](https://www.youtube.com/watch?v=VathrA8RGCU) demonstrating this functionality,
although I haven't found a use case for it yet.

Additionally, this module comes with 3 jumpers that connect to its built-in sensors: 1 potentiometer, 1 LDR (
light-dependent resistor), and a thermistor. The jumpers connect these sensors to A0-A3, so you can read their values
directly. While I haven't shown this in this repository, it's a straightforward process. You can copy and paste the DAC
example from here: https://github.com/esp-rs/esp-idf-hal/blob/master/examples/adc.rs.

### How to Run

```bash
cargo run -p adc-dac
```

### Notes

- Datasheet: https://www.nxp.com/docs/en/data-sheet/PCF8591.pdf
