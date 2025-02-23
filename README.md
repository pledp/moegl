# moegl

Work-In-Progress game framework written in Rust

## How To Run

Run window example:
```shell
cargo run --example window
```

# Features
- "Plugins" with state, initialization and update methods
- Windowing and rendering
- Keyboard input

## In-Progress
- Events
- Mouse input
- Actual rendering
- Hot reloading
- Embedded support

# Design Goals
- Minimal core: Core framework offers **only** must-have things 
- Modularity: Additional abstraction offered in client-initialized modules
- Minimal: Minimum dependencies to get started, infinite modularity
- Simple: Easy to get started, infinitely flexible with modularity
