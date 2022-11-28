pub(crate) fn average(a: u32, b: u32) -> u32 {
    return (a & b) + ((a ^ b) >> 1);
}