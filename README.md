# CBBOX

A CHIP-8 interpreter/emulator written in Rust with SDL2 for graphics rendering.

## About CHIP-8

CHIP-8 is an interpreted programming language developed in the mid-1970s for early microcomputers. It's a popular choice for learning emulator development due to its simplicity and well-documented instruction set.

## Features

- ✅ Core CHIP-8 instruction set
- ✅ 64x32 monochrome display
- ✅ SDL2-based rendering
- ✅ Configurable display scaling
- ✅ **GUI ROM selector** - Browse and load ROMs from a visual interface
- 🚧 Keyboard input (in progress)
- 🚧 Sound support (in progress)

## Requirements

- Rust 1.70+ (2021 edition)
- SDL2 development libraries

### Installing SDL2

**Arch**

```bash
sudo pacman -S sdl2
```

**Fedora**

```bash
sudo dnf install SDL2-devel
```

**OpenSUSE:**

```bash
sudo zypper install libSDL2-devel
```

**Ubuntu/Debian:**

```bash
sudo apt-get install libsdl2-dev
```

**macOS:**

```bash
brew install sdl2
```

**Windows:**
Download SDL2 development libraries from [libsdl.org](https://www.libsdl.org/download-2.0.php)

## Building

```bash
cargo build --release
```

## Usage

**Option 1: Launch GUI ROM Selector (Recommended)**

Simply run without arguments to open the graphical ROM selector:

```bash
cargo run --release
```

The GUI will display all available ROMs in the `assets/` directory. Click on any ROM to launch the emulator.

**Option 2: Direct ROM Loading**

Run directly with a CHIP-8 ROM file path:

```bash
cargo run --release -- <rom_file>
```

Example:

```bash
cargo run --release -- assets/ibm.ch8
```

### Controls

- **ESC** - Exit emulator
- _(Keyboard mapping coming soon)_

## Project Structure

```
emulator/
├── Cargo.toml          # Project dependencies
├── assets/             # CHIP-8 ROM files
│   ├── ibm.ch8
│   ├── Pong.ch8
│   ├── tetris.ch8
│   └── ...
├── src/
│   ├── main.rs         # Entry point, argument handling
│   ├── gui.rs          # GUI ROM selector interface
│   ├── emu.rs          # CHIP-8 CPU implementation
│   ├── audio.rs        # Audio handling
│   └── constants.rs    # Constants and configuration
└── README.md
```

## Implemented Opcodes

| Opcode | Mnemonic       | Description                    |
| ------ | -------------- | ------------------------------ |
| `0000` | NOP            | No operation                   |
| `00E0` | CLS            | Clear screen                   |
| `1NNN` | JMP NNN        | Jump to address NNN            |
| `6XNN` | SET VX, NN     | Set register VX to NN          |
| `7XNN` | ADD VX, NN     | Add NN to register VX          |
| `ANNN` | SET I, NNN     | Set index register to NNN      |
| `DXYN` | DRAW VX, VY, N | Draw N-byte sprite at (VX, VY) |

## TODO

- [ ] Implement remaining opcodes (arithmetic, logic, timers, etc.)
- [ ] Add keyboard input handling (CHIP-8 hex keypad)
- [ ] Implement delay and sound timers
- [ ] Add configurable CPU speed
- [x] ~~ROM file selection UI~~ ✅ Implemented!
- [ ] Save/load state functionality
- [ ] Pause/Resume controls in emulator
- [ ] Display FPS counter

## Resources

- [Cowgod's CHIP-8 Technical Reference](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM)
- [CHIP-8 Wikipedia](https://en.wikipedia.org/wiki/CHIP-8)
- [Test ROMs](https://github.com/corax89/chip8-test-rom)

## License

MIT

## Acknowledgments

Special thanks to the emulator development community and CHIP-8 documentation authors who made this project possible!
