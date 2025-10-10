<img width="30" height="30" alt="extension_icon (3)" src="https://github.com/user-attachments/assets/da3733d6-5447-4b09-b6fd-a6ef0f2e424e" /> #Rivy Game Engine (made in rust)


<img width="3568" height="5728" alt="ray-so-export (4)" src="https://github.com/user-attachments/assets/85969dad-718e-4232-a363-aee034a3ee61" />



<img width="2860" height="4288" alt="ray-so-export (2)" src="https://github.com/user-attachments/assets/30b2f958-45e3-422d-bcd2-7c8fb171b2a5" />





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
