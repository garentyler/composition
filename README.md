# Composition
Composition is a new Minecraft server written from the ground-up in Rust.

Composition is targeting Minecraft version 1.19.4, protocol version 762.
The main goal is to get a working server, then optimize for speed (multi-threading/kubernetes/etc).

## Features
- [x] Server status (favicon included)
- [ ] Authentication
- [ ] Encryption/compression
- [ ] Flat world generation
- [ ] More complex world generation
- [ ] Chat
- [ ] Player movement
- [ ] Collisions
- [ ] World updates (placing/breaking blocks)
- [ ] World saving (probably custom format)
- [ ] Entities
- [ ] Items and inventory
- [ ] Crafting
- [ ] Commands & console input
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

