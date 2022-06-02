use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let file = File::open("/Users/paulgrant/Dev/rosalind_rust/counting_nucleotides/rosalind_dna.txt")?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    // assert_eq!(contents, "Hello, world!");
    let mut count = [0; 4];
    for c in contents.chars() {
        match c {
            'A' => count[0] += 1,
            'C' => count[1] += 1,
            'G' => count[2] += 1,
            'T' => count[3] += 1,
            _ => (),
        }
    }
    println!("{} {} {} {}", count[0], count[1], count[2], count[3]);
    Ok(())
}