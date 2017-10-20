use image::GrayAlphaImage;
use rand::Rng;
use itertools::Itertools;

use utils::*;
use codec::Codec;

pub struct GrayAlphaCodec;

impl Codec for GrayAlphaCodec
{
    type Input = GrayAlphaImage;
    type Mode = GrayAlphaMode;

    fn encode<R: Rng>(
        source: &mut GrayAlphaImage,
        payload: &[u8],
        mode: GrayAlphaMode,
        rng: R)
    {
        unimplemented!()
    }

    fn decode(
        source: &GrayAlphaImage,
        payload: &mut [u8],
        len: usize,
        mode: GrayAlphaMode)
    {
        unimplemented!()
    }

    fn estimate(
        source: &GrayAlphaImage,
        mode: GrayAlphaMode) -> Option<usize>
    {
        Some(match mode
        {
            GrayAlphaMode::Alpha =>
                source.height() as usize * source.width() as usize / 8,
            GrayAlphaMode::All =>
                source.height() as usize * source.width() as usize / 4,
        })
    }
}

#[derive(Copy, Clone)]
pub enum GrayAlphaMode
{
    Alpha,
    All,
}

use std::default::Default;

impl Default for GrayAlphaMode
{
    fn default() -> GrayAlphaMode
    {
        GrayAlphaMode::Alpha
    }
}

use std::str::FromStr;

impl FromStr for GrayAlphaMode
{
    type Err = ();

    fn from_str(s: &str) -> Result<GrayAlphaMode, ()>
    {
        if s == "alpha"
        {
            Ok(GrayAlphaMode::Alpha)
        }
        else if s == "all"
        {
            Ok(GrayAlphaMode::All)
        }
        else
        {
            Err(())
        }
    }
}
