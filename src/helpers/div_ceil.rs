/// Return average value rounded to upper bound
pub fn div_ceil(a: u32, b: u32) -> u32 {
    (a + b - 1) / b
}