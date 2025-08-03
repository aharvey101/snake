# 🐍 Snake Game

A classic Snake game implementation built with Rust and the Bevy game engine. Navigate your snake around the grid, eat food to grow, and try to achieve the highest score possible without crashing into walls or yourself!

## 🎮 Game Features

- **Classic Snake Gameplay**: Control a growing snake on a grid
- **Real-time Score Display**: Your score increases as your snake grows
- **Collision Detection**: Game ends when hitting walls or yourself
- **Smooth Controls**: Responsive keyboard input with WASD or arrow keys
- **Game Over Screen**: Clear feedback when the game ends
- **Instant Restart**: Quick restart functionality with spacebar
- **Blocky Retro Graphics**: Simple, clean visual style without sprites

## 🚀 How to Run

### Prerequisites

Make sure you have Rust installed on your system. If not, install it from [rustup.rs](https://rustup.rs/).

### Installation & Running

1. **Clone or navigate to the project directory:**
   ```bash
   cd snake
   ```

2. **Run the game:**
   ```bash
   cargo run
   ```

The game will compile automatically and open in a new window.

## 🎯 How to Play

### Controls

- **Movement**: Use arrow keys (↑ ↓ ← →) or WASD keys to control your snake
  - `W` or `↑`: Move up
  - `S` or `↓`: Move down  
  - `A` or `←`: Move left
  - `D` or `→`: Move right
- **Restart**: Press `SPACE` after game over to start a new game

### Gameplay Rules

1. **Objective**: Eat the red food squares to grow your snake and increase your score
2. **Movement**: Your snake moves automatically in the direction you choose
3. **Growth**: Each piece of food eaten adds one segment to your snake's length
4. **Scoring**: Your score equals the length of your snake (starts at 1)
5. **Game Over**: The game ends if you:
   - Hit the walls (edges of the grid)
   - Run into your own body
6. **Restart**: After game over, press SPACE to play again

### Visual Guide

- 🟢 **Bright Green Square**: Snake head
- 🟢 **Dark Green Squares**: Snake body segments  
- 🔴 **Red Square**: Food to eat
- **White Text**: Score display (top-left corner)
- **Red Text**: Game over message (center screen)

## ⚙️ Game Configuration

The game includes several configurable constants at the top of `src/main.rs`:

```rust
const GRID_WIDTH: i32 = 20;     // Grid width in cells
const GRID_HEIGHT: i32 = 15;    // Grid height in cells
const CELL_SIZE: f32 = 30.0;    // Size of each cell in pixels
```

You can modify these values to change the game's dimensions and cell size.

The snake's movement speed can be adjusted by changing the timer duration:
```rust
GameTimer(Timer::from_seconds(0.15, TimerMode::Repeating))
```

## 🛠️ Technical Details

### Built With

- **Rust**: Systems programming language
- **Bevy Engine**: Modern game engine for Rust
- **Dependencies**:
  - `bevy = "0.14"` - Game engine
  - `rand = "0.8"` - Random number generation for food placement

### Architecture

The game uses Bevy's Entity Component System (ECS) architecture:

- **Components**: `SnakeHead`, `SnakeSegment`, `Food`, `ScoreText`, `GameOverText`
- **Resources**: `SnakeState` (game state), `GameTimer` (movement timing)
- **States**: `Playing` and `GameOver` game states
- **Systems**: Modular functions handling input, movement, collisions, rendering, and UI

### Performance

The game runs at 60 FPS with smooth movement and responsive controls. The snake moves at a fixed interval (150ms by default) regardless of the frame rate.

## 🎨 Customization

Want to modify the game? Here are some ideas:

- **Colors**: Change snake and food colors in the sprite creation code
- **Speed**: Adjust the `GameTimer` duration for faster/slower gameplay
- **Grid Size**: Modify `GRID_WIDTH`, `GRID_HEIGHT`, and `CELL_SIZE`
- **Scoring**: Implement bonus points or multipliers
- **Power-ups**: Add special food types with different effects
- **Levels**: Create increasing difficulty levels

## 🐛 Troubleshooting

### Common Issues

1. **Game won't compile**: Ensure you have the latest Rust version
2. **Window doesn't appear**: Check that your system supports the required graphics APIs
3. **Controls not responsive**: Make sure the game window has focus

### System Requirements

- **Operating System**: Windows, macOS, or Linux
- **Graphics**: Any system with basic graphics support
- **Memory**: Minimal requirements (< 50MB RAM)

## 📝 License

This project is open source. Feel free to modify and distribute as needed.

## 🤝 Contributing

Feel free to fork this project and submit pull requests for improvements or bug fixes!

---

**Enjoy the game! 🐍**
