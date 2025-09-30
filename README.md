# Rivy Game Engine #(made in rust)

**Rivy** is a lightweight game engine built on top of **Raylib** and **Bevy ECS**.  
It is designed to be **easy to use**, **CPU-friendly**, and **beginner-friendly** while still giving you the power of ECS.  
⚠️ Currently in **demo stage**.

---

## Features

- 🎮 **Raylib integration** → simple and fast rendering  
- 🧩 **Bevy ECS** → modern entity-component-system architecture  
- 🖼️ **Customizable draw system** → add any 3D draw function in `draw3d.rs`  
- 🪟 **Main loop and window setup** → handled in `wint.rs`  
- 📦 **Library exports** → managed through `lib.rs` and `prelude.rs`  

---

## Roadmap

- ✅ Core engine loop (`wint.rs`)  
- ✅ 3D drawing (`draw3d.rs`)  
- 🔄 *Coming soon*: 2D rendering support  
- 🔄 Add more Raylib functions  
- 🔄 *Coming soon*: ECS examples and demos  

---

## Notes

- Files `game.rs` and `add.rs` are **not needed** right now.  
- `lib.rs` links everything together so you can easily `pub use` the engine via `prelude.rs`.  

---

### At a glance
- ✔️ What Rivy is  
- ✔️ How it’s structured  
- ✔️ What’s coming next  
