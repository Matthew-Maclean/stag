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

pub fn split_u8(source: u8) -> [bool; 8]
{
    [
        ((source >> 0) & 1) == 1,
        ((source >> 1) & 1) == 1,
        ((source >> 2) & 1) == 1,
        ((source >> 3) & 1) == 1,
        ((source >> 4) & 1) == 1,
        ((source >> 5) & 1) == 1,
        ((source >> 6) & 1) == 1,
        ((source >> 7) & 1) == 1,
    ]
}
