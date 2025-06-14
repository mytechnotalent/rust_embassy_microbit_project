/// # Font and Graphics Module
///
/// This module contains bitmap fonts and predefined graphics for the LED matrix display.
/// It provides comprehensive character support and common symbols for micro:bit applications.
///
/// ## Font Details
/// - **Character Set**: 95 printable ASCII characters (32-126)
/// - **Font Style**: Pendolino3 - a compact 5x5 pixel font
/// - **Source**: Based on lancaster-university/microbit-dal font
/// - **Format**: Each character stored as 5 bytes (one per row)
///
/// ## Available Graphics
/// - **Arrows**: Left, Right directional indicators
/// - **Symbols**: Check mark, Cross mark for status indication
/// - **Custom Bitmaps**: Easy creation of 5x5 patterns
///
/// ## Character Encoding
/// Characters are stored in row-major order with each row as a single byte:
/// ```text
/// Bit 7: Leftmost pixel
/// Bit 6: Second pixel
/// Bit 5: Third pixel
/// Bit 4: Fourth pixel
/// Bit 3: Rightmost pixel
/// Bits 2-0: Unused (padding)
/// ```
///
/// ## Usage Examples
///
/// ### Text Conversion
/// ```ignore
/// let frame: Frame<5, 5> = 'A'.into();
/// let frame: Frame<5, 5> = b'B'.into();
/// ```
///
/// ### Predefined Graphics
/// ```ignore
/// display.display(fonts::ARROW_LEFT, Duration::from_secs(1)).await;
/// display.display(fonts::CHECK_MARK, Duration::from_secs(1)).await;
/// ```
///
/// ### Custom Bitmaps
/// ```ignore
/// let custom = fonts::frame_5x5(&[
///     0b11111, // Top row
///     0b10001, // Sides
///     0b10101, // Pattern
///     0b10001, // Sides
///     0b11111, // Bottom row
/// ]);
/// ```

use crate::types::*;

/// **ASCII Printable Character Start Index**
/// 
/// Defines the first printable ASCII character (space, ASCII 32)
/// in the font lookup table. Characters before this index are
/// non-printable control characters.
pub const PRINTABLE_START: usize = 32;

/// **Number of Printable Characters**
/// 
/// Total count of printable ASCII characters (95) supported by the font,
/// ranging from space (32) to tilde (126). This covers all standard
/// keyboard characters including letters, numbers, punctuation, and symbols.
pub const PRINTABLE_COUNT: usize = 95;

/// **Pendolino3 Font Data**
///
/// Complete 5x5 bitmap font containing all 95 printable ASCII characters.
/// Each character is represented as an array of 5 bytes, where each byte
/// represents one row of the 5x5 character bitmap.
///
/// **Font Characteristics:**
/// - Monospace: Each character fits in a 5x5 grid
/// - Readable: Optimized for LED matrix display
/// - Complete: Covers ASCII 32-126 (space through tilde)
/// - Compact: Minimal storage requirements
///
/// **Data Format:**
/// ```text
/// Character 'A' example:
/// [0x0c, 0x92, 0x5e, 0xd2, 0x52] represents:
/// 00110  (0x0c >> 3)
/// 10010  (0x92 >> 3) 
/// 11110  (0x5e >> 3)
/// 10010  (0xd2 >> 3)
/// 10010  (0x52 >> 3)
/// ```
///
/// **Source:** Based on lancaster-university/microbit-dal MicroBitFont.cpp v2.1.1
// From lancaster-university/microbit-dal source/core/MicroBitFont.cpp
// as of v2.1.1
pub const PENDOLINO3: [[u8; 5]; PRINTABLE_COUNT] = [
    [0x0, 0x0, 0x0, 0x0, 0x0],
    [0x8, 0x8, 0x8, 0x0, 0x8],
    [0xa, 0x4a, 0x40, 0x0, 0x0],
    [0xa, 0x5f, 0xea, 0x5f, 0xea],
    [0xe, 0xd9, 0x2e, 0xd3, 0x6e],
    [0x19, 0x32, 0x44, 0x89, 0x33],
    [0xc, 0x92, 0x4c, 0x92, 0x4d],
    [0x8, 0x8, 0x0, 0x0, 0x0],
    [0x4, 0x88, 0x8, 0x8, 0x4],
    [0x8, 0x4, 0x84, 0x84, 0x88],
    [0x0, 0xa, 0x44, 0x8a, 0x40],
    [0x0, 0x4, 0x8e, 0xc4, 0x80],
    [0x0, 0x0, 0x0, 0x4, 0x88],
    [0x0, 0x0, 0xe, 0xc0, 0x0],
    [0x0, 0x0, 0x0, 0x8, 0x0],
    [0x1, 0x22, 0x44, 0x88, 0x10],
    [0xc, 0x92, 0x52, 0x52, 0x4c],
    [0x4, 0x8c, 0x84, 0x84, 0x8e],
    [0x1c, 0x82, 0x4c, 0x90, 0x1e],
    [0x1e, 0xc2, 0x44, 0x92, 0x4c],
    [0x6, 0xca, 0x52, 0x5f, 0xe2],
    [0x1f, 0xf0, 0x1e, 0xc1, 0x3e],
    [0x2, 0x44, 0x8e, 0xd1, 0x2e],
    [0x1f, 0xe2, 0x44, 0x88, 0x10],
    [0xe, 0xd1, 0x2e, 0xd1, 0x2e],
    [0xe, 0xd1, 0x2e, 0xc4, 0x88],
    [0x0, 0x8, 0x0, 0x8, 0x0],
    [0x0, 0x4, 0x80, 0x4, 0x88],
    [0x2, 0x44, 0x88, 0x4, 0x82],
    [0x0, 0xe, 0xc0, 0xe, 0xc0],
    [0x8, 0x4, 0x82, 0x44, 0x88],
    [0xe, 0xd1, 0x26, 0xc0, 0x4],
    [0xe, 0xd1, 0x35, 0xb3, 0x6c],
    [0xc, 0x92, 0x5e, 0xd2, 0x52],
    [0x1c, 0x92, 0x5c, 0x92, 0x5c],
    [0xe, 0xd0, 0x10, 0x10, 0xe],
    [0x1c, 0x92, 0x52, 0x52, 0x5c],
    [0x1e, 0xd0, 0x1c, 0x90, 0x1e],
    [0x1e, 0xd0, 0x1c, 0x90, 0x10],
    [0xe, 0xd0, 0x13, 0x71, 0x2e],
    [0x12, 0x52, 0x5e, 0xd2, 0x52],
    [0x1c, 0x88, 0x8, 0x8, 0x1c],
    [0x1f, 0xe2, 0x42, 0x52, 0x4c],
    [0x12, 0x54, 0x98, 0x14, 0x92],
    [0x10, 0x10, 0x10, 0x10, 0x1e],
    [0x11, 0x3b, 0x75, 0xb1, 0x31],
    [0x11, 0x39, 0x35, 0xb3, 0x71],
    [0xc, 0x92, 0x52, 0x52, 0x4c],
    [0x1c, 0x92, 0x5c, 0x90, 0x10],
    [0xc, 0x92, 0x52, 0x4c, 0x86],
    [0x1c, 0x92, 0x5c, 0x92, 0x51],
    [0xe, 0xd0, 0xc, 0x82, 0x5c],
    [0x1f, 0xe4, 0x84, 0x84, 0x84],
    [0x12, 0x52, 0x52, 0x52, 0x4c],
    [0x11, 0x31, 0x31, 0x2a, 0x44],
    [0x11, 0x31, 0x35, 0xbb, 0x71],
    [0x12, 0x52, 0x4c, 0x92, 0x52],
    [0x11, 0x2a, 0x44, 0x84, 0x84],
    [0x1e, 0xc4, 0x88, 0x10, 0x1e],
    [0xe, 0xc8, 0x8, 0x8, 0xe],
    [0x10, 0x8, 0x4, 0x82, 0x41],
    [0xe, 0xc2, 0x42, 0x42, 0x4e],
    [0x4, 0x8a, 0x40, 0x0, 0x0],
    [0x0, 0x0, 0x0, 0x0, 0x1f],
    [0x8, 0x4, 0x80, 0x0, 0x0],
    [0x0, 0xe, 0xd2, 0x52, 0x4f],
    [0x10, 0x10, 0x1c, 0x92, 0x5c],
    [0x0, 0xe, 0xd0, 0x10, 0xe],
    [0x2, 0x42, 0x4e, 0xd2, 0x4e],
    [0xc, 0x92, 0x5c, 0x90, 0xe],
    [0x6, 0xc8, 0x1c, 0x88, 0x8],
    [0xe, 0xd2, 0x4e, 0xc2, 0x4c],
    [0x10, 0x10, 0x1c, 0x92, 0x52],
    [0x8, 0x0, 0x8, 0x8, 0x8],
    [0x2, 0x40, 0x2, 0x42, 0x4c],
    [0x10, 0x14, 0x98, 0x14, 0x92],
    [0x8, 0x8, 0x8, 0x8, 0x6],
    [0x0, 0x1b, 0x75, 0xb1, 0x31],
    [0x0, 0x1c, 0x92, 0x52, 0x52],
    [0x0, 0xc, 0x92, 0x52, 0x4c],
    [0x0, 0x1c, 0x92, 0x5c, 0x90],
    [0x0, 0xe, 0xd2, 0x4e, 0xc2],
    [0x0, 0xe, 0xd0, 0x10, 0x10],
    [0x0, 0x6, 0xc8, 0x4, 0x98],
    [0x8, 0x8, 0xe, 0xc8, 0x7],
    [0x0, 0x12, 0x52, 0x52, 0x4f],
    [0x0, 0x11, 0x31, 0x2a, 0x44],
    [0x0, 0x11, 0x31, 0x35, 0xbb],
    [0x0, 0x12, 0x4c, 0x8c, 0x92],
    [0x0, 0x11, 0x2a, 0x44, 0x98],
    [0x0, 0x1e, 0xc4, 0x88, 0x1e],
    [0x6, 0xc4, 0x8c, 0x84, 0x86],
    [0x8, 0x8, 0x8, 0x8, 0x8],
    [0x18, 0x8, 0xc, 0x88, 0x18],
    [0x0, 0x0, 0xc, 0x83, 0x60],
];

#[rustfmt::skip]
/// **Check Mark Bitmap ✓**
///
/// A 5x5 bitmap representing a check mark or tick symbol.
/// Useful for indicating success, completion, or positive status.
///
/// **Pattern:**
/// ```text
/// ·····
/// ····▪
/// ···▪·
/// ▪·▪··
/// ·▪···
/// ```
///
/// # Usage
/// ```ignore
/// display.display(fonts::CHECK_MARK, Duration::from_secs(1)).await;
/// ```
#[allow(dead_code)]
pub const CHECK_MARK: Frame<5, 5> = frame_5x5(&[
    0b00000,
    0b00001,
    0b00010,
    0b10100,
    0b01000,
]);

#[rustfmt::skip]
/// **Cross Mark Bitmap ✗**
///
/// A 5x5 bitmap representing a cross or X symbol.
/// Useful for indicating errors, cancellation, or negative status.
///
/// **Pattern:**
/// ```text
/// ·····
/// ·▪·▪·
/// ··▪··
/// ·▪·▪·
/// ·····
/// ```
///
/// # Usage
/// ```ignore
/// display.display(fonts::CROSS_MARK, Duration::from_secs(1)).await;
/// ```
#[allow(dead_code)]
pub const CROSS_MARK: Frame<5, 5> = frame_5x5(&[
    0b00000,
    0b01010,
    0b00100,
    0b01010,
    0b00000,
]);

#[rustfmt::skip]
/// **Left Arrow Bitmap ←**
///
/// A 5x5 bitmap representing a left-pointing arrow.
/// Commonly used for navigation, button A indication, or directional guidance.
///
/// **Pattern:**
/// ```text
/// ··▪··
/// ·▪···
/// ▪▪▪▪▪
/// ·▪···
/// ··▪··
/// ```
///
/// # Usage
/// ```ignore
/// display.display(fonts::ARROW_LEFT, Duration::from_secs(1)).await;
/// ```
pub const ARROW_LEFT: Frame<5, 5> = frame_5x5(&[
    0b00100,
    0b01000,
    0b11111,
    0b01000,
    0b00100,
]);

#[rustfmt::skip]
/// **Right Arrow Bitmap →**
///
/// A 5x5 bitmap representing a right-pointing arrow.
/// Commonly used for navigation, button B indication, or directional guidance.
///
/// **Pattern:**
/// ```text
/// ··▪··
/// ···▪·
/// ▪▪▪▪▪
/// ···▪·
/// ··▪··
/// ```
///
/// # Usage
/// ```ignore
/// display.display(fonts::ARROW_RIGHT, Duration::from_secs(1)).await;
/// ```
pub const ARROW_RIGHT: Frame<5, 5> = frame_5x5(&[
    0b00100,
    0b00010,
    0b11111,
    0b00010,
    0b00100,
]);

/// **Create 5x5 Frame from Byte Array**
///
/// Constructs a Frame from a 5-element byte array where each byte
/// represents one row of the 5x5 LED matrix pattern.
///
/// # Arguments
/// * `input` - Array of 5 bytes, each representing a row bitmap
///
/// # Returns
/// A `Frame<XSIZE, YSIZE>` ready for display on the LED matrix
///
/// # Bit Encoding
/// Each byte encodes 5 pixels (bits 7-3), with bits 2-0 unused:
/// ```text
/// Byte: 0b11111000
///          ^^^^^--- Used bits (left to right pixels)
///               ^^^-- Unused padding bits
/// ```
///
/// # Example
/// ```ignore
/// // Create a heart pattern
/// let heart = frame_5x5(&[
///     0b01010,  // ·▪·▪·
///     0b11111,  // ▪▪▪▪▪
///     0b11111,  // ▪▪▪▪▪
///     0b01110,  // ·▪▪▪·
///     0b00100,  // ··▪··
/// ]);
/// display.display(heart, Duration::from_secs(2)).await;
/// ```
pub const fn frame_5x5<const XSIZE: usize, const YSIZE: usize>(input: &[u8; 5]) -> Frame<XSIZE, YSIZE> {
    let mut data = [Bitmap::empty(5); YSIZE];
    data[0] = Bitmap::new(input[0], 5);
    data[1] = Bitmap::new(input[1], 5);
    data[2] = Bitmap::new(input[2], 5);
    data[3] = Bitmap::new(input[3], 5);
    data[4] = Bitmap::new(input[4], 5);
    Frame::new(data)
}

/// **Convert u8 to Frame**
///
/// Implements conversion from byte values to display frames.
/// The byte is first cast to char, then converted using the font lookup.
///
/// # Note
/// This is primarily used for ASCII character codes (0-255).
/// Non-printable characters display as blank frames.
impl<const XSIZE: usize, const YSIZE: usize> Into<Frame<XSIZE, YSIZE>> for u8 {
    fn into(self) -> Frame<XSIZE, YSIZE> {
        (self as char).into()
    }
}

/// **Convert char to Frame**
///
/// Converts ASCII characters to 5x5 bitmap frames using the Pendolino3 font.
/// This enables direct character-to-display conversion for text rendering.
///
/// # Supported Characters
/// - **Printable ASCII**: 32-126 (space through tilde)
/// - **Unsupported**: Characters outside this range display as blank
///
/// # Implementation
/// 1. Checks if character is in printable ASCII range
/// 2. Looks up bitmap data in PENDOLINO3 font array
/// 3. Converts to Frame using frame_5x5 helper
/// 4. Returns blank frame for unsupported characters
///
/// # Example
/// ```ignore
/// let a_frame: Frame<5, 5> = 'A'.into();
/// let space_frame: Frame<5, 5> = ' '.into();
/// let digit_frame: Frame<5, 5> = '7'.into();
/// ```
impl<const XSIZE: usize, const YSIZE: usize> Into<Frame<XSIZE, YSIZE>> for char {
    fn into(self) -> Frame<XSIZE, YSIZE> {
        assert!(XSIZE == 5);
        assert!(YSIZE == 5);

        let n = self as usize;
        if n > PRINTABLE_START && n < PRINTABLE_START + PRINTABLE_COUNT {
            frame_5x5(&PENDOLINO3[n - PRINTABLE_START])
        } else {
            frame_5x5(&[0, 0, 0, 0, 0])
        }
    }
}
