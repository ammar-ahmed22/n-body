<div align="center">
  <h1>n-body</h1>
  <p>A 2D n-body gravitational simulation in Rust using Bevy, featuring spatial partitioning and the Barnes-Hut algorithm for efficient force calculations.</p>
</div>

## Features
- **Bevy Integration:** Leverages the Bevy game engine for efficient rendering and entity management
- **Spatial Partionining:** Utilizes custom written quadtrees to optimize force calculations and collision detection
- **Barnes-Hut Algorithm:** Reduce computational complexity from O(n²) to O(n log n)
- **Performance Metrics:** Displays real-time statistics like FPS and body count to monitor simulation efficiency

## Usage
1. Clone the repository  
```bash
git clone https://github.com/ammar-ahmed22/n-body
cd n-body
```

2. Build and run the project  
```bash
cargo run --release
```
