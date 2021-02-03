# *CleanIt*

<p align="center">
  <img src="https://github.com/Sollimann/CleanIt/blob/master/roomba.gif">
</p>

#### _Open-source Autonomy Software in Rust-lang with gRPC for the Roomba series robot vacuum cleaners_

## Motivation

Motivation is to build a complete DIY autonomy software from scratch (motion planning, guidance and motion control, SLAM, mission control, 2D/3D visualization etc..) with a real-time client-server communication stream using async gRPC for intercommunication and distributed compute.

 ## Structure
 
 #### Project Layout
 ```bash
 ├── Cargo.toml (workspace)
 ├── Cargo.lock
 ├── api (bin)
 |   └── server
 ├── core (bin)
 |   └── mission
 |   └── motion
 |   └── slam
 |   └── perception
 |   └── risk
 ├── drivers (lib)
 |   └── roomba
 |   └── realsense
 |   └── rplidar
 |   └── raspberryPi
 └── proto
 |   └── state.proto
 |   └── map2D.proto
 └── visualization (bin)
 |   └── urdf
 |   └── map
 |   └── camera
 ```
 
#### Crate/package Layout
```bash
├── Cargo.toml
├── Cargo.lock
├── src
│   ├── main.rs
│   ├── lib.rs
│   └── bin
│       └── another_executable.rs
├── tests
│   └── some_integration_tests.rs
├── benches
│   └── simple_bench.rs
└── examples
    └── simple_example.rs
```

## Pre-requisites

### *Software*

#### _Linux_
```bash
$ [sudo] apt-get install libudev-dev pkg-config
```

User serial permission is required to connect to Create over serial. You can add your user to the dialout group to get permission:

```bash
$ [sudo] usermod -a -G dialout $USER
```

Logout and login again for this to take effect.
##### _MacOs_
```bash
$ brew install *TODO*
```

### *Hardware*
 - iRobot Create 2 (or iRobot Roomba 6xx series with serial USB cable - https://store.irobot.com/en_US/parts-and-accessories/create-accessories/communication-cable-for-create-2/4466502.html )
 - Raspberry Pi 4 (4GB)
 - Intel RealSense D435 or D435i depth camera
 
