pub fn get_bit(source: u8, index: u8) -> bool
{
    assert!(index < 8);

    ((source >> index) & 1) == 1
}

pub fn set_bit(source: u8, index: u8, value: bool) -> u8
{
    assert!(index < 8);

    if value
    {
        source | 1 << index
    }
    else
    {
        source & !(1 << index)
    }
}

use rand::Rng;

pub fn fix_u8<R: Rng>(source: &mut u8, value: bool, rng: &mut R)
{
    // true  = 1 = odd
    // false = 0 = even

    if value
    {
        if *source % 2 == 0
        {
            if *source == 0
            {
                *source = 1;
            }
            else
            {
                if rng.gen()
                {
                    *source += 1;
                }
                else
                {
                    *source -= 1;
                }
            }
        }
    }
    else
    {
        if *source % 2 == 1
        {
            if *source == 255
            {
                *source = 254;
            }
            else
            {
                if rng.gen()
                {
                    *source += 1;
                }
                else
                {
                    *source -= 1;
                }
            }
        }
    }
}
