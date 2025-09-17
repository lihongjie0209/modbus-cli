@echo off
REM Examples of using the modbus-cli tool

echo === Modbus CLI Tool Examples ===
echo.

echo 1. List all available serial ports:
echo    modbus-cli list-ports
echo.

echo 2. Read 10 holding registers starting from address 0 (default function code 3):
echo    modbus-cli read -p COM1 -s 1 -a 0 -c 10
echo.

echo 3. Read 28 input registers (function code 4) - example: 01 04 00 00 00 1C:
echo    modbus-cli read -p COM1 -s 1 -a 0 -c 28 -f 4
echo.

echo 4. Read 16 coils (function code 1):
echo    modbus-cli read -p COM1 -s 1 -a 0 -c 16 -f 1
echo.

echo 5. Read discrete inputs with custom settings (function code 2):
echo    modbus-cli read -p COM1 -b 19200 --parity even -s 1 -a 100 -c 8 -f 2
echo.

echo 6. Read with longer timeout:
echo    modbus-cli read -p COM2 -s 2 -a 1000 -c 20 --timeout 5000 -f 4
echo.

echo Note: Replace COM1, COM2, etc. with actual port names from list-ports command