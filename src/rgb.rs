use image::RgbImage;
use rand::Rng;
use itertools::Itertools;

use utils::*;
use codec::Codec;

pub struct RgbCodec;

impl Codec for RgbCodec
{
    type Input = RgbImage;
    type Mode = RgbMode;

    fn encode<R: Rng>(
        source: &mut RgbImage,
        payload: &[u8],
        mode: RgbMode,
        mut rng: R)
    {
        let px_chunk = 24;
        let by_chunk = 3;

        // iterate over pixels mutably
        for (mut pixels, bytes) in source.pixels_mut()
            // ~2.7 pixels per byte, so instead by use 24-pixel
            // and 3-byte chunks
            .chunks(8).into_iter()
            // make them vectors so we can check their lengths
            .map(|ch| ch.collect::<Vec<_>>())
            // give each chunk 3 bytes
            .zip(payload.iter().map(|x| *x)
                 .chunks(3).into_iter()
                 // make them also vectors
                 .map(|ch| ch.collect::<Vec<_>>()))
        {
            // if any of the chunks are short, abort
            if pixels.len() != 8 || bytes.len() != 3
            {
                return;
            }

            match mode
            {
                RgbMode::All =>
                {
                    let rng = &mut rng;
                    // i looked for a better way...
                    fix_u8(&mut pixels[0].data[0], get_bit(bytes[0], 0), rng);
                    fix_u8(&mut pixels[0].data[1], get_bit(bytes[0], 1), rng);
                    fix_u8(&mut pixels[0].data[2], get_bit(bytes[0], 2), rng);
                    fix_u8(&mut pixels[1].data[0], get_bit(bytes[0], 3), rng);
                    fix_u8(&mut pixels[1].data[1], get_bit(bytes[0], 4), rng);
                    fix_u8(&mut pixels[1].data[2], get_bit(bytes[0], 5), rng);
                    fix_u8(&mut pixels[2].data[0], get_bit(bytes[0], 6), rng);
                    fix_u8(&mut pixels[2].data[1], get_bit(bytes[0], 7), rng);

                    fix_u8(&mut pixels[2].data[2], get_bit(bytes[1], 0), rng);
                    fix_u8(&mut pixels[3].data[0], get_bit(bytes[1], 1), rng);
                    fix_u8(&mut pixels[3].data[1], get_bit(bytes[1], 2), rng);
                    fix_u8(&mut pixels[3].data[2], get_bit(bytes[1], 3), rng);
                    fix_u8(&mut pixels[4].data[0], get_bit(bytes[1], 4), rng);
                    fix_u8(&mut pixels[4].data[1], get_bit(bytes[1], 5), rng);
                    fix_u8(&mut pixels[4].data[2], get_bit(bytes[1], 6), rng);
                    fix_u8(&mut pixels[5].data[0], get_bit(bytes[1], 7), rng);

                    fix_u8(&mut pixels[5].data[1], get_bit(bytes[2], 0), rng);
                    fix_u8(&mut pixels[5].data[2], get_bit(bytes[2], 1), rng);
                    fix_u8(&mut pixels[6].data[0], get_bit(bytes[2], 2), rng);
                    fix_u8(&mut pixels[6].data[1], get_bit(bytes[2], 3), rng);
                    fix_u8(&mut pixels[6].data[2], get_bit(bytes[2], 4), rng);
                    fix_u8(&mut pixels[7].data[0], get_bit(bytes[2], 5), rng);
                    fix_u8(&mut pixels[7].data[1], get_bit(bytes[2], 6), rng);
                    fix_u8(&mut pixels[7].data[2], get_bit(bytes[2], 7), rng);
                }
            }
        }
    }

    fn decode(
        source: &RgbImage,
        payload: &mut [u8],
        len: usize,
        mode: RgbMode)
    {
        unimplemented!()
    }

    fn estimate(
        source: &RgbImage,
        mode: RgbMode) -> Option<usize>
    {
        Some(match mode
        {
            RgbMode::All =>
                source.width() as usize * source.height() as usize / 3
        })
    }

}

#[derive(Copy, Clone)]
pub enum RgbMode
{
    All,
}

use std::default::Default;

impl Default for RgbMode
{
    fn default() -> RgbMode
    {
        RgbMode::All
    }
}

use std::str::FromStr;

impl FromStr for RgbMode
{
    type Err = ();

    fn from_str(s: &str) -> Result<RgbMode, ()>
    {
        if s == "all"
        {
            Ok(RgbMode::All)
        }
        else
        {
            Err(())
        }
    }
}
