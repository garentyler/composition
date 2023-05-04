# Composition
Composition is a new Minecraft server written from the ground-up in Rust.

Composition is targeting Minecraft version 1.19.4, protocol version 762.
The main goal is to get a working server, then optimize for speed (multi-threading/kubernetes/etc).

## Features
- [x] Server status (favicon included)
- [ ] [Authentication](https://github.com/garentyler/composition/milestone/1)
- [ ] [Encryption](https://github.com/garentyler/composition/issues/10)/[compression](https://github.com/garentyler/composition/issues/11)
- [ ] World
  - [ ] [World generation](https://github.com/garentyler/composition/milestone/3)
    - [ ] [Flat world generation](https://github.com/garentyler/composition/issues/12)
    - [ ] [More complex world generation](https://github.com/garentyler/composition/issues/13)
  - [ ] [World updates](https://github.com/garentyler/composition/milestone/7) (placing/breaking blocks)
  - [ ] [World saving](https://github.com/garentyler/composition/milestone/8) (probably custom format)
- [ ] [Chat](https://github.com/garentyler/composition/milestone/4)
  - [ ] [Player chat](https://github.com/garentyler/composition/issues/15)
  - [ ] [System chat](https://github.com/garentyler/composition/issues/16)
  - [ ] Console input
  - [ ] Commands
- [ ] [Collisions and physics](https://github.com/garentyler/composition/milestone/6)
  - [ ] Player movement
- [ ] [Entities](https://github.com/garentyler/composition/milestone/9)
  - [ ] Spawning
  - [ ] AI
- [ ] [Inventory](https://github.com/garentyler/composition/milestone/10)
  - [ ] Items
  - [ ] Chests/Shulkers/etc.
  - [ ] Crafting/Smelting/etc.
- [ ] Plugins ([WASM](https://webassembly.org/))
- [ ] Future ideas (k8s, mods, anti-cheat, etc.)

## Project Structure
Composition is broken up into multiple crates to speed up build times and improve modularity.
- `src/main.rs` implements the main server logic, such as handling clients and loading world chunks.
  It also sets up logging and loads the main configuration.
  This is the main binary that is exported with `cargo build`.
- `composition-protocol` handles the types and packets needed for network communication as well as general Minecraft types, such as entities, items, and blocks.
  The library was designed to be able to used by anyone looking to implement a Minecraft server.
- `composition-world` generates the world and updates the entities and blocks within it. In the future, the world might be extracted into its own server so that multiple "server cores" can process players on the same world.

## Useful Resources
- [Protocol Specification](https://wiki.vg/Protocol)
- [Normal Login Sequence](https://wiki.vg/Protocol_FAQ#What.27s_the_normal_login_sequence_for_a_client.3F)
- [Server Ping](https://wiki.vg/Server_List_Ping)
- [Map Format](https://wiki.vg/Map_Format)

