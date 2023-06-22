mod error;

pub use error::{PngError, Result};

/// 4 bytes size
pub const CHUNK_SIZE: usize = 4;