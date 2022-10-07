use std::fmt::{self, Display};

use crate::{DecoError, DecoResult};

const V1: u32 = u32::from_be_bytes(*b":c:\x00");
const V2: u32 = u32::from_be_bytes(*b"\x00pxa");

/// Cartridge encoding verion.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Version {
    V0,
    V1,
    V2,
}

/// Represents spritesheet, map, flags, music and sound effects data.
#[derive(Default, Debug)]
pub struct Gfx {}

/// Holds pico8 lua code.
#[derive(Default, Debug, PartialEq, Eq)]
pub struct Lua {
    txt: String,
}

impl Display for Lua {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.txt)
    }
}

/// Holds parsed gfx data and lua code.
#[derive(Debug)]
pub struct Cartridge {
    pub gfx: Gfx,
    pub lua: Lua,
    version: Version,
    data_len: usize,
}

impl Cartridge {
    pub fn from_bytes(data: &[u8]) -> DecoResult<Self> {
        let version = match to_u32(&data[0x4300..=0x4303]) {
            V1 => Version::V1,
            V2 => Version::V2,
            _ => Version::V0,
        };

        // let decompressed_len = to_u16(&data[0x4304..=0x4305]);
        // let compressed_len = to_u16(&data[0x4306..=0x4307]);

        let lua = match version {
            Version::V0 => get_v0_lua(&data[0x4300..0x7fff])?,
            Version::V1 => todo!(),
            Version::V2 => todo!(),
        };

        Ok(Self {
            lua,
            version,
            gfx: Gfx {},
            data_len: data.len(),
        })
    }

    pub fn version(&self) -> Version {
        self.version.clone()
    }

    pub fn len(&self) -> usize {
        self.data_len
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Trims all zeros and returns a string with lua code.
fn get_v0_lua(data: &[u8]) -> DecoResult<Lua> {
    let first_0_idx = data.iter().position(|&u| u == 0).unwrap_or(data.len());
    let txt = std::str::from_utf8(&data[..first_0_idx])
        .map_err(|_| DecoError::Internal)?
        .to_string();

    Ok(Lua { txt })
}

fn to_u32(b: &[u8]) -> u32 {
    let mut _u32: [u8; 4] = Default::default();
    _u32.copy_from_slice(b);

    u32::from_be_bytes(_u32)
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn test_new_cartridge() {
        let mut data = vec![0u8; 0x7fff];
        // first byte is already set 0.
        data[0x4301] = b'p';
        data[0x4302] = b'x';
        data[0x4303] = b'a';

        // setting only the little endian part.
        data[0x4305] = 2u8;

        //let c = Cartridge::from_bytes(&data).unwrap();

        //assert_eq!(c.version(), Version::V2);
    }
}
