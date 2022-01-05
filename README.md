# Test the uSDHC-Card connection

If the uSDHC connection is fine, it will be converted into a lib, (for sure).

## Deploy

Connect the teensy to the usb port and set it to the bootloader mode.

Use the teensy-cli-loader to deploy the app

```bash
cargo objcopy --release -- -O ihex test.hex && teensy4.1 test.hex
```

Uses the `teensy4.1` alias (`alias teensy4.1='teensy_loader_cli --mcu=TEENSY41 -w '`)
