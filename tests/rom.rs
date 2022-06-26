use gameboy_dot_rs::cartridge::cartridge_type::CartridgeType;
use gameboy_dot_rs::cartridge::header::{Checksum, Header, Validation};
use gameboy_dot_rs::cartridge::parse::Parse;

mod common;

#[test]
fn test_pokemon_red_header() {
    let bytes = common::load_test_rom_bytes("test_roms/static/pokemon_red_header.gb");

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