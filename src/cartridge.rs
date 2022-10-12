use std::{
    fmt::{self, Display},
    io::Read,
    str::from_utf8,
};

use crate::{DecoError, DecoResult};

const COMPRESSION_V1: u32 = u32::from_be_bytes(*b":c:\x00");
const COMPRESSION_V2: u32 = u32::from_be_bytes(*b"\x00pxa");
const OLD_LOOKUP: &[u8] = b"0123456789abcdefghijklmnopqrstuvwxyz!#%(){}[]<>+=/*:;.,~_";

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

/// Trims all zeros and returns a string with lua code.
fn get_v0_lua(data: &[u8]) -> DecoResult<Lua> {
    let first_0_idx = data.iter().position(|&u| u == 0).unwrap_or(data.len());
    let txt = std::str::from_utf8(&data[..first_0_idx])
        .map_err(|_| DecoError::Internal)?
        .to_string();

    Ok(Lua { txt })
}

fn get_v1_lua(data: &[u8]) -> DecoResult<Lua> {
    let mut out: Vec<u8> = Vec::new();
    let mut cursor = 0x4308usize;

    // The next two bytes (0x4304-0x4305) are the length of the decompressed code, stored MSB first.
    // The next two bytes (0x4306-0x4307) are always zero.
    let decompressed_len = to_u16(&data[0x4304..=0x4305]) as usize;

    while cursor < decompressed_len {
        match data[cursor] {
            // 0x00: Copy the next byte directly to the output stream.
            0x0 => {
                out.push(data[cursor + 1]);
                cursor += 2;
            }

            // 0x01-0x3b: Emit a character from a lookup table: newline, space
            n @ 0x01..=0x3b => {
                out.push(OLD_LOOKUP[(n - 1) as usize]);
                cursor += 1;
            }

            // 0x3c-0xff: Calculate an offset and length from this byte and the next byte,
            // then copy those bytes from what has already been emitted. In other words,
            // go back "offset" characters in the output stream, copy "length" characters,
            // then paste them to the end of the output stream.
            n @ 0x3c..=0xff => {
                // TODO: check if the data is in bounds.
                let offset = cursor - ((n - 0x3c) * 16 * (data[cursor + 1] & 0xf)) as usize;
                let length = cursor - offset + ((data[cursor + 1] >> 4) + 2) as usize;

                out.append(&mut out[offset..length].to_vec());
                cursor += 2;
            }
        }
    }

    Ok(Lua {
        txt: from_utf8(&out)
            .map_err(|_| DecoError::Internal)?
            .to_string(),
    })
}

fn get_v2_lua(data: &[u8]) -> DecoResult<Lua> {
    let mut out: Vec<u8> = Vec::new();
    let mut data_cursor = 0x4308usize;

    // Initially, each of the 256 possible bytes maps to itself.
    let mut mtf = &data[data_cursor..data_cursor + 256];
    data_cursor += 256;

    // The next two bytes (0x4304-0x4305) are the length of the decompressed code, stored MSB first.
    let decompressed_len = to_u16(&data[0x4304..=0x4305]) as usize;
    // The next two bytes (0x4306-0x4307) are the length of the compressed data + 8 for this 8-byte header, stored MSB first.
    let compressed_len = to_u16(&data[0x4306..=0x4307]) as usize;

    while data_cursor < compressed_len {
        let mut unary = 0;
        for i in 0..8 {
            if bit_from_byte(data[data_cursor], i) {
                unary += 1;
            }
        }

        data_cursor += 1
    }

    todo!()
}

fn bit_from_byte(byte: u8, bit_idx: usize) -> bool {
    todo!()
}

fn to_u16(b: &[u8]) -> u16 {
    let mut _u16: [u8; 2] = Default::default();
    _u16.copy_from_slice(b);

    u16::from_be_bytes(_u16)
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
