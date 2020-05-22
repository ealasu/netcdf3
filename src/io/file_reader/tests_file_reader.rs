#![cfg(test)]

use byteorder::{WriteBytesExt, BigEndian};

use crate::{
    FileReader, DataSet, DataType,
    error::ReadError,
    error::parse_header_error::{ParseHeaderError, ParseHeaderErrorKind, InvalidBytes},
    io::compute_padding_size,
};

use copy_to_tmp_file::{
    copy_bytes_to_tmp_file,
    NC3_CLASSIC_FILE_NAME, NC3_CLASSIC_FILE_BYTES,
};

const TEMP_I8_VAR_NAME: &str = "temperature_i8";
const TEMP_I8_VAR_DATA: [i8; 30] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29];

const TEMP_U8_VAR_NAME: &str = "temperature_u8";
const TEMP_U8_VAR_DATA: [u8; 30] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29];

const TEMP_I16_VAR_NAME: &str = "temperature_i16";
const TEMP_I16_VAR_DATA: [i16; 30] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29];

const TEMP_I32_VAR_NAME: &str = "temperature_i32";
const TEMP_I32_VAR_DATA: [i32; 30] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29];

const TEMP_F32_VAR_NAME: &str = "temperature_f32";
const TEMP_F32_VAR_DATA: [f32; 30] = [0., 1., 2., 3., 4., 5., 6., 7., 8., 9., 10., 11., 12., 13., 14., 15., 16., 17., 18., 19., 20., 21., 22., 23., 24., 25., 26., 27., 28., 29.];

const TEMP_F64_VAR_NAME: &str = "temperature_f64";
const TEMP_F64_VAR_DATA: [f64; 30] = [0., 1., 2., 3., 4., 5., 6., 7., 8., 9., 10., 11., 12., 13., 14., 15., 16., 17., 18., 19., 20., 21., 22., 23., 24., 25., 26., 27., 28., 29.];


#[test]
fn test_file_reader_read_var_to_i8() {
    let (tmp_dir, input_data_file_path) = copy_bytes_to_tmp_file(NC3_CLASSIC_FILE_BYTES, NC3_CLASSIC_FILE_NAME);

    let mut file_reader = FileReader::open(input_data_file_path).unwrap();

    {
        let data_set: &DataSet = file_reader.data_set();
        assert_eq!(true,                            data_set.has_var(TEMP_I8_VAR_NAME));
        assert_eq!(Some(DataType::I8),              data_set.var_data_type(TEMP_I8_VAR_NAME));
    }

    assert_eq!(Ok(TEMP_I8_VAR_DATA.to_vec()), file_reader.read_var_to_i8(TEMP_I8_VAR_NAME));

    assert_eq!(
        ReadError::VariableMismatchDataType{var_name: String::from(TEMP_U8_VAR_NAME), req: DataType::U8, get: DataType::I8},
        file_reader.read_var_to_i8(TEMP_U8_VAR_NAME).unwrap_err()
    );
    assert_eq!(
        ReadError::VariableMismatchDataType{var_name: String::from(TEMP_I16_VAR_NAME), req: DataType::I16, get: DataType::I8},
        file_reader.read_var_to_i8(TEMP_I16_VAR_NAME).unwrap_err()
    );
    assert_eq!(
        ReadError::VariableMismatchDataType{var_name: String::from(TEMP_I32_VAR_NAME), req: DataType::I32, get: DataType::I8},
        file_reader.read_var_to_i8(TEMP_I32_VAR_NAME).unwrap_err()
    );
    assert_eq!(
        ReadError::VariableMismatchDataType{var_name: String::from(TEMP_F32_VAR_NAME), req: DataType::F32, get: DataType::I8},
        file_reader.read_var_to_i8(TEMP_F32_VAR_NAME).unwrap_err()
    );
    assert_eq!(
        ReadError::VariableMismatchDataType{var_name: String::from(TEMP_F64_VAR_NAME), req: DataType::F64, get: DataType::I8},
        file_reader.read_var_to_i8(TEMP_F64_VAR_NAME).unwrap_err()
    );
    assert_eq!(
        ReadError::VariableNotDefined(String::from("undef_var")),
        file_reader.read_var_to_u8("undef_var").unwrap_err()
    );

    let data_set: DataSet = file_reader.close().0;
    tmp_dir.close().unwrap();

    assert_eq!(true,                            data_set.has_var(TEMP_U8_VAR_NAME));
    assert_eq!(Some(DataType::U8),              data_set.var_data_type(TEMP_U8_VAR_NAME));
}

#[test]
fn test_file_reader_read_var_to_u8() {
    let (tmp_dir, input_data_file_path) = copy_bytes_to_tmp_file(NC3_CLASSIC_FILE_BYTES, NC3_CLASSIC_FILE_NAME);

    let mut file_reader = FileReader::open(input_data_file_path).unwrap();

    {
        let data_set: &DataSet = file_reader.data_set();
        assert_eq!(true,                            data_set.has_var(TEMP_U8_VAR_NAME));
        assert_eq!(Some(DataType::U8),              data_set.var_data_type(TEMP_U8_VAR_NAME));
    }

    assert_eq!(Ok(TEMP_U8_VAR_DATA.to_vec()), file_reader.read_var_to_u8(TEMP_U8_VAR_NAME));

    assert_eq!(
        ReadError::VariableMismatchDataType{var_name: String::from(TEMP_I8_VAR_NAME), req: DataType::I8, get: DataType::U8},
        file_reader.read_var_to_u8(TEMP_I8_VAR_NAME).unwrap_err()
    );
    assert_eq!(
        ReadError::VariableMismatchDataType{var_name: String::from(TEMP_I16_VAR_NAME), req: DataType::I16, get: DataType::U8},
        file_reader.read_var_to_u8(TEMP_I16_VAR_NAME).unwrap_err()
    );
    assert_eq!(
        ReadError::VariableMismatchDataType{var_name: String::from(TEMP_I32_VAR_NAME), req: DataType::I32, get: DataType::U8},
        file_reader.read_var_to_u8(TEMP_I32_VAR_NAME).unwrap_err()
    );
    assert_eq!(
        ReadError::VariableMismatchDataType{var_name: String::from(TEMP_F32_VAR_NAME), req: DataType::F32, get: DataType::U8},
        file_reader.read_var_to_u8(TEMP_F32_VAR_NAME).unwrap_err()
    );
    assert_eq!(
        ReadError::VariableMismatchDataType{var_name: String::from(TEMP_F64_VAR_NAME), req: DataType::F64, get: DataType::U8},
        file_reader.read_var_to_u8(TEMP_F64_VAR_NAME).unwrap_err()
    );
    assert_eq!(
        ReadError::VariableNotDefined(String::from("undef_var")),
        file_reader.read_var_to_u8("undef_var").unwrap_err()
    );

    let data_set: DataSet = file_reader.close().0;
    tmp_dir.close().unwrap();

    assert_eq!(true,                            data_set.has_var(TEMP_U8_VAR_NAME));
    assert_eq!(Some(DataType::U8),              data_set.var_data_type(TEMP_U8_VAR_NAME));
}

#[test]
fn test_file_reader_read_var_to_i16() {
    let (tmp_dir, input_data_file_path) = copy_bytes_to_tmp_file(NC3_CLASSIC_FILE_BYTES, NC3_CLASSIC_FILE_NAME);

    let mut file_reader = FileReader::open(input_data_file_path).unwrap();

    {
        let data_set: &DataSet = file_reader.data_set();
        assert_eq!(true,                            data_set.has_var(TEMP_I16_VAR_NAME));
        assert_eq!(Some(DataType::I16),             data_set.var_data_type(TEMP_I16_VAR_NAME));
    }

    assert_eq!(Ok(TEMP_I16_VAR_DATA.to_vec()),      file_reader.read_var_to_i16(TEMP_I16_VAR_NAME));

    assert_eq!(
        ReadError::VariableMismatchDataType{var_name: String::from(TEMP_I8_VAR_NAME), req: DataType::I8, get: DataType::I16},
        file_reader.read_var_to_i16(TEMP_I8_VAR_NAME).unwrap_err()
    );
    assert_eq!(
        ReadError::VariableMismatchDataType{var_name: String::from(TEMP_U8_VAR_NAME), req: DataType::U8, get: DataType::I16},
        file_reader.read_var_to_i16(TEMP_U8_VAR_NAME).unwrap_err()
    );
    assert_eq!(
        ReadError::VariableMismatchDataType{var_name: String::from(TEMP_I32_VAR_NAME), req: DataType::I32, get: DataType::I16},
        file_reader.read_var_to_i16(TEMP_I32_VAR_NAME).unwrap_err()
    );
    assert_eq!(
        ReadError::VariableMismatchDataType{var_name: String::from(TEMP_F32_VAR_NAME), req: DataType::F32, get: DataType::I16},
        file_reader.read_var_to_i16(TEMP_F32_VAR_NAME).unwrap_err()
    );
    assert_eq!(
        ReadError::VariableMismatchDataType{var_name: String::from(TEMP_F64_VAR_NAME), req: DataType::F64, get: DataType::I16},
        file_reader.read_var_to_i16(TEMP_F64_VAR_NAME).unwrap_err()
    );
    assert_eq!(
        ReadError::VariableNotDefined(String::from("undef_var")),
        file_reader.read_var_to_i16("undef_var").unwrap_err()
    );

    let data_set: DataSet = file_reader.close().0;
    tmp_dir.close().unwrap();
    assert_eq!(true,                            data_set.has_var(TEMP_I16_VAR_NAME));
    assert_eq!(Some(DataType::I16),              data_set.var_data_type(TEMP_I16_VAR_NAME));
}

#[test]
fn test_file_reader_read_var_to_i32() {
    let (tmp_dir, input_data_file_path) = copy_bytes_to_tmp_file(NC3_CLASSIC_FILE_BYTES, NC3_CLASSIC_FILE_NAME);

    let mut file_reader = FileReader::open(input_data_file_path).unwrap();

    {
        let data_set: &DataSet = file_reader.data_set();
        assert_eq!(true,                            data_set.has_var(TEMP_I32_VAR_NAME));
        assert_eq!(Some(DataType::I32),             data_set.var_data_type(TEMP_I32_VAR_NAME));
    }

    assert_eq!(Ok(TEMP_I32_VAR_DATA.to_vec()),      file_reader.read_var_to_i32(TEMP_I32_VAR_NAME));

    assert_eq!(
        ReadError::VariableMismatchDataType{var_name: String::from(TEMP_I8_VAR_NAME), req: DataType::I8, get: DataType::I32},
        file_reader.read_var_to_i32(TEMP_I8_VAR_NAME).unwrap_err()
    );
    assert_eq!(
        ReadError::VariableMismatchDataType{var_name: String::from(TEMP_U8_VAR_NAME), req: DataType::U8, get: DataType::I32},
        file_reader.read_var_to_i32(TEMP_U8_VAR_NAME).unwrap_err()
    );
    assert_eq!(
        ReadError::VariableMismatchDataType{var_name: String::from(TEMP_I16_VAR_NAME), req: DataType::I16, get: DataType::I32},
        file_reader.read_var_to_i32(TEMP_I16_VAR_NAME).unwrap_err()
    );
    assert_eq!(
        ReadError::VariableMismatchDataType{var_name: String::from(TEMP_F32_VAR_NAME), req: DataType::F32, get: DataType::I32},
        file_reader.read_var_to_i32(TEMP_F32_VAR_NAME).unwrap_err()
    );
    assert_eq!(
        ReadError::VariableMismatchDataType{var_name: String::from(TEMP_F64_VAR_NAME), req: DataType::F64, get: DataType::I32},
        file_reader.read_var_to_i32(TEMP_F64_VAR_NAME).unwrap_err()
    );
    assert_eq!(
        ReadError::VariableNotDefined(String::from("undef_var")),
        file_reader.read_var_to_i32("undef_var").unwrap_err()
    );

    let data_set: DataSet = file_reader.close().0;
    tmp_dir.close().unwrap();
    assert_eq!(true,                            data_set.has_var(TEMP_I32_VAR_NAME));
    assert_eq!(Some(DataType::I32),             data_set.var_data_type(TEMP_I32_VAR_NAME));
}

#[test]
fn test_file_reader_read_var_to_f32() {
    let (tmp_dir, input_data_file_path) = copy_bytes_to_tmp_file(NC3_CLASSIC_FILE_BYTES, NC3_CLASSIC_FILE_NAME);

    let mut file_reader = FileReader::open(input_data_file_path).unwrap();

    {
        let data_set: &DataSet = file_reader.data_set();
        assert_eq!(true,                            data_set.has_var(TEMP_F32_VAR_NAME));
        assert_eq!(Some(DataType::F32),             data_set.var_data_type(TEMP_F32_VAR_NAME));
    }

    assert_eq!(Ok(TEMP_F32_VAR_DATA.to_vec()),      file_reader.read_var_to_f32(TEMP_F32_VAR_NAME));

    assert_eq!(
        ReadError::VariableMismatchDataType{var_name: String::from(TEMP_I8_VAR_NAME), req: DataType::I8, get: DataType::F32},
        file_reader.read_var_to_f32(TEMP_I8_VAR_NAME).unwrap_err()
    );
    assert_eq!(
        ReadError::VariableMismatchDataType{var_name: String::from(TEMP_U8_VAR_NAME), req: DataType::U8, get: DataType::F32},
        file_reader.read_var_to_f32(TEMP_U8_VAR_NAME).unwrap_err()
    );
    assert_eq!(
        ReadError::VariableMismatchDataType{var_name: String::from(TEMP_I16_VAR_NAME), req: DataType::I16, get: DataType::F32},
        file_reader.read_var_to_f32(TEMP_I16_VAR_NAME).unwrap_err()
    );
    assert_eq!(
        ReadError::VariableMismatchDataType{var_name: String::from(TEMP_I32_VAR_NAME), req: DataType::I32, get: DataType::F32},
        file_reader.read_var_to_f32(TEMP_I32_VAR_NAME).unwrap_err()
    );
    assert_eq!(
        ReadError::VariableMismatchDataType{var_name: String::from(TEMP_F64_VAR_NAME), req: DataType::F64, get: DataType::F32},
        file_reader.read_var_to_f32(TEMP_F64_VAR_NAME).unwrap_err()
    );
    assert_eq!(
        ReadError::VariableNotDefined(String::from("undef_var")),
        file_reader.read_var_to_f32("undef_var").unwrap_err()
    );

    let data_set: DataSet = file_reader.close().0;
    tmp_dir.close().unwrap();
    assert_eq!(true,                            data_set.has_var(TEMP_F32_VAR_NAME));
    assert_eq!(Some(DataType::F32),             data_set.var_data_type(TEMP_F32_VAR_NAME));
}

#[test]
fn test_file_reader_read_var_to_f64() {
    let (tmp_dir, input_data_file_path) = copy_bytes_to_tmp_file(NC3_CLASSIC_FILE_BYTES, NC3_CLASSIC_FILE_NAME);

    let mut file_reader = FileReader::open(input_data_file_path).unwrap();

    {
        let data_set: &DataSet = file_reader.data_set();
        assert_eq!(true,                            data_set.has_var(TEMP_F64_VAR_NAME));
        assert_eq!(Some(DataType::F64),             data_set.var_data_type(TEMP_F64_VAR_NAME));
    }

    assert_eq!(Ok(TEMP_F64_VAR_DATA.to_vec()),      file_reader.read_var_to_f64(TEMP_F64_VAR_NAME));

    assert_eq!(
        ReadError::VariableMismatchDataType{var_name: String::from(TEMP_I8_VAR_NAME), req: DataType::I8, get: DataType::F64},
        file_reader.read_var_to_f64(TEMP_I8_VAR_NAME).unwrap_err()
    );
    assert_eq!(
        ReadError::VariableMismatchDataType{var_name: String::from(TEMP_U8_VAR_NAME), req: DataType::U8, get: DataType::F64},
        file_reader.read_var_to_f64(TEMP_U8_VAR_NAME).unwrap_err()
    );
    assert_eq!(
        ReadError::VariableMismatchDataType{var_name: String::from(TEMP_I16_VAR_NAME), req: DataType::I16, get: DataType::F64},
        file_reader.read_var_to_f64(TEMP_I16_VAR_NAME).unwrap_err()
    );
    assert_eq!(
        ReadError::VariableMismatchDataType{var_name: String::from(TEMP_I32_VAR_NAME), req: DataType::I32, get: DataType::F64},
        file_reader.read_var_to_f64(TEMP_I32_VAR_NAME).unwrap_err()
    );
    assert_eq!(
        ReadError::VariableMismatchDataType{var_name: String::from(TEMP_F32_VAR_NAME), req: DataType::F32, get: DataType::F64},
        file_reader.read_var_to_f64(TEMP_F32_VAR_NAME).unwrap_err()
    );
    assert_eq!(
        ReadError::VariableNotDefined(String::from("undef_var")),
        file_reader.read_var_to_f64("undef_var").unwrap_err()
    );

    let data_set: DataSet = file_reader.close().0;
    tmp_dir.close().unwrap();
    assert_eq!(true,                            data_set.has_var(TEMP_F64_VAR_NAME));
    assert_eq!(Some(DataType::F64),             data_set.var_data_type(TEMP_F64_VAR_NAME));
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
        assert_eq!(ParseHeaderErrorKind::NonNegativeI32 ,parsing_err.kind);
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
        assert_eq!(ParseHeaderErrorKind::NonNegativeI32 ,parsing_err.kind);
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
        assert_eq!(ParseHeaderErrorKind::NonNegativeI32 ,parsing_err.kind);
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
                let zero_padding_size: usize = compute_padding_size(num_of_bytes);
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
                let zero_padding_size: usize = compute_padding_size(num_of_bytes);
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
                let zero_padding_size: usize = compute_padding_size(num_of_bytes);
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
            assert_eq!(ParseHeaderErrorKind::ZeroPadding ,parsing_err.kind);
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
                let zero_padding_size: usize = compute_padding_size(num_of_bytes);
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
                let zero_padding_size: usize = compute_padding_size(num_of_bytes);
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
            assert_eq!(ParseHeaderErrorKind::Utf8 ,parsing_err.kind);
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
                let zero_padding_size: usize = compute_padding_size(num_of_bytes);
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
            assert_eq!(ParseHeaderErrorKind::ZeroPadding ,parsing_err.kind);
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
        assert_eq!(ParseHeaderErrorKind::NonNegativeI32, parsing_err.kind);
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
        assert_eq!(ParseHeaderErrorKind::NonNegativeI32, parsing_err.kind);
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
            ParseHeaderErrorKind::ZeroPadding,
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
            ParseHeaderErrorKind::ZeroPadding,
            parsing_err.kind,
        );
        assert_eq!(
            InvalidBytes::Incomplete(nom::Needed::Size(3)),
            parsing_err.invalid_bytes
        );
    }
}
