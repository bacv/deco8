use std::{
    fmt::{self, Display},
    str::from_utf8,
};

const V1: &[u8; 4] = b"\x00pxa";

enum Version {
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
#[derive(Default, Debug)]
pub struct Cartridge {
    pub gfx: Gfx,
    pub lua: Lua,
}

impl Cartridge {
    pub fn from_bytes(data: &[u8]) -> Self {
        let version = if &data[0x4300..=0x4303] == V1 {
            Version::V2
        } else {
            Version::V1
        };

        let mut decompressed_len: [u8; 2] = Default::default();
        decompressed_len.copy_from_slice(&data[0x4304..=0x4305]);
        let decompressed_len = u16::from_be_bytes(decompressed_len);

        let mut compressed_len: [u8; 2] = Default::default();
        compressed_len.copy_from_slice(&data[0x4306..=0x4307]);
        let compressed_len = u16::from_be_bytes(compressed_len);

        let compressed_lua = &data[0x4306..compressed_len as usize];

        todo!();
        //  let lua = from_utf8(lua).unwrap().to_string();
        //  let lua = Lua { txt: lua };
        //  Self {
        //      lua,
        //      ..Default::default()
        //  }
    }
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

        let c = Cartridge::from_bytes(&data);

        assert_eq!(c.lua.to_string(), "hi");
    }
}
