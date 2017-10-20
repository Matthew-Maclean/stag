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

#[cfg(test)]
mod test
{
    use image::{ImageBuffer, LumaA};
    use rand::StdRng;
    
    use codec::Codec;
    use super::*;

    #[test]
    fn alpha()
    {
       let mut image = ImageBuffer::from_pixel(
            30,
            8,
            LumaA
            {
                data: [127u8; 2],
            });

        let payload = vec![
            1,2, 3, 4, 5, 6, 7, 8, 9, 10,
            11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
            21, 22, 23, 24, 25
        ];

        let mut buf = vec![0; 25];

        let rng = StdRng::new().unwrap();

        GrayAlphaCodec::encode(&mut image, &payload, GrayAlphaMode::Alpha, rng);

        GrayAlphaCodec::decode(&image, &mut buf, 25, GrayAlphaMode::Alpha);

        assert_eq!(payload, buf);
    }

    #[test]
    fn all()
    {
       let mut image = ImageBuffer::from_pixel(
            30,
            8,
            LumaA
            {
                data: [127u8; 2],
            });

        let payload = vec![
            1,2, 3, 4, 5, 6, 7, 8, 9, 10,
            11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
            21, 22, 23, 24, 25
        ];

        let mut buf = vec![0; 25];

        let rng = StdRng::new().unwrap();

        GrayAlphaCodec::encode(&mut image, &payload, GrayAlphaMode::All, rng);

        GrayAlphaCodec::decode(&image, &mut buf, 25, GrayAlphaMode::All);

        assert_eq!(payload, buf);
    }
}
