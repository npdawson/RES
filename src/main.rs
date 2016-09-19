mod nes;

#[macro_use] extern crate enum_primitive;
extern crate num;

extern crate byteorder;

use std::env;
use std::fs;
use std::io::Read;
use std::path::Path;

fn main() {
    let rom_filename = env::args().nth(1).unwrap();

    let mut nes = nes::Nes::new(rom_filename);
    nes.reset();

    loop {
        nes.step();
    }
}
