#![cfg(test)]

use super::compute_num_bytes_zero_padding;

#[test]
fn test_compute_num_bytes_zero_padding()
{
    assert_eq!(0, compute_num_bytes_zero_padding(0));
    assert_eq!(3, compute_num_bytes_zero_padding(1));
    assert_eq!(2, compute_num_bytes_zero_padding(2));
    assert_eq!(1, compute_num_bytes_zero_padding(3));
    assert_eq!(0, compute_num_bytes_zero_padding(4));
    assert_eq!(3, compute_num_bytes_zero_padding(5));
}
