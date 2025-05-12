# Brwoser Activity Checker

A minimal WebSocket server and client implementation for tracking and managing connection statuses.

## Project Overview

This project implements a lightweight WebSocket server with a simple status management system. It allows real-time status tracking between a server and client, with three primary states: Active, Sharing, and Inactive.

## Features

- 🚀 Simple WebSocket server implementation
- 🔄 Real-time status tracking
- 📡 Multiple status states
- 🔒 Thread-safe global status management

## Technology Stack

- **Language**: Rust
- **WebSocket Library**: tungstenite
- **Serialization**: serde
- **Frontend**: Vanilla JavaScript

## Project Structure

```tree
.
├── src/
│   ├── lib.rs        # Status management and shared types
│   └── main.rs       # WebSocket server implementation
├── index.html        # Client-side WebSocket interface
└── Cargo.toml        # Project dependencies and configuration
```

## Status Enum

The project defines three distinct statuses:

- `Active`: Connection is established and active
- `Sharing`: Specific sharing mode
- `Inactive`: No active connection

## Installation

### Prerequisites

- Rust (latest stable version)
- Cargo package manager

### Steps

1. Clone the repository
2. Navigate to the project directory
3. Build the project:

   ```bash
   cargo build --release
   ```

## Running the Server

```bash
cargo run
```

The server will start on `ws://localhost:8235`

## Client Interface

Open `index.html` in a web browser:

- Input text and click "Send"
- Automatically sets status based on input
- Logs server responses in the console

## WebSocket Communication

### Server Responsibilities

- Manages global connection status
- Handles WebSocket connections
- Broadcasts status changes

### Client Responsibilities

- Establishes WebSocket connection
- Sends status updates
- Receives and processes server status messages

## Dependencies

- `tungstenite`: WebSocket protocol implementation
- `serde`: Serialization and deserialization
- `once_cell`: Lazy static initialization
- `serde_json`: JSON parsing

## Example Usage

```rust
// Setting status
set_status(&Status::Active);

// Getting current status
let current_status = get_brwoser_activity_status();
```

## WebSocket Message Format

```json
{
  "status": "active" | "sharing" | "inactive"
}
```

## Error Handling

- Graceful handling of connection errors
- Detailed error logging
- Automatic status reset on connection issues

## Potential Improvements

- Add authentication
- Implement more robust error handling
- Create a more feature-rich client interface
- Add logging mechanisms

## License

[Specify your license here, e.g., MIT, Apache 2.0]
