# Embassy RP Pico template

## Setup

Use a debug probe like a second Raspberry Pi Pico running [`picoprobe.uf2`][picoprobe-releases] or Raspberry Pi Debug probe running [` debugprobe.uf2`][picoprobe-releases] (a CMSIS v2 compatible firmware).

[picoprobe-release]: https://github.com/raspberrypi/picoprobe/releases

## Running

To run the firmware on the Pico you need to specify the target, disable default features and enable the `firmware` feature like so:
`cargo run --target thumbv6m-none-eabi --no-default-features -F firmware`

Or use the provided alias in [./cargo/config.toml](./cargo/config.toml):

`cargo run-firmware`

## Testing

Run test as regular crate or project by using `cargo test` which will use your host target and the `std` feature (enabled by default)

# License

MIT or APACHE-2.0