# DTMF Decoding / MT8870

https://github.com/milewski/sensors-esp/assets/2874967/e73e97f9-6360-4ea8-a68f-89fd8dab26f5

This is an interesting module that can decode DTMF tones and transmit them over a 4-bit bus. The way it operates is whenever it detects a tone, the pin named `STQ` goes high. Then, you can read each pin Q1-Q4 to obtain a 4-bit value representing which digit the tone is equivalent to.

It's probably possible to connect a microphone to the `IN` input and decode DTMF tones over the air. However, I currently don't have a microphone module, so I used a tone generator app on my phone to produce the tones.

### How to Run

```bash
cargo run -p dtmf
```

### Notes

- Component details: https://components101.com/modules/mt8870-dtmf-decoder-module