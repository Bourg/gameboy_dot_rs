use super::cartridge_type::CartridgeType;
use crate::rom::constants;
use crate::rom::parse::{Parse, ParseResult};
use std::ops::RangeInclusive;

const HEADER_BYTES: usize = 0x50;

const LOGO_ADDRESS_RANGE: RangeInclusive<usize> = 0x0004..=0x0033;
const TITLE_ADDRESS_RANGE: RangeInclusive<usize> = 0x0034..=0x0043;
const CARTRIDGE_TYPE_ADDRESS: usize = 0x0047;
const ROM_BANKS_ADDRESS: usize = 0x0048;
const RAM_BANKS_ADDRESS: usize = 0x0049;
const VERSION_ADDRESS: usize = 0x004C;
const HEADER_CHECKSUM_ADDRESS: usize = 0x004D;
const GLOBAL_CHECKSUM_ADDRESS_RANGE: RangeInclusive<usize> = 0x004E..=0x004F;

#[derive(Debug, Eq, PartialEq)]
pub struct Header {
    title: String,
    cartridge_type: CartridgeType,
    rom_banks: usize,
    ram_banks: usize,
    version: u8,

    validation: Validation,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Validation {
    logo: bool,
    header_checksum: Checksum<u8>,
    global_checksum: u16,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Checksum<T> {
    Ok(T),
    Err { actual: T, expected: T },
}

impl Parse<&[u8]> for Header {
    /// Parse the header from its raw bytes.
    /// The header appears starting at 0x100 in a cartridge's memory.
    /// The slice given here should be just the header, so roughly &rom[100..150]
    fn parse(header: &[u8]) -> ParseResult<Header> {
        Header::precondition_len(header)?;

        let logo_valid = Header::check_logo_valid(&header[LOGO_ADDRESS_RANGE]);
        let title = Header::parse_title(&header[TITLE_ADDRESS_RANGE])?;
        // 0x0143 - CGB Flag TODO IMPORTANT check this flag, unclear what the possible values are
        // 0x0144..=0x0145 - New Licensee Code
        // 0x0146 - SGB Flag
        let cartridge_type = CartridgeType::parse(header[CARTRIDGE_TYPE_ADDRESS])?;
        let rom_banks = Header::parse_rom_banks(header[ROM_BANKS_ADDRESS])?;
        let ram_banks: usize = Header::parse_ram_banks(header[RAM_BANKS_ADDRESS])?;

        // 0x014A - Destination Code
        // 0x014B - Licensee Code
        let version = header[VERSION_ADDRESS];
        let header_checksum = Header::check_header_checksum(header);
        let global_checksum = u16::from_le_bytes([
            header[*GLOBAL_CHECKSUM_ADDRESS_RANGE.start()],
            header[*GLOBAL_CHECKSUM_ADDRESS_RANGE.end()],
        ]);

        Ok(Header {
            title,
            cartridge_type,
            rom_banks,
            ram_banks,
            version,

            validation: Validation {
                logo: logo_valid,
                header_checksum,
                global_checksum,
            },
        })
    }
}

impl Header {
    fn precondition_len(header: &[u8]) -> ParseResult<()> {
        if header.len() < HEADER_BYTES {
            return Err(format!(
                "Provided header data was {} bytes, but header must be {} bytes",
                header.len(),
                HEADER_BYTES
            )
            .to_string());
        }

        Ok(())
    }

    fn check_logo_valid(logo_bytes: &[u8]) -> bool {
        logo_bytes == constants::LOGO
    }

    fn parse_title(title_bytes: &[u8]) -> ParseResult<String> {
        match std::str::from_utf8(title_bytes) {
            Ok(title_str) => Ok(title_str.trim_matches('\0').trim().to_string()),
            Err(e) => Err(e.to_string()),
        }
    }

    // Rom size is defined as number of banks
    fn parse_rom_banks(code: u8) -> ParseResult<usize> {
        // Only codes in 0x00..=0x08 are defined
        if code <= 0x08 {
            Ok(2 << code)
        } else {
            Err(format!("invalid rom banks code {:#04X}", code))
        }
    }

    fn parse_ram_banks(code: u8) -> ParseResult<usize> {
        match code {
            0x00 => Ok(0),
            0x02 => Ok(1),
            0x03 => Ok(4),
            0x04 => Ok(16),
            0x05 => Ok(8),
            _ => Err(format!("invalid ram banks code {:#04X}", code)),
        }
    }

    fn check_header_checksum(header: &[u8]) -> Checksum<u8> {
        let expected_checksum = header[HEADER_CHECKSUM_ADDRESS];

        let mut actual_checksum: u8 = 0;
        for byte in &header[*TITLE_ADDRESS_RANGE.start()..=VERSION_ADDRESS] {
            actual_checksum = actual_checksum.wrapping_sub(*byte).wrapping_sub(1);
        }

        if expected_checksum == actual_checksum {
            Checksum::Ok(actual_checksum)
        } else {
            Checksum::Err {
                actual: actual_checksum,
                expected: expected_checksum,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::cartridge_type::CartridgeType;
    use super::*;
    use crate::rom::header::Checksum::Err;

    #[test]
    fn test_handcrafted_header() {
        let mut header = [0; HEADER_BYTES];

        let title = "POKEMON RED\0\0\0\0\0";
        assert_eq!(16, title.len());

        &header[TITLE_ADDRESS_RANGE].copy_from_slice(title.as_bytes());
        header[CARTRIDGE_TYPE_ADDRESS] = 0x13;
        header[ROM_BANKS_ADDRESS] = 0x05;
        header[RAM_BANKS_ADDRESS] = 0x03;

        let header = Header::parse(&header[..]).unwrap();

        assert_eq!(
            Header {
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
                    logo: false,
                    header_checksum: Err {
                        actual: 184,
                        expected: 0
                    },
                    global_checksum: 0
                },
            },
            header
        );
    }

    #[test]
    fn test_rom_banks() {
        let banks = Header::parse_rom_banks(0x00).unwrap();
        assert_eq!(2, banks);

        let banks = Header::parse_rom_banks(0x08).unwrap();
        assert_eq!(512, banks);

        let banks = Header::parse_rom_banks(0x09);
        assert!(banks.is_err());

        let banks = Header::parse_rom_banks(0xFF);
        assert!(banks.is_err());
    }

    #[test]
    fn test_ram_banks() {
        assert_eq!(0, Header::parse_ram_banks(0x00).unwrap());
        assert!(Header::parse_ram_banks(0x01).is_err());
        assert_eq!(8, Header::parse_ram_banks(0x05).unwrap());
        assert!(Header::parse_ram_banks(0x06).is_err());
    }
}
