# Read Temperature and Humidity
This project is about using Rust to program an Arduino to read temperature and humidity data from a DHT11.

## Installation

We need to setup a few things to use Rust with AVR support. This was taken directly from the [AVR-Rust](https://book.avr-rust.com/002-installing-the-compiler.html) book.

0. Install Rust and rustup.

1. Install the Official Rust nightly compiler and the `rust-src` component.

`$ rustup toolchain install nightly`

`$ rustup component add rust-src --toolchain nightly`

2. Install thirdparty tools using the OS package manager. The ones below are for MacOS.

`brew install avr-binutils avr-gcc avrdude`

## Project Setup

To use the project as is, clone this project and set rust to use the nightly toolchain
```
$ git clone https://github.com/avr-rust/blink.git
$ cd blink
$ rustup override set nightly
```
To create the project manually,here is a guide.

We need to choose an IO library to communicate using the Arduino. For this project, I chose an AVR implementation of the [embedded hal](https://github.com/rust-embedded/embedded-hal) called [avr hal](https://github.com/Rahix/avr-hal). The project being setup follows [avr hal](https://github.com/Rahix/avr-hal) with some modifications to make it work with an Arduino Uno.

Create a rust project using cargo.

`cargo new rustTemperature`

We need to set Rust to use the nightly toolchain for this project. change directory into the project folder `rustTemperature` and then type

`rustup override set nightly`

Copy the contents of the `cargo.toml` [file](https://github.com/arvizard1/rust-arduino-dht11/commit/65cc2a869f3c005e69522636f4e954e8f6af3ae5#diff-2e9d962a08321605940b5a657135052fbcef87b5e360662bb527c96d9a615542). This would setup the project with the necessary dependencies. The file has been setup specifically for Arduino Uno and is using the latest commit of the git project avr-hal at the time of writing this.

There is also `.cargo/config.toml` [file](https://github.com/arvizard1/rust-arduino-dht11/commit/65cc2a869f3c005e69522636f4e954e8f6af3ae5#diff-9a4f3e4537ebb7474452d131b0d969d89a51286f4269aac5ef268e712be17268) that needs to be copied. This helps in reducing the length of the line of `cargo build` by storing some of the build settings. One of the settinngs is the target.

The Rust nightly compiler includes a built-in target for ATmega328 named avr-unknown-gnu-atmega328. However, using that to build caused compile issues. I then used the json file from [here](https://github.com/Rahix/avr-hal/blob/master/avr-specs/avr-atmega328p.json) which worked.

Setup a basic file for the project. I used the [blink](https://github.com/Rahix/avr-hal/blob/master/boards/arduino-uno/examples/uno-blink.rs) example. Copy this file over to the contents of main.rs in the src folder.

### Build Project

Run `cargo build` or `cargo build --release`

The elf file that needs tto be flashed to the Arduino Uno is now ready and will be present in the debug or release folder based on the build command used earlier. If you used the `--release` flag, then the elf would be in the target/avr-atmega32u4/release folder.

The last step is to flash the Arduino. This can be done using `avr-dude`.

`avrdude -q -patmega328p -carduino -P/dev/cu.usbmodem14701 -D "-Uflash:w:rustTemperature.elf:e"`

You would need to change `/dev/cu.usbmodem14701` to the serial path of the Arduino.

Flashing could also be made an automatic process using a cargo runner by updating the `config.toml` with

```
[target.'cfg(target_arch = "avr")']
runner = "./uno-runner.sh"
```

uno-runner file [here](https://raw.githubusercontent.com/Rahix/avr-hal/master/boards/arduino-uno/uno-runner.sh)