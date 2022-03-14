use std::fs::{File, OpenOptions, read_to_string};
use std::io::{Write, Error, BufReader, BufRead};
use sha1::{Sha1, Digest};

pub struct Block {
    pub(crate) epoch: String,
    pub(crate) index: String,
    pub(crate) writer: String,
    pub(crate) previous: String,
    pub(crate) transactions: String,
}

pub fn generate_block() -> Block {
    let epoch = read_to_string("blockchain/blockchain.dat").unwrap();

    let path = &*format!(
        "blockchain/{}.dat",
        epoch
    );
    let blockchain = BufReader::new(File::open(path).unwrap());
    let mut lines: Vec<_> = blockchain.lines().map(|line| {line.unwrap()}).collect();
    lines.reverse();
    let str = lines[0].split("-");
    let vec = str.collect::<Vec<&str>>();
    let last_block = &vec[1];
    let index = (last_block.parse::<u32>().unwrap() + 1).to_string();

    let writer = read_to_string("blockchain/config.dat").unwrap();

    let mut previous = epoch.clone();
    previous.push_str("-");
    previous.push_str(last_block);
    previous.push_str("-");
    previous.push_str(&*writer);

    let mut hasher = Sha1::new();
    hasher.update(previous.as_bytes());
    let generic_previous = hasher.finalize();
    let previous = format!(
        "{:x}",
        generic_previous
    );

    let mempool = BufReader::new(File::open("blockchain/mempool.dat").unwrap());
    let mut lines: Vec<_> = mempool.lines().map(|line| {line.unwrap()}).collect();
    let mut transactions = "[".to_string();

    for line in lines {
        transactions.push_str(&*line);
        transactions.push_str(",");
    }

    transactions.push_str("]");

    let mut block = Block{
        epoch,
        index,
        writer,
        previous,
        transactions
    };

    return block;
}

pub fn write_block(block: Block) -> Result<(), Error> {
    let mut new_block = "".to_string();
    new_block.push_str(&*block.epoch);
    new_block.push_str("-");
    new_block.push_str(&*block.index);
    new_block.push_str("-");
    new_block.push_str(&*block.writer);
    new_block.push_str("-");
    new_block.push_str(&*block.previous);
    new_block.push_str("-");
    new_block.push_str(&*block.transactions);

    let path = &*format!(
        "blockchain/{}.dat",
        block.epoch
    );

    let mut file: File = OpenOptions::new()
        .write(true)
        .append(true)
        .open(path)
        .unwrap();

    writeln!(file, "{}", new_block).unwrap();

    Ok(())
}
