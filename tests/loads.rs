use gameboy_dot_rs::memory::MemoryMapped;
use gameboy_dot_rs::system::Gas;

mod common;

#[test]
fn test_loads() {
    let mut system = common::load_test_system("test_roms/roms/loads.gb");

    system.run_with_gas(Gas::LIMITED(20));

    assert_eq!(0x1A, system.cpu().a);
    assert_eq!(0x2B, system.cpu().b);
    assert_eq!(0x3C, system.cpu().c);
    assert_eq!(0x4D, system.cpu().d);
    assert_eq!(0x5E, system.cpu().e);
    assert_eq!(0x6F, system.cpu().h);
    assert_eq!(0x7F, system.cpu().l);
}

#[test]
fn test_load_memory() {
    let mut system = common::load_test_system("test_roms/roms/load_memory.gb");

    system.run_with_gas(Gas::LIMITED(20));

    assert_eq!(123, system.bus().ram.read_byte(0));
    assert_eq!(0, system.bus().ram.read_byte(1));
    assert_eq!(123, system.cpu().a);
    assert_eq!(0, system.cpu().b);
    assert_eq!(123, system.cpu().e);
}
