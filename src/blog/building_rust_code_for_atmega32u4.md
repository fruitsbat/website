![red led on a circuit playground classic blinking](/assets/blink.webp)

this guide will show you how to get a simple **hello world blink example** working on **circuit playground classic**.
if you have a regular **atmega32u4** (the microcontroller that powers the cicuit playground), or another board based on it you should also be able to follow along.

## prerequisites
first you will need to install `gcc-avr` and `avrdude`, but they might be called something else on your distro.

[the avr-hal repo](https://github.com/Rahix/avr-hal) lists `sudo apt install avr-libc gcc-avr pkg-config avrdude libudev-dev build-essential` as a way to get all dependencies on ubuntu


if you are on nixos like me this shell should work:
```nix
{ pkgs ? import <nixpkgs> { } }:
pkgs.mkShell {
  nativeBuildInputs = with pkgs; [
    pkgsCross.avr.buildPackages.gcc
    avrdude
  ];
}
```

you also need to [install rust](https://www.rust-lang.org/tools/install). if you don't have any experience in that language you should also take a look at [the book](https://doc.rust-lang.org/stable/book/) or [rust by example](https://doc.rust-lang.org/stable/rust-by-example/) if you prefer a more hands on approach.

## making the rust project
to make a new project you run `cargo new <whatever-you-feel-like-naming-your-project>`.
i creatively named mine `avrtest`.

### the toolchain
we need the **nightly toolchain** since rust doesn't have avr support in stable.
at the time of writing this (12.4.2023), **the newest nightly toolchain doesn't actually work for this!**
of course this might change in the future, but right now you need to specify that you want
`2023-03-24`.
to do this, run: `rustup override set nightly-2023-03-24`.
if just setting `rustup override set nightly` works for you that's great.

### `Cargo.toml`
in this folder you will see a file called `Cargo.toml`.
this is where details about your project are specified.

you should edit it so it looks a bit like this:
```toml
[package]
name = "avrtest"
version = "0.1.0"
edition = "2021"

[profile.release]
# tell your program what to do when a panic happens
panic = "abort"
# link time optimization
# our board has very little storage!
# this helps with making the binary smaller
lto = true
# strip debug symbols to save even more space
strip = true
# rust is better at optimizing when compiling on only one core
codegen-units = 1
# optimize for size instead of speed
# remove this if your code is too slow
opt-level = "z"

[profile.dev]
panic = "abort"

[dependencies]
panic-halt = "0.2"

# this is a hardware abstraction layer
# for the atmega32u4
# it will help you program the board without
# having to think of bare registers
# and guarantee at least some level of safety
[dependencies.atmega-hal]
git = "https://github.com/Rahix/avr-hal"
rev = "e29a7be"
features = ["atmega32u4"]

# this provides access to the boards
# hardware
# you will most likely not have to use it a lot
# but we do need the entry macro from here
[dependencies.avr-device]
version = "0.5.1"
features = [
  "atmega32u4",
  # this is needed for 
  # the entry function
  "rt"
]
```

### the target
rust does not have the atmega32u4 as a builtin **target**.
this means we get to have some fun and build our own!

(this is a lie. i have actually stolen mine from [this file](https://raw.githubusercontent.com/Rahix/avr-hal/main/avr-specs/avr-atmega32u4.json))
```json
{
  "arch": "avr",
  "atomic-cas": false,
  "cpu": "atmega32u4",
  "data-layout": "e-P1-p:16:8-i8:8-i16:8-i32:8-i64:8-f32:8-f64:8-n8-a:8",
  "eh-frame-header": false,
  "exe-suffix": ".elf",
  "executables": true,
  "late-link-args": {
    "gcc": [
      "-lgcc"
    ]
  },
  "linker": "avr-gcc",
  "llvm-target": "avr-unknown-unknown",
  "max-atomic-width": 8,
  "no-default-libraries": false,
  "pre-link-args": {
    "gcc": [
      "-mmcu=atmega32u4"
    ]
  },
  "target-c-int-width": "16",
  "target-pointer-width": "16"
}
```
you should be able to get away with pasting this into a file called `atmega32u4.json`, or whatever else seems sensible to you.
just make sure to place it **at the root of your project directory**.

we also need to **tell rust about this file**.
to do that make a folder called `.cargo`
and place a file called `config.toml` inside it.
that file should look like this:

```toml
[build]
# telling rust where to find the file
target = "atmega32u4.json"

[unstable]
# telling rust to just build the 
# core library instead of the standard one
build-std = ["core"]
```

## programming, finally
now we can finally start programming!
open `src/main.rs`
```rust
#![no_std]
#![no_main]
#![feature(lang_items)]

use atmega_hal::{
    clock, delay::Delay, pins, prelude::_embedded_hal_blocking_delay_DelayMs, Peripherals,
};
use core::panic::PanicInfo;


/**
what to do when your code program crashes
just looping forever is definitely not the most ideal way to handle this
but it works for now
and your code will never crash anyways :)
*/
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}


/**
it actually does not matter what this function is called
since we used the no_main macro, but this IS still our
main function
because of avr_device::entry
*/
#[avr_device::entry]
fn main() -> ! {
    let peripherals = Peripherals::take().unwrap();
    let pins = pins!(peripherals);
    let mut delay = Delay::<clock::MHz8>::new();
    // pc7 is the red led on circuit playground classic
    // you might need to change this pin if you are using a different board
    let mut led = pins.pc7.into_output();
    loop {
        led.toggle();
        delay.delay_ms(1000 as u16);
    }
}
```

## building!
now it's time to build!
go to the root of your project and run
`cargo build --release`. (you should always **build in release mode** because there is limited processing power and storage)

this will give you a `.elf` file: `target/atmega32u4/release/<the-name-you-entered>.elf`.

## flashing!
you can now flash this by running
`avrdude -p m32u4 -c avr109 -P /dev/ttyACM0 -U flash:w:target/atmega32u4/release/name-you-entered.elf -b 9600`

- `m32u4` is your board 
- `avr109` is the programmer type
- `/dev/ttyACM0` is the serial port your board is using (if you are unsure about that, check in the arduino ide)
- `9600` is the baud rate
- `target/atmega32u4/release/name-you-entered.elf` is your file

some of those might be different for you, so watch out!

## gaze upon your creation!
the red led on your board should now have started blinking!
**congratulations**!
