# QRx - QRIS Spoofing Tool

QRx is a tool developed in Rust designed to spoof QRIS (Quick Response Indonesian Standard) by replicating information from a target QRIS and applying it to a victim QRIS. The goal of this project is purely educational, demonstrating the power of Rust in handling QR code manipulation and data spoofing.

> **Disclaimer**: This project is for educational purposes only. It is strictly prohibited to use this tool for malicious activities, fraud, or any actions that might cause harm or break the law. The creator is not responsible for any damage or legal consequences arising from the misuse of this project.

## Features

- Spoofs QRIS data by copying information from one QRIS to another.
- Written in Rust for performance and efficiency.
- Works with QRIS QR codes used in payment systems.

## Installation

To use **QRx**, you need to have Rust installed on your machine. If you don't have Rust installed, follow the instructions on the official [Rust website](https://www.rust-lang.org/learn/get-started).

1. Clone the repository:

```bash
git clone https://github.com/krypton-byte/qrx.git
cd qrx
```

2. Build the project:

```bash
cargo build --release
```

3. After building, the executable will be available in the `target/release` directory.

## Usage

To use QRx to spoof a QRIS, use the following command:

```bash
qrx --source <SOURCE> --target <TARGET> --output <OUTPUT>
```

Where:
- `<SOURCE>` is the path to the QRIS file you want to copy data from.
- `<TARGET>` is the path to the QRIS file you want to overwrite with the source data.
- `<OUTPUT>` is the path where the new QRIS (with spoofed data) will be saved.

### Example:

```bash
qrx --source target_qris.png --target victim_qris.png --output spoofed_qris.png
```

This will:
- Copy the information from `target_qris.png`
- Overwrite `victim_qris.png` with the target's information
- Save the new QRIS image as `spoofed_qris.png`

## Options:
- `-s, --source <SOURCE>`: Path to the QRIS file to copy data from.
- `-t, --target <TARGET>`: Path to the QRIS file to overwrite.
- `-o, --output <OUTPUT>`: Path to save the spoofed QRIS.
- `-h, --help`: Print help message.
- `-V, --version`: Print version information.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Disclaimer

This tool is intended solely for educational purposes. The creator disclaims any liability for misuse of this tool. Using this tool to defraud, deceive, or harm others is illegal and unethical. Always use responsibly and respect privacy and legality.
