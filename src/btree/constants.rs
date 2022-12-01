/// It's reserved amount of bytes for Page header. The original structure occupies
/// less amount of bytes, but to avoid any restructuring for the sake of compatibliity
/// we will reserve additional bytes for that purpose.
pub(super) const NODE_HEADER_BYTES: u8 = 16;