We want to write a little 2D game. It should be a game with a top down view. The levels are tile based grid. Characters are humans or robots moving in the levels.

Eventually this should be an online game. But for now let's just focus on the player controlling one character and everything else being simple bots.

We are using Rust. Use modern Rust and add well known dependencies. Write tests as you go along. Ask me if you are unsure about things.

You can use Gemini-CLI to run nanobanana to generate graphics or generate them yourself. Submit to git at meaningful intervals.

Use `cargo fmt` to keep code formatted and `cargo clippy` to lint the code.

## Module Structure

- `main.rs` - Entry point, window config, game loop
- `game.rs` - GameState struct, orchestrates updates and drawing
- `tile_map.rs` - TileMap with Floor/Wall tiles, collision checking, rendering
- `entity.rs` - Position (grid + smooth visual), Player, Bot structs
- `input.rs` - Keyboard (WASD/arrows) and mouse input
- `weapon.rs` - Weapon struct with fire rate, cooldown, bullet speed
- `projectile.rs` - Projectile movement and wall collision

## Architecture Notes

- Using **macroquad** for rendering and input
- Tile size: 32x32 pixels
- Positions use grid coordinates (i32) with visual interpolation (f32) for smooth movement
- Entities only move when `is_at_target()` returns true (grid-locked movement)
- Projectiles use pixel coordinates and continuous movement
- Bots move randomly on a timer

## Current Features

- Player movement with WASD/arrows
- Bots that wander randomly
- Wall collision for all entities
- Mouse aiming with visual aim line
- Shooting with left mouse button (pistol: 4 shots/sec)
- Projectiles collide with walls

## TODO / Future

- Projectile-bot collision (damage/kill bots)
- More weapon types
- Bot AI (chase player, shoot back)
- Health system
- Graphics/sprites
- Online multiplayer
