use ase2::Aseprite;
use std::{env, fs};

fn main() {
    let Some(path_str) = env::args().nth(1) else {
        eprintln!("Usage: read <path_to_aseprite_file>");
        return;
    };
    let mut file = fs::File::open(path_str).unwrap();
    let ase = Aseprite::from_read(&mut file).unwrap();
    println!("{:#?}", ase);
}
