extern crate merkletree;
use sha1::{Sha1, Digest};
use std::env;
use std::fs;
use std::fs::File;
use std::io::Read;

fn main() {
    let args: Vec<String> = env::args().collect();
    

}

fn get_chunks(filename: &str) -> Vec<Vec<u8>> {
    let bytes = std::fs::read(filename).unwrap();
    let mut chunks = vec![];
    let mut i = 0;
    while i < bytes.len() {
        let x = std::cmp::min(bytes.len(), 256000 + i);
        let mut chunk: Vec<u8> = vec![0; 256000];
        chunk[..].clone_from_slice(&bytes[i..x]);
        chunks.append(&mut chunk);
        i = x;
    }
    chunks 
}