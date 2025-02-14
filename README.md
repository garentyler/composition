# Composition

Composition is a new Minecraft server written from the ground-up in Rust.

Composition is targeting Minecraft version 1.21.1, protocol version 767.
The main goal is to get a working server, then optimize for speed (multi-threading/kubernetes/etc).

Composition is my personal project to learn more about networking,
game development, and Rust and is not currently intended for production use.
I want to use it as a platform to experiment with ideas about how a Minecraft
server could work, such as a tickless design, one that could run across a distributed
kubernetes cluster, or a world as a database that's accessed by multiple servers.

## Getting Started

- The only prerequisite for running Composition is to have Docker and Docker Compose installed.
- Clone the project: `git clone https://github.com/garentyler/composition.git`
- Run it in proxy mode: `docker compose --profile proxy up`

  Proxy mode will start an instance of `itzg/minecraft-server` on port 25565 and
  the Composition proxy in 25566. This is useful for testing packet parsing and
  handling.
- Run it in server mode: `docker compose --profile server up`

  Server mode starts Composition on port 25566.
  This is useful for testing the server logic and world generation.
- To build release images, use `docker buildx bake -f docker-bake.hcl`.
  This will create a multi-arch image that can be run on amd64 and arm64.

## Project Structure

Composition is built using the async runtime and networking from Tokio.
On startup, Composition sets up logging, reads the configuration, and
parses command line arguments. From there, it decides which mode to
start in (server, proxy, world, etc). The subcommand performs any
startup tasks, such as loading the world, and then begins the main
event loop.

### Cargo Features

Composition has a non-optional core and multiple cargo features that can be enabled or disabled to configure functionality.

The `server` feature enables the `composition server` command, which runs a standalone Minecraft server.

The `proxy` feature enables the `composition proxy` command, which runs a proxy server that forwards packets to another server.

The `world` feature is not yet implemented. When finished, it will host a world server that can be accessed by multiple server cores.

## Useful Resources

- [wiki.vg archive](https://minecraft.wiki/w/Minecraft_Wiki:Projects/wiki.vg_merge) (now merged with the Minecraft wiki)
- [Protocol Specification](https://minecraft.wiki/w/Java_Edition_protocol)
- [Normal Login Sequence](https://minecraft.wiki/w/Minecraft_Wiki:Projects/wiki.vg_merge/Protocol_FAQ#What's_the_normal_login_sequence_for_a_client?)
- [Server Ping](https://minecraft.wiki/w/Minecraft_Wiki:Projects/wiki.vg_merge/Server_List_Ping)
- [Map Format](https://minecraft.wiki/w/Minecraft_Wiki:Projects/wiki.vg_merge/Map_Format)
