# EternalWeb Architecture

## Overview

EternalWeb is a decentralized web archiving platform built on a peer-to-peer architecture, combining IPFS for content addressing, Hypercore for versioning, and libp2p for networking. It's designed for Web 3.0 (decentralization) and Web 4.0 (AI autonomy).

## Core Components

### 1. Archiving Engine (`eternalweb-core/src/archiving/`)

Responsible for capturing, storing, and versioning web content.

**Key Features:**
- Content-addressable storage using IPFS CIDs
- Hypercore feeds for versioning
- Semantic metadata extraction (Schema.org, Open Graph, Dublin Core)
- Support for offline archiving

**Data Flow:**
```
URL → Fetch → Parse → Extract Metadata → Generate CID → Store in IPFS → Create Feed Entry
```

### 2. Networking Module (`eternalweb-core/src/networking/`)

Handles P2P communication and peer discovery.

**Protocols:**
- **libp2p**: Core networking stack
  - QUIC/TCP for transport
  - Noise for encryption
  - Yamux for multiplexing
  - Kademlia DHT for peer discovery
  - GossipSub for content propagation
  - mDNS for local discovery

**Discovery Methods:**
- DHT-based topic discovery
- ENS (Ethereum Name Service) for Web3 integration
- Bootstrap nodes for initial connection

### 3. Security Module (`eternalweb-core/src/security/`)

Provides cryptographic primitives and trust management.

**Cryptography:**
- Ed25519 for current operations
- Kyber (post-quantum) for key exchange
- Dilithium (post-quantum) for signatures
- ChaCha20-Poly1305 for encryption
- zk-SNARKs for privacy proofs

**Trust System:**
- Peer reputation scoring
- DAO-governed trust rules
- Anti-Sybil mechanisms

### 4. Search Engine (`eternalweb-core/src/search/`)

Enables discovery of archived content.

**Features:**
- Full-text search with Tantivy
- Semantic search with AI embeddings
- Community curation via voting
- Ethical content filtering

### 5. Blockchain Module (`eternalweb-core/src/blockchain/`)

Provides governance and immutability.

**Components:**
- Substrate-based chain for governance
- DAO for proposal voting
- Content anchoring (hash-on-chain)
- Non-speculative reputation tokens

### 6. AI Module (`eternalweb-core/src/ai/`)

Enables intelligent features.

**Capabilities:**
- Semantic tagging with local ML (Tract/ONNX)
- Trend prediction for proactive archiving
- Autonomous curation (Web 4.0)
- NLP for keyword extraction

### 7. Collaboration (`eternalweb-core/src/collaboration/`)

Facilitates group archiving.

**Features:**
- Collective creation and management
- Real-time sync over libp2p
- License compliance checking
- E2EE for private shares

### 8. Incentives (`eternalweb-core/src/incentives/`)

Rewards contributions.

**System:**
- On-chain badges for achievements
- Reputation scoring
- Non-speculative token model
- Community recognition

## Data Model

### Content Identifier (CID)
```rust
CID = multihash(content)
// CIDv1 with BLAKE3-256 hash
```

### Snapshot
```rust
struct Snapshot {
    cid: Cid,
    url: String,
    timestamp: DateTime<Utc>,
    metadata: SnapshotMetadata,
    mime_type: String,
    size: u64,
}
```

### HyperFeed
```rust
struct HyperFeed {
    id: String,
    public_key: Vec<u8>,
    sequence: u64,
    entries: Vec<FeedEntry>,
}
```

## Network Topology

```
┌─────────────┐
│  Bootstrap  │
│    Nodes    │
└──────┬──────┘
       │
       ├──────────┬──────────┬──────────┐
       │          │          │          │
   ┌───▼───┐  ┌───▼───┐  ┌───▼───┐  ┌───▼───┐
   │ Peer  │  │ Peer  │  │ Peer  │  │ Peer  │
   │   A   │◄─┤   B   │◄─┤   C   │◄─┤   D   │
   └───┬───┘  └───┬───┘  └───┬───┘  └───────┘
       │          │          │
       └──────────┴──────────┘
         (Content Routing)
```

**Characteristics:**
- Decentralized (no single point of failure)
- Self-healing (automatic re-routing)
- Scalable (DHT-based discovery)

## Storage Architecture

### Local Storage
- Sled embedded database for metadata
- IPFS-compatible local store for content
- Sharded for scalability

### Distributed Storage
- Content replicated across N peers
- Configurable replication factor
- Erasure coding for efficiency (future)

## Security Model

### Threat Model
- Assumes: APTs, state actors, AI-driven attacks
- Defends against: DDoS, Sybil, MITM, quantum attacks

### Defense Layers
1. **Network**: Rate limiting, PoW challenges, encryption
2. **Application**: Input validation, sandboxing, least privilege
3. **Data**: E2EE, integrity checks, blockchain anchoring
4. **Trust**: Reputation, DAO governance, community moderation

## Web 3.0 Integration

- **Blockchain**: Substrate for governance
- **Smart Contracts**: ink! for DAOs
- **Wallets**: WalletConnect for user identity
- **Tokens**: Non-speculative reputation tokens
- **ENS**: Decentralized naming

## Web 4.0 Readiness

- **AI Agents**: Autonomous curation and archiving
- **Semantic Web**: RDF metadata for interoperability
- **IoT Support**: Stream archiving
- **VR/AR**: Metadata extensions
- **Predictive**: Trend-based proactive archiving

## Scalability

### Current (v0.1)
- Target: 1,000 nodes
- Storage: 100GB per node
- Bandwidth: Optimized for low-bandwidth regions

### Future (v1.0+)
- Target: 100,000+ nodes
- Sharding for horizontal scaling
- CDN-like edge caching
- AI-driven load balancing

## Deployment

### Node Types
1. **Full Node**: Stores and serves content
2. **Light Node**: Queries only (mobile/browser)
3. **Archive Node**: High-capacity storage
4. **Bootstrap Node**: Network entry points

### Recommended Specs (Full Node)
- **CPU**: 2+ cores
- **RAM**: 4GB+
- **Storage**: 100GB+
- **Network**: 1Mbps+ (optimized for low-bandwidth)

## Future Extensions

- Video/audio archiving
- Real-time collaborative editing
- Decentralized computation (WASM plugins)
- Cross-chain bridges
- Federated search integration

## References

- [IPFS Whitepaper](https://ipfs.io/ipfs/QmR7GSQM93Cx5eAg6a6yRzNde1FQv7uL6X1o4k7zrJa3LX/ipfs.draft3.pdf)
- [libp2p Documentation](https://docs.libp2p.io/)
- [Substrate Documentation](https://docs.substrate.io/)
- [Web 3.0 Primer](https://en.wikipedia.org/wiki/Web3)
