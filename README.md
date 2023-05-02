# Composition
Composition is a new Minecraft server written from the ground-up in Rust.

Composition is targeting Minecraft version 1.19.3, protocol version 761.
The main goal is to get a working server, then optimize for speed (multi-threading/kubernetes/etc).

## Project Structure
Composition is broken up into multiple crates to speed up build times and improve modularity.
- `src/main.rs` is a wrapper around `composition-core` that sets up logging among other things.
  This is the main binary that is exported with `cargo build`.
- `composition-core` implements the main server logic, such as handling clients and loading world chunks.
- `composition-protocol` handles the types and packets needed for network communication.
  The library was designed to be able to used by anyone looking to implement a Minecraft server.
- `composition-config` handles the server configuration files and command line argument parsing.

## Useful Resources
- [Protocol Specification](https://wiki.vg/Protocol)
- [Normal Login Sequence](https://wiki.vg/Protocol_FAQ#What.27s_the_normal_login_sequence_for_a_client.3F)
- [Server Ping](https://wiki.vg/Server_List_Ping)
- [Map Format](https://wiki.vg/Map_Format)
