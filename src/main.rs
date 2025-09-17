use anyhow::Result;
use clap::{Parser, Subcommand};
use std::time::Duration;
use tokio_modbus::prelude::*;
use tokio_serial::SerialStream;

#[derive(Parser)]
#[command(name = "modbus-cli")]
#[command(about = "A CLI tool for Modbus serial communication")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List all available serial ports
    ListPorts,
    /// Read data from a Modbus device
    Read {
        /// Serial port path (e.g., COM1, /dev/ttyUSB0)
        #[arg(short, long)]
        port: String,
        
        /// Baud rate
        #[arg(short, long, default_value = "9600")]
        baud: u32,
        
        /// Data bits (5, 6, 7, 8)
        #[arg(long, default_value = "8")]
        data_bits: u8,
        
        /// Stop bits (1, 2)
        #[arg(long, default_value = "1")]
        stop_bits: u8,
        
        /// Parity (none, odd, even)
        #[arg(long, default_value = "none")]
        parity: String,
        
        /// Modbus slave address
        #[arg(short, long)]
        slave: u8,
        
        /// Starting address to read from
        #[arg(short, long)]
        address: u16,
        
        /// Number of registers to read
        #[arg(short, long)]
        count: u16,
        
        /// Modbus function code (1=coils, 2=discrete_inputs, 3=holding_registers, 4=input_registers)
        #[arg(short, long, default_value = "3")]
        function_code: u8,
        
        /// Timeout in milliseconds
        #[arg(long, default_value = "1000")]
        timeout: u64,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::ListPorts => list_ports().await?,
        Commands::Read {
            port,
            baud,
            data_bits,
            stop_bits,
            parity,
            slave,
            address,
            count,
            function_code,
            timeout,
        } => {
            read_modbus_data(
                &port, baud, data_bits, stop_bits, &parity, slave, address, count, function_code, timeout,
            )
            .await?;
        }
    }

    Ok(())
}

async fn list_ports() -> Result<()> {
    println!("Available serial ports:");
    println!("{:-<60}", "");
    
    match tokio_serial::available_ports() {
        Ok(ports) => {
            if ports.is_empty() {
                println!("No serial ports found.");
            } else {
                for (i, port) in ports.iter().enumerate() {
                    println!("{}. Port: {}", i + 1, port.port_name);
                    
                    match &port.port_type {
                        tokio_serial::SerialPortType::UsbPort(usb_info) => {
                            println!("   Type: USB");
                            if let Some(manufacturer) = &usb_info.manufacturer {
                                println!("   Manufacturer: {}", manufacturer);
                            }
                            if let Some(product) = &usb_info.product {
                                println!("   Product: {}", product);
                            }
                            if let Some(serial_number) = &usb_info.serial_number {
                                println!("   Serial Number: {}", serial_number);
                            }
                            println!("   VID: {:04X}, PID: {:04X}", usb_info.vid, usb_info.pid);
                        }
                        tokio_serial::SerialPortType::BluetoothPort => {
                            println!("   Type: Bluetooth");
                        }
                        tokio_serial::SerialPortType::PciPort => {
                            println!("   Type: PCI");
                        }
                        tokio_serial::SerialPortType::Unknown => {
                            println!("   Type: Unknown");
                        }
                    }
                    println!();
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to list ports: {}", e);
        }
    }
    
    Ok(())
}

async fn read_modbus_data(
    port_name: &str,
    baud_rate: u32,
    data_bits: u8,
    stop_bits: u8,
    parity_str: &str,
    slave_id: u8,
    start_address: u16,
    count: u16,
    function_code: u8,
    timeout_ms: u64,
) -> Result<()> {
    // Parse parity
    let parity = match parity_str.to_lowercase().as_str() {
        "none" => tokio_serial::Parity::None,
        "odd" => tokio_serial::Parity::Odd,
        "even" => tokio_serial::Parity::Even,
        _ => {
            eprintln!("Invalid parity. Use: none, odd, or even");
            return Ok(());
        }
    };

    // Parse data bits
    let data_bits = match data_bits {
        5 => tokio_serial::DataBits::Five,
        6 => tokio_serial::DataBits::Six,
        7 => tokio_serial::DataBits::Seven,
        8 => tokio_serial::DataBits::Eight,
        _ => {
            eprintln!("Invalid data bits. Use: 5, 6, 7, or 8");
            return Ok(());
        }
    };

    // Parse stop bits
    let stop_bits = match stop_bits {
        1 => tokio_serial::StopBits::One,
        2 => tokio_serial::StopBits::Two,
        _ => {
            eprintln!("Invalid stop bits. Use: 1 or 2");
            return Ok(());
        }
    };

    println!("Connecting to Modbus device:");
    println!("  Port: {}", port_name);
    println!("  Baud Rate: {}", baud_rate);
    println!("  Data Bits: {:?}", data_bits);
    println!("  Stop Bits: {:?}", stop_bits);
    println!("  Parity: {:?}", parity);
    println!("  Slave ID: {}", slave_id);
    println!("  Function Code: {} (0x{:02X})", function_code, function_code);
    println!("  Address Range: {} - {}", start_address, start_address + count - 1);
    println!("  Timeout: {}ms", timeout_ms);
    println!();

    // Create serial port
    let builder = tokio_serial::new(port_name, baud_rate)
        .data_bits(data_bits)
        .stop_bits(stop_bits)
        .parity(parity)
        .timeout(Duration::from_millis(timeout_ms));

    let serial_stream = SerialStream::open(&builder)?;
    
    // Create Modbus RTU context
    let mut ctx = rtu::attach_slave(serial_stream, Slave(slave_id));

    // Execute Modbus function based on function code
    match function_code {
        1 => {
            // Read Coils (0x01)
            let result = ctx.read_coils(start_address, count).await?;
            match result {
                Ok(coils) => {
                    println!("Successfully read {} coils:", coils.len());
                    display_coil_data(&coils, start_address);
                }
                Err(e) => {
                    handle_modbus_error(e);
                }
            }
        }
        2 => {
            // Read Discrete Inputs (0x02)
            let result = ctx.read_discrete_inputs(start_address, count).await?;
            match result {
                Ok(inputs) => {
                    println!("Successfully read {} discrete inputs:", inputs.len());
                    display_coil_data(&inputs, start_address);
                }
                Err(e) => {
                    handle_modbus_error(e);
                }
            }
        }
        3 => {
            // Read Holding Registers (0x03)
            let result = ctx.read_holding_registers(start_address, count).await?;
            match result {
                Ok(registers) => {
                    println!("Successfully read {} holding registers:", registers.len());
                    display_register_data(&registers, start_address);
                }
                Err(e) => {
                    handle_modbus_error(e);
                }
            }
        }
        4 => {
            // Read Input Registers (0x04)
            let result = ctx.read_input_registers(start_address, count).await?;
            match result {
                Ok(registers) => {
                    println!("Successfully read {} input registers:", registers.len());
                    display_register_data(&registers, start_address);
                }
                Err(e) => {
                    handle_modbus_error(e);
                }
            }
        }
        _ => {
            eprintln!("Unsupported function code: {}. Supported codes: 1 (coils), 2 (discrete inputs), 3 (holding registers), 4 (input registers)", function_code);
            return Ok(());
        }
    }

    Ok(())
}

fn display_coil_data(coils: &[bool], start_address: u16) {
    println!("{:-<80}", "");
    
    // Display coil data
    for (i, &value) in coils.iter().enumerate() {
        let addr = start_address + i as u16;
        println!(
            "Address: {:5} (0x{:04X}) | Value: {:5} | State: {}",
            addr, addr, if value { 1 } else { 0 }, if value { "ON" } else { "OFF" }
        );
    }
    
    println!("{:-<80}", "");
    
    // Display as hex dump for coils (packed bits)
    println!("Bit dump (8 coils per line):");
    for (i, chunk) in coils.chunks(8).enumerate() {
        let addr = start_address + (i * 8) as u16;
        print!("{:04X}: ", addr);
        
        for &bit in chunk {
            print!("{} ", if bit { 1 } else { 0 });
        }
        
        // Fill remaining space for alignment
        for _ in chunk.len()..8 {
            print!("  ");
        }
        
        print!("| ");
        
        // Display as byte value
        let mut byte_val = 0u8;
        for (j, &bit) in chunk.iter().enumerate() {
            if bit {
                byte_val |= 1 << j;
            }
        }
        print!("0x{:02X}", byte_val);
        
        println!();
    }
}

fn display_register_data(registers: &[u16], start_address: u16) {
    println!("{:-<80}", "");
    
    // Display data in hex format
    for (i, value) in registers.iter().enumerate() {
        let addr = start_address + i as u16;
        println!(
            "Address: {:5} (0x{:04X}) | Value: {:5} (0x{:04X}) | Binary: {:016b}",
            addr, addr, value, value, value
        );
    }
    
    println!("{:-<80}", "");
    
    // Display as hex dump
    println!("Hex dump:");
    for (i, chunk) in registers.chunks(8).enumerate() {
        let addr = start_address + (i * 8) as u16;
        print!("{:04X}: ", addr);
        
        for value in chunk {
            print!("{:04X} ", value);
        }
        
        // Fill remaining space for alignment
        for _ in chunk.len()..8 {
            print!("     ");
        }
        
        print!("| ");
        
        // Display ASCII representation
        for value in chunk {
            let bytes = value.to_be_bytes();
            for byte in bytes {
                if byte >= 32 && byte <= 126 {
                    print!("{}", byte as char);
                } else {
                    print!(".");
                }
            }
        }
        
        println!();
    }
}

fn handle_modbus_error(e: tokio_modbus::Exception) {
    eprintln!("Modbus Exception: {:?}", e);
    eprintln!("This usually means:");
    match e {
        tokio_modbus::Exception::IllegalDataAddress => {
            eprintln!("  - The register address range is not valid for this device");
        }
        tokio_modbus::Exception::IllegalFunction => {
            eprintln!("  - The function code is not supported by this device");
        }
        tokio_modbus::Exception::ServerDeviceFailure => {
            eprintln!("  - The slave device has encountered an error");
        }
        _ => {
            eprintln!("  - Check the device documentation for error details");
        }
    }
}