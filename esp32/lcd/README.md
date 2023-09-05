# Liquid Crystal Display (LCD 1602A)

https://github.com/milewski/sensors-esp/assets/2874967/0ef1889a-b7da-4e31-a9fd-872e113114c0

This example uses the `lcd` display connected through a 8bit `I2C` expander `8574T` to condense the number of pins required to drive the display.

### How to Run

```bash
cargo run -p lcd
```

### Notes

- Pretty good explanation on how the LCD works: https://www.youtube.com/watch?v=cXpeTxC3_A4
- This video had some nice visualization of how the data is sent to the LCD: https://www.youtube.com/watch?v=vV8FbwobrKY
- And this explains in depth everything there is to know about the LCD: https://www.youtube.com/watch?v=wEbGhYjn4QI