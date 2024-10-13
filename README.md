# Honeypot

## Description

This Rust-based honeypot is designed to simulate a vulnerable server, capturing and logging any interaction from potential attackers. It listens for incoming TCP connections on a specified port, allowing it to attract and analyze malicious activity. The honeypot logs all received data to a file, enabling further examination and understanding of attack patterns and methods.

## Features

- **TCP Listener**: Listens for incoming connections on a specified port (default: 8080).
- **Data Logging**: Captures and logs all data sent by clients to a log file (`honeypot_log.txt`) for analysis.
- **Multi-threaded Handling**: Supports concurrent connections using threads to manage multiple clients simultaneously.
- **Simple Response**: Responds with a basic HTTP message to clients, simulating a live service.

## Purpose

Tool for cybersecurity enthusiasts and researchers to study attack behaviors and improve defensive strategies. It provides insights into common techniques used by attackers, helping to enhance the security posture of real-world applications.

## Usage

### Prerequisites

- Ensure you have Rust installed on your machine. You can install it from the [official Rust website](https://www.rust-lang.org/).

### Compiling the Honeypot

1. Clone the repository:
   ```bash
   git clone https://github.com/b17w1z4rd/Honeypot.git
   cd Honeypot
2. Build the Project :
   **cargo build --release**
3. Run the honeypot :
   **cargo run**
4. Test the honeypot by connecting to it using a tool like telnet or netcat:
   **nc localhost 8080**
5. Monitor the log file (honeypot_log.txt) for recorded interactions.
