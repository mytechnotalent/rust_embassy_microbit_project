//! # BBC micro:bit LED Matrix Display Example
//!
//! This is a comprehensive example demonstrating how to use the BBC micro:bit's
//! 5x5 LED matrix display with Embassy async runtime. The application shows:
//!
//! - **LED Matrix Control**: Direct manipulation of the 5x5 LED matrix
//! - **Button Input Handling**: Responding to button A and B presses
//! - **Text Scrolling**: Animated text display across the LED matrix
//! - **Custom Graphics**: Displaying arrows and other bitmap patterns
//! - **Brightness Control**: Adjustable LED brightness levels
//! - **Async Programming**: Non-blocking operations using Embassy
//!
//! ## Hardware Requirements
//! - BBC micro:bit v2 (nRF52833-based)
//! - USB connection for programming and power
//!
//! ## Features Demonstrated
//! - Embedded graphics and font rendering
//! - Real-time button event handling
//! - Smooth text scrolling animations
//! - Custom bitmap display with timing
//! - Hardware abstraction layer (HAL) usage
//!
//! ## Usage
//! 1. Flash the program to your micro:bit
//! 2. The device will display "Hello, World!" on startup
//! 3. Press button A to show a left arrow
//! 4. Press button B to show a right arrow
//! 5. Watch the scrolling text and button responses
//!
//! ## Architecture
//! This example is now organized into separate modules:
//! - `board`: Hardware abstraction and peripheral initialization
//! - `button`: Button event handling and visual feedback logic
//! - `display`: LED matrix driver with graphics and animation support
//! - `fonts`: Character bitmaps and predefined graphics
//! - `types`: Core data structures for bitmaps and frames
//!
//! The main.rs file contains only the core application logic and imports from
//! the modular components for better code organization and maintainability.

#![no_std]
#![no_main]
#![warn(missing_docs)]
#![doc(html_root_url = "https://github.com/embassy-rs/embassy")]

use embassy_executor::Spawner;
use embassy_futures::select::{select, Either};
use {defmt_rtt as _, panic_probe as _};

// Import the modules we created
mod board;
mod button;
mod display;
mod fonts;
mod types;

// Import the types we need from our modules
use board::Microbit;
use button::{handle_button_a_press, handle_button_b_press};
use types::Brightness;

/// **Main Application Entry Point**
///
/// The primary async function that initializes the micro:bit hardware and
/// runs the main application loop. This function demonstrates:
///
/// ## Initialization Sequence
/// 1. **Hardware Setup**: Initialize micro:bit board with default config
/// 2. **Peripheral Access**: Extract display and button peripherals
/// 3. **Display Config**: Set maximum brightness for clear visibility
/// 4. **Welcome Message**: Show "Hello, World!" greeting with scrolling text
/// 5. **Ready State**: Log startup completion and begin button monitoring
///
/// ## Main Loop Operation
/// The application runs an infinite loop that:
/// - **Waits for Input**: Uses `select()` to wait for either button press
/// - **Handles Events**: Responds immediately to button A or B activation
/// - **Shows Feedback**: Displays appropriate arrow for pressed button
/// - **Continues**: Returns to waiting state after handling each press
///
/// ## Async Architecture
/// - **Non-blocking**: All operations use Embassy async/await
/// - **Efficient**: CPU yields during display timing and button waits
/// - **Responsive**: Immediate response to button events
/// - **Concurrent**: Multiple async operations can be composed
///
/// ## Hardware Interaction
/// - **LED Matrix**: Continuous multiplexing at 2000 Hz refresh rate
/// - **Button Input**: Edge detection with debouncing via Embassy
/// - **Power Management**: Efficient async waits reduce power consumption
///
/// ## Error Handling
/// The application uses Embassy's robust error handling and will:
/// - Gracefully handle button debouncing
/// - Recover from display timing issues
/// - Continue operation despite individual peripheral errors
///
/// # Parameters
/// * `_spawner` - Embassy task spawner (unused in this simple example)
///
/// # Example Interaction
/// ```text
/// 1. Device boots and shows "Hello, World!" scrolling
/// 2. User sees "Application started, press buttons!" in debug log
/// 3. Pressing button A shows left arrow (←) for 1 second
/// 4. Pressing button B shows right arrow (→) for 1 second
/// 5. Process repeats indefinitely
/// ```
///
/// # Note on Embassy Executor
/// This function is marked with `#[embassy_executor::main]` which makes it
/// the entry point for the Embassy async runtime. The executor handles:
/// - Task scheduling and timing
/// - Interrupt-driven I/O
/// - Power management during idle periods
/// - Async/await coordination
#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let board = Microbit::default();

    let mut display = board.display;
    let mut btn_a = board.btn_a;
    let mut btn_b = board.btn_b;

    display.set_brightness(Brightness::MAX);
    display.scroll("Hello, World!").await;
    defmt::info!("Application started, press buttons!");
    loop {
        match select(btn_a.wait_for_low(), btn_b.wait_for_low()).await {
            Either::First(_) => {
                handle_button_a_press(&mut display).await;
            }
            Either::Second(_) => {
                handle_button_b_press(&mut display).await;
            }
        }
    }
}
