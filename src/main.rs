use merkletree::merkle::MerkleTree;
use merkletree::hash::{Algorithm, Hashable};
use sha2::Sha256;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::iter::Map;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let chunks = get_chunks(&args[1]);
    let t: MerkleTree<[u8; 32], ExampleAlgorithm> = MerkleTree::from_iter(chunks);
}

fn get_chunks(filename: &str) -> Vec<Vec<u8>> {
    let bytes = std::fs::read(filename).unwrap();
    let mut chunks: Vec<Vec<u8>> = vec![];
    let mut i = 0;
    while i < bytes.len() {
        let x = std::cmp::min(bytes.len(), 256000 + i);
        let mut chunk: Vec<u8> = vec![0; 256000];
        chunk[..].clone_from_slice(&bytes[i..x]);
        chunks.push(chunk);
        i = x;
    }
    chunks 
}