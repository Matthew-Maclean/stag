use image::RgbaImage;
use rand::Rng;
use itertools::Itertools;

use utils::*;

pub fn encode_rgba<R: Rng>(source: &mut RgbaImage, payload: &[u8], mode: RgbaMode, mut rng: R)
{
    // number of pixels per byte
    let ppb = match mode
    {
        // 1 values per pixel = 8 pixels per byte
        RgbaMode::Alpha => 8,
        // 4 values per pixel = 2 pixels per byte
        RgbaMode::Each => 2,
    };
    
    for (mut pixels, byte) in source.pixels_mut() // iterate pixels mutably
        .chunks(ppb).into_iter()              // split them into ppb-size chunks
        .map(|ch| ch.collect::<Vec<_>>())     // make them vectors so we can check their lengths later
        .zip(payload.iter().map(|x| *x))      // give each chunk a byte
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
            RgbaMode::Each =>
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

pub fn decode_rgba(source: &RgbaImage, mode: RgbaMode) -> Vec<u8>
{
    // pixels per byte
    let ppb: usize = match mode
    {
        // one value per pixel = 8 pixels per byte
        RgbaMode::Alpha => 8,
        // four values per pixel = 2 pixels per byte
        RgbaMode::Each => 2,
    };

    let mut buf = Vec::with_capacity(source.width() as usize* source.height() as usize/ ppb);
    
    for pixels in source.pixels()         // iterate pixels
        .chunks(ppb).into_iter()          // split into ppb-sized chunks
        .map(|ch| ch.collect::<Vec<_>>()) // make them vectors so we can check their lengths
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

                buf.push(byte)
            },
            RgbaMode::Each =>
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

                buf.push(byte);
            }
        }
    }

    buf

}

/// The encoding/decoding mode
#[derive(Copy, Clone)]
pub enum RgbaMode
{
    /// encode in alpha even/odd
    Alpha,
    /// encode in each field even/odd
    Each,
}

use std::default::Default;

impl Default for RgbaMode
{
    fn default() -> RgbaMode
    {
        RgbaMode::Alpha
    }
}

#[cfg(test)]
mod test
{
    use image::{ImageBuffer, Rgba};
    use rand::StdRng;

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

        let rng = StdRng::new().unwrap();

        encode_rgba(&mut image, &payload, RgbaMode::Alpha, rng);

        let dec = decode_rgba(&image, RgbaMode::Alpha);

        assert_eq!(payload, dec);
    }

    #[test]
    fn each()
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

        let rng = StdRng::new().unwrap();

        encode_rgba(&mut image, &payload, RgbaMode::Each, rng);

        let dec = decode_rgba(&image, RgbaMode::Each);

        assert_eq!(payload, dec);
    }
}
