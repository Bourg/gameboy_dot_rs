use std::fs;
use std::io::Read;
use std::path::PathBuf;
use gameboy_dot_rs::cartridge::cartridge_type::CartridgeType;
use gameboy_dot_rs::cartridge::header::{Checksum, Header, Validation};
use gameboy_dot_rs::cartridge::mbc1::Mbc1;
use gameboy_dot_rs::cartridge::parse::Parse;
use gameboy_dot_rs::system::{Gas, System};

mod common;

#[test]
fn test_jumps() {
    let mut system = common::load_test_system("test_roms/roms/jumps.gb");

    system.run_with_gas(Gas::LIMITED(10));

    assert_eq!(0x150, system.cpu().pc());
}