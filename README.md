# EternalWeb

[![GitHub license](https://img.shields.io/github/license/Tokeloshe/EternalWeb)](https://github.com/Tokeloshe/EternalWeb/blob/main/LICENSE)
[![GitHub stars](https://img.shields.io/github/stars/Tokeloshe/EternalWeb)](https://github.com/Tokeloshe/EternalWeb/stargazers)
[![GitHub issues](https://img.shields.io/github/issues/Tokeloshe/EternalWeb)](https://github.com/Tokeloshe/EternalWeb/issues)
[![GitHub forks](https://img.shields.io/github/forks/Tokeloshe/EternalWeb)](https://github.com/Tokeloshe/EternalWeb/network)

EternalWeb is a decentralized, peer-to-peer web archiving and sharing platform designed to reclaim the internet for humankind. By empowering users to preserve, share, and discover content without corporate or governmental gatekeepers, it makes the web more free, egalitarian, and resilient. Built with future-proof technologies for Web 3.0 (decentralization, blockchain) and Web 4.0 (AI intelligence, autonomy), EternalWeb operates primarily on the clearnet for transparency and accessibility, while offering optional privacy features for censored regions. It's not just another darkweb tool—it's a positive force for global knowledge preservation, education, and collaboration.

<p align="center">
  <img src="docs/media/eternalweb.svg" width="100%"
       alt="Animation: a county flood-gauge page changes and then returns 404 at the same URL. EternalWeb names it by a BLAKE3 content address instead, which changes completely when one character changes and rejects edited bytes. An archivist's Ed25519 signature verifies for the address and fails on the edited page. Each version keeps its own address, so the March readings survive the removal. Peer replication is shown as the roadmap design, followed by reputation and curation scores and a list of what works today and what is next.">
  <br>
  <sub>Addresses, signatures and scores are computed by <code>eternalweb-core</code> (<code>cargo run -p eternalweb-core --release --example film</code>).
  <a href="https://tokeloshe.github.io/EternalWeb/">Open the interactive version</a> to pause, scrub and jump between chapters.</sub>
</p>

Inspired by IPFS, Hypercore, and emerging standards, EternalWeb turns every user into a node in a distributed network, ensuring data immortality against link rot, censorship, and profit-driven erasure. Licensed under AGPL-3.0 to keep it forever open and community-owned.

## Table of Contents
- [Features](#features)
- [Why EternalWeb?](#why-eternalweb)
- [Tech Stack](#tech-stack)
- [Installation](#installation)
- [Usage](#usage)
- [Contributing](#contributing)
- [Roadmap](#roadmap)
- [License](#license)
- [Contact](#contact)

## Features
- **Decentralized Archiving**: Snapshot webpages, files, or IoT data with content-addressable storage (CID hashing). Versioning tracks changes over time.
- **P2P Sharing and Replication**: Data replicates across nodes automatically, resilient to outages or takedowns.
- **Semantic Search and Discovery**: AI-powered, community-curated search with filters for ethical, non-corporate content.
- **Collaboration Collectives**: Form groups for themed archiving (e.g., climate data, cultural heritage) with E2EE for private shares.
- **Future-Proof Integrations**: Web 3.0 blockchain for governance and immutability; Web 4.0 AI for semantic tagging and autonomous curation.
- **Security Layers**: Post-quantum cryptography, zero-trust verification, and anti-abuse tools to resist hacks from state actors or corporations.
- **Transparency Focus**: Clearnet-first with public audits, reputation systems, and optional privacy overlays—promoting positive, verifiable content.
- **Incentives**: Earn badges and reputation for contributions, fostering a global community without speculative tokens.
- **Cross-Platform**: Desktop, mobile, CLI, and browser extension support.

## Why EternalWeb?
The internet is fragile: Corporations prioritize profits through ads and data mining, while censorship and data loss erode access. EternalWeb reclaims it by:
- **Empowering Users**: Anyone can archive and share without intermediaries.
- **Promoting Egalitarianism**: Optimized for low-bandwidth regions, bridging digital divides.
- **Future-Proofing**: Ready for Web 3.0/4.0 with modular upgrades for AI agents, IoT, and VR/AR.
- **Ethical Design**: Encourages positive use cases like journalism, education, and cultural preservation—avoiding darkweb associations through transparency.

Join us in building a people's internet: Immortal, accessible, and for all humankind.

## Tech Stack
- **Core**: Rust (performance, safety, WASM for browser compatibility).
- **Frontend**: React/TypeScript with Web3.js for wallet integrations.
- **Networking**: libp2p (QUIC/TCP), WebRTC for mesh.
- **Storage/Protocols**: IPFS for hashing, Hypercore for feeds.
- **Blockchain**: Substrate for DAOs and anchoring.
- **Cryptography**: Kyber/Dilithium (post-quantum), Signal Protocol.
- **AI**: Tract (ONNX) for local ML; extensible for decentralized models.
- **Other**: Tantivy for search, Tauri for apps.

## Installation
### Prerequisites
- Rust (stable toolchain: \`rustup install stable\`)
- Node.js (v18+ for frontend)
- Git

### Steps
1. Clone the repo:
\`\`\`bash
git clone https://github.com/Tokeloshe/EternalWeb.git
cd EternalWeb
\`\`\`

2. Build the core (Rust):
\`\`\`bash
cargo build --release
\`\`\`

3. Install frontend dependencies:
\`\`\`bash
cd frontend
npm install
\`\`\`

4. Run the node:
\`\`\`bash
cargo run --release --bin eternalweb node start
\`\`\`

For browser extension: Build via \`npm run build\` in \`/extension\`.

Binaries for Windows/Mac/Linux coming soon—check releases.

## Usage
### CLI Examples
- Archive a page: \`eternalweb archive https://example.com --public\`
- Search: \`eternalweb search "climate change data" --filter=ethical\`
- Join network: \`eternalweb node start\`
- Create collective: \`eternalweb collective create "Open Knowledge Group"\`

### Web App
Run \`npm run dev\` in \`/frontend\` and access at \`http://localhost:3000\`. Connect your Web3 wallet for enhanced features.

Detailed docs in \`/docs\` folder.

## Contributing
We welcome contributions to make EternalWeb even better! Follow these steps:
1. Fork the repo and create a branch (\`git checkout -b feature/awesome-feature\`).
2. Commit changes (\`git commit -m "Add awesome feature"\`).
3. Push (\`git push origin feature/awesome-feature\`).
4. Open a Pull Request—include tests and update docs.

See \`CONTRIBUTING.md\` for guidelines. Focus on ethical features; all PRs audited for security.

- **Issues**: Report bugs or suggest ideas [here](https://github.com/Tokeloshe/EternalWeb/issues).
- **Discussions**: Join on GitHub Discussions or our Discord (link coming soon).
- **Code of Conduct**: We pledge to a harassment-free community—see \`CODE_OF_CONDUCT.md\`.

## Roadmap
- **v0.1 (Q4 2025)**: MVP with basic archiving and P2P.
- **v1.0 (Q1 2026)**: Full features, Web 3.0 integrations.
- **v2.0 (Q3 2026)**: AI agents, Web 4.0 autonomy.
- **Long-Term**: IoT/VR support, global partnerships.

Track progress in Issues or Projects tab.

## License
This project is licensed under the GNU Affero General Public License v3.0—see the [LICENSE](LICENSE) file for details. This ensures EternalWeb remains free and open forever.

## Contact
- Creator: @e_honiball on X (Twitter)
- Community: Star the repo, spread the word, and let's change the world!

---

*Last updated: November 15, 2025*  
Built with ❤️ by the community for humankind. Fork the future!
