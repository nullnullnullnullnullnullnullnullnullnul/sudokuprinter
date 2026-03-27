# Sudoku Printer

Sudoku Printer is a desktop application that generates Sudoku puzzles at selectable difficulty levels and prints them directly to a FutureLogic Gen2 Universal thermal ticket printer over an RS232 serial connection.

## Architecture

The application is built with a modern, modular architecture:

*   **Core Algorithm (C++)**: Handles the generation of Sudoku puzzles, ensuring uniqueness and grading the difficulty correctly. Also includes the solving logic.
*   **User Interface (Slint)**: A modern, declarative UI built using Slint, interfaced via its C++ bindings. This provides a clean and responsive user experience.
*   **Serial Communication (C++)**: Manages the RS232 connection to the printer. It uses platform-specific APIs for optimal performance and reliability:
    *   **Linux**: `termios`
    *   **Windows**: Win32 API (`windows.h`)
*   **Printer Output Engine**: Formats the generated Sudoku grid into printer-compatible commands. It utilizes ESC/P2 raster bitmap commands for high-quality graphics formatting. If the specific graphics mode is unsupported by the printer's current state, it falls back to a clean ASCII/box-drawing character representation.

## Printer Specifications & Configuration

The application is specifically tuned for the **FutureLogic Gen2 Universal (Model: GURUSAGE8, PSA-60-S12RU)** thermal ticket printer.

*   **Connection**: RS232 Serial
*   **Baud Rate**: 19200
*   **Data Bits**: 8
*   **Parity**: None
*   **Protocol**: XON/XOFF or Hardware Handshaking
*   **Print Dimensions**: 
    *   Width: 62mm
    *   Resolution: 203 dpi (~495 pixels horizontally)
    *   Ticket Size: 65mm x 156mm
*   **Language Support**: TCL printer language and a subset of ESC/P2 commands.

## Features

*   **Dynamic Generation**: Generate entirely new Sudoku puzzles on the fly.
*   **Difficulty Selection**: Choose from multiple difficulty levels tailored to different skill sets.
*   **Batch Printing**: Select the number of Sudoku puzzles to generate and print in a single queue.
*   **Cross-Platform UI**: A sleek, modern interface powered by Slint.

## Build Instructions

### Prerequisites (All Platforms)

*   A C++20 compatible compiler (GCC, Clang, or MSVC)
*   CMake (ver. 3.21 or higher)
*   [Slint Dependencies](https://slint.dev/docs/cpp/) (Rust toolchain is usually required by Slint's build process)

### Linux (Primary Target: Arch Linux)

1.  **Install Dependencies**:
    ```bash
    sudo pacman -S base-devel cmake rust upower fontconfig
    ```
    *(Note: `rust` is required by the Slint compiler during the build process)*

2.  **Configure and Build**:
    ```bash
    mkdir build && cd build
    cmake ..
    make -j$(nproc)
    ```

3.  **Serial Port Permissions**:
    Ensure your user is in the `uucp` (or `dialout` depending on the distro) group to access the serial port (typically `/dev/ttyS0` or `/dev/ttyUSB0`) without root privileges:
    ```bash
    sudo usermod -aG uucp $USER
    ```
    *You will need to log out and log back in for this to take effect.*

### Windows

1.  **Install Dependencies**:
    *   Install [Visual Studio 2022](https://visualstudio.microsoft.com/) with the "Desktop development with C++" workload.
    *   Install [CMake](https://cmake.org/download/).
    *   Install [Rustup](https://rustup.rs/) (required for the Slint compiler).

2.  **Configure and Build**:
    Open a "x64 Native Tools Command Prompt for VS 2022" and run:
    ```cmd
    mkdir build
    cd build
    cmake ..
    cmake --build . --config Release
    ```

3.  **Serial Port Configuration**:
    On Windows, serial ports are accessed via `COM1`, `COM2`, etc. You can find the correct COM port by checking the "Ports (COM & LPT)" section in the Device Manager. The application will use the Win32 `CreateFile` API to open the COM port. Make sure no other application (like PuTTY or Tera Term) is actively holding the port open.
