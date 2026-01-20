# Sprite Sheet Specifications

**Game**: Top-down 2D tile-based shooter
**Style**: Simple pixel art, readable at small sizes. Theme is a dark slightly horror "bots gone mad" shooter on a space station.
**Tile Size**: 32x32 pixels
**Item Size**: 16x16 pixels

---

## Tiles (32x32 each)

| Tile | Description | Color Palette |
|------|-------------|---------------|
| Floor | Stone/concrete floor tiles | Gray (#3C3C50) |
| Wall | Solid brick/stone wall | Brown (#64503C) |
| Sand | Desert sand texture | Tan (#C2B280) |
| Water | Blue water with ripples | Blue (#4068A4) |
| Lava | Glowing molten rock | Orange/Red (#CF573C) |
| Pit | Dark hole/void | Near-black (#14141E) |
| DoorPlayer | Green door (player-only) | Green (#3C783C) |
| DoorBot | Red door (bot-only) | Red (#783C3C) |
| DoorBoth | Yellow/brown door (universal) | Yellow-brown (#78643C) |
| Crate | Wooden crate | Wood brown (#8B5A2B) |
| WallDestructible | Cracked/damaged wall | Gray with cracks (#786450) |

---

## Entities (32x32 or 28x28 with padding)

| Entity | Description | Color |
|--------|-------------|-------|
| Player | Human figure, top-down view | Green (#50B450) |
| Bot | Robot/enemy figure, top-down view | Red (#B45050) |

There should be rotated versions of these entities with 45 degree rotations.

---

## Items (16x16)

| Item | Description | Color |
|------|-------------|-------|
| Pistol | Small handgun | Silver (#B4B4B4) |
| Shotgun | Double-barrel shotgun | Brown (#8B5A2B) |
| MachinePistol | Compact SMG | Dark gray (#646478) |
| Rifle | Long rifle | Dark green (#3C503C) |
| HealthPack | Red cross / medical kit | Red (#DC3C3C) |
| SpeedBoost | Lightning bolt or blue potion | Blue (#3C96DC) |
| Invulnerability | Star or golden shield | Yellow (#DCC83C) |

---

## Projectile (8x8)

| Type | Description | Color |
|------|-------------|-------|
| Bullet | Small glowing dot/pellet | Yellow (#FFFF00) |

---

## Suggested Spritesheet Layout

```
+------------------------------------------------------------------+
| Row 0: Tiles (11 × 32px = 352px)                                 |
| [Floor][Wall][Sand][Water][Lava][Pit][DoorP][DoorB][Door+][Crate][CrackWall] |
+------------------------------------------------------------------+
| Row 1: Entities (8 × 32px)                                       |
| [Player...][empty...]                                            |
+------------------------------------------------------------------+
| Row 2: Entities (8 × 32px)                                       |
| [Bot...][empty...]                                               |
+------------------------------------------------------------------+
| Row 3: Items (7 × 16px, padded to 32px height)                   |
| [Pistol][Shotgun][MP][Rifle][Health][Speed][Invuln]              |
+------------------------------------------------------------------+
| Row 4: Effects (1 × 8px, padded)                                 |
| [Bullet]                                                         |
+------------------------------------------------------------------+
```

**Recommended PNG size**: 384 × 160 pixels

---

## Index Reference

For loading in code:

**Tiles (row 0, y=0):**
- 0: Floor
- 1: Wall
- 2: Sand
- 3: Water
- 4: Lava
- 5: Pit
- 6: DoorPlayer
- 7: DoorBot
- 8: DoorBoth
- 9: Crate
- 10: WallDestructible

**Player (row 1, y=32):**
- 0-7: Human soldier wearing green armor holding a rifle. The sequence shows 8 rotations in 45-degree increments: Facing Down, Down-Right, Right, Up-Right, Up, Up-Left, Left, Down-Left.

**Bot (row 2, y=64):**
- 0-7: Red robot figure with glowing eyes. The sequence shows 8 rotations in 45-degree increments: Facing Down, Down-Right, Right, Up-Right, Up, Up-Left, Left, Down-Left.

**Items (row 2, y=96, 16px sprites):**
- 0: Pistol
- 1: Shotgun
- 2: Machine Pistol
- 3: Rifle
- 4: HealthPack
- 5: SpeedBoost
- 6: Invulnerability

**Effects (row 3, y=128, 8px sprites):**
- 0: Bullet
