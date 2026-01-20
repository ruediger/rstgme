We want to write a little 2D game. It should be a game with a top down view. The levels are tile based grid. Characters are humans or robots moving in the levels.

Eventually this should be an online game. But for now let's just focus on the player controlling one character and everything else being simple bots.

We are using Rust. Use modern Rust and add well known dependencies. Write tests as you go along. Ask me if you are unsure about things.

You can use Gemini-CLI to run nanobanana to generate graphics or generate them yourself. Submit to git at meaningful intervals.

Use `cargo fmt` to keep code formatted and `cargo clippy` to lint the code.

## Module Structure

- `main.rs` - Entry point, window config, game loop
- `game.rs` - GameState struct, orchestrates updates and drawing
- `tile_map.rs` - TileMap with multiple tile types, collision, speed modifiers, destructibles
- `entity.rs` - Position (grid + smooth visual), Player, Bot structs, EntityType enum
- `input.rs` - Keyboard (WASD/arrows) and mouse input
- `weapon.rs` - Weapon struct with fire rate, cooldown, bullet speed
- `projectile.rs` - Projectile movement and wall collision

## Architecture Notes

- Using **macroquad** for rendering and input
- Tile size: 32x32 pixels
- Positions use grid coordinates (i32) with visual interpolation (f32) for smooth movement
- Entities only move when `is_at_target()` returns true (grid-locked movement)
- Movement speed affected by tile type (sand/water slow down)
- Projectiles use pixel coordinates and continuous movement
- Projectiles check `blocks_projectile()` for collision (pit lets projectiles pass)
- Destructible tiles tracked via `HashMap<(usize, usize), u8>` for health
- EntityType (Player/Bot) determines door access
- Bots move randomly on a timer

## Current Features

- Player movement with WASD/arrows
- Bots that wander randomly (respawn 5-15s after death)
- Large random map (60x45) with camera following player
- Mouse aiming with visual aim line
- Shooting with left mouse button
- Projectile-bot collision with score tracking
- Multiple weapons (1-5 keys): Fist, Pistol, Shotgun, Machine Pistol, Rifle
- Extended tile system with different behaviors
- Player health system (100 HP, respawn on death)
- Lava deals 25 damage per second
- Health bar in HUD (green/yellow/red based on health)

## Tile Types

| Tile | Walkable | Speed | Projectile | Destructible |
|------|----------|-------|------------|--------------|
| Floor | All | 1.0x | Pass | No |
| Wall | None | - | Block | No |
| Sand | All | 0.5x | Pass | No |
| Water | All | 0.3x | Pass | No |
| Lava | All | 0.4x | Pass | No (25 dmg/s) |
| Pit | None | - | Pass | No |
| DoorPlayer | Player | 1.0x | Block | No |
| DoorBot | Bot | 1.0x | Block | No |
| DoorBoth | All | 1.0x | Block | No |
| Crate | None | - | Block | 1 hit |
| WallDestructible | None | - | Block | 3 hits |

## TODO / Future

- Bot AI (chase player, shoot back)
- Keycards for doors
- Graphics/sprites
- Online multiplayer
