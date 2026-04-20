# Tile Word Game

<p align="center">
    <video src="assets/tile.mp4" controls width="600"></video>
</p>

A word construction game built with the [Bevy engine](https://bevyengine.org/). Arrange tiles to form valid words and rack up your score based on letter rarity!

## Features

- **Dynamic Tile Scoring**: Different point values for letters based on rarity (Vowels: 1, Common: 2, Rare: 5).
- **Dictionary Validation**: Words are checked against a comprehensive wordlist on submission.
- **Interactive Gameplay**: Drag and drop tiles with the mouse or use keyboard typing for fast placement.
- **Balanced Generation**: Grid automatically refills with a guaranteed distribution of vowels and consonants.
- **Shuffle & Skip**: Shuffle your grid to see new patterns or skip the entire board for a point penalty.

## Controls

### Mouse

- **Drag and Drop**: Move tiles between the grid and the submission slots.
- **Buttons**: Click the **Shuffle** (bottom-left) or **Skip** (bottom-right) icons.

### Keyboard

- **[A-Z]**: Type a letter to move it from the grid to the next available submission slot.
- **Backspace**: Move the last placed tile back to its original position in the grid.
- **Space**: Reset all tiles from the submission slots back to the grid.
- **Enter**: Submit the current word for validation and scoring.

## Installation

Ensure you have [Rust and Cargo](https://rustup.rs/) installed on your system. On Linux install the [mold](https://github.com/rui314/mold) linker or disable it in `.cargo/config.toml`

1. Clone the repository:
   ```bash
   git clone https://github.com/your-username/tile-game.git
   cd tile-game
   ```

2. Run the game:
   ```bash
   cargo run --release
   ```
