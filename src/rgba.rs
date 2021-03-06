use image::RgbaImage;
use rand::Rng;
use itertools::Itertools;

use utils::*;
use codec::Codec;

pub struct RgbaCodec;

impl Codec for RgbaCodec
{
    type Input = RgbaImage;
    type Mode = RgbaMode;

    fn encode<R: Rng>(
        source: &mut RgbaImage,
        payload: &[u8],
        mode: RgbaMode,
        mut rng: R)
    {
        // number of pixels per byte
        let ppb = match mode
        {
            // 1 values per pixel = 8 pixels per byte
            RgbaMode::Alpha => 8,
            // 4 values per pixel = 2 pixels per byte
            RgbaMode::All => 2,
        };
        
        // iterate pixels mutably
        for (mut pixels, byte) in source.pixels_mut()
             // split them into ppb-size chunks
            .chunks(ppb).into_iter()
            // make them vectors so we can check their lengths later
            .map(|ch| ch.collect::<Vec<_>>())
            // give each chunk a byte
            .zip(payload.iter().map(|x| *x))
        {
            // if the pixels don't fit exactly into their chunks,
            // the last chunk will be short. we won't use that chunk
            // (making sure the payload fits into the image is not
            // this functions job!)
            if pixels.len() != ppb
            {
                return;
            }

            match mode
            {
                RgbaMode::Alpha =>
                {
                    for (i, px) in pixels.into_iter().enumerate()
                    {
                        // a true, or 1 bit
                        if get_bit(byte, i as u8)
                        {
                            // even = 0 bit, so it needs to be changed
                            if px.data[3] % 2 == 0
                            {
                                // special case: if it's zero, we have to go up
                                if px.data[3] == 0
                                {
                                    px.data[3] = 1;
                                }
                                else
                                {
                                    if rng.gen()
                                    {
                                        px.data[3] += 1;
                                    }
                                    else
                                    {
                                        px.data[3] -= 1;
                                    }
                                }
                            }
                        }
                        // a false, or 0 bit
                        else
                        {
                            // odd = 1 bit, so it needs to be changed
                            if px.data[3] % 2 == 1
                            {
                                // special case: if it's 255, we have to go down
                                if px.data[3] == 255
                                {
                                    px.data[3] = 254;
                                }
                                else
                                {
                                    if rng.gen()
                                    {
                                        px.data[3] += 1;
                                    }
                                    else
                                    {
                                        px.data[3] -= 1;
                                    }
                                }
                            }
                        }
                    }
                },
                RgbaMode::All =>
                {
                    for i in 0..8
                    {
                        let val = if i < 4
                        {
                            &mut pixels[0].data[i]
                        }
                        else
                        {
                            &mut pixels[1].data[i - 4]
                        };

                        if get_bit(byte, i as u8)
                        {
                            // even = 0 bit, so it needs to be changed
                            if *val % 2 == 0
                            {
                                // special case: if it's zero, we have to go up
                                if *val == 0
                                {
                                    *val = 1;
                                }
                                else
                                {
                                    if rng.gen()
                                    {
                                        *val += 1;
                                    }
                                    else
                                    {
                                        *val -= 1;
                                    }
                                }
                            }
                        }
                        // a false, or 0 bit
                        else
                        {
                            // odd = 1 bit, so it needs to be changed
                            if *val % 2 == 1
                            {
                                // special case: if it's 255, we have to go down
                                if *val == 255
                                {
                                    *val = 254;
                                }
                                else
                                {
                                    if rng.gen()
                                    {
                                        *val += 1;
                                    }
                                    else
                                    {
                                        *val -= 1;
                                    }
                                }
                            }
                        }
                    }
                },
            }
        }
    }

    fn decode(
        source: &RgbaImage,
        buffer: &mut [u8],
        len: usize,
        mode: RgbaMode)
    {
        assert!(len <= buffer.len());

        // pixels per byte
        let ppb: usize = match mode
        {
            // one value per pixel = 8 pixels per byte
            RgbaMode::Alpha => 8,
            // four values per pixel = 2 pixels per byte
            RgbaMode::All => 2,
        };

        // iterate pixels
        for (index, pixels) in source.pixels()
            // split into ppb-sized chunks
            .chunks(ppb).into_iter()
            // make them vectors so we can check their lengths
            .map(|ch| ch.collect::<Vec<_>>())
            .take(len)
            .enumerate()
        {
            // only decode while we have enough data
            if pixels.len() != ppb
            {
                break;
            }

            match mode
            {
                RgbaMode::Alpha =>
                {
                    let mut byte = 0u8;

                    for (i, px) in pixels.into_iter().enumerate()
                    {
                        byte = set_bit(byte, i as u8, px.data[3] % 2 == 1);
                    }

                    buffer[index] = byte;
                },
                RgbaMode::All =>
                {
                    let mut byte = 0u8;
                    
                    for i in 0..8
                    {
                        let val = if i < 4
                        {
                            pixels[0].data[i]
                        }
                        else
                        {
                            pixels[1].data[i - 4]
                        };

                        byte = set_bit(byte, i as u8, val % 2 == 1);
                    }
                    
                    buffer[index] = byte;
                }
            }
        }
    }

    fn estimate(
        source: &RgbaImage,
        mode: RgbaMode) -> Option<usize>
    {
        Some(match mode
        {
            RgbaMode::Alpha =>
                source.width() as usize * source.height() as usize / 8,
            RgbaMode::All =>
                source.width() as usize * source.height() as usize / 2,
        })
    }
}

/// The encoding/decoding mode
#[derive(Copy, Clone)]
pub enum RgbaMode
{
    /// encode in alpha even/odd
    Alpha,
    /// encode in all field even/odd
    All,
}

use std::default::Default;

impl Default for RgbaMode
{
    fn default() -> RgbaMode
    {
        RgbaMode::Alpha
    }
}

use std::str::FromStr;

impl FromStr for RgbaMode
{
    type Err = ();

    fn from_str(s: &str) -> Result<RgbaMode, ()>
    {
        if s == "alpha"
        {
            Ok(RgbaMode::Alpha)
        }
        else if s == "all"
        {
            Ok(RgbaMode::All)
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
    use image::{ImageBuffer, Rgba};
    use rand::StdRng;
    
    use codec::Codec;
    use super::*;

    #[test]
    fn alpha()
    {
        let mut image = ImageBuffer::from_pixel(
            25,
            8,
            Rgba
            {
                data: [127u8; 4],
            });

        let payload = vec![
            1,2, 3, 4, 5, 6, 7, 8, 9, 10,
            11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
            21, 22, 23, 24, 25
        ];

        let mut buf = vec![0; 25];

        let rng = StdRng::new().unwrap();

        RgbaCodec::encode(&mut image, &payload, RgbaMode::Alpha, rng);

        RgbaCodec::decode(&image, &mut buf, 25, RgbaMode::Alpha);

        assert_eq!(payload, buf);
    }

    #[test]
    fn all()
    {
        let mut image = ImageBuffer::from_pixel(
            25,
            2,
            Rgba
            {
                data: [127u8; 4],
            });

        let payload = vec![
            1,2, 3, 4, 5, 6, 7, 8, 9, 10,
            11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
            21, 22, 23, 24, 25
        ];

        let mut buf = vec![0; 25];

        let rng = StdRng::new().unwrap();

        RgbaCodec::encode(&mut image, &payload, RgbaMode::All, rng);

        RgbaCodec::decode(&image, &mut buf, 25, RgbaMode::All);

        assert_eq!(payload, buf);
    }
}
