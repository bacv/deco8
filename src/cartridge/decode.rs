use std::str::from_utf8;

use crate::{DecoError, DecoResult};

use super::cartridge::Lua;

const OLD_LOOKUP: &[u8] = b"0123456789abcdefghijklmnopqrstuvwxyz!#%(){}[]<>+=/*:;.,~_";

/// Trims all trailing zeros and returns a string with lua code.
pub(super) fn get_v0_lua(data: &[u8]) -> DecoResult<Lua> {
    let first_0_idx = data.iter().position(|&u| u == 0).unwrap_or(data.len());
    let txt = std::str::from_utf8(&data[..first_0_idx])
        .map_err(|_| DecoError::Internal)?
        .to_string();

    Ok(Lua { txt })
}

pub(super) fn get_v1_lua(data: &[u8]) -> DecoResult<Lua> {
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

pub(super) fn get_v2_lua(data: &[u8]) -> DecoResult<Lua> {
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
