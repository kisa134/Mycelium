# Mycelium Node v0.1.0

Welcome to Mycelium — a decentralized computing network that unites idle resources from home computers to solve global tasks and create an environment for the next generation of AI.

## Our Mission

We believe that 99% of the world's computing power is idle. Instead of building energy-intensive data centers, we create a network where everyone can contribute. This is not just resource conservation, it's a step towards an open, transparent, and collective future of artificial intelligence.

## Features

- **P2P Network**: Built on libp2p with Kademlia DHT, Identify, Ping, and mDNS protocols
- **Real-time Monitoring**: CPU and RAM usage monitoring with live updates
- **Cross-platform**: Native desktop applications for Windows, macOS, and Linux
- **Modern UI**: Clean, responsive interface built with SvelteKit and TailwindCSS
- **Automatic Discovery**: Automatic peer discovery and connection management

## Technology Stack

- **Backend**: Rust + libp2p + Tauri
- **Frontend**: SvelteKit + TypeScript + TailwindCSS
- **P2P Protocols**: Kademlia DHT, Identify, Ping, GossipSub, mDNS
- **Build System**: GitHub Actions with automated releases

## Installation

Download the latest version for your operating system from the [Releases](https://github.com/kisa134/Mycelium/releases) section.

### Prerequisites

- Windows 10/11, macOS 10.15+, or Linux (Ubuntu 18.04+)
- 4GB RAM minimum
- 100MB free disk space

## Development

### Prerequisites

- [Rust](https://rustup.rs/) (latest stable)
- [Node.js](https://nodejs.org/) (v18 or later)
- [Tauri CLI](https://tauri.app/v1/guides/getting-started/setup/)

### Setup

1. Clone the repository:
   ```bash
   git clone https://github.com/kisa134/Mycelium.git
   cd Mycelium
   ```

2. Install dependencies:
   ```bash
   npm install
   ```

3. Run in development mode:
   ```bash
   npm run tauri dev
   ```

### Building

To build the application:

```bash
npm run tauri build
```

This will create platform-specific installers in `src-tauri/target/release/`.

## Architecture

### P2P Network

The application implements a peer-to-peer network using libp2p with the following components:

- **Kademlia DHT**: Distributed hash table for peer discovery
- **Identify Protocol**: Exchange of peer metadata
- **Ping Protocol**: Peer health monitoring
- **mDNS**: Local network peer discovery
- **GossipSub**: Future task distribution (configured but not yet implemented)

### System Monitoring

Real-time monitoring of system resources:

- CPU usage percentage
- RAM usage with total/used memory display
- Automatic updates every 2 seconds

### User Interface

Modern, responsive interface with:

- Connection status indicator
- Peer count display
- Resource usage graphs
- Real-time event logs
- Start/Stop controls

## Contributing

We welcome contributions! Please see our [Contributing Guidelines](CONTRIBUTING.md) for details.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Roadmap

- [ ] Task distribution and execution
- [ ] Blockchain integration for rewards
- [ ] Advanced peer discovery algorithms
- [ ] Mobile applications
- [ ] Web dashboard

## Support

If you encounter any issues or have questions, please:

1. Check the [Issues](https://github.com/kisa134/Mycelium/issues) page
2. Create a new issue with detailed information
3. Join our community discussions

## Acknowledgments

- [libp2p](https://libp2p.io/) for the P2P networking stack
- [Tauri](https://tauri.app/) for the desktop application framework
- [SvelteKit](https://kit.svelte.dev/) for the frontend framework
