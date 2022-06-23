use std::fs;
use std::io::Read;
use std::path::PathBuf;
use gameboy_dot_rs::cartridge::mbc1::Mbc1;
use gameboy_dot_rs::system::System;

pub fn load_test_rom_bytes(path: &str) -> Vec<u8> {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push(path);

    let mut file = fs::File::open(d).unwrap();
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes).unwrap();

    bytes
}

pub fn load_test_system(path: &str) -> System {
    let bytes = load_test_rom_bytes(path);

    let cartridge = Mbc1::from_bytes(&bytes).unwrap();

    System::load_cartridge(cartridge)
}
