# 3D Game with Bevy

A 3D game built with the Bevy game engine in Rust, featuring two different versions: basic and enhanced.

## Features

### Basic Version

- 3D scene with multiple objects (cube, sphere, plane)
- Dynamic lighting with shadows
- Camera controls for exploration
- Animated rotating cube

### Enhanced Version

- All basic features plus:
- Physics simulation with bouncing sphere
- Color-cycling cubes with timers
- Enhanced materials with metallic and roughness properties
- Ambient lighting
- Improved window configuration
- Multiple colored objects

## Controls

- **W/A/S/D**: Move camera forward/left/backward/right
- **Space**: Move camera up
- **Left Shift**: Move camera down

## Running the Game

### Quick Start

```bash
# Run the current version
cargo run

# Or use the convenience script
./run_game.sh
```

### Version Switching

```bash
# Switch to basic version
./switch_version.sh basic

# Switch to enhanced version
./switch_version.sh enhanced
```

### Manual Build

```bash
# Debug build
cargo build

# Release build (better performance)
cargo build --release
cargo run --release
```

## Project Structure

```
game_3d/
├── src/
│   ├── main.rs              # Current active version
│   └── enhanced_main.rs     # Enhanced version source
├── Cargo.toml              # Project dependencies
├── README.md               # This file
├── run_game.sh            # Quick run script
└── switch_version.sh      # Version switcher
```

## Dependencies

- [Bevy](https://bevyengine.org/) 0.15 - A refreshingly simple data-driven game engine built in Rust

## Game Objects

### Basic Version

1. **Rotating Cube**: A textured cube that rotates continuously
2. **Sphere**: A purple sphere positioned to the right
3. **Ground Plane**: A green plane serving as the ground
4. **Point Light**: Provides lighting and shadows to the scene
5. **Camera**: 3D camera with WASD movement controls

### Enhanced Version

1. **Rotating Cube**: Multi-axis rotating cube with metallic material
2. **Bouncing Sphere**: Physics-enabled sphere with gravity and collision
3. **Ground Plane**: Larger green plane with realistic materials
4. **Color-Cycling Cubes**: Multiple cubes that change colors over time
5. **Enhanced Lighting**: Point light with ambient lighting
6. **Camera**: Improved camera with faster movement

## Development Concepts

This project demonstrates:

- **Mesh rendering** - 3D object display
- **Material systems** - Surface properties and appearance
- **Component-based architecture** - Bevy's ECS (Entity Component System)
- **Input handling** - Keyboard controls
- **Transform animations** - Object movement and rotation
- **Physics simulation** - Gravity and collision detection (enhanced version)
- **Timer systems** - Time-based color changes (enhanced version)
- **Resource management** - Game state and asset handling

## Learning Path

1. Start with the **basic version** to understand core 3D concepts
2. Examine the code structure and Bevy's ECS pattern
3. Switch to the **enhanced version** to see advanced features
4. Experiment with modifying colors, speeds, and object properties
5. Try adding your own game objects and behaviors

## Next Steps

Consider extending the game with:

- Player-controlled character
- Collectible items
- Sound effects and music
- More complex physics interactions
- Game objectives and scoring
- Multiple levels or scenes
