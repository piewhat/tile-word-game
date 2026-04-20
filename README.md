# Tile Word Game

https://private-user-images.githubusercontent.com/17803752/581018960-2a0b9ae5-08ba-424f-9fe6-80bcb1e7934f.mp4?jwt=eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpc3MiOiJnaXRodWIuY29tIiwiYXVkIjoicmF3LmdpdGh1YnVzZXJjb250ZW50LmNvbSIsImtleSI6ImtleTUiLCJleHAiOjE3NzY3MTQ5NzAsIm5iZiI6MTc3NjcxNDY3MCwicGF0aCI6Ii8xNzgwMzc1Mi81ODEwMTg5NjAtMmEwYjlhZTUtMDhiYS00MjRmLTlmZTYtODBiY2IxZTc5MzRmLm1wND9YLUFtei1BbGdvcml0aG09QVdTNC1ITUFDLVNIQTI1NiZYLUFtei1DcmVkZW50aWFsPUFLSUFWQ09EWUxTQTUzUFFLNFpBJTJGMjAyNjA0MjAlMkZ1cy1lYXN0LTElMkZzMyUyRmF3czRfcmVxdWVzdCZYLUFtei1EYXRlPTIwMjYwNDIwVDE5NTExMFomWC1BbXotRXhwaXJlcz0zMDAmWC1BbXotU2lnbmF0dXJlPWQ1MzhjYWI2OWRkYTk2MDBmZjlmMTU5NzJlMGFmMTFjYmJiYzNiYTVhYzQwNTc4NTQ4ZWIyN2JlNmViMTVkNzcmWC1BbXotU2lnbmVkSGVhZGVycz1ob3N0JnJlc3BvbnNlLWNvbnRlbnQtdHlwZT12aWRlbyUyRm1wNCJ9._hpGFeH-1RYRSrp_X4Sf652Btr3GYTnBuvf-O2BDaqE

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
