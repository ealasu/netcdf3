#![cfg(test)]

use byteorder::{WriteBytesExt, BigEndian};

use copy_to_tmp_file::{
    copy_bytes_to_tmp_file,
    NC3_CLASSIC_FILE_NAME, NC3_CLASSIC_FILE_BYTES,
};

use crate::{
    FileReader,
    DataSet, Variable, DataType, Version,
    error::{
        IOError,
        input_error::{ReadDataError, ParseHeaderError, InvalidBytes, ParseErrorKind},
    },
    io::compute_num_bytes_zero_padding,
};


#[test]
fn test_file_reader_open()
{
    // Copy bytes to a temporary file
    let (tmp_dir, input_data_file_path) = copy_bytes_to_tmp_file(NC3_CLASSIC_FILE_BYTES, NC3_CLASSIC_FILE_NAME);

    // Read the data set
    let data_set: DataSet = {
        let file_reader = FileReader::open(input_data_file_path).unwrap();
        assert_eq!(Version::Classic, file_reader.version());
        file_reader.close().0
    };
    tmp_dir.close().unwrap();

    // Check the parsing of the header
    assert!(data_set.has_unlimited_dim());
    assert_eq!(3, data_set.num_dims());
    assert_eq!(1, data_set.num_global_attrs());
    assert_eq!(9, data_set.num_vars());

    // Check no variable data have not been load from the file, only metadata stored in the header have been loaded previousl
    // latitude
    {
        assert!(data_set.has_var("latitude"));
        let var: &Variable = data_set.get_var("latitude").unwrap();
        assert_eq!(DataType::F32, var.data_type());
        assert!(var.get_f32().is_none()); // only metadata stored in the header have been loaded previously
    }
    // longitude
    {
        assert!(data_set.has_var("longitude"));
        let var: &Variable = data_set.get_var("longitude").unwrap();
        assert_eq!(DataType::F32, var.data_type());
        assert!(var.get_f32().is_none()); // only metadata stored in the header have been loaded previously
    }
    // time
    {
        assert!(data_set.has_var("time"));
        let var: &Variable = data_set.get_var("time").unwrap();
        assert_eq!(DataType::F32, var.data_type());
        assert!(var.get_f32().is_none()); // only metadata stored in the header have been loaded previously
    }
    // temperature_i8
    {
        assert!(data_set.has_var("temperature_i8"));
        let var: &Variable = data_set.get_var("temperature_i8").unwrap();
        assert_eq!(DataType::I8, var.data_type());
        assert!(var.get_i8().is_none()); // only metadata stored in the header have been loaded previously
    }
    // temperature_u8
    {
        assert!(data_set.has_var("temperature_u8"));
        let var: &Variable = data_set.get_var("temperature_u8").unwrap();
        assert_eq!(DataType::U8, var.data_type());
        assert!(var.get_u8().is_none()); // only metadata stored in the header have been loaded previously
    }
    // temperature_i16
    {
        assert!(data_set.has_var("temperature_i16"));
        let var: &Variable = data_set.get_var("temperature_i16").unwrap();
        assert_eq!(DataType::I16, var.data_type());
        assert!(var.get_i16().is_none()); // only metadata stored in the header have been loaded previously
    }
    // temperature_i32
    {
        assert!(data_set.has_var("temperature_i32"));
        let var: &Variable = data_set.get_var("temperature_i32").unwrap();
        assert_eq!(DataType::I32, var.data_type());
        assert!(var.get_i32().is_none()); // only metadata stored in the header have been loaded previously
    }
    // temperature_f32
    {
        assert!(data_set.has_var("temperature_f32"));
        let var: &Variable = data_set.get_var("temperature_f32").unwrap();
        assert_eq!(DataType::F32, var.data_type());
        assert!(var.get_f32().is_none()); // only metadata stored in the header have been loaded previously
    }
    // temperature_f64
    {
        assert!(data_set.has_var("temperature_f64"));
        let var: &Variable = data_set.get_var("temperature_f64").unwrap();
        assert_eq!(DataType::F64, var.data_type());
        assert!(var.get_f64().is_none()); // only metadata stored in the header have been loaded previously
    }
}

#[test]
fn test_file_reader_read_all_vars()
{
    // Copy bytes to a temporary file
    let (tmp_dir, input_data_file_path) = copy_bytes_to_tmp_file(NC3_CLASSIC_FILE_BYTES, NC3_CLASSIC_FILE_NAME);

    // Read the data set
    let data_set: DataSet = {
        let mut file_reader = FileReader::open(input_data_file_path).unwrap();
        file_reader.read_all_vars().unwrap();
        file_reader.close().0
    };
    tmp_dir.close().unwrap();

    // Check all variable data have been loaded from the file
    // latitude
    {
        assert!(data_set.has_var("latitude"));
        let var: &Variable = data_set.get_var("latitude").unwrap();
        assert_eq!(DataType::F32, var.data_type());
        assert!(var.get_f32().is_some());
    }
    // longitude
    {
        assert!(data_set.has_var("longitude"));
        let var: &Variable = data_set.get_var("longitude").unwrap();
        assert_eq!(DataType::F32, var.data_type());
        assert!(var.get_f32().is_some());
    }
    // time
    {
        assert!(data_set.has_var("time"));
        let var: &Variable = data_set.get_var("time").unwrap();
        assert_eq!(DataType::F32, var.data_type());
        assert!(var.get_f32().is_some());
    }
    // temperature_i8
    {
        assert!(data_set.has_var("temperature_i8"));
        let var: &Variable = data_set.get_var("temperature_i8").unwrap();
        assert_eq!(DataType::I8, var.data_type());
        assert!(var.get_i8().is_some());
    }
    // temperature_u8
    {
        assert!(data_set.has_var("temperature_u8"));
        let var: &Variable = data_set.get_var("temperature_u8").unwrap();
        assert_eq!(DataType::U8, var.data_type());
        assert!(var.get_u8().is_some());
    }
    // temperature_i16
    {
        assert!(data_set.has_var("temperature_i16"));
        let var: &Variable = data_set.get_var("temperature_i16").unwrap();
        assert_eq!(DataType::I16, var.data_type());
        assert!(var.get_i16().is_some());
    }
    // temperature_i32
    {
        assert!(data_set.has_var("temperature_i32"));
        let var: &Variable = data_set.get_var("temperature_i32").unwrap();
        assert_eq!(DataType::I32, var.data_type());
        assert!(var.get_i32().is_some());
    }
    // temperature_f32
    {
        assert!(data_set.has_var("temperature_f32"));
        let var: &Variable = data_set.get_var("temperature_f32").unwrap();
        assert_eq!(DataType::F32, var.data_type());
        assert!(var.get_f32().is_some());
    }
    // temperature_f64
    {
        assert!(data_set.has_var("temperature_f64"));
        let var: &Variable = data_set.get_var("temperature_f64").unwrap();
        assert_eq!(DataType::F64, var.data_type());
        assert!(var.get_f64().is_some());
    }
}

#[test]
fn test_file_reader_read_vars()
{
    // Copy bytes to a temporary file
    let (tmp_dir, input_data_file_path) = copy_bytes_to_tmp_file(NC3_CLASSIC_FILE_BYTES, NC3_CLASSIC_FILE_NAME);

    // Read the data set
    let data_set: DataSet = {
        let mut file_reader = FileReader::open(input_data_file_path).unwrap();
        // Load 3 variables
        file_reader.read_vars(&["latitude", "longitude"]).unwrap();
        file_reader.read_var("temperature_i32").unwrap();

        // Test the reading of undefined variables
        match file_reader.read_vars(&["undef_var_1", "undef_var_2"]) {
            Err(err) => {
                assert_eq!(
                    IOError::ReadData(ReadDataError::VariablesNotDefined(vec![String::from("undef_var_1"), String::from("undef_var_2")])),
                    err,
                )
            },
            _ => panic!("Unexpected error."),
        }
        match file_reader.read_var("undef_var_3") {
            Err(err) => {
                assert_eq!(
                    IOError::ReadData(ReadDataError::VariablesNotDefined(vec![String::from("undef_var_3")])),
                    err,
                )
            },
            _ => panic!("Unexpected error."),
        }
        file_reader.close().0
    };
    tmp_dir.close().unwrap();

    // Check variables previously loaded
    // ---------------------------------
    // latitude
    {
        assert!(data_set.has_var("latitude"));
        let var: &Variable = data_set.get_var("latitude").unwrap();
        assert_eq!(DataType::F32, var.data_type());
        assert!(var.get_f32().is_some());
    }
    // longitude
    {
        assert!(data_set.has_var("longitude"));
        let var: &Variable = data_set.get_var("longitude").unwrap();
        assert_eq!(DataType::F32, var.data_type());
        assert!(var.get_f32().is_some());
    }
    // temperature_i32
    {
        assert!(data_set.has_var("temperature_i32"));
        let var: &Variable = data_set.get_var("temperature_i32").unwrap();
        assert_eq!(DataType::I32, var.data_type());
        assert!(var.get_i32().is_some());
    }

    // Check variables not previously loaded
    // -------------------------------------
    // time
    {
        assert!(data_set.has_var("time"));
        let var: &Variable = data_set.get_var("time").unwrap();
        assert_eq!(DataType::F32, var.data_type());
        assert!(var.get_f32().is_none());
    }
    // temperature_i8
    {
        assert!(data_set.has_var("temperature_i8"));
        let var: &Variable = data_set.get_var("temperature_i8").unwrap();
        assert_eq!(DataType::I8, var.data_type());
        assert!(var.get_i8().is_none());
    }
    // temperature_u8
    {
        assert!(data_set.has_var("temperature_u8"));
        let var: &Variable = data_set.get_var("temperature_u8").unwrap();
        assert_eq!(DataType::U8, var.data_type());
        assert!(var.get_u8().is_none());
    }
    // temperature_i16
    {
        assert!(data_set.has_var("temperature_i16"));
        let var: &Variable = data_set.get_var("temperature_i16").unwrap();
        assert_eq!(DataType::I16, var.data_type());
        assert!(var.get_i16().is_none());
    }

    // temperature_f32
    {
        assert!(data_set.has_var("temperature_f32"));
        let var: &Variable = data_set.get_var("temperature_f32").unwrap();
        assert_eq!(DataType::F32, var.data_type());
        assert!(var.get_f32().is_none());
    }
    // temperature_f64
    {
        assert!(data_set.has_var("temperature_f64"));
        let var: &Variable = data_set.get_var("temperature_f64").unwrap();
        assert_eq!(DataType::F64, var.data_type());
        assert!(var.get_f64().is_none());
    }
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
                let num_bytes_zero_padding: usize = compute_num_bytes_zero_padding(num_of_bytes);
                for _ in 0..num_bytes_zero_padding
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
                let num_bytes_zero_padding: usize = compute_num_bytes_zero_padding(num_of_bytes);
                for _ in 0..num_bytes_zero_padding
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
                let num_bytes_zero_padding: usize = compute_num_bytes_zero_padding(num_of_bytes);
                assert!(num_bytes_zero_padding > 0);
                for i in 0..num_bytes_zero_padding
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
                let num_bytes_zero_padding: usize = compute_num_bytes_zero_padding(num_of_bytes);
                for _ in 0..num_bytes_zero_padding
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
                let num_bytes_zero_padding: usize = compute_num_bytes_zero_padding(num_of_bytes);
                for _ in 0..num_bytes_zero_padding
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
                let num_bytes_zero_padding: usize = compute_num_bytes_zero_padding(num_of_bytes);
                for _ in 0..num_bytes_zero_padding
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