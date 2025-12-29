# RustDesk CLI Controller

A command-line interface for RustDesk that allows remote system control without a GUI.

## Features

- Connect to remote systems via command line
- Interactive terminal access to remote systems
- Execute commands and view output in real-time
- Support for custom ID and relay servers
- Secure authentication with password prompts
- Minimal dependencies for server environments
- Cross-platform support (Linux, macOS, Windows)

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

### Connecting to a Remote System with Interactive Terminal

To establish an interactive terminal session with a remote system:

```bash
./rustdeskcli --remoteId 227628712 \
              --idServer 35.222.12.44 \
              --relayServer 35.222.12.44 \
              --key gAZauLXlFpe+s0CSpqbkpfKnuQCNeJBOj38bM0+2I+wer
```

Parameters:
- `--remoteId`: The ID of the remote system to connect to (required)
- `--idServer`: The ID/Rendezvous server address (optional)
- `--relayServer`: The relay server address (optional)
- `--key`: Server public key for authentication (optional, defaults to empty)
- `--password`: Password for authentication (optional, will prompt if not provided)

### Interactive Terminal Mode

Once connected, you'll be in an interactive terminal where you can:

1. Type commands directly (e.g., `ls -la`, `pwd`, `whoami`)
2. View command output in real-time
3. Execute multiple commands sequentially
4. Type `exit` or `quit` to close the connection

Example session:
```bash
$ ./rustdeskcli --remoteId 227628712 --idServer 35.222.12.44 --key mykey

=== RustDesk CLI - Interactive Terminal ===
Connecting to remote system...
Once connected, you can type commands directly.
Type 'exit' or 'quit' to close the connection.

Connected successfully! (direct: true)
Login successful!

$ ls -la
total 48
drwxr-xr-x 5 user user 4096 Dec 29 08:00 .
drwxr-xr-x 3 user user 4096 Dec 29 07:55 ..
-rw-r--r-- 1 user user  220 Dec 29 07:55 .bash_logout

$ pwd
/home/user

$ exit
Connection closed successfully
```

### Single Command Execution (Future Feature)

Execute a single command without maintaining an interactive session:

```bash
./rustdeskcli --command "ls -la"
```

**Note**: This feature is currently limited. For now, use the interactive terminal mode for command execution.

## Architecture

### Components

1. **CliSession**: Implements the `Interface` trait for handling CLI-specific interactions
   - Manages connection state
   - Handles terminal open/close operations
   - Sends terminal input to remote system
2. **Connection Manager**: Handles establishing and maintaining connections to remote systems
3. **Interactive Terminal**: Provides real-time command execution with output streaming
4. **Message Handler**: Processes messages from remote system including terminal output

### Connection Flow

1. Parse CLI arguments
2. Initialize RustDesk client with specified parameters
3. Establish connection using `Client::start()`
4. Handle authentication (password prompt if needed)
5. Open remote terminal session
6. Start interactive input loop
7. Send commands and display output in real-time
8. Maintain session until user exits

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

- Terminal size is fixed at 24 rows x 80 columns (resizing not yet implemented)
- No support for advanced terminal control sequences (colors, cursor positioning, etc.)
- Single-command execution mode requires session persistence (planned)
- File transfer not available in CLI mode
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

- [x] Complete terminal command execution implementation
- [x] Interactive shell mode
- [ ] Terminal resizing support
- [ ] Better terminal emulation with control sequence handling
- [ ] Session persistence for single-command execution
- [ ] File transfer support via CLI
- [ ] Configuration file support
- [ ] Command history and autocomplete
- [ ] Multi-session support (connect to multiple systems simultaneously)
- [ ] Tab completion for remote filesystem
- [ ] Support for terminal colors and formatting

## Contributing

Contributions are welcome! Please ensure:

1. Code follows existing style patterns
2. New features include documentation
3. Changes are tested on target platforms (Linux/macOS)

## License

This project follows the same license as the main RustDesk project.
