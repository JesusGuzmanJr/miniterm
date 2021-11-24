# Miniterm

A simple serial terminal.

## Inspiration

The code has been adapted from a Ruby [script](https://github.com/rust-embedded/rust-raspberrypi-OS-tutorials/blob/master/common/serial/miniterm.rb) in the Rust Raspberry Pi OS Tutorials.

## Goals

- Learn to read Ruby scripts.
- Learn about terminals and unix standard streams.
- To facilitate kernel development.

## Discussion

Take note of the [dependencies](https://gitlab.com/susurrus/serialport-rs/-/tree/master#dependencies) requires to interact with the serial port on your computer. You will also need a serial cable for you device.


- Run `just install` to build and install the tool.

## Known Issues

Your computer will thrash when you run miniterm. This is caused by 2 busy loops running on 2 threads. Each one is waiting or shuffling bytes in one direction or the other.