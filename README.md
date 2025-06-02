# Shitsploit Documentation
![Alt Text](https://raw.githubusercontent.com/penguin-cmyk/memory_utils/refs/heads/master/must_logo.png)

---

**Shitsploit** is an external cheat base written in Rust using my memory library [`memory_utils`](https://crates.io/crates/memory_utils).

It is meant to be a **base**, not a complete external cheat.

---

## 📁 Files You Should Care About

You primarily need to work with:

- `entry/entry.rs`
- Any custom module files you create

The main source works fine as-is, but you're free to modify it if needed.

---

## 🔁 Offset Updates

No need to manually update offsets — I’ll handle that after each Roblox update, as long as I'm available.

---

## ✨ Features

### ✅ Working Components

-  [x] Teleport handler

### 🧰 Roblox Utility Library (`utils/cheat/rbx.rs`)

Provides basic Roblox API-like access:

- `rbx::world_to_screen`
- `rbx::GetChildren`
- `rbx::FindFirstChild`
- `rbx::GetService`
- `rbx::GetCameraPos`
- `rbx::MoveDirection`
- `rbx::GetClass`
- `rbx::GetLocalPlayer`
- `rbx::GetPlayers`
- `rbx::name`
- `rbx::datamodel`

---

## 🧩 Classes & Structs

### `Offsets`

Used to store memory offsets. Internally updated when needed.

---

### `Player`

Represents a Roblox player instance with several utility methods:

- `.health` – Get current health
- `.set_health` – Set health value
- `.get_position` – Get current position (Vector3)
- `.set_position` – Teleport player to a position
- `.team` – Get the team of the player
- `.primitive` – Method to retrieve the primitive of a parts address

---

### `Globals`

Stores global states used by the external.

---

### Math and Roblox Structs

- `Vector3` – 3D vector for position/direction
- `Vector2` – 2D vector (e.g., screen space)
- `Color3` – RGB color structure
- `Quaternion` – Rotation representation
- `Matrix4` – 4x4 transformation matrix
