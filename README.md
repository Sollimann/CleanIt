<h1 align="center">CleanIt</h1>

[![Build Status](https://github.com/Sollimann/CleanIt/actions/workflows/rust-ci.yml/badge.svg)](https://github.com/Sollimann/CleanIt/actions)
[![codecov](https://codecov.io/gh/Sollimann/CleanIt/branch/main/graph/badge.svg?token=EY3JRZN71M)](https://codecov.io/gh/Sollimann/CleanIt)
[![minimum rustc 1.45](https://img.shields.io/badge/rustc-1.45+-blue.svg)](https://rust-lang.github.io/rfcs/2495-min-rust-version.html)
[![version](https://img.shields.io/badge/version-1.0.0-blue)](https://GitHub.com/Sollimann/CleanIt/releases/)
[![Maintenance](https://img.shields.io/badge/Maintained%3F-yes-green.svg)](https://GitHub.com/Sollimann/CleanIt/graphs/commit-activity)
[![GitHub pull-requests](https://img.shields.io/github/issues-pr/Sollimann/CleanIt.svg)](https://GitHub.com/Sollimann/CleanIt/pulls)
[![GitHub pull-requests closed](https://img.shields.io/github/issues-pr-closed/Sollimann/CleanIt.svg)](https://GitHub.com/Sollimann/CleanIt/pulls)
![ViewCount](https://views.whatilearened.today/views/github/Sollimann/CleanIt.svg)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

<p align="center">
    <em>Open-source Autonomy Software in Rust-lang with gRPC for the Roomba series robot vacuum cleaners</em>
</p>

<p align="center">
  <img src="https://github.com/Sollimann/CleanIt/blob/main/resources/gifs/roomba.gif">
</p>

## Motivation

Motivation is to build a complete DIY autonomy software from scratch (motion planning, guidance and motion control, SLAM, mission control, 2D/3D visualization etc..) with a real-time client-server communication stream using async gRPC for intercommunication and distributed compute.

The SLAM part is work-in-progress here: https://github.com/Sollimann/Occupancy-Grid-FastSLAM/tree/main

## Contributions are welcome!

Would you like to contribute with work and/or ideas, feel free to check out the [Project Backlog](https://github.com/Sollimann/CleanIt/projects)

## Run the Roomba client and API

#### Run the server

```
$ cargo run --bin streaming-server
```

#### Run the client

```
$ cargo run --bin roomba-client
```

## Structure
 
#### Project Layout
 ```bash
 ├── Cargo.toml (workspace)
 ├── Cargo.lock
 ├── api (lib/bin)
 |   └── client
 |   └── server
 ├── autonomy (bin)
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
 └── proto (lib)
 |   └── roomba_service.proto
 |   └── messages.proto
 |   └── types.proto
 |   └── robot_state.proto
 |   └── map2D.proto
 ├── setup (bin)
 |   └── config
 |   └── main
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
 
