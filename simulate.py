import hashlib
import random
import streamlit as st

# Constants
CHUNK_SIZE = 4  # Size of each chunk in bytes
NUM_CHUNKS = 8  # Number of chunks

# Simulate firmware as chunks
firmware = [
    [random.randint(0, 255) for _ in range(CHUNK_SIZE)] for _ in range(NUM_CHUNKS)
]

# Compute SHA-256 hash of each chunk
def hash_chunk(chunk):
    return hashlib.sha256(bytes(chunk)).hexdigest()

hashes = [hash_chunk(c) for c in firmware]

# Build a simple Merkle root (pairwise hashing)
def build_merkle(hashes):
    while len(hashes) > 1:
        if len(hashes) % 2:
            hashes.append(hashes[-1])  # duplicate last if odd
        new_level = []
        for i in range(0, len(hashes), 2):
            combined = hashes[i] + hashes[i + 1]
            new_level.append(hashlib.sha256(combined.encode()).hexdigest())
        hashes = new_level
    return hashes[0]

root = build_merkle(hashes)

# Streamlit UI
st.title("EV Firmware Update Simulator")

# Display firmware chunks and their hashes
st.subheader("Firmware Chunks and Hashes")
for i, (chunk, h) in enumerate(zip(firmware, hashes)):
    st.write(f"Chunk {i}: {chunk} -> Hash: {h}")

st.write(f"Merkle Root: {root}")

# Simulate a changed slice
changed_index = st.slider("Select Changed Chunk Index", 0, NUM_CHUNKS - 1, 2)
firmware[changed_index][0] ^= 0xFF  # flip first byte to simulate change
changed_hash = hash_chunk(firmware[changed_index])

st.write(f"Changed Chunk {changed_index} Hash: {changed_hash}")

# Build Merkle tree and generate proof for the changed chunk
def generate_proof(hashes, index):
    proof = []
    while len(hashes) > 1:
        if len(hashes) % 2:
            hashes.append(hashes[-1])  # duplicate last if odd
        new_level = []
        for i in range(0, len(hashes), 2):
            combined = hashes[i] + hashes[i + 1]
            new_level.append(hashlib.sha256(combined.encode()).hexdigest())
            if i == index or i + 1 == index:
                proof.append(hashes[i + 1] if i == index else hashes[i])
        hashes = new_level
    return proof

proof = generate_proof(hashes, changed_index)

st.subheader("Merkle Proof for Changed Chunk")
st.write(f"Proof: {proof}")

# Verify the proof
def verify_proof(leaf_hash, proof, root):
    cur = leaf_hash
    for sibling in proof:
        cur = hashlib.sha256((cur + sibling).encode()).hexdigest()
    return cur == root

is_valid = verify_proof(changed_hash, proof, root)
st.write(f"Proof Valid: {is_valid}")
