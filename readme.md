⚡ EV OTA Firmware Updates with Merkle Trees

Rust PoC + Interactive Playground

Firmware updates in electric vehicles (EVs) are usually huge multi-GB downloads — even if only a few bytes change.
This project shows how Merkle trees can make updates smarter: download only the changed parts (chunks) + small proofs, saving up to 95% bandwidth.

We include:

✅ Rust PoC (end-to-end Merkle tree builder + proof verification)

✅ Interactive Python Playground (Streamlit) to play with slices in your browser

✅ Security notes (root of trust, signing, replay protection)

🚀 Quick Start
1. Clone the repo
git clone https://github.com/yourusername/ev-ota-merkle-poc.git
cd ev-ota-merkle-poc

2. Run the Rust PoC

Requirements: Rust (edition 2021+)

cargo run


👉 What it does:

Creates a demo 16 MB “firmware” file

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

Merkle trees = cryptographic structures that let you prove data integrity efficiently

Why EVs need it:

Multi-GB updates waste bandwidth + cost

Incremental updates = only changed chunks + proofs

Strong cryptography = EVs only trust signed roots from the OEM

🔐 Security Checklist

OEM signs Merkle root || version || timestamp

EV verifies signature with pre-installed public key

Prevent replay/stale updates with version checks

Encrypt firmware chunks in transit (integrity ≠ confidentiality)

Consider Sparse Merkle Trees or Verkle Trees for very large firmware sets

📚 References

Bitcoin Whitepaper (Merkle Trees)

RFC 6962: Certificate Transparency

IPFS Merkle DAGs

ed25519-dalek crate

🤝 Contributing

PRs welcome! Try modifying:

Chunk size

Hash algorithm (SHA-256 → BLAKE2)

Adding signature verification

📜 License

MIT — free to use, modify, and share.

🔥 That’s it. Clone → cargo run → streamlit run app.py. You’ll see how EVs can cut update downloads by up to 95%.
