use crate::rom::parse::{Parse, ParseResult};

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum CartridgeType {
    Rom {
        battery: bool,
        ram: bool,
    },
    Mbc1 {
        battery: bool,
        ram: bool,
    },
    Mbc2 {
        battery: bool,
    },
    Mbc3 {
        battery: bool,
        ram: bool,
        timer: bool,
    },
    Mbc5 {
        battery: bool,
        ram: bool,
        rumble: bool,
    },
    Mbc6,
    Mbc7,
    HuC1,
    HuC3,
    Mmm01 {
        battery: bool,
        ram: bool,
    },
    BandaiTama5,
    PocketCamera,
}

impl Parse for CartridgeType {
    fn parse(code: u8) -> ParseResult<Self> {
        match code {
            0x00 => Ok(CartridgeType::Rom {
                battery: false,
                ram: false,
            }),
            0x01 => Ok(CartridgeType::Mbc1 {
                battery: false,
                ram: false,
            }),
            0x02 => Ok(CartridgeType::Mbc1 {
                battery: false,
                ram: true,
            }),
            0x03 => Ok(CartridgeType::Mbc1 {
                battery: true,
                ram: true,
            }),
            0x05 => Ok(CartridgeType::Mbc2 { battery: false }),
            0x06 => Ok(CartridgeType::Mbc2 { battery: true }),
            0x08 => Ok(CartridgeType::Rom {
                battery: false,
                ram: true,
            }),
            0x09 => Ok(CartridgeType::Rom {
                battery: true,
                ram: true,
            }),
            0x0B => Ok(CartridgeType::Mmm01 {
                battery: false,
                ram: false,
            }),
            0x0C => Ok(CartridgeType::Mmm01 {
                battery: false,
                ram: true,
            }),
            0x0D => Ok(CartridgeType::Mmm01 {
                battery: true,
                ram: true,
            }),
            0x0F => Ok(CartridgeType::Mbc3 {
                battery: true,
                timer: true,
                ram: false,
            }),
            0x10 => Ok(CartridgeType::Mbc3 {
                battery: true,
                timer: true,
                ram: true,
            }),
            0x11 => Ok(CartridgeType::Mbc3 {
                battery: false,
                ram: false,
                timer: false,
            }),
            0x12 => Ok(CartridgeType::Mbc3 {
                battery: false,
                ram: true,
                timer: false,
            }),
            0x13 => Ok(CartridgeType::Mbc3 {
                battery: true,
                ram: true,
                timer: false,
            }),
            0x19 => Ok(CartridgeType::Mbc5 {
                battery: false,
                ram: false,
                rumble: false
            }),
            0x1A => Ok(CartridgeType::Mbc5 {
                battery: false,
                ram: true,
                rumble: false
            }),
            0x1B => Ok(CartridgeType::Mbc5 {
                battery: true,
                ram: true,
                rumble: false
            }),
            0x1C => Ok(CartridgeType::Mbc5 {
                battery: false,
                ram: false,
                rumble: true
            }),
            0x1D => Ok(CartridgeType::Mbc5 {
                battery: false,
                ram: true,
                rumble: true
            }),
            0x1E => Ok(CartridgeType::Mbc5 {
                battery: true,
                ram: true,
                rumble: true
            }),
            0x20 => Ok(CartridgeType::Mbc6),
            0x22 => Ok(CartridgeType::Mbc7),
            0xFC => Ok(CartridgeType::PocketCamera),
            0xFD => Ok(CartridgeType::BandaiTama5),
            0xFE => Ok(CartridgeType::HuC3),
            0xFF => Ok(CartridgeType::HuC1),
            _ => Err(format!(
                "unrecognized cartridge type code {:#04X}",
                code
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use super::*;

    macro_rules! test_parse {
        ($test_name: ident, $code: expr, $expected: expr) => {
            #[test]
            fn $test_name() {
                let actual = CartridgeType::parse($code).unwrap();
                assert_eq!($expected, actual);
            }
        }
    }

    // Some extra carefulness with Mbc1 since that's what is actually supported to play
    test_parse!(test_mbc1_basic, 0x01, CartridgeType::Mbc1{ battery: false, ram: false});
    test_parse!(test_mbc1_ram, 0x02, CartridgeType::Mbc1{ battery: false, ram: true});
    test_parse!(test_mbc1_battery_ram, 0x03, CartridgeType::Mbc1{ battery: true, ram: true});

    #[test]
    fn test_uniqueness() {
        // As a crude test to make sure there aren't any copy paste errors,
        // check that each entry is distinct from the rest

        let mut set = HashSet::new();

        for code in 0x00..=0xFF {
            if let Ok(cartridge_type) = CartridgeType::parse(code) {
                // HashSet::insert returns true if the insert was unique
                assert_eq!(true, set.insert(cartridge_type));
            }
        }

        // There are 28 total distinct types
        assert_eq!(28, set.len());
    }
}