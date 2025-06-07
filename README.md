# 📓 Composition

Composition is a new Minecraft server written from the ground-up in Rust.

Composition is targeting Minecraft version 1.21.5, protocol version 770.
The main goal is to get a working server, then optimize for speed
(multi-threading/kubernetes/etc).

Composition is my personal project to learn more about networking,
game development, Rust, Docker, and CI/CD pipelines.
Composition is not currently intended for production use.
I want to use it as a platform to experiment with ideas about how a Minecraft
server could work, such as a tickless design, one that could run across a distributed
kubernetes cluster, or a world as a database that's accessed by multiple servers.

## 🚀 Getting Started

- The only prerequisite for running Composition is to have Docker installed.
- Pull the latest version of the Docker image:
  `docker pull git.garen.dev/garentyler/composition:latest`
- Make sure a folder exists for the it to store its data. The uid/gid of the
  process inside the container is `25565:25565`, so make sure the folder
  is writable by that user.

  To get the right file permissions, you can run the following:

  ```sh
  mkdir -p composition-server
  sudo chown -R 25565:25565 composition-server
  sudo chmod -R 775 composition-server
  ```

### 🕹️ Running Composition as a Server

Standalone command:

```sh
docker run -it -v .:/app/data --network host git.garen.dev/garentyler/composition server
```

Example docker-compose.yml file:

```yml
services:
  # Composition running as a server.
  composition-server:
    image: git.garen.dev/garentyler/composition:latest
    container_name: composition-server
    restart: unless-stopped
    ports:
      - "25565:25565"
    volumes:
      - ./composition-server:/app/data
    command: server
```

### 🌐 Running Composition as a Proxy

Standalone command:

```sh
docker run -it -v .:/app/data --network host git.garen.dev/garentyler/composition proxy
```

Example docker-compose.yml file:

```yml
services:
  # Example Minecraft server to run as a reference.
  reference:
    image: itzg/minecraft-server
    container_name: reference
    restart: unless-stopped
    ports:
      - "25565:25565"
    environment:
      EULA: "TRUE"
      VERSION: "1.21.1"
  # Composition running as a proxy.
  composition-proxy:
    image: git.garen.dev/garentyler/composition:latest
    container_name: composition-proxy
    restart: unless-stopped
    depends_on:
      reference:
        condition: service_healthy
        restart: true
    ports:
      - "25566:25566"
    volumes:
      - ./composition-proxy:/app/data
    command: proxy -U reference
```

## 🛠️ Building From Source

To build Composition from source, you will need to have Rust and Cargo installed.
The stable toolchain is recommended, but the nightly toolchain may be required for some features.

```sh
cargo build --release
```

To build the Docker image, you will need to have Docker configured with Bake.

```sh
# To build both linux/amd64 and linux/arm64 images, use the following command:
docker buildx bake -f docker-bake.hcl

# To build only the linux/amd64 image, use the following command:
docker buildx bake -f docker-bake.hcl --set default.platform=linux/amd64
```

## 🏗️ Project Structure

Composition is built using the async runtime and networking from Tokio.
On startup, Composition sets up logging, reads the configuration, and
parses command line arguments. From there, it decides which mode to
start in (server, proxy, world, etc). The subcommand performs any
startup tasks, such as loading the world, and then begins the main
event loop.

### 🦀 Cargo Features

Composition has a non-optional core and multiple cargo features that can be enabled or disabled to configure functionality.

The `server` feature enables the `composition server` command, which runs a standalone Minecraft server.

The `proxy` feature enables the `composition proxy` command, which runs a proxy server that forwards packets to another server.

The `world` feature is not yet implemented. When finished, it will host a world server that can be accessed by multiple server cores.

## 📋 Useful Resources

- [wiki.vg archive](https://minecraft.wiki/w/Minecraft_Wiki:Projects/wiki.vg_merge) (now merged with the Minecraft wiki)
- [Protocol Specification](https://minecraft.wiki/w/Java_Edition_protocol)
- [Normal Login Sequence](https://minecraft.wiki/w/Minecraft_Wiki:Projects/wiki.vg_merge/Protocol_FAQ#What's_the_normal_login_sequence_for_a_client?)
- [Server Ping](https://minecraft.wiki/w/Minecraft_Wiki:Projects/wiki.vg_merge/Server_List_Ping)
- [Map Format](https://minecraft.wiki/w/Minecraft_Wiki:Projects/wiki.vg_merge/Map_Format)
