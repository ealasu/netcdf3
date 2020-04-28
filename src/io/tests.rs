#![cfg(test)]
use crate::{
    DataType,
    error::{ParseHeaderError, ParseErrorKind, InvalidBytes},
};

use byteorder::{WriteBytesExt, BigEndian};
use super::{
    FileReader,
    compute_zero_padding_size,
};

#[test]
fn test_get_zero_padding_size()
{
    assert_eq!(0, compute_zero_padding_size(0));
    assert_eq!(3, compute_zero_padding_size(1));
    assert_eq!(2, compute_zero_padding_size(2));
    assert_eq!(1, compute_zero_padding_size(3));
    assert_eq!(0, compute_zero_padding_size(4));
    assert_eq!(3, compute_zero_padding_size(5));
}

#[test]
fn test_parse_non_neg_i32() {
    // Test `0_i32`
    {
        let a: i32 = 0_i32;
        let bytes: [u8; 4] = a.to_be_bytes();
        // parse the integer
        let (rem_bytes, b): (&[u8], i32) = FileReader::parse_non_neg_i32(&bytes[..]).unwrap();
        // test remaining bytes and the parsed value
        assert_eq!(&[] as &[u8], rem_bytes);
        assert_eq!(0_i32, b);
    }

    // Test `1_i32`
    {
        let a: i32 = 1_i32;
        let bytes: [u8; 4] = a.to_be_bytes();
        // parse the integer
        let (rem_bytes, b): (&[u8], i32) = FileReader::parse_non_neg_i32(&bytes[..]).unwrap();
        // test remaining bytes and the parsed value
        assert_eq!(&[] as &[u8], rem_bytes);
        assert_eq!(1_i32, b);
    }

    // Test `std::i32::MAX`
    {
        let a: i32 = std::i32::MAX;
        let bytes: [u8; 4] = a.to_be_bytes();
        // parse the integer
        let (rem_bytes, b): (&[u8], i32) = FileReader::parse_non_neg_i32(&bytes[..]).unwrap();
        // test remaining bytes and the parsed value
        assert_eq!(&[] as &[u8], rem_bytes);
        assert_eq!(std::i32::MAX, b);
    }

    // Test `-1_i32`
    {
        let a: i32 = -1_i32;
        let bytes: [u8; 4] = a.to_be_bytes();
        // parse the integer
        let parsing_result = FileReader::parse_non_neg_i32(&bytes[..]);
        // check the returned error
        assert!(parsing_result.is_err());
        let parsing_result = FileReader::parse_non_neg_i32(&bytes[..]);
        assert!(parsing_result.is_err());
        assert!(parsing_result.is_err());
        let parsing_err: ParseHeaderError = parsing_result.unwrap_err();
        assert!(!parsing_err.is_incomplete());
        assert_eq!(ParseErrorKind::NonNegativeI32 ,parsing_err.kind);
        assert_eq!(
            InvalidBytes::Bytes(bytes.to_vec()),
            parsing_err.invalid_bytes,
        );
    }

    // Test `std::i32::MIN`
    {
        let a: i32 = std::i32::MIN;
        let bytes: [u8; 4] = a.to_be_bytes();
        // parse the integer
        let parsing_result = FileReader::parse_non_neg_i32(&bytes[..]);
        // check the returned error
        assert!(parsing_result.is_err());
        let parsing_err: ParseHeaderError = parsing_result.unwrap_err();
        assert!(!parsing_err.is_incomplete());
        assert_eq!(ParseErrorKind::NonNegativeI32 ,parsing_err.kind);
        assert_eq!(
            InvalidBytes::Bytes(bytes.to_vec()),
            parsing_err.invalid_bytes,
        );
    }

    // Test with a larger input
    {
        let a: i32 = 1_i32;
        let bytes: [u8; 4] = a.to_be_bytes();
        // Add some bytes
        let mut bytes: Vec<u8> = Vec::from(&bytes[..]);
        bytes.push(42);
        bytes.push(43);
        bytes.push(44);
        // parse the integer
        let (rem_bytes, b): (&[u8], i32) = FileReader::parse_non_neg_i32(&bytes[..]).unwrap();
        // test remaining bytes and the parsed value
        assert_eq!(&[42, 43, 44], rem_bytes);
        assert_eq!(1_i32, b);
    }

    // Missing input bytes
    {
        let a: i32 = 1_i32;
        let bytes: Vec<u8> = Vec::from(&a.to_be_bytes()[..2]);
        assert_eq!(2, bytes.len());
        // check the returned error
        let parsing_result = FileReader::parse_non_neg_i32(&bytes[..]);
        assert!(parsing_result.is_err());
        assert!(parsing_result.is_err());
        let parsing_err: ParseHeaderError = parsing_result.unwrap_err();
        assert!(parsing_err.is_incomplete());
        assert_eq!(ParseErrorKind::NonNegativeI32 ,parsing_err.kind);
        assert_eq!(
            InvalidBytes::Incomplete(nom::Needed::Size(4)),
            parsing_err.invalid_bytes
        );
    }
}

#[test]
fn test_parse_name_string() {
    {
        // Test a ASCII word
        {
            let bytes: Vec<u8> = {
                let word : String = String::from("foo");

                // Write the name
                let mut bytes: Vec<u8> = vec![];
                let num_of_bytes = word.len();
                bytes.write_i32::<BigEndian>(num_of_bytes as i32).unwrap();
                bytes.extend(word.as_bytes());
                // Append zero-padding bytes if necessary
                let zero_padding_size: usize = compute_zero_padding_size(num_of_bytes);
                for _ in 0..zero_padding_size
                {
                    bytes.push(0_u8);
                }
            
                bytes
            };
            // Parse the bytes into a string
            let (rem_bytes, name): (&[u8], String)= FileReader::parse_name_string(&bytes).unwrap();
            // Test the parsed string
            assert_eq!("foo", name);
            // And test the remaining bytes
            assert_eq!(0, rem_bytes.len());
        }

        // Test a ASCII word extended by other bytes
        {
            let bytes: Vec<u8> = {
                let word : String = String::from("foo");

                // Write the name
                let mut bytes: Vec<u8> = vec![];
                let num_of_bytes = word.len();
                bytes.write_i32::<BigEndian>(num_of_bytes as i32).unwrap();
                bytes.extend(word.as_bytes());
                // Append zero-padding bytes if necessary
                let zero_padding_size: usize = compute_zero_padding_size(num_of_bytes);
                for _ in 0..zero_padding_size
                {
                    bytes.push(0_u8);
                }
                // Append other bytes
                bytes.extend(&[1, 2, 3]);

                bytes
            };
            // Parse the bytes into a string
            let (rem_bytes, name): (&[u8], String)= FileReader::parse_name_string(&bytes).unwrap();
            // Test the parsed string
            assert_eq!("foo", name);
            // And test the remaining bytes
            assert_eq!(&[1, 2, 3], rem_bytes);
        }

        // Test with a wrong zero-padding bytes
        {
            let bytes: Vec<u8> = {
                let word : String = String::from("foooo");

                // Write the name
                let mut bytes: Vec<u8> = vec![];
                let num_of_bytes = word.len();
                bytes.write_i32::<BigEndian>(num_of_bytes as i32).unwrap();
                bytes.extend(word.as_bytes());
                // Append zero-padding bytes if necessary
                let zero_padding_size: usize = compute_zero_padding_size(num_of_bytes);
                assert!(zero_padding_size > 0);
                for i in 0..zero_padding_size
                {
                    if i == 0 {
                        // Append a wrong bytes here
                        bytes.push(1_u8);
                    }
                    else {
                        bytes.push(0_u8);
                    }
                }
                bytes
            };
            // check the returned error
            let parsing_result = FileReader::parse_name_string(&bytes[..]);
            assert!(parsing_result.is_err());
            let parsing_err: ParseHeaderError = parsing_result.unwrap_err();
            assert!(!parsing_err.is_incomplete());
            assert_eq!(ParseErrorKind::ZeroPadding ,parsing_err.kind);
            assert_eq!(InvalidBytes::Bytes(vec![1, 0, 0]) ,parsing_err.invalid_bytes);
        }

        // Test a valid UTF-8 word
        {
            let bytes: Vec<u8> = {
                let word : String = String::from("café");

                // Write the name
                let mut bytes: Vec<u8> = vec![];
                let num_of_bytes = word.len();
                bytes.write_i32::<BigEndian>(num_of_bytes as i32).unwrap();
                bytes.extend(word.as_bytes());
                // Append zero-padding bytes if necessary
                let zero_padding_size: usize = compute_zero_padding_size(num_of_bytes);
                for _ in 0..zero_padding_size
                {
                    bytes.push(0_u8);
                }
            
                bytes
            };
            // Parse the bytes into a string
            let (rem_bytes, name): (&[u8], String)= FileReader::parse_name_string(&bytes).unwrap();
            // Test the parsed string
            assert_eq!("café", name);
            // And test the remaining bytes
            assert_eq!(0, rem_bytes.len());
        }


        // Test a latin-1 word (not valid UTF-8)
        {
            let bytes: Vec<u8> = {
                let word : Vec<u8> = vec![b'c', b'a', b'f', b'\xe9'];  // latin-1 encoding

                // Write the name
                let mut bytes: Vec<u8> = vec![];
                let num_of_bytes = word.len();
                bytes.write_i32::<BigEndian>(num_of_bytes as i32).unwrap();
                bytes.extend(&word);
                // Append zero-padding bytes if necessary
                let zero_padding_size: usize = compute_zero_padding_size(num_of_bytes);
                for _ in 0..zero_padding_size
                {
                    bytes.push(0_u8);
                }
            
                bytes
            };
            // Parse the bytes into a string
            let parsing_result: Result<_, _> = FileReader::parse_name_string(&bytes);
            // Test the parsed string
            assert!(parsing_result.is_err());
            assert!(parsing_result.is_err());
            let parsing_err: ParseHeaderError = parsing_result.unwrap_err();
            assert!(!parsing_err.is_incomplete());
            assert_eq!(ParseErrorKind::Utf8 ,parsing_err.kind);
            assert_eq!(
                InvalidBytes::Bytes(vec![b'c', b'a', b'f', b'\xe9']),
                parsing_err.invalid_bytes,
            );
        }

        // Test missing zero padding bytes
        {
            let bytes: Vec<u8> = {
                let word : String = String::from("foobar");

                // Write the name
                let mut bytes: Vec<u8> = vec![];
                let num_of_bytes = word.len();
                bytes.write_i32::<BigEndian>(num_of_bytes as i32).unwrap();
                bytes.extend(word.as_bytes());
                // Append zero-padding bytes if necessary
                let zero_padding_size: usize = compute_zero_padding_size(num_of_bytes);
                for _ in 0..zero_padding_size
                {
                    bytes.push(0_u8);
                }
                // remove the last byte
                assert!(bytes.len() >= 2);
                bytes.remove(bytes.len() - 1);

                bytes
            };
            // Parse the bytes into a string
            let parsing_result: Result<_, _> = FileReader::parse_name_string(&bytes);
            // Test the parsed string
            assert!(parsing_result.is_err());
            assert!(parsing_result.is_err());
            let parsing_err: ParseHeaderError = parsing_result.unwrap_err();
            assert!(parsing_err.is_incomplete());
            assert_eq!(ParseErrorKind::ZeroPadding ,parsing_err.kind);
            assert_eq!(
                InvalidBytes::Incomplete(nom::Needed::Size(2)),
                parsing_err.invalid_bytes,
            );
        }
    }
}


#[test]
fn test_parse_data_type() {
    use std::convert::TryFrom;

    // test parse `DataType::I8`
    {
        let a: u32 = DataType::I8 as u32;
        let bytes: [u8; 4] = a.to_be_bytes();
        let (rem_input, data_type): (&[u8], DataType) = FileReader::parse_data_type(&bytes[..]).unwrap();
        assert_eq!(DataType::I8, data_type);
        assert_eq!(&[] as &[u8], rem_input);
    }

    // test parse `DataType::U8`
    {
        let a: u32 = DataType::U8 as u32;
        let bytes: [u8; 4] = a.to_be_bytes();
        let (rem_input, data_type): (&[u8], DataType) = FileReader::parse_data_type(&bytes[..]).unwrap();
        assert_eq!(DataType::U8, data_type);
        assert_eq!(&[] as &[u8], rem_input);
    }

    // // test parse `DataType::I16`
    {
        let a: u32 = DataType::I16 as u32;
        let bytes: [u8; 4] = a.to_be_bytes();
        let (rem_input, data_type): (&[u8], DataType) = FileReader::parse_data_type(&bytes[..]).unwrap();
        assert_eq!(DataType::I16, data_type);
        assert_eq!(&[] as &[u8], rem_input);
    }

    // // test parse `DataType::I32`
    {
        let a: u32 = DataType::I32 as u32;
        let bytes: [u8; 4] = a.to_be_bytes();
        let (rem_input, data_type): (&[u8], DataType) = FileReader::parse_data_type(&bytes[..]).unwrap();
        assert_eq!(DataType::I32, data_type);
        assert_eq!(&[] as &[u8], rem_input);
    }

    // // test parse `DataType::F32`
    {
        let a: u32 = DataType::F32 as u32;
        let bytes: [u8; 4] = a.to_be_bytes();
        let (rem_input, data_type): (&[u8], DataType) = FileReader::parse_data_type(&bytes[..]).unwrap();
        assert_eq!(DataType::F32, data_type);
        assert_eq!(&[] as &[u8], rem_input);
    }

    // test parse `DataType::F64`
    {
        let a: u32 = DataType::F64 as u32;
        let bytes: [u8; 4] = a.to_be_bytes();
        let (rem_input, data_type): (&[u8], DataType) = FileReader::parse_data_type(&bytes[..]).unwrap();
        assert_eq!(DataType::F64, data_type);
        assert_eq!(&[] as &[u8], rem_input);
    }

    // test parse a non-existant `DataType`
    {
        let a: u32 = 0_u32;
        assert!(DataType::try_from(a).is_err());

        let bytes: [u8; 4] = a.to_be_bytes();
        let parsing_result = FileReader::parse_data_type(&bytes[..]);
        assert!(parsing_result.is_err());
    }

    // test parse a negative `DataType` number
    {
        let a: i32 = -1_i32;

        let bytes: [u8; 4] = a.to_be_bytes();
        let parsing_result = FileReader::parse_data_type(&bytes[..]);
        // Check the return error
        assert!(parsing_result.is_err());
        let parsing_err: ParseHeaderError = parsing_result.unwrap_err();
        assert!(!parsing_err.is_incomplete());
        assert_eq!(ParseErrorKind::NonNegativeI32, parsing_err.kind);
        assert_eq!(
            InvalidBytes::Bytes(bytes.to_vec()),
            parsing_err.invalid_bytes
        );
    }

    // check the remaining bytes
    {
        let a: u32 = DataType::F64 as u32;

        let mut bytes: Vec<u8> = Vec::from(&a.to_be_bytes()[..]);
        bytes.push(42);
        bytes.push(43);
        bytes.push(44);

        let (rem_input, data_type): (&[u8], DataType) = FileReader::parse_data_type(&bytes[..]).unwrap();
        assert_eq!(DataType::F64, data_type);
        assert_eq!(
            &[42, 43, 44],
            rem_input
        );
    }

    // test missing input bytes
    {
        let a: u32 = DataType::F64 as u32;
        let bytes: Vec<u8> = Vec::from(&a.to_be_bytes()[..3]);
        assert_eq!(3, bytes.len());
        let parsing_result = FileReader::parse_data_type(&bytes[..]);
        // Check the return error
        assert!(parsing_result.is_err());
        let parsing_err: ParseHeaderError = parsing_result.unwrap_err();
        assert!(parsing_err.is_incomplete());
        assert_eq!(ParseErrorKind::NonNegativeI32, parsing_err.kind);
        assert_eq!(
            InvalidBytes::Incomplete(nom::Needed::Size(4)),
            parsing_err.invalid_bytes
        );
    }
}


#[test]
fn test_parse_zero_padding() {
    // Test valid zero padding
    {
        let bytes: [u8; 3] = [0_u8; 3];
        let (rem_input, zero_padding): (&[u8], &[u8]) = FileReader::parse_zero_padding(&bytes, 3).unwrap();
        assert_eq!(0, rem_input.len());
        assert_eq!(&[0, 0, 0], zero_padding);

    }
    // Test not valid zero padding
    {
        let bytes: [u8; 3] = [0, 1, 0];
        let parsing_result = FileReader::parse_zero_padding(&bytes, 3);
        // Check the return error
        assert!(parsing_result.is_err());
        let parsing_err = parsing_result.unwrap_err();
        assert!(!parsing_err.is_incomplete());
        assert_eq!(
            ParseErrorKind::ZeroPadding,
            parsing_err.kind,
        );
        assert_eq!(
            InvalidBytes::Bytes(bytes.to_vec()),
            parsing_err.invalid_bytes
        );
    }
    // Test missing bytes
    {
        let bytes: [u8; 3] = [0_u8; 3];
        let parsing_result = FileReader::parse_zero_padding(&bytes[0..2], 3);
        // Check the return error
        assert!(parsing_result.is_err());
        let parsing_err = parsing_result.unwrap_err();
        assert!(parsing_err.is_incomplete());
        assert_eq!(
            ParseErrorKind::ZeroPadding,
            parsing_err.kind,
        );
        assert_eq!(
            InvalidBytes::Incomplete(nom::Needed::Size(3)),
            parsing_err.invalid_bytes
        );
    }
}