use std::fs::OpenOptions;
use std::io::{Write};

pub fn new_txn(hash: &str) {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("blockchain/mempool.dat")
        .unwrap();

    writeln!(file, "{}", hash).expect("Cant write to mempool!");
}