⚡ EV OTA Firmware Updates with Merkle Trees
Rust PoC + Interactive Playground

Firmware updates in electric vehicles (EVs) are usually multi-GB downloads — even if only a few bytes change.
This project shows how Merkle trees make updates smarter:

✅ Download only the changed chunks (plus small proofs)

✅ Save up to 95% bandwidth

✅ Maintain strong cryptographic integrity

✨ Features

Rust PoC → end-to-end Merkle tree builder + proof verification

Interactive Playground (Python + Streamlit) → visualize updates in your browser

Security Notes → root of trust, signing, replay protection

🚀 Quick Start
1. Clone the Repo
git clone
cd ev-ota-merkle-poc

2. Run the Rust PoC

Requirements: Rust (edition 2021+)

cargo run


👉 What it does:

Creates a demo 16 MB firmware file

Splits into 4 MB slices

Builds a Merkle tree + root

Lets you modify slices, generate Merkle proofs, and verify them interactively

Example CLI session:

=== EV OTA Merkle Simulation ===
1: Show Merkle root
2: Modify a chunk
3: Generate & verify proof for a chunk
4: Exit

3. Try the Interactive Playground (Python + Streamlit)

Requirements: Python 3.9+

Install dependencies:

pip install streamlit


Run the app:

streamlit run playground/app.py


👉 What it does:

Visualizes firmware as chunks

Lets you flip bytes to simulate updates

Shows the Merkle root, changed chunk, proof, and verification result

📖 Background

Merkle Trees = cryptographic structures that let you prove data integrity efficiently.

Why EVs need it:

Multi-GB updates waste bandwidth + cost

Incremental updates = only changed chunks + proofs

Strong cryptography = EVs trust signed roots from the OEM

🔐 Security Checklist

✅ OEM signs Merkle root || version || timestamp

✅ EV verifies signature with pre-installed public key

✅ Prevent replay/stale updates with version checks

✅ Encrypt firmware chunks in transit (integrity ≠ confidentiality)

✅ Consider Sparse Merkle Trees or Verkle Trees for very large firmware sets

📚 References

📚 References & Further Reading

## 📚 References & Further Reading  

- **[Bitcoin Whitepaper (Merkle Trees)](https://bitcoin.org/bitcoin.pdf)** – *Satoshi Nakamoto, 2008*  
  Introduced Merkle trees for efficient verification in Bitcoin. Same principle applies to verifying firmware chunks in EV updates.  

- **[RFC 6962: Certificate Transparency](https://datatracker.ietf.org/doc/html/rfc6962)** – *Laurie, Langley, and Kasper, IETF*  
  Defines a log structure using Merkle trees to ensure tamper-evident transparency at scale.  

- **[IPFS Merkle DAGs (Directed Acyclic Graphs)](https://ipfs.io)** – *Protocol Labs*  
  IPFS organizes data into Merkle DAGs, enabling deduplication, integrity verification, and efficient distribution — useful for EV firmware.  

- **[`ed25519-dalek` crate](https://crates.io/crates/ed25519-dalek)** – Rust implementation of Ed25519 signatures, ideal for signing Merkle roots and verifying authenticity on embedded devices.  


🤝 Contributing

PRs welcome! You can try modifying:

Chunk size

Hash algorithm (e.g. SHA-256 → BLAKE2)

Adding signature verification

📜 License

MIT License — free to use, modify, and share.

🔥 That’s it!
Clone → cargo run → streamlit run app.py
See how EVs can cut update downloads by up to 95%.
