use merkletree;
use ring::digest::{Context, Digest, SHA256};
use std::env;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::iter::Map;

fn main() {
    let args: Vec<String> = env::args().collect();
    let chunks = get_chunks(&args[1]);
    let tree = merkletree::from_byte
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