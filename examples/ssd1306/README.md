# SSD1306 Example

The structure for this example was generated using [knurling-rs/app-template](https://github.com/knurling-rs/app-template). It runs on an [STM32F3DISCOVERY board](https://www.st.com/en/evaluation-tools/stm32f3discovery.html) with a connected SSD1306 (128x64 pixel) display.

## Setup

This example requires the nightly version of Rust.

### Software

#### 1. `flip-link`:

```console
$ cargo install flip-link
```

#### 2. `probe-run`:

``` console
$ # make sure to install v0.2.0 or later
$ cargo install probe-run
```

### Hardware

The only required hardware is a SSD1306 display and an STM32F3DISCOVERY board.

Make the following connections:

| SSD1306 | STM32F3DISCOVERY |
|---------|------------------|
| GND     | GND              |
| VCC     | 5V               |
| SCL     | PB6              |
| SDA     | PB7              |

## Running

```console
cargo run
```
