#<img width="30" height="30" alt="extension_icon (3)" src="https://github.com/user-attachments/assets/da3733d6-5447-4b09-b6fd-a6ef0f2e424e" /> #Rivy Game Engine (made in rust)

<img width="3680" height="6688" alt="ray-so-export (3)" src="https://github.com/user-attachments/assets/6a5cb005-cb6e-479c-9a49-e91c74cad447" />









**Rivy** is a lightweight game engine built on top of **Raylib** and **Bevy ECS**.  
It is designed to be **easy to use**, **CPU-friendly**, and **beginner-friendly** while still giving you the power of ECS.  
⚠️ Not done adding all of raylibs func *no crate* and right now it is just a bevy_ecs wrapper for Raylib

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
- ✅ keybord input system  (`runtimesys.rs`)
- 🔄 *Coming soon*: 2D rendering support  
- 🔄 Add more Raylib functions  


---

## Notes

- Files `game.rs` and `add.rs` are **not needed** right now.  
- `lib.rs` links everything together so you can easily `pub use` the engine via `prelude.rs`.  

### At a glance
- ✔️ What Rivy is  
- ✔️ How it’s structured  
- ✔️ What’s coming next  
