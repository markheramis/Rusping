# Rusping

This is a simple Rust command-line tool that continuously pings a domain name or IP address to resolve its current IP address. The program runs indefinitely, displaying the resolved IP address every 2 seconds until the user manually stops it using `Ctrl+C`.

## Features

- Accepts a domain name or an IP address as a command-line argument.
- Repeatedly resolves the target to its corresponding IP address.
- Displays the resolved IP address every 2 seconds.
- Gracefully handles `Ctrl+C` to stop the program.

## Installation

1. Clone the repository or create a new Rust project:
    ```bash
    git clone https://github.com/markheramis/Rusping.git
    cd Rusping
    ```
2. Build the project using Cargo:
    ```bash
    cargo build --release
    ```

## Usage

To run the program, provide a domain name or an IP address as a command-line argument. The program will continuously resolve the target to its IP address and print the result.

## Example:

```bash
cargo run -- www.google.com
```

Output:

```bash
Pinging to resolve IP address for: www.google.com
Resolved IP: 142.250.190.132
Resolved IP: 142.250.190.132
Resolved IP: 142.250.190.132
...
```

You can also pass an IP address directly:

```bash
cargo run -- 8.8.8.8
```

To stop the program, simply press Ctrl+C:

```bash
Received Ctrl+C, exiting...
Program terminated.
```

## Dependencies

This program relies on the following Rust crates:

- `clap` for parsing command-line arguments.
- `dns-lookup` for resolving domain names to IP addresses.
- `ctrlc` for handling the Ctrl+C signal to gracefully stop the program.

## How it Works

- **Argument Parsing**: The program uses `clap` to accept a domain name or IP address as input.
- **Resolving IP**: The provided domain or IP is resolved using the `ToSocketAddrs` trait.
- **Infinite Loop**: Every 2 seconds, the program resolves the target's IP address and prints it to the console.
- **Graceful Exit**: The program listens for `Ctrl` + `C` (SIGINT). When this signal is received, the infinite loop is stopped, and the program exits cleanly.

## Handling Errors

If the program encounters an error while trying to resolve the target (e.g., if the domain name is invalid), it will print an error message and continue trying to resolve the target in the next iteration.

Example:

```
Error resolving IP: Failed to resolve invalid.domain: Name or service not known
```
