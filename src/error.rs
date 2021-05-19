//! Error type

/// Result type
pub type Result<T> = core::result::Result<T, Error>;

/// Error type
#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    /// I/O errors
    #[cfg(feature = "std")]
    Io(std::io::Error),

    /// PKCS#8 errors
    Pkcs8(pkcs8::Error),
}

impl From<pkcs8::Error> for Error {
    fn from(err: pkcs8::Error) -> Error {
        Error::Pkcs8(err)
    }
}

#[cfg(feature = "std")]
impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::Io(err)
    }
}
