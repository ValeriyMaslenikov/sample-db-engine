pub(super) trait KeyReferenceGeneral {
    fn offset_by_index(index: u32) -> usize;

    fn new_by_index(buf: &[u8], index: u32) -> Self;

    fn save_by_index(&self, buf: &mut [u8], index: u32);

    fn bytes_per_item() -> u8;
}