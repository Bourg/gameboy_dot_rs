use std::fs;
use std::io::Read;
use std::path::PathBuf;
use gameboy_dot_rs::cartridge::cartridge_type::CartridgeType;
use gameboy_dot_rs::cartridge::header::{Checksum, Header, Validation};
use gameboy_dot_rs::cartridge::parse::Parse;

#[test]
fn test_pokemon_red_header() {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("test_resources/rom/pokemon_red_header.gb");

    let mut file = fs::File::open(d).unwrap();
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes).unwrap();

    let header = Header::parse(&bytes[..]).unwrap();
    assert_eq!(Header {
        title: "POKEMON RED".to_string(),
        cartridge_type: CartridgeType::Mbc3 {
            battery: true,
            ram: true,
            timer: false
        },
        rom_banks: 64,
        ram_banks: 4,
        version: 0,
        validation: Validation {
            logo: true,
            header_checksum: Checksum::Ok(0x20),
            global_checksum: 0xE691,
        }
    }, header)
}