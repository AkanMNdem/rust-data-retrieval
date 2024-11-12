# Rust-Based Data Retrieval

## Overview
This project is a Rust-based application designed for IoT data retrieval and processing, focusing on integration with The Things Network (TTN) using MQTT. It is part of an undergraduate research initiative to enhance IoT data handling and analysis.

## Features
- **MQTT Connectivity**: Securely connects to TTN via the `paho-mqtt` library with SSL/TLS support.
- **IoT Data Retrieval**: Subscribes to device topics for real-time data acquisition.
- **JSON Parsing**: Utilizes `serde` and `serde_json` for efficient payload parsing and metadata extraction.

## Technologies Used
- **Programming Language**: Rust
- **Libraries**:
  - `paho-mqtt`: For MQTT connectivity.
  - `serde`, `serde_json`: For JSON serialization and deserialization.
  - `openssl`: For SSL/TLS functionality.

## Project Structure
- **`src/main.rs`**:
  - Sets up MQTT client and manages topic subscriptions.
  - Integrates JSON parsing for payload processing.
- **`src/json_parsing.rs`**:
  - Defines data structures like `TTNPayload` and `UplinkMessage`.
  - Implements parsing logic for IoT metadata and uplink messages.
- **`Cargo.toml`**:
  - Specifies dependencies for MQTT, JSON, and SSL support.

## Getting Started

### Clone the Repository
```bash
git clone https://github.com/AkanMNdem/rust-data-retrieval.git
cd rust-data-retrieval
```

### Install Dependencies
Ensure Rust and Cargo are installed, then build the project:
```bash
cargo build
```

### Run the Application
Execute the application with:
```bash
cargo run
```

## Limitations
- **Work in Progress**: This project is part of ongoing research and not production-ready.
- **Not Open for Contributions**: The repository is maintained for research collaboration purposes.

## License
This repository is intended for research purposes only and does not include an open-source license for general use.
