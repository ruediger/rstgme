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
- `item.rs` - Item pickups (weapons, health packs, buffs)
- `sprites.rs` - SpriteSheet loading and drawing, direction helpers

## Architecture Notes

- Using **macroquad** for rendering and input
- Tile size: 32x32 pixels
- Positions use grid coordinates (i32) with visual interpolation (f32) for smooth movement
- Entities only move when `is_at_target()` returns true (grid-locked movement)
- Movement speed affected by tile type (sand/water slow down)
- Projectiles use pixel coordinates and continuous movement
- Projectiles check `blocks_projectile()` for collision (pit lets projectiles pass)
- Projectiles track source (`from_player`) for collision filtering
- Destructible tiles tracked via `HashMap<(usize, usize), u8>` for health
- EntityType (Player/Bot) determines door access
- Bots move randomly on a timer; hostile bots chase player
- Sprites loaded from `data/sprites.png` (see SPRITES.md for layout)

## Current Features

- Player movement with WASD/arrows
- Bots that wander randomly (respawn 5-15s after death)
- **Hostile bots** that chase and shoot at the player (red tinted)
- Large random map (60x45) with camera following player
- Mouse aiming with visual aim line
- Shooting with left mouse button
- Melee attack animation (knife swing arc)
- Projectile-bot collision with score tracking
- Projectile source tracking (player vs bot projectiles)
- Multiple weapons (1-5 keys): Knife, Pistol, Shotgun, Machine Pistol, Rifle
- Extended tile system with different behaviors
- Player health system (100 HP, respawn on death)
- Lava deals 25 damage per second
- **Red screen flash** when taking damage (pulsing effect)
- Health bar in HUD (green/yellow/red based on health)
- Item pickup system (weapons, health packs, buffs)
- Items spawn on floor and drop from destroyed crates/walls
- Speed boost (5s, 2x speed + lava immunity)
- Invulnerability (3s, no damage)
- **Sprite-based rendering** with 8-directional rotation for entities

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

## Item Types

| Item | Effect | Spawn Location |
|------|--------|----------------|
| Pistol | Adds weapon | Floor, Crates |
| Shotgun | Adds weapon | Crates, Walls |
| Machine Pistol | Adds weapon | Crates, Walls |
| Rifle | Adds weapon | Walls only |
| HealthPack | +25 HP | Floor, Crates |
| SpeedBoost | 2x speed + lava immunity 5s | Crates |
| Invulnerability | No damage 3s | Walls |

## TODO / Future

- Ammunition system
- Keycards for doors
- Different sprite for hostile bots
- Sound effects
- Online multiplayer
