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