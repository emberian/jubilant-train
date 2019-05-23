use std::io::prelude::*;
use smeagol::Life;
fn main() {
    let mut v = Vec::new();
    let input = std::io::stdin().lock().read_to_end(&mut v);
    let l = Life::from_rle_file_contents(&v).unwrap();
    for pos in  l.get_alive_cells() {
        println!("{} {}", pos.x, pos.y);
    }
}
