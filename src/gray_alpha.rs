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
        mut rng: R)
    {
        // pixels per bytes
        let ppb = match mode
        {
            GrayAlphaMode::Alpha => 8,
            GrayAlphaMode::All => 4,
        };

        for (pixels, byte) in source.pixels_mut()
            .chunks(ppb).into_iter()
            .map(|ch| ch.collect::<Vec<_>>())
            .zip(payload.iter().map(|x| *x))
        {
            if pixels.len() != ppb
            {
                return;
            }

            match mode
            {
                GrayAlphaMode::Alpha =>
                {
                    for (i, px) in pixels.into_iter()
                        .enumerate()
                    {
                        fix_u8(&mut px.data[1],
                               get_bit(byte, i as u8), &mut rng);
                    }
                },
                GrayAlphaMode::All =>
                {
                    for (i, px) in pixels.into_iter()
                        .enumerate()
                    {
                        fix_u8(&mut px.data[0],
                               get_bit(byte, i as u8 * 2), &mut rng);
                        fix_u8(&mut px.data[1],
                               get_bit(byte, i as u8 * 2 + 1), &mut rng);
                    }
                },
            }
        }
    }

    fn decode(
        source: &GrayAlphaImage,
        payload: &mut [u8],
        len: usize,
        mode: GrayAlphaMode)
    {
        assert!(len <= payload.len());

        // pixels per bytes
        let ppb = match mode
        {
            GrayAlphaMode::Alpha => 8,
            GrayAlphaMode::All => 4,
        };

        for (index, pixels) in source.pixels()
            .chunks(ppb).into_iter()
            .map(|ch| ch.collect::<Vec<_>>())
            .enumerate()
            .take(len)
        {
            if pixels.len() != ppb
            {
                return;
            }

            match mode
            {
                GrayAlphaMode::Alpha =>
                {
                    let mut byte = 0u8;

                    for (i, px) in pixels.into_iter()
                        .enumerate()
                    {
                        byte = set_bit(byte,
                            i as u8, px.data[1] % 2 == 1);
                    }

                    payload[index] = byte;
                },
                GrayAlphaMode::All =>
                {
                    let mut byte = 0u8;

                    for (i, px) in pixels.into_iter()
                        .enumerate()
                    {
                        byte = set_bit(byte,
                            i as u8 * 2, px.data[0] % 2 == 1);
                        byte = set_bit(byte,
                            i as u8* 2 + 1, px.data[1] % 2 == 1);
                    }

                    payload[index] = byte;
                },
            }
        }
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
