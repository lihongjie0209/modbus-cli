# Modbus CLI Tool

A command-line tool for Modbus RTU communication over serial ports, built with Rust using tokio-serial and tokio-modbus.

## Features

1. **List Serial Ports**: Display all available serial ports with detailed information
2. **Read Modbus Data**: Read holding registers from Modbus devices with hex display

## Installation

Make sure you have Rust installed, then clone and build:

```bash
git clone <repository-url>
cd modbus-cli
cargo build --release
```

## Usage

### List Available Ports

```bash
modbus-cli list-ports
```

This will display all available serial ports with information like:
- Port name (COM1, /dev/ttyUSB0, etc.)
- Port type (USB, Bluetooth, PCI, Unknown)
- For USB ports: Manufacturer, Product, Serial Number, VID/PID

### Read Modbus Data

```bash
modbus-cli read --port COM1 --slave 1 --address 0 --count 10 --function-code 4
```

#### Required Parameters:
- `--port` or `-p`: Serial port path (e.g., COM1, /dev/ttyUSB0)
- `--slave` or `-s`: Modbus slave address (1-255)
- `--address` or `-a`: Starting register address
- `--count` or `-c`: Number of registers/coils to read

#### Optional Parameters:
- `--function-code` or `-f`: Modbus function code (default: 3)
  - `1`: Read Coils (0x01)
  - `2`: Read Discrete Inputs (0x02) 
  - `3`: Read Holding Registers (0x03) - default
  - `4`: Read Input Registers (0x04)
- `--baud` or `-b`: Baud rate (default: 9600)
- `--data-bits`: Data bits - 5, 6, 7, or 8 (default: 8)
- `--stop-bits`: Stop bits - 1 or 2 (default: 1)
- `--parity`: Parity - none, odd, or even (default: none)
- `--timeout`: Timeout in milliseconds (default: 1000)

#### Example Commands:

```bash
# Read 10 holding registers from address 0 on slave 1 (default function code 3)
modbus-cli read -p COM1 -s 1 -a 0 -c 10

# Read 8 input registers (function code 4) - your example: 01 04 00 00 00 1C
modbus-cli read -p COM1 -s 1 -a 0 -c 28 -f 4

# Read 16 coils (function code 1)
modbus-cli read -p COM1 -s 1 -a 0 -c 16 -f 1

# Read discrete inputs (function code 2)
modbus-cli read -p COM1 -s 1 -a 100 -c 8 -f 2

# Read with custom serial settings
modbus-cli read -p COM1 -b 19200 --data-bits 8 --stop-bits 1 --parity even -s 1 -a 100 -c 5 -f 3

# Read with longer timeout
modbus-cli read -p /dev/ttyUSB0 -s 2 -a 1000 -c 20 --timeout 5000 -f 4
```

## Output Format

The tool displays data in different formats depending on the function code:

### For Registers (Function Code 3 & 4):
1. **Detailed table**: Shows address, decimal value, hex value, and binary representation
2. **Hex dump**: Traditional hex dump format with ASCII representation

Example output:
```
Address:     0 (0x0000) | Value:  1234 (0x04D2) | Binary: 0000010011010010
Address:     1 (0x0001) | Value:  5678 (0x162E) | Binary: 0001011000101110
...

Hex dump:
0000: 04D2 162E 0ABC 1234 5678 9ABC DEF0 1234 | .Ò.®.¼.4Vx.¼Þð.4
0008: 5678 9ABC                             | Vx.¼
```

### For Coils & Discrete Inputs (Function Code 1 & 2):
1. **Individual bit status**: Shows each coil/input state
2. **Bit dump**: Groups of 8 bits with byte representation

Example output:
```
Address:     0 (0x0000) | Value:     1 | State: ON
Address:     1 (0x0001) | Value:     0 | State: OFF
...

Bit dump (8 coils per line):
0000: 1 0 1 1 0 0 1 0 | 0x4D
0008: 0 1 1 0         | 0x06
```

## Error Handling

The tool provides helpful error messages for common issues:
- Invalid serial port settings
- Connection failures
- Modbus communication errors
- Invalid parameter values

## Dependencies

- `tokio`: Async runtime
- `tokio-serial`: Serial port communication
- `tokio-modbus`: Modbus protocol implementation
- `clap`: Command-line argument parsing
- `anyhow`: Error handling

## Platform Support

This tool works on:
- Windows (COM ports)
- Linux (/dev/ttyUSB*, /dev/ttyACM*)
- macOS (/dev/cu.*, /dev/tty.*)

## Troubleshooting

1. **Permission denied**: On Linux/macOS, you may need to add your user to the dialout group:
   ```bash
   sudo usermod -a -G dialout $USER
   ```

2. **Port not found**: Use `list-ports` command to see available ports

3. **Communication timeout**: Check cable connections, baud rate, and device settings

4. **Invalid response**: Verify slave address and register addresses are correct