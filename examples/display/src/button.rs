//! # Button Handler Module
//!
//! This module provides button event handling functionality for the BBC micro:bit.
//! It abstracts the button press logic and visual feedback mechanisms to keep
//! the main application loop clean and focused.
//!
//! ## Features
//! - **Button A Handler**: Left arrow display on button A press
//! - **Button B Handler**: Right arrow display on button B press
//! - **Visual Feedback**: Immediate arrow display for user interaction
//! - **Debug Logging**: Button press events logged for debugging
//! - **Async Operations**: Non-blocking button response handling
//!
//! ## Usage
//! ```ignore
//! use button::{handle_button_a_press, handle_button_b_press};
//!
//! // In main loop
//! match select(btn_a.wait_for_low(), btn_b.wait_for_low()).await {
//!     Either::First(_) => {
//!         handle_button_a_press(&mut display).await;
//!     }
//!     Either::Second(_) => {
//!         handle_button_b_press(&mut display).await;
//!     }
//! }
//! ```

use crate::board::LedMatrix;
use crate::fonts::{ARROW_LEFT, ARROW_RIGHT};
use crate::types::Frame;
use embassy_time::Duration;

/// **Display Button Press Feedback**
///
/// Generic function to show visual feedback when a button is pressed.
/// This function displays a specified arrow pattern for 1 second and
/// logs the button press event for debugging purposes.
///
/// # Arguments
/// * `display` - Mutable reference to the LED matrix display driver
/// * `button_name` - Name of the button pressed (for logging)
/// * `arrow` - The arrow pattern to display (Frame<5,5> bitmap)
///
/// # Behavior
/// - Logs button press event with button name
/// - Displays the arrow pattern for exactly 1 second
/// - Returns control to caller after display period
///
/// # Async Operation
/// This function is async and will yield control during the display period,
/// allowing other tasks to run while the arrow is being shown.
///
/// # Example
/// ```ignore
/// show_button_press(&mut display, "A", ARROW_LEFT).await;
/// ```
async fn show_button_press(display: &mut LedMatrix, button_name: &str, arrow: Frame<5, 5>) {
    defmt::info!("{} pressed", button_name);
    display.display(arrow, Duration::from_secs(1)).await;
}

/// **Handle Button A Press Events**
///
/// Responds to button A press by displaying a left arrow indication.
/// This provides immediate visual feedback when the left button is activated.
///
/// # Arguments
/// * `display` - Mutable reference to the LED matrix display driver
///
/// # Behavior
/// - Shows `ARROW_LEFT` pattern for 1 second
/// - Logs "A pressed" message for debugging
/// - Uses non-blocking async display operation
///
/// # Visual Indication
/// ```text
/// ··▪··
/// ·▪···
/// ▪▪▪▪▪
/// ·▪···
/// ··▪··
/// ```
///
/// # Example
/// ```ignore
/// if button_a_pressed {
///     handle_button_a_press(&mut display).await;
/// }
/// ```
pub async fn handle_button_a_press(display: &mut LedMatrix) {
    show_button_press(display, "A", ARROW_LEFT).await;
}

/// **Handle Button B Press Events**
///
/// Responds to button B press by displaying a right arrow indication.
/// This provides immediate visual feedback when the right button is activated.
///
/// # Arguments
/// * `display` - Mutable reference to the LED matrix display driver
///
/// # Behavior
/// - Shows `ARROW_RIGHT` pattern for 1 second
/// - Logs "B pressed" message for debugging
/// - Uses non-blocking async display operation
///
/// # Visual Indication
/// ```text
/// ··▪··
/// ···▪·
/// ▪▪▪▪▪
/// ···▪·
/// ··▪··
/// ```
///
/// # Example
/// ```ignore
/// if button_b_pressed {
///     handle_button_b_press(&mut display).await;
/// }
/// ```
pub async fn handle_button_b_press(display: &mut LedMatrix) {
    show_button_press(display, "B", ARROW_RIGHT).await;
}
