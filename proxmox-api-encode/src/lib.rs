use percent_encoding::AsciiSet;
use percent_encoding::CONTROLS;

pub mod de;
pub mod error;
pub mod ser;

pub trait DefaultKey {
    fn default_key() -> String;
}

pub const CUSTOM_ENCODING_SET: &AsciiSet = &CONTROLS.add(b'/').add(b',').add(b':').add(b';').add(b'=');

#[cfg(test)]
mod test;
