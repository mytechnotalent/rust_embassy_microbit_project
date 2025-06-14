![image](https://github.com/mytechnotalent/rust_embassy_microbit_project/blob/main/rust_embassy_microbit_project.jpg?raw=true)

# Rust Embassy microbit Project

A simple embedded Rust project running on the microbit v2, built with Embassy async framework and no_std runtime.

<br>

## FREE Reverse Engineering Self-Study Course [HERE](https://github.com/mytechnotalent/Reverse-Engineering-Tutorial)

<br>

## Features
- **5x5 LED Matrix Display**: Async display driver with custom fonts and animations
- **Dual Button Support**: Button A and Button B with debouncing
- **Motion Sensing**: LSM303AGR accelerometer and magnetometer support
- **Audio**: Speaker and microphone interfaces
- **Bluetooth Low Energy**: Optional BLE support with trouble stack
- **GPIO Access**: All edge connector pins available (P0-P20)
- **Communication**: UART, I2C, SPI peripheral access
- **Timers & PWM**: Multiple timer and PWM channel support
- **Async/Await**: Built on Embassy's cooperative scheduler
- **No Heap**: Runs entirely in static memory with deterministic behavior

## Project Structure
- `src/display/`: 5x5 LED matrix driver with fonts and bitmap support
- `examples/`: Various example applications demonstrating features

## Examples

### Display Example
Demonstrates LED matrix control with button interactions:
```bash
cd examples/display
cargo run --release
```

## How It Works (Step-by-Step)

1. **Startup**
   - The nRF52833 boot ROM loads your program from flash memory
   - The Cortex-M `cortex-m-rt` runtime (`#[no_main]`) bypasses traditional `main()` 
   - The reset vector jumps to `__cortex_m_rt_main_trampoline`
   - Embassy executor is initialized and starts the async runtime

2. **Board Initialization**
   - `embassy_nrf::init()` configures clocks, GPIO, and peripherals
   - The `Microbit::default()` creates instances of all peripherals:
     - 5x5 LED matrix (rows: P0_21, P0_22, P0_15, P0_24, P0_19)
     - Button A (P0_14) and Button B (P0_23)
     - Speaker (P0_00), Microphone (P0_05)
     - All edge connector pins (P0-P20)
     - Internal I2C for accelerometer/magnetometer

3. **Executor Task Management**
   - Embassy's executor uses a lock-free task queue for cooperative scheduling
   - Tasks are enqueued when spawned or when wakers are triggered
   - The executor polls tasks in FIFO order
   - When all tasks are pending, CPU enters WFI (Wait-For-Interrupt) for power efficiency

4. **Peripheral Abstractions**
   - **LED Matrix**: Time-multiplexed 5x5 display with async frame timing
   - **Buttons**: GPIO inputs with internal pull-ups, async edge detection
   - **Motion Sensors**: I2C communication with LSM303AGR via async interface
   - **Audio**: PWM-based speaker control and ADC microphone sampling
   - **BLE**: Optional Bluetooth stack integration with async event handling

5. **Async Event Handling**
   - GPIO interrupts trigger task wakers for button presses
   - Timer interrupts handle display refresh and delays
   - I2C/SPI interrupts manage sensor communication
   - All operations are non-blocking, allowing concurrent execution

6. **Memory Management**
   - Static allocation only - no heap fragmentation
   - Compile-time memory layout with predictable behavior
   - Embassy's static task allocation ensures deterministic performance

---

## Embassy Executor Deep Dive

### Task Scheduling Architecture
- **Enqueue Operation**: Tasks are added to the tail of a bounded queue when spawned or awakened
- **Dequeue Operation**: Executor pops tasks from the head (FIFO) for polling
- **Cooperative Scheduling**: Tasks must yield (await) to allow others to run
- **Waker System**: Peripheral interrupts trigger task re-scheduling via wakers

### Memory Layout
```
Flash Memory:
├── Vector Table (0x00000000)
├── Program Code 
├── Static Data
└── Embassy Runtime

RAM Memory:
├── Task Queue (statically allocated)
├── Task Control Blocks
├── Stack Space
└── Peripheral Buffers
```

### Interrupt Integration
- **GPIOTE**: Button press/release detection
- **RTC**: Timer-based delays and scheduling
- **TWI**: I2C sensor communication
- **PWM**: Audio output generation
- **RADIO**: Bluetooth communication (optional)

---

## Peripheral Pin Mapping

### Internal Connections
| Function         | Pin                               | Description                |
| ---------------- | --------------------------------- | -------------------------- |
| LED Matrix Rows  | P0_21, P0_22, P0_15, P0_24, P0_19 | Row drivers                |
| LED Matrix Cols  | P0_28, P0_11, P0_31, P1_05, P0_30 | Column drivers             |
| Button A         | P0_14                             | Pull-up enabled            |
| Button B         | P0_23                             | Pull-up enabled            |
| Speaker          | P0_00                             | PWM audio output           |
| Microphone       | P0_05                             | ADC input                  |
| Mic Enable       | P0_20                             | Microphone power           |
| Internal I2C SCL | P0_08                             | Accelerometer/Magnetometer |
| Internal I2C SDA | P0_16                             | Accelerometer/Magnetometer |

### Edge Connector
| Connector | Pin   | GPIO  | Description         |
| --------- | ----- | ----- | ------------------- |
| P0        | Large | P0_02 | General purpose I/O |
| P1        | Large | P0_03 | General purpose I/O |
| P2        | Large | P0_04 | General purpose I/O |
| P8        | Small | P0_10 | General purpose I/O |
| P9        | Small | P0_09 | General purpose I/O |
| P12       | Small | P0_12 | General purpose I/O |
| P13       | Small | P0_17 | General purpose I/O |
| P14       | Small | P0_01 | General purpose I/O |
| P15       | Small | P0_13 | General purpose I/O |
| P16       | Small | P1_02 | General purpose I/O |
| P19       | Small | P0_26 | General purpose I/O |
| P20       | Small | P1_00 | General purpose I/O |

---

## Building and Flashing

### Prerequisites
**Software:**
- Rust toolchain with `thumbv7em-none-eabihf` target
- [`probe-rs`](https://probe.rs/) for flashing and debugging
- [`rustup`](https://rustup.rs/) for Rust installation

**Hardware:**
- [BBC micro:bit v2](https://microbit.org/)
- USB cable for power and programming
- Optional: External debugger probe for advanced debugging

### Installation
```bash
# Install Rust if not already installed
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add the target for micro:bit
rustup target add thumbv7em-none-eabihf

# Install probe-rs
cargo install probe-rs-tools --locked
```

### Build Commands
```bash
# Build the library
cargo build

# Build with optimizations
cargo build --release

# Build specific example
cd examples/display
cargo build --release
```

### Flash Commands
```bash
# Flash example to micro:bit
cd examples/display
cargo run --release

# Flash with probe-rs directly
probe-rs run --chip nRF52833_xxAA target/thumbv7em-none-eabihf/release/display-example

# Flash and attach debugger
probe-rs run --chip nRF52833_xxAA --attach-under-reset target/thumbv7em-none-eabihf/release/display-example
```

### Debugging
```bash
# Start GDB session
probe-rs gdb --chip nRF52833_xxAA target/thumbv7em-none-eabihf/release/display-example

# View defmt logs
probe-rs run --chip nRF52833_xxAA target/thumbv7em-none-eabihf/release/display-example
```

---

## Feature Flags

| Feature   | Description                            | Dependencies                   |
| --------- | -------------------------------------- | ------------------------------ |
| `default` | Basic functionality with defmt logging | `defmt`                        |
| `defmt`   | Logging and debugging support          | defmt crates                   |
| `trouble` | Bluetooth Low Energy support           | nrf-sdc, nrf-mpsl, static_cell |

Enable features in Cargo.toml:
```toml
[dependencies]
microbit-bsp = { version = "0.4.0", features = ["trouble"] }
```

---

## API Examples

### Basic LED Matrix Control
```rust
use microbit_bsp::*;
use embassy_time::Duration;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let board = Microbit::default();
    let mut display = board.display;
    
    // Show a heart pattern
    let heart = display::bitmap![
        [0, 1, 0, 1, 0]
        [1, 1, 1, 1, 1]
        [1, 1, 1, 1, 1]
        [0, 1, 1, 1, 0]
        [0, 0, 1, 0, 0]
    ];
    
    display.display(heart, Duration::from_secs(2)).await;
}
```

### Button Handling
```rust
use embassy_futures::select::{select, Either};

loop {
    match select(board.btn_a.wait_for_low(), board.btn_b.wait_for_low()).await {
        Either::First(_) => {
            // Button A pressed
            display.display(display::fonts::ARROW_LEFT, Duration::from_millis(500)).await;
        }
        Either::Second(_) => {
            // Button B pressed  
            display.display(display::fonts::ARROW_RIGHT, Duration::from_millis(500)).await;
        }
    }
}
```

---

## Requirements
- **Rust**: Nightly channel (for async embedded features)
- **Target**: thumbv7em-none-eabihf (Cortex-M4F with hardware FPU)
- **Hardware**: BBC micro:bit v2 with nRF52833 SoC
- **Memory**: 512KB Flash, 128KB RAM
- **Clock**: 64MHz ARM Cortex-M4F with FPU

---

## License
Apache-2.0 License

---

## References
- [Embassy Framework](https://embassy.dev/) - Embedded async executor
- [BBC micro:bit v2 Datasheet](https://tech.microbit.org/hardware/schematic/) - Hardware specifications
- [nRF52833 Product Specification](https://www.nordicsemi.com/products/nrf52833) - Nordic SoC documentation  
- [Rust Embedded Book](https://doc.rust-lang.org/embedded-book/) - Embedded Rust programming guide
- [probe-rs Documentation](https://probe.rs/) - Debugging and flashing tool
