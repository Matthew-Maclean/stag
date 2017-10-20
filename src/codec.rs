use rand::Rng;

use std::str::FromStr;

/// A type that can encode and decode
pub trait Codec
{
    /// The input or source type that it incodes into
    type Input;
    /// The input mode or modes that it supports, usually an enum
    type Mode: Default + FromStr;

    /// Encode a payload into an input
    fn encode<R: Rng>(
        source: &mut Self::Input,
        payload: &[u8],
        mode: Self::Mode,
        rng: R);
    /// Decode a payload into a buffer from an input
    fn decode(
        source: &Self::Input,
        buffer: &mut [u8],
        len: usize,
        mode: Self::Mode);
    
    /// Estimate how many bytes can be encoded into an image
    fn estimate(
        _source: &Self::Input,
        _mode: Self::Mode) -> Option<usize>
    {
        None
    }
}
