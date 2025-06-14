//! # Display Types Module
//!
//! This module provides core data structures for bitmap graphics and LED matrix display
//! operations. It includes efficient bitmap storage, frame buffering, and brightness
//! control for the BBC micro:bit LED matrix.
//!
//! ## Core Types
//! - **Bitmap**: Compact bit storage for LED patterns
//! - **Frame**: NxM frame buffer for matrix display
//! - **Brightness**: LED intensity control enumeration
//!
//! ## Features
//! - **Efficient Storage**: Optimized bitmap representation using u8 arrays
//! - **Generic Frames**: Configurable frame sizes using const generics
//! - **Brightness Levels**: 11 discrete brightness levels (0-10)
//! - **Bit Manipulation**: Fast set/clear/test operations on individual pixels
//! - **Debug Support**: Comprehensive debugging and formatting support
//!
//! ## Usage Examples
//!
//! ### Creating Bitmaps
//! ```ignore
//! let mut bitmap = Bitmap::new(0b11111000, 5);
//! bitmap.set(2);  // Set bit 2
//! bitmap.clear(4); // Clear bit 4
//! ```
//!
//! ### Frame Operations
//! ```ignore
//! let mut frame = Frame::<5, 5>::new();
//! frame.set(2, 3, true);  // Set pixel at (2,3)
//! frame.shift_left(1);    // Shift entire frame left by 1
//! ```
//!
//! ### Brightness Control
//! ```ignore
//! display.set_brightness(Brightness::MAX);
//! display.set_brightness(Brightness::from_ratio(0.5));
//! ```

use core::ops::{AddAssign, SubAssign};

// TODO: Use const generic expressions to derive data size when stabilized
/// **Bitmap Storage Width**
///
/// Defines the width of the bitmap storage array in u8 words.
/// Currently set to 1 for optimal performance with 5x5 LED matrices.
const BITMAP_WIDTH: usize = 1;

/// **Bitmap Word Size**
///
/// Size of each storage word in bits (8 bits per u8).
/// Used for bit manipulation calculations and storage optimization.
const BITMAP_WORD_SIZE: usize = 8;

/// **Compact Bitmap Storage**
///
/// A bitmap with room for 8 bits used by Frame to create a compact frame buffer.
/// Provides efficient storage and manipulation of LED patterns for matrix displays.
#[derive(Clone, Copy, PartialEq)]
pub struct Bitmap {
    data: [u8; BITMAP_WIDTH],
    nbits: usize,
}

impl core::fmt::Debug for Bitmap {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        for i in 0..self.nbits {
            if self.is_set(i) {
                write!(f, "1")?;
            } else {
                write!(f, "0")?;
            }
        }
        Ok(())
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for Bitmap {
    fn format(&self, f: defmt::Formatter<'_>) {
        let mut s: heapless::String<32> = heapless::String::new();
        for i in 0..self.nbits {
            if self.is_set(i) {
                s.push('1').unwrap();
            } else {
                s.push('0').unwrap();
            }
        }
        defmt::write!(f, "{}", s.as_str());
    }
}

impl Bitmap {
    /// **Create New Bitmap**
    ///
    /// Creates a new bitmap with initial input data and specified number of bits.
    /// The input data is shifted to align with the most significant bits.
    ///
    /// # Arguments
    /// * `input` - Initial bitmap data as u8
    /// * `nbits` - Number of bits to use (must be ≤ 8)
    ///
    /// # Returns
    /// A new Bitmap instance with the specified data and bit count
    ///
    /// # Example
    /// ```ignore
    /// let bitmap = Bitmap::new(0b11100000, 3); // 3 bits set
    /// ```
    // TODO: Change input to array when const generics are fully stabilized
    pub const fn new(input: u8, nbits: usize) -> Self {
        let mut data = [0; BITMAP_WIDTH];
        //for i in 0..input.len() {
        if nbits < BITMAP_WORD_SIZE {
            data[0] = input << (BITMAP_WORD_SIZE - nbits);
        } else {
            data[0] = input;
        }
        //}
        Self { data, nbits }
    }

    /// **Create Empty Bitmap**
    ///
    /// Creates an empty bitmap with the specified number of bits.
    /// All bits are initially cleared (set to 0).
    ///
    /// # Arguments
    /// * `nbits` - Number of bits the bitmap should contain
    ///
    /// # Returns
    /// A new empty Bitmap instance
    ///
    /// # Example
    /// ```ignore
    /// let bitmap = Bitmap::empty(5); // 5-bit empty bitmap
    /// ```
    pub const fn empty(nbits: usize) -> Self {
        Self { data: [0; 1], nbits }
    }

    /// **Set Bit**
    ///
    /// Sets the specified bit position to 1 in the bitmap.
    ///
    /// # Arguments
    /// * `bit` - Zero-based bit position to set (must be < nbits)
    ///
    /// # Panics
    /// Panics if `bit` is greater than or equal to `nbits`
    ///
    /// # Example
    /// ```ignore
    /// let mut bitmap = Bitmap::empty(5);
    /// bitmap.set(2); // Set bit 2
    /// ```
    pub fn set(&mut self, bit: usize) {
        assert!(bit < self.nbits);
        let idx: usize = bit / BITMAP_WORD_SIZE;
        let p: usize = bit % BITMAP_WORD_SIZE;
        self.data[idx] |= 1 << ((BITMAP_WORD_SIZE - 1) - p);
    }

    /// **Clear All Bits**
    ///
    /// Clears all bits in the bitmap, setting them to 0.
    /// This is a bulk operation that resets the entire bitmap.
    ///
    /// # Example
    /// ```ignore
    /// let mut bitmap = Bitmap::new(0b11111111, 8);
    /// bitmap.clear_all(); // All bits now 0
    /// ```
    pub fn clear_all(&mut self) {
        for i in 0..self.data.len() {
            self.data[i] = 0;
        }
    }

    /// **Clear Bit**
    ///
    /// Clears the specified bit position to 0 in the bitmap.
    ///
    /// # Arguments
    /// * `bit` - Zero-based bit position to clear (must be < nbits)
    ///
    /// # Panics
    /// Panics if `bit` is greater than or equal to `nbits`
    ///
    /// # Example
    /// ```ignore
    /// let mut bitmap = Bitmap::new(0b11111111, 8);
    /// bitmap.clear(3); // Clear bit 3
    /// ```
    pub fn clear(&mut self, bit: usize) {
        assert!(bit < self.nbits);
        let idx: usize = bit / BITMAP_WORD_SIZE;
        let p: usize = bit % BITMAP_WORD_SIZE;
        self.data[idx] &= !(1 << ((BITMAP_WORD_SIZE - 1) - p));
    }

    /// **Check If Bit Is Set**
    ///
    /// Tests whether the specified bit position is set (1) in the bitmap.
    ///
    /// # Arguments
    /// * `bit` - Zero-based bit position to test (must be < nbits)
    ///
    /// # Returns
    /// `true` if the bit is set, `false` if it is clear
    ///
    /// # Panics
    /// Panics if `bit` is greater than or equal to `nbits`
    ///
    /// # Example
    /// ```ignore
    /// let bitmap = Bitmap::new(0b10100000, 3);
    /// assert!(bitmap.is_set(0)); // First bit is set
    /// assert!(!bitmap.is_set(1)); // Second bit is clear
    /// ```
    pub fn is_set(&self, bit: usize) -> bool {
        assert!(bit < self.nbits);
        let idx: usize = bit / BITMAP_WORD_SIZE;
        let p: usize = bit % BITMAP_WORD_SIZE;
        (self.data[idx] & (1 << ((BITMAP_WORD_SIZE - 1) - p))) != 0
    }

    /// **Shift Left**
    ///
    /// Shifts all bits in the bitmap to the left by the specified number of positions.
    /// Bits shifted beyond the left boundary are lost.
    ///
    /// # Arguments
    /// * `nbits` - Number of positions to shift left
    ///
    /// # Example
    /// ```ignore
    /// let mut bitmap = Bitmap::new(0b11000000, 8);
    /// bitmap.shift_left(2); // Now 0b00000000 (bits shifted out)
    /// ```
    pub fn shift_left(&mut self, nbits: usize) {
        for b in self.data.iter_mut() {
            *b <<= nbits;
        }
    }

    /// **Shift Right**
    ///
    /// Shifts all bits in the bitmap to the right by the specified number of positions.
    /// Bits shifted beyond the right boundary are lost.
    ///
    /// # Arguments
    /// * `nbits` - Number of positions to shift right
    ///
    /// # Example
    /// ```ignore
    /// let mut bitmap = Bitmap::new(0b11000000, 8);
    /// bitmap.shift_right(2); // Now 0b00110000
    /// ```
    pub fn shift_right(&mut self, nbits: usize) {
        for b in self.data.iter_mut() {
            *b >>= nbits;
        }
    }

    /// **Logical OR Operation**
    ///
    /// Performs a bitwise OR operation with another bitmap.
    /// Sets bits to 1 where either this bitmap or the other has a 1.
    ///
    /// # Arguments
    /// * `other` - Reference to another bitmap to OR with
    ///
    /// # Example
    /// ```ignore
    /// let mut bitmap1 = Bitmap::new(0b11000000, 8);
    /// let bitmap2 = Bitmap::new(0b00110000, 8);
    /// bitmap1.or(&bitmap2); // Result: 0b11110000
    /// ```
    pub fn or(&mut self, other: &Bitmap) {
        for i in 0..self.data.len() {
            self.data[i] |= other.data[i];
        }
    }

    /// **Logical AND Operation**
    ///
    /// Performs a bitwise AND operation with another bitmap.
    /// Sets bits to 1 only where both this bitmap and the other have a 1.
    ///
    /// # Arguments
    /// * `other` - Reference to another bitmap to AND with
    ///
    /// # Example
    /// ```ignore
    /// let mut bitmap1 = Bitmap::new(0b11110000, 8);
    /// let bitmap2 = Bitmap::new(0b11000000, 8);
    /// bitmap1.and(&bitmap2); // Result: 0b11000000
    /// ```
    pub fn and(&mut self, other: &Bitmap) {
        for i in 0..self.data.len() {
            self.data[i] &= other.data[i];
        }
    }
}

/// **Generic Frame Buffer for LED Matrix Display**
///
/// An NxM frame that can be displayed on a LED matrix display.
/// Provides efficient storage and manipulation of 2D bitmap patterns.
///
/// ## Features
/// - **Generic Dimensions**: Configurable width (XSIZE) and height (YSIZE)
/// - **Efficient Storage**: Uses compact Bitmap arrays for each row
/// - **Pixel Operations**: Individual pixel set/clear/test operations
/// - **Frame Manipulations**: Shift, OR, clear operations on entire frames
/// - **Display Ready**: Direct compatibility with LED matrix drivers
///
/// ## Type Parameters
/// - `XSIZE`: Frame width in pixels (typically 5 for micro:bit)
/// - `YSIZE`: Frame height in pixels (typically 5 for micro:bit)
///
/// ## Current Limitations
/// - **Width Restriction**: Currently limited to 8-bit width per row
/// - **Future Enhancement**: Will support arbitrary widths with const generics
///
/// ## Usage Examples
/// ```ignore
/// let mut frame = Frame::<5, 5>::new();
/// frame.set(2, 3, true);   // Set pixel at column 2, row 3
/// frame.shift_left(1);     // Shift entire frame left
/// ```
///
/// # Note on Coordinate System
/// - **X-axis**: Horizontal (columns), 0 = leftmost
/// - **Y-axis**: Vertical (rows), 0 = topmost
/// - **Origin**: Top-left corner (0,0)
#[derive(Clone, Copy, PartialEq)]
pub struct Frame<const XSIZE: usize, const YSIZE: usize> {
    bitmap: [Bitmap; YSIZE],
}

impl<const XSIZE: usize, const YSIZE: usize> core::fmt::Debug for Frame<XSIZE, YSIZE> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        for (i, b) in self.bitmap.iter().enumerate() {
            for j in 0..b.nbits {
                if self.bitmap[i].is_set(j) {
                    write!(f, "1")?;
                } else {
                    write!(f, "0")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(feature = "defmt")]
impl<const XSIZE: usize, const YSIZE: usize> defmt::Format for Frame<XSIZE, YSIZE> {
    fn format(&self, f: defmt::Formatter<'_>) {
        let mut s: heapless::String<1056> = heapless::String::new();
        for (i, b) in self.bitmap.iter().enumerate() {
            for j in 0..b.nbits {
                if self.bitmap[i].is_set(j) {
                    s.push('1').unwrap();
                } else {
                    s.push('0').unwrap();
                }
            }
            s.push('\n').unwrap();
        }
        defmt::write!(f, "{}", s.as_str());
    }
}

impl<const XSIZE: usize, const YSIZE: usize> Frame<XSIZE, YSIZE> {
    /// **Create Empty Frame**
    ///
    /// Creates a new frame with all pixels cleared (off).
    /// All bitmap rows are initialized to empty state.
    ///
    /// # Returns
    /// A new Frame instance with all pixels set to 0
    ///
    /// # Example
    /// ```ignore
    /// let frame = Frame::<5, 5>::empty();
    /// // All pixels are initially off
    /// ```
    pub const fn empty() -> Self {
        Self {
            bitmap: [Bitmap::empty(XSIZE); YSIZE],
        }
    }

    /// **Create Frame from Bitmap Array**
    ///
    /// Creates a new frame using a pre-configured array of bitmaps.
    /// Each bitmap represents one row of the frame.
    ///
    /// # Arguments
    /// * `bitmap` - Array of Bitmap instances, one per row
    ///
    /// # Returns
    /// A new Frame instance using the provided bitmap data
    ///
    /// # Example
    /// ```ignore
    /// let bitmaps = [
    ///     Bitmap::new(0b11111000, 5), // Top row
    ///     Bitmap::new(0b10001000, 5), // Second row
    ///     // ... more rows
    /// ];
    /// let frame = Frame::new(bitmaps);
    /// ```
    pub const fn new(bitmap: [Bitmap; YSIZE]) -> Self {
        Self { bitmap }
    }

    /// **Clear Frame**
    ///
    /// Clears all pixels in the frame, setting them to the off state.
    /// This is equivalent to turning off all LEDs in the matrix.
    ///
    /// # Example
    /// ```ignore
    /// let mut frame = Frame::<5, 5>::new(some_pattern);
    /// frame.clear(); // All pixels now off
    /// ```
    pub fn clear(&mut self) {
        for m in self.bitmap.iter_mut() {
            m.clear_all();
        }
    }

    /// **Set Pixel**
    ///
    /// Sets the pixel at the specified coordinates to the on state.
    /// Uses zero-based indexing with (0,0) at the top-left corner.
    ///
    /// # Arguments
    /// * `x` - Column position (0 to XSIZE-1)
    /// * `y` - Row position (0 to YSIZE-1)
    ///
    /// # Panics
    /// Panics if x >= XSIZE or y >= YSIZE
    ///
    /// # Example
    /// ```ignore
    /// let mut frame = Frame::<5, 5>::empty();
    /// frame.set(2, 3); // Set pixel at column 2, row 3
    /// ```
    pub fn set(&mut self, x: usize, y: usize) {
        self.bitmap[y].set(x);
    }

    /// **Clear Pixel**
    ///
    /// Clears the pixel at the specified coordinates to the off state.
    /// Uses zero-based indexing with (0,0) at the top-left corner.
    ///
    /// # Arguments
    /// * `x` - Column position (0 to XSIZE-1)
    /// * `y` - Row position (0 to YSIZE-1)
    ///
    /// # Panics
    /// Panics if x >= XSIZE or y >= YSIZE
    ///
    /// # Example
    /// ```ignore
    /// let mut frame = Frame::<5, 5>::new(some_pattern);
    /// frame.unset(1, 2); // Clear pixel at column 1, row 2
    /// ```
    pub fn unset(&mut self, x: usize, y: usize) {
        self.bitmap[y].clear(x);
    }

    /// **Check If Pixel Is Set**
    ///
    /// Tests whether the pixel at the specified coordinates is in the on state.
    /// Uses zero-based indexing with (0,0) at the top-left corner.
    ///
    /// # Arguments
    /// * `x` - Column position (0 to XSIZE-1)
    /// * `y` - Row position (0 to YSIZE-1)
    ///
    /// # Returns
    /// `true` if the pixel is on, `false` if it is off
    ///
    /// # Panics
    /// Panics if x >= XSIZE or y >= YSIZE
    ///
    /// # Example
    /// ```ignore
    /// let frame = Frame::<5, 5>::new(some_pattern);
    /// if frame.is_set(2, 3) {
    ///     // Pixel at (2,3) is on
    /// }
    /// ```
    pub fn is_set(&self, x: usize, y: usize) -> bool {
        self.bitmap[y].is_set(x)
    }

    /// **Logical OR with Another Frame**
    ///
    /// Performs a bitwise OR operation with another frame of the same size.
    /// Sets pixels to on where either this frame or the other has them on.
    ///
    /// # Arguments
    /// * `other` - Reference to another frame to OR with
    ///
    /// # Example
    /// ```ignore
    /// let mut frame1 = Frame::<5, 5>::new(pattern1);
    /// let frame2 = Frame::<5, 5>::new(pattern2);
    /// frame1.or(&frame2); // Combines both patterns
    /// ```
    pub fn or(&mut self, other: &Frame<XSIZE, YSIZE>) {
        for i in 0..self.bitmap.len() {
            self.bitmap[i].or(&other.bitmap[i]);
        }
    }

    /// **Shift Frame Left**
    ///
    /// Shifts all pixels in the frame to the left by the specified number of positions.
    /// Pixels shifted beyond the left edge are lost, and empty pixels appear on the right.
    ///
    /// # Arguments
    /// * `nbits` - Number of positions to shift left
    ///
    /// # Example
    /// ```ignore
    /// let mut frame = Frame::<5, 5>::new(some_pattern);
    /// frame.shift_left(1); // Shift entire frame left by 1 pixel
    /// ```
    pub fn shift_left(&mut self, nbits: usize) {
        for i in 0..self.bitmap.len() {
            self.bitmap[i].shift_left(nbits);
        }
    }

    /// **Shift Frame Right**
    ///
    /// Shifts all pixels in the frame to the right by the specified number of positions.
    /// Pixels shifted beyond the right edge are lost, and empty pixels appear on the left.
    ///
    /// # Arguments
    /// * `nbits` - Number of positions to shift right
    ///
    /// # Example
    /// ```ignore
    /// let mut frame = Frame::<5, 5>::new(some_pattern);
    /// frame.shift_right(1); // Shift entire frame right by 1 pixel
    /// ```
    pub fn shift_right(&mut self, nbits: usize) {
        for i in 0..self.bitmap.len() {
            self.bitmap[i].shift_right(nbits);
        }
    }

    /// **Logical AND with Another Frame**
    ///
    /// Performs a bitwise AND operation with another frame of the same size.
    /// Sets pixels to on only where both this frame and the other have them on.
    ///
    /// # Arguments
    /// * `other` - Reference to another frame to AND with
    ///
    /// # Example
    /// ```ignore
    /// let mut frame1 = Frame::<5, 5>::new(pattern1);
    /// let frame2 = Frame::<5, 5>::new(pattern2);
    /// frame1.and(&frame2); // Intersection of both patterns
    /// ```
    pub fn and(&mut self, other: &Frame<XSIZE, YSIZE>) {
        for i in 0..self.bitmap.len() {
            self.bitmap[i].and(&other.bitmap[i]);
        }
    }
}

impl<const XSIZE: usize, const YSIZE: usize> Default for Frame<XSIZE, YSIZE> {
    fn default() -> Self {
        Frame::empty()
    }
}

/// **LED Matrix Brightness Control**
///
/// A brightness setting for the LED matrix display that provides
/// 11 discrete brightness levels (0-10) for optimal visibility control.
///
/// ## Features
/// - **Range**: 11 brightness levels from 0 (off) to 10 (maximum)
/// - **Clamping**: Automatic clamping to valid range
/// - **Arithmetic**: Support for += and -= operations
/// - **Defaults**: Reasonable default brightness level (5)
///
/// ## Brightness Levels
/// - **0**: LEDs off (minimum)
/// - **1-4**: Low brightness levels
/// - **5**: Default/medium brightness
/// - **6-9**: High brightness levels  
/// - **10**: Maximum brightness
///
/// ## Usage Examples
/// ```ignore
/// // Set maximum brightness
/// display.set_brightness(Brightness::MAX);
///
/// // Set minimum brightness
/// display.set_brightness(Brightness::MIN);
///
/// // Custom brightness level
/// let brightness = Brightness::new(7);
///
/// // Adjust brightness dynamically
/// let mut brightness = Brightness::default();
/// brightness += 2; // Increase by 2 levels
/// brightness -= 1; // Decrease by 1 level
/// ```
#[derive(Clone, Copy)]
pub struct Brightness(u8);

impl Brightness {
    /// **Maximum Brightness Level**
    ///
    /// The highest brightness setting (level 10) for maximum LED intensity.
    /// Use this for optimal visibility in bright environments.
    pub const MAX: Brightness = Brightness(10);

    /// **Minimum Brightness Level**
    ///
    /// The lowest brightness setting (level 0) which turns LEDs off.
    /// Use this to disable the display or for very dark environments.
    #[allow(dead_code)]
    pub const MIN: Brightness = Brightness(0);

    /// **Create Custom Brightness Level**
    ///
    /// Creates a new brightness setting with the specified level.
    /// The level is automatically clamped to the valid range (0-10).
    ///
    /// # Arguments
    /// * `level` - Desired brightness level (will be clamped to 0-10)
    ///
    /// # Returns
    /// A new Brightness instance with the clamped level
    ///
    /// # Example
    /// ```ignore
    /// let brightness = Brightness::new(15); // Clamped to 10
    /// let brightness = Brightness::new(3);  // Level 3
    /// ```
    #[allow(dead_code)]
    pub fn new(level: u8) -> Self {
        Self(level.clamp(Self::MIN.0, Self::MAX.0))
    }

    /// **Get Brightness Level**
    ///
    /// Returns the current brightness level as a u8 value.
    ///
    /// # Returns
    /// The brightness level (0-10)
    ///
    /// # Example
    /// ```ignore
    /// let brightness = Brightness::new(7);
    /// assert_eq!(brightness.level(), 7);
    /// ```
    pub fn level(&self) -> u8 {
        self.0
    }
}

impl Default for Brightness {
    fn default() -> Self {
        Self(5)
    }
}

impl AddAssign<u8> for Brightness {
    fn add_assign(&mut self, rhs: u8) {
        self.0 += core::cmp::min(Self::MAX.level() - self.0, rhs);
    }
}

impl SubAssign<u8> for Brightness {
    fn sub_assign(&mut self, rhs: u8) {
        self.0 -= core::cmp::min(self.0, rhs);
    }
}
