mod cartridge;

pub use cartridge::*;

type DecoResult<T> = Result<T, DecoError>;

#[derive(Debug)]
pub enum DecoError {
    Internal,
}
