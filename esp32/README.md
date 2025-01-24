Configured for and tested on an ESP32-WROOM-32.

Completed:
- Successfully compiles the Spatial_LED crate.

TODO:
- Output LED data using GPIO pin
- Simulates a time-driven effect
- Make use of Drivers/Schedulers

# Running
Install the following cargo sub-commands:
```shell
cargo install cargo-generate
cargo install ldproxy
cargo install espup
cargo install espflash
```

Flash the program onto your ESP32 and enable monitoring:
```shell
cargo espflash flash --monitor
```