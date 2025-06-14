# Reverse Engineering Analysis of micro:bit Binary

## Overview
This is a Rust embedded application for the micro:bit (nRF52833) using the Embassy async framework for display control and button handling.

## Memory Layout
- **Flash (Code)**: 0x00000000 - 0x000FFFFF (1MB)
- **SRAM**: 0x20000000 - 0x2001FFFF (128KB) 
- **Stack**: 0x20008624 - 0x20020000 (~96KB)

## Key Application Functions

### Main Application Logic
- `main` @ 0x00000ca0 - Entry point
- `__cortex_m_rt_main` @ 0x00000ca8 - Cortex-M runtime main
- `__embassy_main` @ 0x00000c7c - Embassy async main task
- `____embassy_main_task` @ 0x00000c62 - Main application task

### Button Handling
- `handle_button_a_press` @ 0x00000c2a - Button A press handler
- `handle_button_b_press` @ 0x00000c46 - Button B press handler  
- `show_button_press` @ 0x00000bec - Shared button press display logic

### Display System
The application uses a sophisticated LED matrix display system:

#### Core Display Types
- `LedMatrix<P,_,_>` - Main LED matrix controller
- `Frame<_,_>` - Individual display frames
- `Bitmap` - Bitmap graphics support
- `Animation<_,_>` - Animation sequences

#### Display Operations
- `LedMatrix::new` @ 0x00003fca - Initialize LED matrix
- `LedMatrix::display` @ 0x00001e8c - Show content on display
- `LedMatrix::scroll` @ 0x00001b96 - Scroll text/graphics
- `LedMatrix::animate` @ 0x00001c98 - Play animations
- `LedMatrix::clear` @ 0x00001982 - Clear display
- `LedMatrix::render` @ 0x000019f4 - Render frame to hardware

#### Font System
- Font rendering @ 0x00002728 - Convert chars to display frames
- Bitmap font data @ 0x0000244a - 5x5 character bitmaps

## Hardware Abstraction

### GPIO Control (Embassy-nRF)
- `embassy_nrf::gpio::*` - GPIO pin control
- `Flex` @ 0x00004518 - Flexible GPIO pin
- `Input` @ 0x00004532 - GPIO input pin
- `Output` @ 0x0000455a - GPIO output pin
- Pin configuration for P0_11, P0_14, P0_15, P0_19, P0_21, P0_22, P0_23, P0_24, P0_28, P0_30, P0_31, P1_05

### GPIOTE (GPIO Tasks and Events)
- `GPIOTE` interrupt handler @ 0x000059ea
- `PortInputFuture` @ 0x000006fa - Async GPIO input waiting
- `wait_for_low` @ 0x000051ee - Wait for button press (low signal)

### Timer System
- `embassy_time::Timer` - Async timer
- `embassy_time::Duration` - Time duration handling
- RTC driver @ 0x00006884 - Real-time clock for timing

## Async Runtime (Embassy)

### Task Management
- `TaskPool` @ 0x00000dc4 - Task pool for async tasks
- `Spawner` @ 0x00009980 - Task spawner
- `Executor` @ 0x0000984a - Async task executor

### Memory Management
- Task arena @ 0x20000220 - Memory pool for tasks
- Stack management with bounds checking

## Communication & Debugging

### DEFMT Logging System
- RTT (Real-Time Transfer) logging @ 0x20000008
- Log levels: TRACE, DEBUG, INFO, WARN, ERROR
- Formatted output with timestamps

### Panic Handling
- `panic_probe` integration
- Hardware fault handlers (HardFault, etc.)

## Key Data Structures

### Static Memory Allocations
- `_EMBASSY_DEVICE_PERIPHERALS` @ 0x2000021c - Device peripheral singleton
- `_SEGGER_RTT` @ 0x20000008 - RTT communication buffer
- `GPIOTE` channel wakers @ 0x2000005c - GPIO event handling
- Task arena @ 0x20000220 - Async task memory

## Application Flow Analysis

Based on the symbol table, the application flow appears to be:

1. **Initialization** (`main` → `__embassy_main`)
   - Initialize micro:bit hardware
   - Set up GPIO pins for buttons and LED matrix
   - Configure GPIOTE for async button handling
   - Initialize display system

2. **Main Loop** (`____embassy_main_task`)
   - Spawn button handler tasks
   - Wait for button press events
   - Handle display updates

3. **Button Handling**
   - Button A/B press detection via GPIOTE
   - Shared display logic in `show_button_press`
   - Async waiting using `wait_for_low`

4. **Display Updates**
   - Frame rendering to LED matrix
   - Animation and scrolling support
   - 5x5 bitmap font rendering

## Compiler Optimizations

The binary shows heavy optimization:
- Function inlining (many closures embedded)
- Dead code elimination
- Const propagation (many compile-time constants)
- LLVM intrinsics for low-level operations

## Security Notes

- No obvious encryption or obfuscation
- Standard Rust panic handling
- RTT debugging interface exposed
- Memory-safe Rust code with bounds checking

## Reverse Engineering Approach

To fully understand this binary:
1. Focus on the main application functions listed above
2. Trace data flow from button press to display update
3. Analyze the async task scheduling in Embassy
4. Map out the GPIO pin assignments for the LED matrix
5. Understand the display refresh timing and multiplexing

The symbol table provides excellent visibility into the application structure, making this a relatively straightforward binary to reverse engineer.
