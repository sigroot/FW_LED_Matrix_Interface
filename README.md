# Framework LED Matrix Interface
## Preamble
**THIS IS UNOFFICIAL SOFTWARE. I AM NOT AFFILIATED WITH FRAMEWORK**

This repository contains a rust library for communicating with a [Framework LED Matrix](https://frame.work/products/16-led-matrix) which is loaded with the custom [FW_LED_Matrix_Firmware](https://github.com/sigroot/FW_LED_Matrix_Firmware). 
## Capabilities
This library contains an LedMatrixInterface struct which can either connect automatically with an installed [Framework LED Matrix](https://frame.work/products/16-led-matrix) or manually receive a LED matrix's port name in the form of a &str. When the LedMatrixInterface struct is created, it confirms that the firmware is correct and communicating. The LedMatrixInterface struct contains two 9x34 u8 2D-arrays. These correspond to the LED matrix's PWM and scale values. The arrays can be set separately or written to the LED matrix either separately or together. Should a write to the LED matrix fail, the struct can flush the LED matrix's read buffer.
## Usage
### LedMatrixInterface Struct
pwm_matrix: a 9x34 u8 2D-array representing the next PWM to be written
scale_matrix: a 9x34 u8 2D-array representing the next scale to be written
led_matrix_port: a SerialPort representing the port of the LED matrix
### Methods
>new(baud_rate: u32, timeout: u64) -> Self

Returns a new LedMatrixInterface struct with a port automatically discovered by get_matrix_port(). baud_rate is the rate of communication with the LED matrix when discovering a port (default: 1000000). timeout is the duration in milliseconds before returning an error when discovering a port (default: 10000).

>new_manual(port_name: &str, baud_rate: u32, timeout: u64) -> Self

Returns a new LedMatrixInterface struct with a manually entered port name. baud_rate is the rate of communication with the LED matrix when discovering a port (default: 1000000). timeout is the duration in milliseconds before returning an error when discovering a port (default: 10000).

>set_pwm(&mut self, input_matrix: &[[u8; 9]; 34])

Sets the struct's pwm_matrix to a copy of input_matrix.

>set_scale(&mut self, input_matrix: &[[u8; 9]; 34])

Sets the struct's scale_matrix to a copy of input_matrix.

>write_pwm(&mut self)

Writes the struct's pwm_matrix to the LED matrix's PWM matrix. May panic.

>write_scale(&mut self)

Writes the struct's scale_matrix to the LED matrix's scale matrix. May panic.

>write(&mut self)

Performs both write_pwm() and write_scale().

>set_port(&mut self, baud_rate: u32, timeout: u64) -> Result<(), Error>

Resets the LedMatrixInterface struct's port automatically. baud_rate is the rate of communication with the LED matrix when discovering a port (default: 1000000). timeout is the duration in milliseconds before returning an error when discovering a port (default: 10000).

>set_port_manual(&mut self, port_name: &str, baud_rate: u32, timeout: u64) -> Result<(), serialport::Error>

Resets the LedMatrixInterface struct's port with a manually entered port name. baud_rate is the rate of communication with the LED matrix when discovering a port (default: 1000000). timeout is the duration in milliseconds before returning an error when discovering a port (default: 10000).

>pub fn flush_operation(&mut self, bytes: u32)

Writes bytes-number of null operations to the LED matrix.
### Functions
>get_ports() -> Option<Vec<serialport::SerialPortInfo>>

Returns a list of all available ports with names containing "COM" or "ACM".

>get_matrix_port(baud_rate: u32, timeout: u64) -> Result<Box<dyn serialport::SerialPort>, Error>

Returns a single port that is verified to be an LED matrix running the correct firmware. Will return an error if there is more than one LED matrix port.
### Associated Software
[FW_LED_Matrix_Firmware](https://github.com/sigroot/FW_LED_Matrix_Firmware) is Arduino-based firmware and is a prerequisite installation for this library.

[FW_LED_Matrix_Board](https://github.com/sigroot/FW_LED_Matrix_Board) divides the 9x34 LED Matrix into 3 smaller 9x11 "applets" and provides a language agnostic interface between these applets and other programs.

[FW_LED_Matrix_Applet](https://github.com/sigroot/FW_LED_Matrix_Applet) is a Rust Library for interfacing between Rust programs and [FW_LED_Matrix_Board](https://github.com/sigroot/FW_LED_Matrix_Board).
