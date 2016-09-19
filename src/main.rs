mod nes;

#[macro_use] extern crate enum_primitive;
extern crate num;

use std::env;
use std::fs;
use std::io::Read;
use std::path::Path;

fn main() {
    let rom_filename = env::args().nth(1).unwrap();
    let rom_file = read_bin(rom_filename);

    let mut nes = nes::Nes::new(rom_file);
    nes.reset();

    loop {
        nes.step();
    }
}

fn read_bin<P: AsRef<Path>>(path: P) -> Box<[u8]> {
    let mut file = fs::File::open(path).unwrap();
    let mut file_buf = Vec::new();
    file.read_to_end(&mut file_buf).unwrap();
    file_buf.into_boxed_slice()
}

