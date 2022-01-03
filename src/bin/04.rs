use md5::{Digest, Md5};
use rayon::prelude::*;

fn main() {
    let mine = |p: fn(hash: &[u8]) -> bool| (0..)
        .step_by(1000000)
        .find_map(|i| (i..i + 1000000).into_par_iter().find_first(|j| p(&Md5::digest(format!("yzbqklnj{}", j)))))
        .unwrap();

    println!("Part 1: {}", mine(|hash| hash[0..2] == [0; 2] && hash[2] & 0xf0 == 0));
    println!("Part 2: {}", mine(|hash| hash[0..3] == [0; 3]));
}
