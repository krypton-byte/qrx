# QRx - QRIS Spoofing Tool
<p align="center">
  <img src="https://upload.wikimedia.org/wikipedia/commons/e/e1/QRIS_logo.svg" alt="QRIS Logo" width="300">
</p>


QRx is a tool developed in Rust designed to spoof QRIS (Quick Response Indonesian Standard) by replicating information from a source QRIS and applying it to a target QRIS. The goal of this project is purely educational, demonstrating the power of Rust in handling QR code manipulation and data spoofing.

> **Disclaimer**: This project is for educational purposes only. It is strictly prohibited to use this tool for malicious activities, fraud, or any actions that might cause harm or break the law. The creator is not responsible for any damage or legal consequences arising from the misuse of this project.

## Features

- Spoofs QRIS data by copying information from one QRIS to another.
- Allows customization of the output QRIS size and raw QR code generation.
- Parses QRIS data efficiently using the [`qris-rs`](https://github.com/krypton-byte/qris-rs) library.
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
qrx [OPTIONS] --source <SOURCE> --target <TARGET> --output <OUTPUT>
```

### Options:
- `-s, --source <SOURCE>`: Path to the source QRIS file to copy data from.
- `-t, --target <TARGET>`: Path to the target QRIS file to overwrite with source data.
- `-o, --output <OUTPUT>`: Path to the directory where the output QRIS file will be saved.
- `--raw`: Use the target QRIS's frame to create a new QRIS image with the source's information. When enabled, generates a plain QR code with the source's information instead.
- `--size <SIZE>`: Set the canvas size for the QRIS image in pixels (default: 500).
- `-h, --help`: Print help message.
- `-V, --version`: Print version information.

### Example:

1. **Basic Usage:**
   ```bash
   qrx --source target_qris.png --target victim_qris.png --output ./spoofed_qris.png
   ```

   This will:
   - Copy information from `target_qris.png`.
   - Overwrite `victim_qris.png` with the target's information.
   - Save the new QRIS as `spoofed_qris.png`.

2. **Generate Raw QR Code:**
   ```bash
   qrx --source target_qris.png --output ./raw_qris.png --raw
   ```

   This will:
   - Use the information from `target_qris.png`.
   - Create a plain QR code without additional formatting.
   - Save the result as `raw_qris.png`.

3. **Specify QR Code Size:**
   ```bash
   qrx --source target_qris.png --target victim_qris.png --output ./spoofed_qris.png --raw --size 800
   ```

   This will:
   - Set the QR code's canvas size to 800x800 pixels.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Disclaimer

This tool is intended solely for educational purposes. The creator disclaims any liability for misuse of this tool. Using this tool to defraud, deceive, or harm others is illegal and unethical. Always use responsibly and respect privacy and legality.
