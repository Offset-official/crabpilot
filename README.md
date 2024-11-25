# 🦀 CrabPilot

A blazingly fast bare metal firmware to control the BlueRobotics `Navigator Flight Controller`.

Current release only allows to move the AUV in a a forward motion for a set period of time.

## Notes

### AUV thruster configuration:
- Thruster 1 is fit with the counter clockwise propeller.
- Thruster 2 is fit with the clockwise propeller.

### Control
Once the program is executed, after 5 seconds, the thruster will move the AUV in the forward direction for 5 seconds.


## Installation

Follow the offical [instructions](https://www.rust-lang.org/tools/install) to install rust on your device.

1. Clone the repo

```sh
git clone https://github.com/Offset-official/crabpilot
cd crabpilot
```

2. Test build and install dependencies
```sh
cargo build --release
```

## Usage

1. Build the project for production
```sh
cargo build --release
```

2. Install the built binary onto the system path
```sh
cargo install --path .
```

