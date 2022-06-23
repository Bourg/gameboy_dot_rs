use std::fs;
use std::io::Read;
use std::path::PathBuf;
use gameboy_dot_rs::cartridge::cartridge_type::CartridgeType;
use gameboy_dot_rs::cartridge::header::{Checksum, Header, Validation};
use gameboy_dot_rs::cartridge::mbc1::Mbc1;
use gameboy_dot_rs::cartridge::parse::Parse;
use gameboy_dot_rs::system::{Gas, System};

#[test]
fn test_jumps() {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("test_roms/roms/jumps.gb");

    let mut file = fs::File::open(d).unwrap();
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes).unwrap();
    let cartridge = Mbc1::from_bytes(&bytes).unwrap();
    let mut system = System::load_cartridge(cartridge);

    system.run_with_gas(Gas::LIMITED(10));

    assert_eq!(0x150, system.cpu().pc());
}