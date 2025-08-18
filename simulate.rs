use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom, Write};
use anyhow::{Context, Result};
use hex::encode as hex_encode;

type Hash = [u8; 32];

fn sha256_bytes(data: &[u8]) -> Hash {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();
    let mut out = [0u8; 32];
    out.copy_from_slice(&result);
    out
}

fn hash_pair(left: &Hash, right: &Hash) -> Hash {
    let mut hasher = Sha256::new();
    hasher.update(left);
    hasher.update(right);
    let r = hasher.finalize();
    let mut out = [0u8; 32];
    out.copy_from_slice(&r);
    out
}

fn leaf_hashes_from_file(path: &str, chunk_size: usize) -> Result<Vec<Hash>> {
    let mut f = File::open(path).with_context(|| format!("open {}", path))?;
    let mut hashes = Vec::new();
    let mut buf = vec![0u8; chunk_size];
    loop {
        let n = f.read(&mut buf)?;
        if n == 0 { break; }
        hashes.push(sha256_bytes(&buf[..n]));
    }
    Ok(hashes)
}

fn build_merkle_root(mut nodes: Vec<Hash>) -> Hash {
    if nodes.is_empty() { return sha256_bytes(b""); }
    while nodes.len() > 1 {
        if nodes.len() % 2 == 1 {
            let last = *nodes.last().unwrap();
            nodes.push(last);
        }
        let mut next = Vec::with_capacity(nodes.len() / 2);
        for i in (0..nodes.len()).step_by(2) {
            let parent = hash_pair(&nodes[i], &nodes[i+1]);
            next.push(parent);
        }
        nodes = next;
    }
    nodes[0]
}

fn gen_proof(mut nodes: Vec<Hash>, mut index: usize) -> Vec<Hash> {
    let mut proof = Vec::new();
    while nodes.len() > 1 {
        if nodes.len() % 2 == 1 { nodes.push(*nodes.last().unwrap()); }
        let mut next = Vec::with_capacity(nodes.len() / 2);
        for i in (0..nodes.len()).step_by(2) {
            let left = nodes[i];
            let right = nodes[i+1];
            if i == index || i+1 == index {
                let sibling = if i == index { right } else { left };
                proof.push(sibling);
                index = next.len();
            }
            next.push(hash_pair(&left, &right));
        }
        nodes = next;
    }
    proof
}

fn verify_proof(leaf_hash: &Hash, index: usize, proof: &[Hash], root: &Hash) -> bool {
    let mut cur = *leaf_hash;
    let mut idx = index;
    for sibling in proof {
        if idx % 2 == 0 { cur = hash_pair(&cur, sibling); }
        else { cur = hash_pair(sibling, &cur); }
        idx /= 2;
    }
    &cur == root
}

fn interactive_modify_chunk(file_path: &str, chunk_index: usize, chunk_size: usize) -> Result<()> {
    let mut f = File::open(file_path)?;
    f.seek(SeekFrom::Start((chunk_index * chunk_size) as u64))?;
    let mut buf = vec![0u8; chunk_size];
    let n = f.read(&mut buf)?;
    if n == 0 { println!("Chunk {} empty!", chunk_index); return Ok(()); }

    // Modify first byte for demo
    buf[0] ^= 0xFF;
    let mut f = File::options().write(true).open(file_path)?;
    f.seek(SeekFrom::Start((chunk_index * chunk_size) as u64))?;
    f.write_all(&buf[..n])?;

    println!("Modified chunk {} (first byte flipped).", chunk_index);
    Ok(())
}

fn main() -> Result<()> {
    let chunk_size = 4 * 1024 * 1024; // 4 MB
    let demo_file = "demo_firmware.bin";

    // Create demo firmware if missing (16 MB)
    if !std::path::Path::new(demo_file).exists() {
        println!("Creating demo firmware file: {}", demo_file);
        let mut f = File::create(demo_file)?;
        for i in 0..(16 * 1024) {
            let mut buf = vec![0u8; 1024];
            for (j, b) in buf.iter_mut().enumerate() { *b = ((i+j) % 256) as u8; }
            f.write_all(&buf)?;
        }
    }

    loop {
        println!("\n=== EV OTA Merkle Simulation ===");
        println!("1: Show Merkle root");
        println!("2: Modify a chunk");
        println!("3: Generate & verify proof for a chunk");
        println!("4: Exit");
        println!("Enter choice: ");

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        match input.trim() {
            "1" => {
                let leaves = leaf_hashes_from_file(demo_file, chunk_size)?;
                let root = build_merkle_root(leaves);
                println!("Merkle root: {}", hex_encode(root));
            }
            "2" => {
                println!("Enter chunk index to modify:");
                let mut idx_str = String::new();
                io::stdin().read_line(&mut idx_str)?;
                let idx: usize = idx_str.trim().parse().unwrap_or(0);
                interactive_modify_chunk(demo_file, idx, chunk_size)?;
            }
            "3" => {
                println!("Enter chunk index to generate proof:");
                let mut idx_str = String::new();
                io::stdin().read_line(&mut idx_str)?;
                let idx: usize = idx_str.trim().parse().unwrap_or(0);
                let leaves = leaf_hashes_from_file(demo_file, chunk_size)?;
                let root = build_merkle_root(leaves.clone());
                let proof = gen_proof(leaves.clone(), idx);
                let leaf_hash = leaves[idx];
                println!("Proof for chunk {}:", idx);
                for (i, h) in proof.iter().enumerate() {
                    println!("{}: {}", i, hex_encode(h));
                }
                let ok = verify_proof(&leaf_hash, idx, &proof, &root);
                println!("Proof verification OK? {}", ok);
            }
            "4" => break,
            _ => println!("Invalid choice"),
        }
    }

    Ok(())
}
