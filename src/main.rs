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
    let mut f = File::open(filename).expect("File not found!");
    let metadata = fs::metadata(&filename).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");

    let mut chunks: 
}