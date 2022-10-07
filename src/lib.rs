mod cartridge;
mod decode;

pub use cartridge::*;
pub use decode::decode_pico_txt;

type DecoResult<T> = Result<T, DecoError>;

#[derive(Debug)]
pub enum DecoError {
    Internal,
}
