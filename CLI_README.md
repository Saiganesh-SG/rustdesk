# RustDesk CLI Controller

A command-line interface for RustDesk that allows remote system control without a GUI.

## Features

- Connect to remote systems via command line
- Execute commands on remote terminals
- Support for custom ID and relay servers
- Minimal dependencies for server environments

## Building

### Prerequisites

- Rust toolchain (1.75 or higher)
- Git with submodules initialized

### Linux

```bash
# Clone and initialize submodules
git clone https://github.com/Saiganesh-SG/rustdesk.git
cd rustdesk
git checkout feature/cli-version
git submodule update --init --recursive

# Build the CLI binary
cargo build --bin rustdeskcli --release

# The binary will be at: target/release/rustdeskcli
```

### macOS

```bash
# Clone and initialize submodules
git clone https://github.com/Saiganesh-SG/rustdesk.git
cd rustdesk
git checkout feature/cli-version
git submodule update --init --recursive

# Build the CLI binary
cargo build --bin rustdeskcli --release

# The binary will be at: target/release/rustdeskcli
```

### Windows

```bash
# Clone and initialize submodules
git clone https://github.com/Saiganesh-SG/rustdesk.git
cd rustdesk
git checkout feature/cli-version
git submodule update --init --recursive

# Build the CLI binary
cargo build --bin rustdeskcli --release

# The binary will be at: target\release\rustdeskcli.exe
```

## Usage

### Connecting to a Remote System

To establish a connection to a remote system:

```bash
./rustdeskcli --remoteId 227628712 \
              --idServer 35.222.12.44 \
              --relayServer 35.222.12.44 \
              --key gAZauLXlFpe+s0CSpqbkpfKnuQCNeJBOj38bM0+2I+wer
```

Parameters:
- `--remoteId`: The ID of the remote system to connect to
- `--idServer`: The ID/Rendezvous server address (optional)
- `--relayServer`: The relay server address (optional)
- `--key`: Server public key for authentication (optional)
- `--password`: Password for authentication (optional, will prompt if not provided)

### Executing Commands

After establishing a connection, you can execute commands:

```bash
./rustdeskcli --command "ls -la"
```

**Note**: Command execution requires an active connection session. The full remote terminal integration is planned for future releases.

## Architecture

### Components

1. **CliSession**: Implements the `Interface` trait for handling CLI-specific interactions
2. **Connection Manager**: Handles establishing and maintaining connections to remote systems
3. **Command Executor**: Processes commands and returns output (in development)

### Connection Flow

1. Parse CLI arguments
2. Initialize RustDesk client with specified parameters
3. Establish connection using `Client::start()`
4. Handle authentication (password prompt if needed)
5. Maintain session for command execution

## Development

### Project Structure

```
src/
├── cli_controller.rs   # Main CLI controller logic
├── rustdeskcli.rs      # CLI binary entry point
├── client.rs           # Client connection handling
└── lib.rs              # Library exports
```

### Adding New Features

The CLI controller is built on top of the existing RustDesk client infrastructure. To add new features:

1. Implement necessary message handling in `cli_controller.rs`
2. Update the `CliSession` struct with required state
3. Add new command-line arguments in `rustdeskcli.rs`

## Limitations

- Currently supports connection establishment and basic authentication
- Full terminal command execution is under development
- Requires network access to rendezvous and relay servers

## Troubleshooting

### Connection Issues

If you encounter connection issues:

1. Verify the remote ID is correct
2. Check network connectivity to ID and relay servers
3. Ensure the server key is valid
4. Verify password is correct

### Build Issues

If the build fails:

1. Ensure all git submodules are initialized: `git submodule update --init --recursive`
2. Update Rust toolchain: `rustup update`
3. Check that required system dependencies are installed

## Future Enhancements

- [ ] Complete terminal command execution implementation
- [ ] Session persistence for multiple commands
- [ ] File transfer support via CLI
- [ ] Configuration file support
- [ ] Interactive shell mode
- [ ] Command history and autocomplete

## Contributing

Contributions are welcome! Please ensure:

1. Code follows existing style patterns
2. New features include documentation
3. Changes are tested on target platforms (Linux/macOS)

## License

This project follows the same license as the main RustDesk project.
