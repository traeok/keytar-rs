#[allow(dead_code)]
pub fn u8_slice_to_array<const N: usize>(arr: &[u8]) -> [u8; N] {
  arr.try_into().expect("Cannot convert this slice to array")
}
