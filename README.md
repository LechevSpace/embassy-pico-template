# Embassy RP Pico template

[![sponsor-us]](https://github.com/sponsors/LechevSpace)

## Setup

Use a debug probe like a second Raspberry Pi Pico running [`picoprobe.uf2`][picoprobe-releases] or Raspberry Pi Debug probe running [`debugprobe.uf2`][picoprobe-releases] (a CMSIS v2 compatible firmware).

[picoprobe-releases]: https://github.com/raspberrypi/picoprobe/releases

## Running

To run the firmware on the Pico you need to specify the target, disable default features and enable the `firmware` feature like so:
`cargo run --target thumbv6m-none-eabi --no-default-features -F firmware`

Or use the provided alias in [./.cargo/config.toml](./.cargo/config.toml):

`cargo run-firmware`

### Logging

By default we use `defmt` for logging and include a variaty of macros for both `log` & `defmt` seemless integration in your project.

Use the `DEFMT_LOG` environment variable to set the necessary verbosity.
`DEFMT_LOG=info` is the default which can be tweaked in [./.cargo/config.toml](./.cargo/config.toml).

## Testing

Run test as regular crate or project by using `cargo test` which will use your host target and the `std` feature (enabled by default)

# License

Licensed under either of [Apache License, Version 2.0](./LICENSE-APACHE)
or [MIT license](./LICENSE-MIT) at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this project by you, as defined in the Apache-2.0 license,
shall be dual licensed as above, without any additional terms or conditions.

[sponsor-us]: https://img.shields.io/github/sponsors/LechevSpace?color=bf3989&label=Sponsor%20us&style=for-the-badge&logoColor=bf3989&logo=data%3Aimage%2Fsvg%2Bxml%3Bbase64%2CPHN2ZyBoZWlnaHQ9IjE2IiB2aWV3Qm94PSIwIDAgMTYgMTYiIHZlcnNpb249IjEuMSIgd2lkdGg9IjE2IiB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciPgogICAgPHBhdGggZmlsbD0iI2JmMzk4OSIgZmlsbC1ydWxlPSJldmVub2RkIiBkPSJNNC4yNSAyLjVjLTEuMzM2IDAtMi43NSAxLjE2NC0yLjc1IDMgMCAyLjE1IDEuNTggNC4xNDQgMy4zNjUgNS42ODJBMjAuNTY1IDIwLjU2NSAwIDAwOCAxMy4zOTNhMjAuNTYxIDIwLjU2MSAwIDAwMy4xMzUtMi4yMTFDMTIuOTIgOS42NDQgMTQuNSA3LjY1IDE0LjUgNS41YzAtMS44MzYtMS40MTQtMy0yLjc1LTMtMS4zNzMgMC0yLjYwOS45ODYtMy4wMjkgMi40NTZhLjc1Ljc1IDAgMDEtMS40NDIgMEM2Ljg1OSAzLjQ4NiA1LjYyMyAyLjUgNC4yNSAyLjV6TTggMTQuMjVsLS4zNDUuNjY2LS4wMDItLjAwMS0uMDA2LS4wMDMtLjAxOC0uMDFhNy42NDMgNy42NDMgMCAwMS0uMzEtLjE3IDIyLjA3NSAyMi4wNzUgMCAwMS0zLjQzNC0yLjQxNEMyLjA0NSAxMC43MzEgMCA4LjM1IDAgNS41IDAgMi44MzYgMi4wODYgMSA0LjI1IDEgNS43OTcgMSA3LjE1MyAxLjgwMiA4IDMuMDIgOC44NDcgMS44MDIgMTAuMjAzIDEgMTEuNzUgMSAxMy45MTQgMSAxNiAyLjgzNiAxNiA1LjVjMCAyLjg1LTIuMDQ1IDUuMjMxLTMuODg1IDYuODE4YTIyLjA4IDIyLjA4IDAgMDEtMy43NDQgMi41ODRsLS4wMTguMDEtLjAwNi4wMDNoLS4wMDJMOCAxNC4yNXptMCAwbC4zNDUuNjY2YS43NTIuNzUyIDAgMDEtLjY5IDBMOCAxNC4yNXoiPjwvcGF0aD4KPC9zdmc%2BCg%3D%3D
