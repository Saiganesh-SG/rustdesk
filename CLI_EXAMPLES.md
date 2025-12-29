# RustDesk CLI Examples

This file contains example commands for using the RustDesk CLI.

## Basic Connection

Connect to a remote system with all parameters:

```bash
./rustdeskcli \
  --remoteId 227628712 \
  --idServer 35.222.12.44 \
  --relayServer 35.222.12.44 \
  --key gAZauLXlFpe+s0CSpqbkpfKnuQCNeJBOj38bM0+2I+wer \
  --password mypassword
```

## Connection with Password Prompt

If you don't provide a password, you'll be prompted:

```bash
./rustdeskcli \
  --remoteId 227628712 \
  --idServer 35.222.12.44 \
  --relayServer 35.222.12.44 \
  --key gAZauLXlFpe+s0CSpqbkpfKnuQCNeJBOj38bM0+2I+wer
```

## Connection without Custom Servers

Connect using default RustDesk servers:

```bash
./rustdeskcli --remoteId 227628712
```

## Using the Interactive Terminal

Once connected, you can run commands interactively:

```
$ ls -la
total 48
drwxr-xr-x 5 user user 4096 Dec 29 08:00 .
drwxr-xr-x 3 user user 4096 Dec 29 07:55 ..

$ pwd
/home/user

$ whoami
user

$ cd /tmp

$ ls
file1.txt  file2.txt

$ exit
```

## Example Commands to Try

Once in the interactive terminal:

- **File operations**: `ls`, `cd`, `pwd`, `cat`, `less`, `head`, `tail`
- **System info**: `uname -a`, `whoami`, `hostname`, `df -h`, `free -h`
- **Process management**: `ps aux`, `top`, `htop` (if installed)
- **Network**: `ip addr`, `ifconfig`, `netstat`, `ping`
- **Text editing**: `nano`, `vi`, `vim` (note: interactive editors may not work well)

## Platform-Specific Examples

### Linux

```bash
# Connect and check system
./rustdeskcli --remoteId 227628712

# Once connected:
$ uname -a
$ cat /etc/os-release
$ systemctl status
```

### macOS

```bash
# Connect and check system
./rustdeskcli --remoteId 227628712

# Once connected:
$ sw_vers
$ system_profiler SPSoftwareDataType
$ brew list
```

### Windows

```cmd
REM Connect using PowerShell
.\rustdeskcli.exe --remoteId 227628712

# Once connected (PowerShell commands):
$ Get-ComputerInfo
$ Get-Process
$ dir
```

## Tips

1. **Terminal Size**: The terminal is fixed at 24 rows x 80 columns. Adjust your terminal window accordingly.

2. **Exit Commands**: Type `exit` or `quit` to close the connection cleanly.

3. **Long-Running Commands**: Be aware that very long-running commands may timeout.

4. **Interactive Programs**: Avoid programs that require extensive terminal control (like vim, top with updates) as they may not display correctly.

5. **Scripting**: For automation, consider piping commands or using a script file.

## Troubleshooting

### Connection Fails

- Verify the remote ID is correct
- Check that the remote system is online
- Ensure network connectivity to ID and relay servers
- Verify the server key if using custom servers

### Authentication Fails

- Check that the password is correct
- Ensure the remote system has password authentication enabled
- Verify that you have permission to access the remote system

### Commands Not Working

- Some commands require a proper shell environment
- Try using absolute paths for commands
- Check that the command exists on the remote system

### No Output

- Some commands may take time to execute
- Check your network connection
- Verify the remote system is responsive
