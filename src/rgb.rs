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
        _mode: RgbMode,
        mut rng: R)
    {
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

    fn decode(
        source: &RgbImage,
        payload: &mut [u8],
        len: usize,
        _mode: RgbMode)
    {
        assert!(len <= payload.len());

        let exlen = len + (len % 3);

        // iterate over the pixels
        for (index, pixels) in source.pixels()
            // grab them in chunks of 8
            .chunks(8).into_iter()
            // make them vectors
            .map(|ch| ch.collect::<Vec<_>>())
            .take(exlen / 3) // exlen is always exactly divisible by 3
            .enumerate()
        {
            if pixels.len() != 8
            {
                return;
            }

            let mut byte = 0u8;
            byte = set_bit(byte, 0, pixels[0].data[0] % 2 == 1);
            byte = set_bit(byte, 1, pixels[0].data[1] % 2 == 1);
            byte = set_bit(byte, 2, pixels[0].data[2] % 2 == 1);
            byte = set_bit(byte, 3, pixels[1].data[0] % 2 == 1);
            byte = set_bit(byte, 4, pixels[1].data[1] % 2 == 1);
            byte = set_bit(byte, 5, pixels[1].data[2] % 2 == 1);
            byte = set_bit(byte, 6, pixels[2].data[0] % 2 == 1);
            byte = set_bit(byte, 7, pixels[2].data[1] % 2 == 1);

            if index * 3 < len
            {
                payload[index * 3] = byte;
            }

            byte = set_bit(byte, 0, pixels[2].data[2] % 2 == 1);
            byte = set_bit(byte, 1, pixels[3].data[0] % 2 == 1);
            byte = set_bit(byte, 2, pixels[3].data[1] % 2 == 1);
            byte = set_bit(byte, 3, pixels[3].data[2] % 2 == 1);
            byte = set_bit(byte, 4, pixels[4].data[0] % 2 == 1);
            byte = set_bit(byte, 5, pixels[4].data[1] % 2 == 1);
            byte = set_bit(byte, 6, pixels[4].data[2] % 2 == 1);
            byte = set_bit(byte, 7, pixels[5].data[0] % 2 == 1);

            if index * 3 + 1 < len
            {
                payload[index * 3 + 1] = byte;
            }

            byte = set_bit(byte, 0, pixels[5].data[1] % 2 == 1);
            byte = set_bit(byte, 1, pixels[5].data[2] % 2 == 1);
            byte = set_bit(byte, 2, pixels[6].data[0] % 2 == 1);
            byte = set_bit(byte, 3, pixels[6].data[1] % 2 == 1);
            byte = set_bit(byte, 4, pixels[6].data[2] % 2 == 1);
            byte = set_bit(byte, 5, pixels[7].data[0] % 2 == 1);
            byte = set_bit(byte, 6, pixels[7].data[1] % 2 == 1);
            byte = set_bit(byte, 7, pixels[7].data[2] % 2 == 1);

            if index * 3 + 2 < len
            {
                payload[index * 3 + 2] = byte;
            }
        }
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
