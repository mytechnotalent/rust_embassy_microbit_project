#!/bin/bash

# BBC micro:bit Binary Data Extraction Script
# This script uses nm and objcopy to extract various data formats from the compiled binary
# Useful for firmware analysis, symbol inspection, and binary data generation

set -e # Exit on any error

# Configuration
PROJECT_NAME="microbit-async-display-example"
BUILD_TYPE="debug"
TARGET_DIR="examples/display/target/thumbv7em-none-eabihf"
BINARY_PATH="${TARGET_DIR}/${BUILD_TYPE}/${PROJECT_NAME}"
OUTPUT_DIR="data"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Print colored output
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if required tools are available
check_tools() {
    print_status "Checking required tools..."

    local missing_tools=()

    if ! command -v arm-none-eabi-nm &>/dev/null; then
        missing_tools+=("arm-none-eabi-nm")
    fi

    if ! command -v arm-none-eabi-objcopy &>/dev/null; then
        missing_tools+=("arm-none-eabi-objcopy")
    fi

    if ! command -v arm-none-eabi-objdump &>/dev/null; then
        missing_tools+=("arm-none-eabi-objdump")
    fi

    if [ ${#missing_tools[@]} -ne 0 ]; then
        print_error "Missing required tools: ${missing_tools[*]}"
        print_error "Please install ARM embedded toolchain:"
        print_error "  macOS: brew install --cask gcc-arm-embedded"
        print_error "  Ubuntu: sudo apt-get install gcc-arm-none-eabi"
        exit 1
    fi

    print_success "All required tools found"
}

# Build the project if binary doesn't exist
build_project() {
    if [ ! -f "$BINARY_PATH" ]; then
        print_status "Binary not found, building project..."
        cd examples/display
        cargo build
        cd ../..

        if [ ! -f "$BINARY_PATH" ]; then
            print_error "Build failed or binary not found at: $BINARY_PATH"
            exit 1
        fi
        print_success "Project built successfully"
    else
        print_status "Using existing binary: $BINARY_PATH"
    fi
}

# Create output directory
setup_output() {
    if [ -d "$OUTPUT_DIR" ]; then
        print_warning "Output directory exists, cleaning..."
        rm -rf "$OUTPUT_DIR"
    fi

    mkdir -p "$OUTPUT_DIR"
    print_success "Output directory created: $OUTPUT_DIR"
}

# Extract symbol table using nm
extract_symbols() {
    print_status "Extracting symbol table..."

    # All symbols
    arm-none-eabi-nm -n "$BINARY_PATH" >"$OUTPUT_DIR/symbols_all.txt"

    # Only defined symbols
    arm-none-eabi-nm -n --defined-only "$BINARY_PATH" >"$OUTPUT_DIR/symbols_defined.txt"

    # Only external symbols
    arm-none-eabi-nm -g "$BINARY_PATH" >"$OUTPUT_DIR/symbols_global.txt"

    # Symbols with size
    arm-none-eabi-nm -S --size-sort "$BINARY_PATH" >"$OUTPUT_DIR/symbols_by_size.txt"

    # Function symbols only
    arm-none-eabi-nm "$BINARY_PATH" | grep -E " [Tt] " >"$OUTPUT_DIR/symbols_functions.txt" || true

    # Data symbols only
    arm-none-eabi-nm "$BINARY_PATH" | grep -E " [DdBbRr] " >"$OUTPUT_DIR/symbols_data.txt" || true

    print_success "Symbol tables extracted"
}

# Extract binary sections using objcopy
extract_sections() {
    print_status "Extracting binary sections..."

    # Extract .text section (code)
    arm-none-eabi-objcopy -O binary -j .text "$BINARY_PATH" "$OUTPUT_DIR/section_text.bin"

    # Extract .rodata section (read-only data)
    arm-none-eabi-objcopy -O binary -j .rodata "$BINARY_PATH" "$OUTPUT_DIR/section_rodata.bin" || true

    # Extract .data section (initialized data)
    arm-none-eabi-objcopy -O binary -j .data "$BINARY_PATH" "$OUTPUT_DIR/section_data.bin" || true

    # Extract entire binary as raw data
    arm-none-eabi-objcopy -O binary "$BINARY_PATH" "$OUTPUT_DIR/firmware_raw.bin"

    # Create Intel HEX format
    arm-none-eabi-objcopy -O ihex "$BINARY_PATH" "$OUTPUT_DIR/firmware.hex"

    # Create Motorola S-record format
    arm-none-eabi-objcopy -O srec "$BINARY_PATH" "$OUTPUT_DIR/firmware.srec"

    print_success "Binary sections extracted"
}

# Generate disassembly
generate_disassembly() {
    print_status "Generating disassembly..."

    # Full disassembly
    arm-none-eabi-objdump -d "$BINARY_PATH" >"$OUTPUT_DIR/disassembly_full.txt"

    # Disassembly with source (if available)
    arm-none-eabi-objdump -S "$BINARY_PATH" >"$OUTPUT_DIR/disassembly_with_source.txt" || true

    # Headers and section info
    arm-none-eabi-objdump -h "$BINARY_PATH" >"$OUTPUT_DIR/section_headers.txt"

    # All headers
    arm-none-eabi-objdump -x "$BINARY_PATH" >"$OUTPUT_DIR/all_headers.txt"

    print_success "Disassembly generated"
}

# Generate analysis report
generate_report() {
    print_status "Generating analysis report..."

    local report_file="$OUTPUT_DIR/analysis_report.txt"

    {
        echo "BBC micro:bit Binary Analysis Report"
        echo "Generated on: $(date)"
        echo "Binary: $BINARY_PATH"
        echo "Build Type: $BUILD_TYPE"
        echo "=========================================="
        echo

        echo "BINARY SIZE INFORMATION:"
        ls -lh "$BINARY_PATH" | awk '{print "  Total Size: " $5}'
        echo

        echo "SECTION SIZES:"
        if [ -f "$OUTPUT_DIR/section_text.bin" ]; then
            echo "  .text (code): $(ls -lh "$OUTPUT_DIR/section_text.bin" | awk '{print $5}')"
        fi
        if [ -f "$OUTPUT_DIR/section_rodata.bin" ]; then
            echo "  .rodata (read-only): $(ls -lh "$OUTPUT_DIR/section_rodata.bin" | awk '{print $5}')"
        fi
        if [ -f "$OUTPUT_DIR/section_data.bin" ]; then
            echo "  .data (initialized): $(ls -lh "$OUTPUT_DIR/section_data.bin" | awk '{print $5}')"
        fi
        echo

        echo "SYMBOL STATISTICS:"
        if [ -f "$OUTPUT_DIR/symbols_all.txt" ]; then
            echo "  Total symbols: $(wc -l <"$OUTPUT_DIR/symbols_all.txt")"
        fi
        if [ -f "$OUTPUT_DIR/symbols_functions.txt" ]; then
            echo "  Function symbols: $(wc -l <"$OUTPUT_DIR/symbols_functions.txt")"
        fi
        if [ -f "$OUTPUT_DIR/symbols_data.txt" ]; then
            echo "  Data symbols: $(wc -l <"$OUTPUT_DIR/symbols_data.txt")"
        fi
        echo

        echo "MEMORY LAYOUT:"
        echo "  (See section_headers.txt for detailed memory layout)"
        echo

        echo "FILES GENERATED:"
        find "$OUTPUT_DIR" -type f -exec basename {} \; | sort | sed 's/^/  /'

    } >"$report_file"

    print_success "Analysis report generated: $report_file"
}

# Main execution
main() {
    echo "BBC micro:bit Binary Data Extraction Tool"
    echo "========================================="
    echo

    check_tools
    build_project
    setup_output
    extract_symbols
    extract_sections
    generate_disassembly
    generate_report

    echo
    print_success "Data extraction completed successfully!"
    print_status "Output directory: $OUTPUT_DIR"
    print_status "Check analysis_report.txt for summary"

    # Show directory contents
    echo
    print_status "Generated files:"
    ls -la "$OUTPUT_DIR" | tail -n +2 | awk '{printf "  %-20s %s\n", $9, $5}'
}

# Run main function
main "$@"
