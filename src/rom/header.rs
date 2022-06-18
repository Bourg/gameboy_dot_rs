use super::cartridge_type::CartridgeType;
use crate::rom::parse::{Parse, ParseResult};
use std::ops::RangeInclusive;

const HEADER_BYTES: usize = 0x50;

const TITLE_ADDRESS_RANGE: RangeInclusive<usize> = 0x0034..=0x0043;
const CARTRIDGE_TYPE_ADDRESS: usize = 0x0047;

#[derive(Debug)]
pub struct Header {
    title: String,
    cartridge_type: CartridgeType,
}

impl Parse<&[u8]> for Header {
    /// Parse the header from its raw bytes.
    /// The header appears starting at 0x100 in a cartridge's memory.
    /// The slice given here should be just the header, so roughly &rom[100..150]
    fn parse(header: &[u8]) -> ParseResult<Header> {
        Header::precondition_len(header)?;

        // 0x0100..=0x0103 - Entrypoint, typically contains jump instruction to 0x0150
        // 0x0104..=0x0133 - Nintendo Logo TODO need to verify the presence of the logo bytes
        let title = Header::parse_title(header)?;
        // 0x0143 - CGB Flag TODO IMPORTANT check this flag, unclear what the possible values are
        // 0x0144..=0x0145 - New Licensee Code
        // 0x0146 - SGB Flag
        // 0x0147 - Cartridge Type TODO IMPORTANT
        let cartridge_type = CartridgeType::parse(header[CARTRIDGE_TYPE_ADDRESS])?;
        // 0x0148 - ROM Size TODO IMPORTANT
        // 0x0149 - RAM Size TODO IMPORTANT
        // 0x014A - Destination Code
        // 0x014B - Licensee Code
        // 0x014C - Mask ROM Version Number
        // 0x014D - Header Checksum TODO important verify the checksum
        // 0x014E..=0x014F - Global Checksum
        // TODO the rest of the header

        Ok(Header {
            title,
            cartridge_type,
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

    fn parse_title(header: &[u8]) -> ParseResult<String> {
        let title_bytes = &header[TITLE_ADDRESS_RANGE];
        match std::str::from_utf8(title_bytes) {
            Ok(title_str) => Ok(title_str.trim_matches('\0').trim().to_string()),
            Err(e) => Err(e.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handcrafted_header() {
        let mut header = [0; HEADER_BYTES];

        let title = "Pokemon Red\0\0\0\0\0";
        assert_eq!(16, title.len());

        &header[TITLE_ADDRESS_RANGE].copy_from_slice(title.as_bytes());

        let header = Header::parse(&header[..]).unwrap();

        assert_eq!("Pokemon Red", header.title);
    }
}
