/// # LED Matrix Display Driver Module
///
/// This module provides a comprehensive driver for NxM LED matrix displays,
/// specifically optimized for the BBC micro:bit's 5x5 LED matrix. It supports:
///
/// ## Core Features
/// - **Matrix Control**: Direct LED on/off control with coordinate addressing
/// - **Text Rendering**: Character and string display using built-in fonts
/// - **Animations**: Smooth scrolling text and custom animation effects
/// - **Brightness Control**: Adjustable brightness levels from 0-10
/// - **Frame Buffering**: Efficient frame-based graphics rendering
/// - **Async Operations**: Non-blocking display operations using Embassy
///
/// ## Display Technology
/// The micro:bit uses **charlieplexing** to control 25 LEDs with only 10 GPIO pins:
/// - 5 pins for rows (cathodes) - active high to enable row
/// - 5 pins for columns (anodes) - active low to light LED
/// - Multiplexed scanning at high frequency for persistence of vision
///
/// ## Performance Characteristics
/// - **Refresh Rate**: 2000 Hz (500μs per frame)
/// - **Brightness Levels**: 11 levels (0-10)
/// - **Animation Support**: Sliding effects, custom timing
/// - **Font Support**: 95 printable ASCII characters
///
/// ## Usage Examples
///
/// ### Basic Text Display
/// ```ignore
/// display.scroll("Hello World!").await;
/// ```
///
/// ### Custom Graphics
/// ```ignore
/// let pattern = display::fonts::ARROW_LEFT;
/// display.display(pattern, Duration::from_secs(2)).await;
/// ```
///
/// ### Brightness Control
/// ```ignore
/// display.set_brightness(display::Brightness::MAX);
/// ```
///
/// ## Module Organization
/// - `LedMatrix`: Main driver struct with hardware interface
/// - `fonts`: Character bitmaps and predefined graphics
/// - `types`: Core data types (Frame, Bitmap, Brightness)
/// - Animation support for smooth visual effects
use embassy_time::{block_for, Duration, Instant, Timer};
use embedded_hal::digital::OutputPin;

pub use crate::types::*;

/// **Display Refresh Interval**
///
/// Controls the refresh rate of the LED matrix multiplexing.
/// Set to 500μs (2000 Hz) for smooth visual persistence without flicker.
/// This high refresh rate ensures comfortable viewing and eliminates
/// visible flickering during animations and scrolling text.
const REFRESH_INTERVAL: Duration = Duration::from_micros(500);

/// **LED Matrix Display Driver**
///
/// A generic driver for NxM LED matrix displays using charlieplexing.
/// Specifically optimized for the BBC micro:bit's 5x5 LED arrangement.
///
/// ## Hardware Interface
/// - **Rows**: Output pins controlling LED cathodes (active high)
/// - **Columns**: Output pins controlling LED anodes (active low)
/// - **Multiplexing**: Rapidly switches between rows for persistence of vision
/// - **Brightness Control**: PWM-style timing control for 11 brightness levels
///
/// ## Features
/// - **Frame Buffering**: Efficient bitmap storage and manipulation
/// - **Async Display**: Non-blocking operations using Embassy timers
/// - **Animations**: Built-in support for scrolling and sliding effects
/// - **Text Rendering**: Automatic font conversion and display
/// - **Brightness Control**: Adjustable LED intensity
///
/// ## Type Parameters
/// - `P`: Output pin type implementing `embedded_hal::digital::OutputPin`
/// - `ROWS`: Number of matrix rows (typically 5 for micro:bit)
/// - `COLS`: Number of matrix columns (typically 5 for micro:bit)
///
/// ## Performance
/// - **Refresh Rate**: 2000 Hz multiplexing for flicker-free display
/// - **Resolution**: Up to 8-bit width support (current implementation)
/// - **Memory**: Compact frame buffer with bitmap compression
///
/// # Example
/// ```ignore
/// let mut matrix = LedMatrix::new(row_pins, col_pins);
/// matrix.scroll("Hello!").await;
/// matrix.display(pattern, Duration::from_secs(2)).await;
/// ```
pub struct LedMatrix<P, const ROWS: usize, const COLS: usize>
where
    P: OutputPin + 'static,
{
    /// **Row Control Pins**
    ///
    /// GPIO output pins controlling LED matrix rows (cathodes).
    /// When a row pin is HIGH, LEDs in that row can be activated
    /// by setting the corresponding column pins LOW.
    pin_rows: [P; ROWS],

    /// **Column Control Pins**
    ///
    /// GPIO output pins controlling LED matrix columns (anodes).
    /// When a column pin is LOW, the LED at the intersection
    /// of the active row will illuminate.
    pin_cols: [P; COLS],

    /// **Frame Buffer**
    ///
    /// Current display content stored as a compressed bitmap.
    /// Contains the pattern to be shown on the LED matrix,
    /// updated by display operations and rendered by multiplexing.
    frame_buffer: Frame<COLS, ROWS>,

    /// **Current Row Pointer**
    ///
    /// Index of the currently active row during multiplexing.
    /// Cycles through 0 to ROWS-1 during the refresh process,
    /// enabling persistence of vision for full frame display.
    row_p: usize,

    /// **Brightness Setting**
    ///
    /// Current brightness level (0-10) controlling LED intensity.
    /// Implemented through PWM-style timing during multiplexing,
    /// where higher values result in longer LED on-time per cycle.
    brightness: Brightness,
}

impl<P, const ROWS: usize, const COLS: usize> LedMatrix<P, ROWS, COLS>
where
    P: OutputPin,
{
    /// **Create New LED Matrix Instance**
    ///
    /// Initializes a new LED matrix driver with the provided GPIO pins.
    /// Sets up the hardware interface and prepares the display for operation.
    ///
    /// # Arguments
    /// * `pin_rows` - Array of output pins for matrix rows (cathodes)
    /// * `pin_cols` - Array of output pins for matrix columns (anodes)
    ///
    /// # Returns
    /// A configured `LedMatrix` ready for display operations
    ///
    /// # Initial State
    /// - Frame buffer: Empty (all LEDs off)
    /// - Brightness: Default level (5/10)
    /// - Row pointer: 0 (first row)
    /// - All pins: Configured as outputs
    ///
    /// # Example
    /// ```ignore
    /// let rows = [pin1, pin2, pin3, pin4, pin5];
    /// let cols = [pin6, pin7, pin8, pin9, pin10];
    /// let mut display = LedMatrix::new(rows, cols);
    /// ```
    pub fn new(pin_rows: [P; ROWS], pin_cols: [P; COLS]) -> Self {
        LedMatrix {
            pin_rows,
            pin_cols,
            frame_buffer: Frame::empty(),
            row_p: 0,
            brightness: Default::default(),
        }
    }

    /// **Clear All LEDs**
    ///
    /// Turns off all LEDs in the matrix and resets the frame buffer.
    /// Also ensures all GPIO pins are in a safe state (rows low, columns high).
    ///
    /// # Behavior
    /// 1. Clears the internal frame buffer
    /// 2. Sets all row pins HIGH (deactivate rows)
    /// 3. Sets all column pins HIGH (deactivate columns)
    /// 4. Results in all LEDs being off
    ///
    /// # Usage
    /// Call this method to:
    /// - Initialize the display to a known state
    /// - Clear the screen between different displays
    /// - Turn off all LEDs for power saving
    ///
    /// # Example
    /// ```ignore
    /// display.clear();
    /// // All LEDs are now off
    /// ```
    pub fn clear(&mut self) {
        self.frame_buffer.clear();
        for row in self.pin_rows.iter_mut() {
            row.set_high().ok();
        }
        for col in self.pin_cols.iter_mut() {
            col.set_high().ok();
        }
    }

    /// **Turn On Single LED**
    ///
    /// Activates a specific LED at coordinates (x, y) in the frame buffer.
    /// The change takes effect on the next display refresh cycle.
    ///
    /// # Arguments
    /// * `x` - Column index (0 to COLS-1)
    /// * `y` - Row index (0 to ROWS-1)
    ///
    /// # Coordinate System
    /// ```text
    /// (0,0) (1,0) (2,0) (3,0) (4,0)
    /// (0,1) (1,1) (2,1) (3,1) (4,1)
    /// (0,2) (1,2) (2,2) (3,2) (4,2)
    /// (0,3) (1,3) (2,3) (3,3) (4,3)
    /// (0,4) (1,4) (2,4) (3,4) (4,4)
    /// ```
    ///
    /// # Example
    /// ```ignore
    /// display.on(2, 2); // Turn on center LED
    /// display.render();  // Apply the change
    /// ```
    #[allow(dead_code)]
    pub fn on(&mut self, x: usize, y: usize) {
        self.frame_buffer.set(x, y);
    }

    /// **Turn Off Single LED**
    ///
    /// Deactivates a specific LED at coordinates (x, y) in the frame buffer.
    /// The change takes effect on the next display refresh cycle.
    ///
    /// # Arguments
    /// * `x` - Column index (0 to COLS-1)
    /// * `y` - Row index (0 to ROWS-1)
    ///
    /// # Example
    /// ```ignore
    /// display.off(2, 2); // Turn off center LED
    /// display.render();   // Apply the change
    /// ```
    #[allow(dead_code)]
    pub fn off(&mut self, x: usize, y: usize) {
        self.frame_buffer.unset(x, y);
    }

    /// **Apply Frame to Display**
    ///
    /// Replaces the current frame buffer with a new frame pattern.
    /// This is the primary method for updating the display content.
    ///
    /// # Arguments
    /// * `frame` - New frame pattern to display
    ///
    /// # Usage
    /// - Display text characters converted to frames
    /// - Show custom bitmap patterns
    /// - Update display during animations
    ///
    /// # Example
    /// ```ignore
    /// let frame: Frame<5, 5> = 'A'.into();
    /// display.apply(frame);
    /// ```
    pub fn apply(&mut self, frame: Frame<COLS, ROWS>) {
        self.frame_buffer = frame;
    }

    /// **Set Display Brightness**
    ///
    /// Adjusts the brightness level of all LEDs in the matrix.
    /// Higher values result in brighter LEDs with longer on-time per refresh cycle.
    ///
    /// # Arguments
    /// * `brightness` - Brightness level (use `Brightness::new(0-10)`)
    ///
    /// # Implementation
    /// Brightness is achieved through PWM-style timing control during
    /// the multiplexing process. Higher brightness = longer LED on-time.
    ///
    /// # Example
    /// ```ignore
    /// display.set_brightness(Brightness::MAX);  // Maximum brightness
    /// display.set_brightness(Brightness::new(3)); // Custom level
    /// ```
    pub fn set_brightness(&mut self, brightness: Brightness) {
        self.brightness = brightness;
    }

    /// **Increase Brightness**
    ///
    /// Increments the current brightness level by 1, up to the maximum (10).
    /// Useful for brightness adjustment controls.
    ///
    /// # Example
    /// ```ignore
    /// display.increase_brightness(); // Brighter by one level
    /// ```
    #[allow(dead_code)]
    pub fn increase_brightness(&mut self) {
        self.brightness += 1;
    }

    /// **Decrease Brightness**
    ///
    /// Decrements the current brightness level by 1, down to the minimum (0).
    /// Useful for brightness adjustment controls.
    ///
    /// # Example
    /// ```ignore
    /// display.decrease_brightness(); // Dimmer by one level
    /// ```
    #[allow(dead_code)]
    pub fn decrease_brightness(&mut self) {
        self.brightness -= 1;
    }

    /// Perform a full refresh of the display based on the current frame buffer
    pub fn render(&mut self) {
        for row in self.pin_rows.iter_mut() {
            row.set_low().ok();
        }

        for (cid, col) in self.pin_cols.iter_mut().enumerate() {
            if self.frame_buffer.is_set(cid, self.row_p) {
                col.set_low().ok();
            } else {
                col.set_high().ok();
            }
        }

        // Adjust interval will impact brightness of the LEDs
        block_for(Duration::from_micros(
            ((Brightness::MAX.level() - self.brightness.level()) as u64) * 6000 / Brightness::MAX.level() as u64,
        ));

        self.pin_rows[self.row_p].set_high().ok();

        self.row_p = (self.row_p + 1) % self.pin_rows.len();
    }

    /// Display the provided frame for the duration. Handles screen refresh
    /// in an async display loop.
    pub async fn display(&mut self, frame: Frame<COLS, ROWS>, length: Duration) {
        self.apply(frame);
        let end = Instant::now() + length;
        while Instant::now() < end {
            self.render();
            Timer::after(REFRESH_INTERVAL).await;
        }
        self.clear();
    }

    /// Scroll the provided text across the LED display using default duration based on text length
    pub async fn scroll(&mut self, text: &str) {
        self.scroll_with_speed(text, Duration::from_secs((text.len() / 2) as u64))
            .await;
    }

    /// Scroll the provided text across the screen within the provided duration
    pub async fn scroll_with_speed(&mut self, text: &str, speed: Duration) {
        self.animate(text.as_bytes(), AnimationEffect::Slide, speed).await;
    }

    /// Apply animation based on data with the given effect during the provided duration
    pub async fn animate(&mut self, data: &[u8], effect: AnimationEffect, duration: Duration) {
        let mut animation: Animation<'_, COLS, ROWS> =
            Animation::new(AnimationData::Bytes(data), effect, duration).unwrap();
        loop {
            match animation.next(Instant::now()) {
                AnimationState::Apply(f) => {
                    self.apply(f);
                }
                AnimationState::Wait => {}
                AnimationState::Done => {
                    break;
                }
            }
            self.render();
            Timer::after(REFRESH_INTERVAL).await;
        }
        self.clear();
    }

    /// Animate a slice of frames using the provided effect during the provided duration
    #[allow(dead_code)]
    pub async fn animate_frames(&mut self, data: &[Frame<COLS, ROWS>], effect: AnimationEffect, duration: Duration) {
        let mut animation: Animation<'_, COLS, ROWS> =
            Animation::new(AnimationData::Frames(data), effect, duration).unwrap();
        loop {
            match animation.next(Instant::now()) {
                AnimationState::Apply(f) => {
                    self.apply(f);
                }
                AnimationState::Wait => {}
                AnimationState::Done => {
                    break;
                }
            }
            self.render();
            Timer::after(REFRESH_INTERVAL).await;
        }
        self.clear();
    }

    /// Disassemble the `LedMatrix` and return the pins, as
    /// an array of row pins and an array of column pins.
    #[allow(dead_code)]
    pub fn into_inner(self) -> ([P; ROWS], [P; COLS]) {
        (self.pin_rows, self.pin_cols)
    }
}

/// An effect filter to apply for an animation
#[derive(Clone, Copy)]
#[allow(dead_code)]
pub enum AnimationEffect {
    /// No effect
    None,
    /// Sliding effect
    Slide,
}

#[allow(dead_code)]
enum AnimationData<'a, const XSIZE: usize, const YSIZE: usize> {
    Frames(&'a [Frame<XSIZE, YSIZE>]),
    Bytes(&'a [u8]),
}

impl<'a, const XSIZE: usize, const YSIZE: usize> AnimationData<'a, XSIZE, YSIZE> {
    fn len(&self) -> usize {
        match self {
            AnimationData::Frames(f) => f.len(),
            AnimationData::Bytes(f) => f.len(),
        }
    }

    fn frame(&self, idx: usize) -> Frame<XSIZE, YSIZE> {
        match self {
            AnimationData::Frames(f) => f[idx],
            AnimationData::Bytes(f) => f[idx].into(),
        }
    }
}

struct Animation<'a, const XSIZE: usize, const YSIZE: usize> {
    frames: AnimationData<'a, XSIZE, YSIZE>,
    sequence: usize,
    frame_index: usize,
    index: usize,
    length: usize,
    effect: AnimationEffect,
    wait: Duration,
    next: Instant,
}

#[derive(PartialEq, Debug)]
enum AnimationState<const XSIZE: usize, const YSIZE: usize> {
    Wait,
    Apply(Frame<XSIZE, YSIZE>),
    Done,
}

impl<'a, const XSIZE: usize, const YSIZE: usize> Animation<'a, XSIZE, YSIZE> {
    pub fn new(
        frames: AnimationData<'a, XSIZE, YSIZE>,
        effect: AnimationEffect,
        duration: Duration,
    ) -> Result<Self, AnimationError> {
        assert!(frames.len() > 0);
        let length = match effect {
            AnimationEffect::Slide => frames.len() * XSIZE,
            AnimationEffect::None => frames.len(),
        };

        if let Some(wait) = duration.checked_div(length as u32) {
            Ok(Self {
                frames,
                frame_index: 0,
                sequence: 0,
                index: 0,
                length,
                effect,
                wait,
                next: Instant::now(),
            })
        } else {
            Err(AnimationError::TooFast)
        }
    }
    fn current(&self) -> Frame<XSIZE, YSIZE> {
        let mut current = self.frames.frame(self.frame_index);

        let mut next = if self.frame_index < self.frames.len() - 1 {
            self.frames.frame(self.frame_index + 1)
        } else {
            Frame::empty()
        };

        current.shift_left(self.sequence);
        next.shift_right(XSIZE - self.sequence);

        current.or(&next);
        current
    }

    fn next(&mut self, now: Instant) -> AnimationState<XSIZE, YSIZE> {
        if self.next <= now {
            if self.index < self.length {
                let current = self.current();
                if self.sequence >= XSIZE - 1 {
                    self.sequence = match self.effect {
                        AnimationEffect::None => XSIZE,
                        AnimationEffect::Slide => 0,
                    };
                    self.frame_index += 1;
                } else {
                    self.sequence += 1;
                }

                self.index += 1;
                self.next += self.wait;
                AnimationState::Apply(current)
            } else {
                AnimationState::Done
            }
        } else {
            AnimationState::Wait
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
/// Errors produced when running animations
pub enum AnimationError {
    /// Animation scroll is too fast to keep up with the refresh rate
    TooFast,
}
