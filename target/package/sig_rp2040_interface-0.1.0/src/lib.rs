// Written by sigroot
//! sig_rp2040_interface - Allows use of the LED Matrix as an object
//!
//! Communicates only with Framework LED matrices loaded with the
//! rp2040_firmware files

use std::io::{Error, ErrorKind};
use std::result::Result;
use std::thread::sleep;
use std::time::Duration;

const VERSION_STATEMENT: &str = "Sig FW LED Matrix FW V1.1\0\0\0\0\0\0\0";
const LED_RESPONSE_TIME: u64 = 12;

pub struct LedMatrixInterface {
    pwm_matrix: Box<[[u8; 9]; 34]>,
    scale_matrix: Box<[[u8; 9]; 34]>,
    led_matrix_port: Box<dyn serialport::SerialPort>,
}

impl LedMatrixInterface {
    /// Creates a new interface automatically getting the port name
    pub fn new(baud_rate: u32, timeout: u64) -> Self {
        Self {
            pwm_matrix: Box::new([[0; 9]; 34]),
            scale_matrix: Box::new([[255; 9]; 34]),
            led_matrix_port: get_matrix_port(baud_rate, timeout).expect("No ports found"),
        }
    }
    /// Creates a new interface with a user specified port name
    pub fn new_manual(port_name: &str, baud_rate: u32, timeout: u64) -> Self {
        Self {
            pwm_matrix: Box::new([[0; 9]; 34]),
            scale_matrix: Box::new([[255; 9]; 34]),
            led_matrix_port: serialport::new(port_name, baud_rate)
                .timeout(Duration::from_millis(timeout))
                .open()
                .expect("Unable to find port"),
        }
    }
    /// Sets the PWM of each u8 in the interface's PWM matrix
    pub fn set_pwm(&mut self, input_matrix: &[[u8; 9]; 34]) {
        for i in 0..self.pwm_matrix.len() {
            for j in 0..self.pwm_matrix[i].len() {
                self.pwm_matrix[i][j] = input_matrix[i][j];
            }
        }
    }
    /// Sets the scale of each u8 in the interface's scale matrix
    pub fn set_scale(&mut self, input_matrix: &[[u8; 9]; 34]) {
        for i in 0..self.scale_matrix.len() {
            for j in 0..self.scale_matrix[i].len() {
                self.scale_matrix[i][j] = input_matrix[i][j];
            }
        }
    }
    /// Writes the interface's PWM matrix to the LED matrix's PWM registers
    pub fn write_pwm(&mut self) {
        let mut write_buffer = Vec::<u8>::with_capacity(307);
        let mut read_buffer = [0];
        write_buffer.extend_from_slice(&[b'M']);
        for i in 0..self.pwm_matrix.len() {
            for j in 0..self.pwm_matrix[i].len() {
                write_buffer.extend_from_slice(&[self.pwm_matrix[i][j]]);
            }
        }
        match self.led_matrix_port.write_all(write_buffer.as_slice()) {
            Ok(_) => (),
            Err(_) => self.flush_operation(307),
        }
        // Block until matrix is updated
        loop {
            match self.led_matrix_port.bytes_to_read() {
                Ok(x) => {
                    if x > 0 {
                        let _ = self.led_matrix_port.read_exact(&mut read_buffer);
                        break;
                    }
                }
                Err(_) => panic!(),
            }
            sleep(Duration::from_millis(LED_RESPONSE_TIME));
        }
        if read_buffer[0] != b'M' {
            panic!();
        }
    }
    /// Writes the interface's scale matrix to the LED matrix's scale registers
    pub fn write_scale(&mut self) {
        let mut write_buffer = Vec::<u8>::with_capacity(307);
        let mut read_buffer = [0];
        write_buffer.extend_from_slice(&[b'N']);
        for i in 0..self.scale_matrix.len() {
            for j in 0..self.scale_matrix[i].len() {
                write_buffer.extend_from_slice(&[self.scale_matrix[i][j]]);
            }
        }
        match self.led_matrix_port.write_all(write_buffer.as_slice()) {
            Ok(_) => (),
            Err(_) => self.flush_operation(307),
        }
        // Block until matrix is updated
        loop {
            match self.led_matrix_port.bytes_to_read() {
                Ok(x) => {
                    if x > 0 {
                        let _ = self.led_matrix_port.read_exact(&mut read_buffer);
                        break;
                    }
                }
                Err(_) => panic!(),
            }
            sleep(Duration::from_millis(LED_RESPONSE_TIME));
        }
        if read_buffer[0] != b'N' {
            panic!();
        }
    }
    /// Writes both interface matrices to the LED matrix
    pub fn write(&mut self) {
        self.write_scale();
        self.write_pwm();
    }
    /// Sets the interface's port automatically
    pub fn set_port(&mut self, baud_rate: u32, timeout: u64) -> Result<(), Error> {
        match get_matrix_port(baud_rate, timeout) {
            Ok(x) => Ok(self.led_matrix_port = x),
            Err(x) => Err(x),
        }
    }
    /// Sets the interface's port by name
    pub fn set_port_manual(
        &mut self,
        port_name: &str,
        baud_rate: u32,
        timeout: u64,
    ) -> Result<(), serialport::Error> {
        let port_result = serialport::new(port_name, baud_rate)
            .timeout(Duration::from_millis(timeout))
            .open();
        match port_result {
            Ok(x) => Ok(self.led_matrix_port = x),
            Err(x) => Err(x),
        }
    }
    /// Writes zeros to clear the communication buffer
    pub fn flush_operation(&mut self, bytes: u32) {
        let mut current_byte = 0;
        while current_byte < bytes {
            match self.led_matrix_port.write(&[0]) {
                Ok(x) => current_byte += x as u32,
                Err(_) => continue,
            }
        }
    }
}
/// Gets possible serial ports automatically
pub fn get_ports() -> Option<Vec<serialport::SerialPortInfo>> {
    let mut ports = match serialport::available_ports() {
        Ok(x) => x,
        Err(_) => Vec::new(),
    };
    let mut i = 0;
    for p in ports.clone() {
        if !(p.port_name.contains("COM") || p.port_name.contains("ACM")) {
            ports.remove(i);
            i -= 1;
        }
        i += 1;
    }
    if ports.len() > 0 {
        Some(ports)
    } else {
        None
    }
}
/// Gets a serial port by name
pub fn get_matrix_port(
    baud_rate: u32,
    timeout: u64,
) -> Result<Box<dyn serialport::SerialPort>, Error> {
    let ports = get_ports();
    let port_info;
    match ports {
        Some(x) => {
            if x.len() == 1 {
                port_info = x[0].clone();
            } else {
                return Err(Error::new(ErrorKind::InvalidInput, "Too many ports found!"));
            }
        }
        None => {
            return Err(Error::new(
                ErrorKind::NotFound,
                "No ACM or Com ports found!",
            ))
        }
    }

    let mut port = serialport::new(port_info.port_name, baud_rate)
        .timeout(Duration::from_millis(timeout))
        .open()?;
    port.write(&[127])?;

    let mut read_buffer: Vec<u8> = vec![0; 32];
    port.read(&mut read_buffer)?;
    match std::str::from_utf8(&read_buffer) {
        Ok(x) => {
            if x == VERSION_STATEMENT {
                return Ok(port);
            } else {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    format!(
                        "Incorrect version statement from port!: {:?}",
                        read_buffer.as_slice()
                    ),
                ));
            }
        }
        Err(_) => {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "Incorrect string format from port!",
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ports_available() {
        let ports = get_ports().expect("No ACM or COM ports found!");
        for p in ports {
            let mut vid = 0;
            let mut pid = 0;
            let mut sn = "None".to_string();
            let mut man = "None".to_string();
            let mut prod = "None".to_string();
            let ptype = match p.port_type {
                serialport::SerialPortType::UsbPort(info) => {
                    vid = info.vid;
                    pid = info.pid;
                    sn = match info.serial_number {
                        Some(x) => x,
                        None => "None".to_string(),
                    };
                    man = match info.manufacturer {
                        Some(x) => x,
                        None => "None".to_string(),
                    };
                    prod = match info.product {
                        Some(x) => x,
                        None => "None".to_string(),
                    };
                    "USB"
                }
                serialport::SerialPortType::PciPort => "PCI",
                serialport::SerialPortType::BluetoothPort => "BT",
                serialport::SerialPortType::Unknown => "Unknown",
            };
            println!("{}\t{}", p.port_name, ptype);
            if ptype == "USB" {
                println!("\t{} {} {:?} {:?} {:?}", vid, pid, sn, man, prod);
            }
        }
        assert!(true);
    }

    #[test]
    fn port_correct() {
        let port = get_matrix_port(1000000, 10000);
        match port {
            Ok(x) => {
                println!("{:?}", x);
            }
            Err(x) => {
                println!("{:?}", x);
                assert!(false);
            }
        }
        assert!(true);
    }
}
