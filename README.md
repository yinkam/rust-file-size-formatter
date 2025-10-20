# Rust File Size Formatter
#### This repository is part of [Pragmatic AI Labs Rust Bootcamp](https://ds500.paiml.com/bootcamps/rust).

A command-line tool for converting file sizes between different units (bytes, kilobytes, megabytes, gigabytes).

## Overview

This project was developed as part of a comprehensive 6-week journey learning Rust fundamentals, from basic concepts to building professional command-line applications. The file size formatter takes a size value with a unit and displays the equivalent values in all supported units.

## Features

- ✅ File size conversion between bytes, KB, MB, and GB
- ✅ Command-line interface with simple argument parsing
- ✅ Support for multiple input units (B, KB, MB, GB)
- ✅ Formatted output showing all unit conversions
- ✅ Rust enum-based architecture for type safety

## Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/yinkam/rust-file-size-formatter.git
   cd rust-file-size-formatter
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

## Usage

The application takes a file size with a unit as a single argument. The format is `"<number> <unit>"` where the unit can be:
- `b` or `B` for bytes
- `kb` or `KB` for kilobytes  
- `mb` or `MB` for megabytes
- `gb` or `GB` for gigabytes

### Examples

```bash
# Convert 1024 bytes to all units
cargo run "1024 b"
# Output: Sizes { bytes: "1024 bytes", kilobytes: "1 kilobytes", megabytes: "0 megabytes", gigabytes: "0 gigabytes" }

# Convert 5 megabytes to all units
cargo run "5 mb"
# Output: Sizes { bytes: "5242880 bytes", kilobytes: "5120 kilobytes", megabytes: "5 megabytes", gigabytes: "0 gigabytes" }

# Convert 2 gigabytes to all units
cargo run "2 gb"
# Output: Sizes { bytes: "2147483648 bytes", kilobytes: "2097152 kilobytes", megabytes: "2048 megabytes", gigabytes: "2 gigabytes" }
```

### Using the compiled binary

```bash
# Build in release mode for better performance
cargo build --release

# Run the binary directly
./target/release/rust-file-size-formatter "1024 kb"
```

## Architecture

The application uses a clean, type-safe architecture with the following key components:

- **`FileSize` enum**: Represents different file size units (Bytes, Kilobytes, Megabytes, Gigabytes)
- **`Sizes` struct**: Contains formatted string representations of a file size in all supported units
- **`unit_scale()` function**: Converts unit strings to their byte multipliers (case-insensitive)
- **`format_size()` function**: Formats FileSize enum variants into human-readable strings
- **Command-line parsing**: Simple argument parsing that expects size and unit as a single space-separated argument

### Design Decisions

- Uses Rust enums for type safety and pattern matching
- Implements the `Debug` trait for easy output formatting
- Case-insensitive unit parsing for better user experience
- Conversion calculations use standard binary prefixes (1 KB = 1024 bytes)

## Testing

Currently, the project focuses on demonstrating core Rust concepts. Future improvements could include:
- Unit tests for conversion functions
- Integration tests for command-line interface
- Error handling for invalid inputs
- Property-based testing for conversion accuracy

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines on how to contribute to this project.

## Future Enhancements

Potential improvements for this project:
- Add error handling for invalid inputs
- Support for decimal inputs (e.g., "1.5 GB")
- Additional units (TB, PB, etc.)
- JSON output format option
- Batch processing of multiple file sizes
- Interactive mode

## License

MIT License - feel free to use this project for learning and development purposes.

---

*Built during the [Pragmatic AI Labs Rust Bootcamp](https://github.com/paiml/ds500-rust-bootcamp)*
