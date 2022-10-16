use std::{
    fmt::{self, Display},
    io::Read,
};

use crate::{DecoError, DecoResult};

use super::{
    decode::{get_v0_lua, get_v1_lua, get_v2_lua},
    encode::to_v0_png,
};

const COMPRESSION_V1: u32 = u32::from_be_bytes(*b":c:\x00");
const COMPRESSION_V2: u32 = u32::from_be_bytes(*b"\x00pxa");

/// Cartridge encoding version.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Compression {
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
    pub txt: String,
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
    version: Compression,
    data_len: usize,
}

impl Cartridge {
    pub fn from_png<R: Read>(r: R) -> DecoResult<Self> {
        let mut decoder = png::Decoder::new(r);

        let bpp = decoder.read_header_info().unwrap().bytes_per_pixel();
        // Pico8 cartridge should encode RGBA into 4 bytes per pixel.
        if bpp != 4 {
            return Err(DecoError::Internal);
        }

        let mut reader = decoder.read_info().unwrap();
        // Allocate the output buffer.
        let mut buf = vec![0; reader.output_buffer_size()];
        // Read the next frame. An APNG might contain multiple frames.
        let info = reader.next_frame(&mut buf).unwrap();

        // Grab the bytes of the image.
        let bytes = &buf[..info.buffer_size()];

        let mut card_bytes = Vec::default();

        // Loop over chunks of four bytes and collect 2 lsb bits from them to combine one cartridge
        // byte.
        for argb in bytes.chunks(bpp) {
            let r = argb[0] & 3;
            let g = argb[1] & 3;
            let b = argb[2] & 3;
            let a = argb[3] & 3;

            card_bytes.push(a << 6 | r << 4 | g << 2 | b);
        }

        Self::from_bytes(&card_bytes)
    }

    pub fn from_bytes(data: &[u8]) -> DecoResult<Self> {
        let version = match to_u32(&data[0x4300..=0x4303]) {
            COMPRESSION_V1 => Compression::V1,
            COMPRESSION_V2 => Compression::V2,
            _ => Compression::V0,
        };

        let data = &data[0x4300..0x7fff];
        let lua = match version {
            Compression::V0 => get_v0_lua(data)?,
            Compression::V1 => get_v1_lua(data)?,
            Compression::V2 => get_v2_lua(data)?,
        };

        Ok(Self {
            lua,
            version,
            gfx: Gfx {},
            data_len: data.len(),
        })
    }

    pub fn to_png(&self, compression: Compression) -> DecoResult<Vec<u8>> {
        match compression {
            Compression::V0 => to_v0_png(&self.gfx, &self.lua),
            Compression::V1 => todo!(),
            Compression::V2 => todo!(),
        }
        todo!()
    }

    pub fn version(&self) -> Compression {
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

fn to_u32(b: &[u8]) -> u32 {
    let mut _u32: [u8; 4] = Default::default();
    _u32.copy_from_slice(b);

    u32::from_be_bytes(_u32)
}

#[cfg(test)]
mod tests {
    use std::vec;

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

        //assert_eq!(c.version(), Compression::V2);
    }
}
