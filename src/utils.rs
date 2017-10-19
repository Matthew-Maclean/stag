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
