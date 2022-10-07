use std::io::Read;

use crate::{Cartridge, DecoError, DecoResult};

pub fn decode_pico_txt<R: Read>(r: R) -> DecoResult<Cartridge> {
    let mut decoder = png::Decoder::new(r);

    let bpp = decoder.read_header_info().unwrap().bytes_per_pixel();
    // Pico8 cartridge should encode ARGB into 4 bytes per pixel.
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

    Cartridge::from_bytes(&card_bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_to_vec() {
        // reference png: https://garethrees.org/2007/11/14/pngcrush/
        let smol_png: Vec<u8> = vec![
            0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a, 0x00, 0x00, 0x00, 0x0d, 0x49, 0x48,
            0x44, 0x52, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x01, 0x00, 0x00, 0x00,
            0x00, 0x37, 0x6e, 0xf9, 0x24, 0x00, 0x00, 0x00, 0x10, 0x49, 0x44, 0x41, 0x54, 0x78,
            0x9c, 0x62, 0x60, 0x01, 0x00, 0x00, 0x00, 0xff, 0xff, 0x03, 0x00, 0x00, 0x06, 0x00,
            0x05, 0x57, 0xbf, 0xab, 0xd4, 0x00, 0x00, 0x00, 0x00, 0x49, 0x45, 0x4e, 0x44, 0xae,
            0x42, 0x60, 0x82,
        ];

        let png = decode_pico_txt(&*smol_png);
        assert!(png.is_err());
    }
}
