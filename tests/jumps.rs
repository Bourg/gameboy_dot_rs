use gameboy_dot_rs::system::Gas;

mod common;

#[test]
fn test_jumps() {
    let mut system = common::load_test_system("test_roms/roms/jumps.gb");

    system.run_with_gas(Gas::LIMITED(10));

    assert_eq!(0x150, system.cpu().pc);
}