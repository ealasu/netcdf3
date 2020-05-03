

mod file_reader;
mod tests;

pub use file_reader::FileReader;

/// These bytes mean the list (dimensions, attributes or variable) is not defined.
const ABSENT_TAG: [u8; 8] = [0; 8];
/// Bytes for the list of dimensions
const DIMENSION_TAG: [u8; 4] = [0, 0, 0, 0x0A];
/// Bytes for the list of variables
const VARIABLE_TAG: [u8; 4] = [0, 0, 0, 0x0b];
/// Bytes for the lists attributes (global or for each variable).
const ATTRIBUTE_TAG: [u8; 4] = [0, 0, 0, 0x0C];

#[inline]
/// Compute and return the number of bytes of the zero padding required to fill remaining bytes up.
///
/// Arguments :
/// - `number_of_bytes` : number of used bytes
pub fn compute_num_bytes_zero_padding(number_of_bytes: usize) -> usize {
    const ALIGNMENT_SIZE: usize = 4;
    return match number_of_bytes % 4 {
        0 => 0,
        n => ALIGNMENT_SIZE - n,
    };
}