use gameboy_dot_rs::memory::MemoryMapped;
use gameboy_dot_rs::ram::Ram;

#[test]
fn test_ram_in_bounds() {
    let mut ram: Ram<0x10> = Ram::new();

    assert_eq!(0, ram.read_byte(0x0));
    assert_eq!(0, ram.read_byte(0xA));
    assert_eq!(0, ram.read_byte(0xF));

    ram.write_byte(0xA, 0xDE);
    assert_eq!(0xDE, ram.read_byte(0xA));
    assert_eq!(0x0, ram.read_byte(0x9));
    assert_eq!(0x0, ram.read_byte(0xB));
}