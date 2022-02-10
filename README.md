# Bullet Echo - clone using Rust

1. [Basic usage](#basic-usage)
2. [Changelog](#changelog)
3. [Bugs and TODOs](#bugs-and-todos)

## Basic usage

Compile with

```ps1
cargo build #--release
```

Run with

```ps1
cargo run #--release
```

## Changelog

1.  Initial release
2.  ray tracing algorithm (line-line intersection from wikipedia)
3.  the drawing is player-centered and the player is always facing up
4.  some piston_window translations and rotations
5.  drawing the enemies but only the parts that are visible
6.  got rid of some square-roots
7.  updated the vector library to something more 3d and general

## Bugs and TODOs

1.  we need guns... and ammo
2.  octree acceleration (the ray tracing algorithm is quite slow, and the drawing too)
3.  health bar
4.  more maps and levels
5.  better drawings
