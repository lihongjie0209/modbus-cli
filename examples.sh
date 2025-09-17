#!/bin/bash

# Examples of using the modbus-cli tool

echo "=== Modbus CLI Tool Examples ==="
echo

echo "1. List all available serial ports:"
echo "   modbus-cli list-ports"
echo

echo "2. Read 10 holding registers starting from address 0:"
echo "   modbus-cli read -p COM1 -s 1 -a 0 -c 10"
echo

echo "3. Read with custom serial settings (19200 baud, even parity):"
echo "   modbus-cli read -p COM1 -b 19200 --parity even -s 1 -a 100 -c 5"
echo

echo "4. Read with longer timeout (5 seconds):"
echo "   modbus-cli read -p /dev/ttyUSB0 -s 2 -a 1000 -c 20 --timeout 5000"
echo

echo "5. Read from a specific device type (RTU over USB):"
echo "   modbus-cli read -p COM3 -b 38400 --data-bits 8 --stop-bits 2 -s 3 -a 40001 -c 8"
echo

echo "Note: Replace COM1, COM3, etc. with actual port names from list-ports command"