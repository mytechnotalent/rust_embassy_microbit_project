/// # Board Support Package (BSP) Module
///
/// This module provides hardware abstraction for the BBC micro:bit v2 board,
/// which is based on the Nordic nRF52833 microcontroller. It handles:
///
/// - **Pin Mapping**: Maps physical pins to their micro:bit functions
/// - **Peripheral Access**: Provides structured access to hardware peripherals
/// - **Hardware Initialization**: Sets up GPIO pins, display matrix, and buttons
/// - **Type Aliases**: Creates convenient type definitions for hardware components
///
/// ## Pin Layout (micro:bit v2)
/// - **LED Matrix**: Uses pins P0_21-P0_19 (rows) and P0_28-P0_30 (columns)
/// - **Buttons**: Button A (P0_14), Button B (P0_23)
/// - **Edge Connector**: P0_02, P0_03, P0_04, etc. for external connections
/// - **Internal I2C**: P0_08 (SCL), P0_16 (SDA) for accelerometer/magnetometer
/// - **UART Debug**: P1_08 (TX), P0_06 (RX) for debug communication
///
/// ## Usage Example
/// ```no_run
/// let board = Microbit::default();
/// let mut display = board.display;
/// let mut button_a = board.btn_a;
/// ```
use embassy_nrf::gpio::{AnyPin, Input, Level, Output, OutputDrive, Pin, Pull};
use embassy_nrf::peripherals::{
    P0_00, P0_01, P0_02, P0_03, P0_04, P0_05, P0_06, P0_08, P0_09, P0_10, P0_12, P0_13, P0_16, P0_17, P0_20, P0_26,
    P1_00, P1_02, P1_08, PPI_CH0, PPI_CH1, PWM0, PWM1, PWM2, PWM3, RNG, SAADC, TIMER0, TWISPI0, TWISPI1, UARTE0,
    UARTE1,
};

use crate::display::LedMatrix as LedMatrixDriver;

/// Type alias for the micro:bit's 5x5 LED matrix display.
///
/// This creates a convenient shorthand for the LED matrix driver configured
/// specifically for the micro:bit's hardware layout with 5 rows and 5 columns.
pub type LedMatrix = LedMatrixDriver<Output<'static>, 5, 5>;

/// Type alias for micro:bit button inputs.
///
/// Represents the physical buttons A and B on the micro:bit board.
/// Configured as GPIO inputs without internal pull-up/pull-down resistors
/// since the micro:bit has external pull-up resistors on the button lines.
pub type Button = Input<'static>;

/// Main board structure containing all available peripherals and pins.
///
/// This structure provides organized access to all the hardware components
/// available on the BBC micro:bit v2 board. It groups related functionality
/// and provides a clean interface for application code.
///
/// # Pin Organization
/// - **Display**: Pre-configured 5x5 LED matrix
/// - **Buttons**: Button A and B inputs
/// - **Edge Connector**: Pins P0-P20 accessible via the edge connector
/// - **Internal Interfaces**: I2C, UART, and other internal connections
/// - **Peripherals**: Timers, PWM, ADC, RNG, and communication interfaces
///
/// # Example
/// ```no_run
/// let board = Microbit::new(Default::default());
/// let display = board.display;
/// let button_a = board.btn_a;
/// ```
pub struct Microbit {
    /// **5x5 LED Matrix Display**
    ///
    /// Pre-configured LED matrix driver ready for displaying text, graphics,
    /// and animations. The matrix uses charlieplexing to control 25 LEDs
    /// with only 10 GPIO pins (5 rows + 5 columns).
    ///
    /// # Usage
    /// ```ignore
    /// board.display.scroll("Hello!").await;
    /// board.display.display(pattern, Duration::from_secs(2)).await;
    /// ```
    pub display: LedMatrix,

    /// **Button A Input (Left Button)**
    ///
    /// GPIO input for the left button on the micro:bit front face.
    /// Connected to pin P0_14 with external pull-up resistor.
    /// Button press pulls the pin to ground (active low).
    ///
    /// # Usage
    /// ```ignore
    /// board.btn_a.wait_for_low().await; // Wait for button press
    /// let pressed = board.btn_a.is_low(); // Check current state
    /// ```
    pub btn_a: Button,

    /// **Button B Input (Right Button)**
    ///
    /// GPIO input for the right button on the micro:bit front face.
    /// Connected to pin P0_23 with external pull-up resistor.
    /// Button press pulls the pin to ground (active low).
    ///
    /// # Usage
    /// ```ignore
    /// board.btn_b.wait_for_low().await; // Wait for button press
    /// let pressed = board.btn_b.is_low(); // Check current state
    /// ```
    pub btn_b: Button,

    /// **UART0 Peripheral**
    ///
    /// First UART peripheral available for serial communication.
    /// Can be used for external serial devices or debugging.
    #[allow(dead_code)]
    pub uarte0: UARTE0,

    /// **UART1 Peripheral**
    ///
    /// Second UART peripheral available for serial communication.
    /// Provides additional serial interface capability.
    #[allow(dead_code)]
    pub uarte1: UARTE1,

    /// **TIMER0 Peripheral**
    ///
    /// Hardware timer peripheral for precise timing operations,
    /// PWM generation, or time-based event scheduling.
    #[allow(dead_code)]
    pub timer0: TIMER0,

    /// **Speaker Pin (P0_00)**
    ///
    /// Connected to the built-in speaker/buzzer on micro:bit v2.
    /// Can be used with PWM to generate audio tones and melodies.
    ///
    /// # Note
    /// Requires PWM peripheral configuration for audio generation.
    #[allow(dead_code)]
    pub speaker: P0_00,

    /// **Microphone Pin (P0_05)**
    ///
    /// Analog input connected to the built-in microphone on micro:bit v2.
    /// Requires ADC configuration for audio input processing.
    #[allow(dead_code)]
    pub microphone: P0_05,

    /// **Microphone Enable Pin (P0_20)**
    ///
    /// Digital output to enable/disable the built-in microphone.
    /// Must be set high to activate microphone functionality.
    #[allow(dead_code)]
    pub micen: P0_20,

    // Edge Connector Pins (Large Pins)
    /// **Edge Connector Pin 0 (P0_02)**
    ///
    /// Large pin on the edge connector, suitable for analog input,
    /// digital I/O, or PWM output. Often used for sensors.
    #[allow(dead_code)]
    pub p0: P0_02,

    /// **Edge Connector Pin 1 (P0_03)**
    ///
    /// Large pin on the edge connector, suitable for analog input,
    /// digital I/O, or PWM output. Often used for actuators.
    #[allow(dead_code)]
    pub p1: P0_03,

    /// **Edge Connector Pin 2 (P0_04)**
    ///
    /// Large pin on the edge connector, suitable for analog input,
    /// digital I/O, or PWM output. Commonly used for external devices.
    #[allow(dead_code)]
    pub p2: P0_04,

    // Small Edge Connector Pins
    /// **Edge Connector Pin 8 (P0_10)**
    ///
    /// Small pin on the edge connector for digital I/O operations.
    /// Part of the extended pin set for advanced projects.
    #[allow(dead_code)]
    pub p8: P0_10,

    /// **Edge Connector Pin 9 (P0_09)**
    ///
    /// Small pin on the edge connector for digital I/O operations.
    /// Can be configured for various peripheral functions.
    #[allow(dead_code)]
    pub p9: P0_09,

    /// **Edge Connector Pin 12 (P0_12)**
    ///
    /// Small pin on the edge connector for digital I/O operations.
    /// Available for custom hardware interfacing.
    #[allow(dead_code)]
    pub p12: P0_12,

    /// **Edge Connector Pin 13 (P0_17)**
    ///
    /// Small pin on the edge connector, can be used for SPI SCK
    /// or general digital I/O operations.
    #[allow(dead_code)]
    pub p13: P0_17,

    /// **Edge Connector Pin 14 (P0_01)**
    ///
    /// Small pin on the edge connector, can be used for SPI MISO
    /// or general digital I/O operations.
    #[allow(dead_code)]
    pub p14: P0_01,

    /// **Edge Connector Pin 15 (P0_13)**
    ///
    /// Small pin on the edge connector, can be used for SPI MOSI
    /// or general digital I/O operations.
    #[allow(dead_code)]
    pub p15: P0_13,

    /// **Edge Connector Pin 16 (P1_02)**
    ///
    /// Small pin on the edge connector, can be used for SPI CS
    /// or general digital I/O operations.
    #[allow(dead_code)]
    pub p16: P1_02,

    /// **Edge Connector Pin 19 (P0_26)**
    ///
    /// Small pin on the edge connector, can be used for I2C SCL
    /// or general digital I/O operations.
    #[allow(dead_code)]
    pub p19: P0_26,

    /// **Edge Connector Pin 20 (P1_00)**
    ///
    /// Small pin on the edge connector, can be used for I2C SDA
    /// or general digital I/O operations.
    #[allow(dead_code)]
    pub p20: P1_00,

    // Internal Interface Pins
    /// **Internal I2C SCL (P0_08)**
    ///
    /// Clock line for the internal I2C bus connecting to:
    /// - LSM303AGR accelerometer/magnetometer
    /// - Debug interface MCU
    ///
    /// # Warning
    /// Modifying this pin may interfere with onboard sensors.
    #[allow(dead_code)]
    pub i2c_int_scl: P0_08,

    /// **Internal I2C SDA (P0_16)**
    ///
    /// Data line for the internal I2C bus connecting to:
    /// - LSM303AGR accelerometer/magnetometer  
    /// - Debug interface MCU
    ///
    /// # Warning
    /// Modifying this pin may interfere with onboard sensors.
    #[allow(dead_code)]
    pub i2c_int_sda: P0_16,

    /// **Debug UART TX (P1_08)**
    ///
    /// UART transmit line to the debug MCU for USB serial communication.
    /// Used for debug output and programming interface.
    #[allow(dead_code)]
    pub uart_int_tx: P1_08,

    /// **Debug UART RX (P0_06)**
    ///
    /// UART receive line from the debug MCU for USB serial communication.
    /// Used for debug input and programming interface.
    #[allow(dead_code)]
    pub uart_int_rx: P0_06,

    // Communication Peripherals
    /// **SPI0/I2C0 Peripheral (TWISPI0)**
    ///
    /// Flexible communication peripheral that can be configured as:
    /// - SPI master/slave for high-speed serial communication
    /// - I2C master/slave for multi-device bus communication
    #[allow(dead_code)]
    pub twispi0: TWISPI0,

    /// **SPI1/I2C1 Peripheral (TWISPI1)**
    ///
    /// Second flexible communication peripheral for additional
    /// SPI or I2C interfaces when multiple buses are needed.
    #[allow(dead_code)]
    pub twispi1: TWISPI1,

    // PWM Peripherals
    /// **PWM0 Peripheral**
    ///
    /// First PWM peripheral for generating precise timing signals,
    /// motor control, audio generation, or LED brightness control.
    #[allow(dead_code)]
    pub pwm0: PWM0,

    /// **PWM1 Peripheral**
    ///
    /// Second PWM peripheral for additional PWM channels
    /// when multiple PWM outputs are required.
    #[allow(dead_code)]
    pub pwm1: PWM1,

    /// **PWM2 Peripheral**
    ///
    /// Third PWM peripheral providing more PWM channels
    /// for complex applications requiring many PWM signals.
    #[allow(dead_code)]
    pub pwm2: PWM2,

    /// **PWM3 Peripheral**
    ///
    /// Fourth PWM peripheral, completing the set of available
    /// PWM peripherals for maximum PWM channel availability.
    #[allow(dead_code)]
    pub pwm3: PWM3,

    // System Peripherals
    /// **PPI Channel 0**
    ///
    /// Programmable Peripheral Interconnect channel for creating
    /// hardware-triggered events between peripherals without CPU intervention.
    #[allow(dead_code)]
    pub ppi_ch0: PPI_CH0,

    /// **PPI Channel 1**
    ///
    /// Second PPI channel for additional hardware event routing
    /// and peripheral interconnection capabilities.
    #[allow(dead_code)]
    pub ppi_ch1: PPI_CH1,

    /// **Random Number Generator (RNG)**
    ///
    /// Hardware random number generator for cryptographic applications,
    /// random seed generation, or game mechanics requiring true randomness.
    #[allow(dead_code)]
    pub rng: RNG,

    /// **Successive Approximation ADC (SAADC)**
    ///
    /// High-resolution analog-to-digital converter for reading
    /// analog sensors, battery voltage, or other analog signals.
    #[allow(dead_code)]
    pub saadc: SAADC,
}

impl Default for Microbit {
    /// Creates a new Microbit instance with default Embassy configuration.
    ///
    /// This is a convenience method that initializes the board with
    /// Embassy's default nRF52833 configuration settings.
    ///
    /// # Returns
    /// A fully initialized `Microbit` struct with all peripherals ready to use.
    ///
    /// # Example
    /// ```no_run
    /// let board = Microbit::default();
    /// ```
    fn default() -> Self {
        Self::new(Default::default())
    }
}

impl Microbit {
    /// Creates a new Microbit instance with custom Embassy configuration.
    ///
    /// This method initializes the nRF52833 peripherals and configures
    /// the GPIO pins according to the micro:bit v2 hardware layout.
    ///
    /// # Pin Configuration Details
    /// - **LED Matrix Rows**: P0_21, P0_22, P0_15, P0_24, P0_19
    /// - **LED Matrix Columns**: P0_28, P0_11, P0_31, P1_05, P0_30
    /// - **Button A**: P0_14 (active low, external pull-up)
    /// - **Button B**: P0_23 (active low, external pull-up)
    ///
    /// # Arguments
    /// * `config` - Embassy nRF configuration struct for customizing hardware settings
    ///
    /// # Returns
    /// A fully initialized `Microbit` struct with all peripherals configured
    ///
    /// # Example
    /// ```no_run
    /// let mut config = embassy_nrf::config::Config::default();
    /// config.hfclk_source = embassy_nrf::config::HfclkSource::Internal;
    /// let board = Microbit::new(config);
    /// ```
    pub fn new(config: embassy_nrf::config::Config) -> Self {
        let p = embassy_nrf::init(config);
        // LED Matrix
        let rows = [
            output_pin(p.P0_21.degrade()),
            output_pin(p.P0_22.degrade()),
            output_pin(p.P0_15.degrade()),
            output_pin(p.P0_24.degrade()),
            output_pin(p.P0_19.degrade()),
        ];

        let cols = [
            output_pin(p.P0_28.degrade()),
            output_pin(p.P0_11.degrade()),
            output_pin(p.P0_31.degrade()),
            output_pin(p.P1_05.degrade()),
            output_pin(p.P0_30.degrade()),
        ];

        Self {
            display: LedMatrixDriver::new(rows, cols),
            btn_a: Input::new(p.P0_14.degrade(), Pull::None),
            btn_b: Input::new(p.P0_23.degrade(), Pull::None),
            uarte0: p.UARTE0,
            uarte1: p.UARTE1,
            timer0: p.TIMER0,
            speaker: p.P0_00,
            microphone: p.P0_05,
            micen: p.P0_20,
            p0: p.P0_02,
            p1: p.P0_03,
            p2: p.P0_04,
            p8: p.P0_10,
            p9: p.P0_09,
            p12: p.P0_12,
            p13: p.P0_17,
            p14: p.P0_01,
            p15: p.P0_13,
            p16: p.P1_02,
            p19: p.P0_26,
            p20: p.P1_00,
            i2c_int_scl: p.P0_08,
            i2c_int_sda: p.P0_16,
            uart_int_tx: p.P1_08,
            uart_int_rx: p.P0_06,
            ppi_ch0: p.PPI_CH0,
            ppi_ch1: p.PPI_CH1,
            twispi0: p.TWISPI0,
            twispi1: p.TWISPI1,
            pwm0: p.PWM0,
            pwm1: p.PWM1,
            pwm2: p.PWM2,
            pwm3: p.PWM3,
            rng: p.RNG,
            saadc: p.SAADC,
        }
    }
}

/// Creates a GPIO output pin with standard configuration.
///
/// This helper function configures a GPIO pin as an output with:
/// - **Initial State**: Low (0V)
/// - **Drive Strength**: Standard (sufficient for LED matrix)
/// - **Lifetime**: Static (lives for the entire program duration)
///
/// # Arguments
/// * `pin` - Any GPIO pin that implements the `Pin` trait
///
/// # Returns
/// A configured `Output` pin ready for use
///
/// # Usage
/// This function is used internally to configure the LED matrix row and column pins.
fn output_pin(pin: AnyPin) -> Output<'static> {
    Output::new(pin, Level::Low, OutputDrive::Standard)
}
