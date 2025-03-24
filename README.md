# Peak CAN Sys

`peak-can-sys` provides Rust FFI bindings to the PEAK-System PCAN-Basic API, enabling interaction with PEAK CAN interfaces in Rust applications.

## Features
- Low-level FFI bindings to the PCAN-Basic API
- Compatible with Windows and Linux systems
- Supports various PEAK CAN hardware

## Installation
To use `peak-can-sys`, ensure that you have the PCAN-Basic library installed on your system.

### Windows
1. Download and install the PEAK PCAN-Basic package from [PEAK-System Drivers](https://www.peak-system.com/Drivers.523.0.html?&L=1).
2. Ensure that `PCANBasic.dll` is available in your system path.

### Linux
1. Install `libpcanbasic.so` from PEAK-System's official drivers.
2. Verify that the shared library is accessible in your system.

## Usage
Add `peak-can-sys` to your `Cargo.toml`:
```toml
[dependencies]
peak-can-sys = "0.1"
```

Include it in your Rust code:
```rust
use peak_can_sys::*;
```

## License & Legal Notice
This project is **not affiliated** with PEAK-System. The PCAN-Basic API is owned and maintained by PEAK-System.

- PEAK-System is the owner of the PCAN-Basic library.
- Users must download PCAN-Basic from the official [PEAK-System website](https://www.peak-system.com/Drivers.523.0.html?&L=1).
- By using this crate, you agree to PEAK-System's [End User License Agreement (EULA)](https://www.peak-system.com/EULA.495.0.html).

For support related to this Rust binding, please contact the maintainer of this repository, **not** PEAK-System.

