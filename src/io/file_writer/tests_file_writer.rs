#![cfg(test)]
use std::rc::Rc;
use std::io::{Read, Cursor};
use std::path::PathBuf;

use tempdir::TempDir;

use byteorder::{ReadBytesExt, BigEndian};

use crate::Dimension;

use crate::FileReader;
use crate::error::WriteError;

use super::{
    FileWriter, DataSet, Version,
    ABSENT_TAG, DIMENSION_TAG,
};


#[test]
fn test_file_writer_open() {
    const TMP_DIR_PREFIX: &str = "netcdf3_tests_";
    const TEST_FILE_NAME: &str = "example.nc";
    const GLOBAL_ATTR_NAME_1: &str = "comment_1";
    const GLOBAL_ATTR_NAME_2: &str = "comment_2";

    let tmp_dir: TempDir = TempDir::new(TMP_DIR_PREFIX).unwrap();
    let test_file_path: PathBuf = tmp_dir.path().join(TEST_FILE_NAME);
    assert_eq!(false,                test_file_path.exists());
    
    // First create a new NetCDF-3 file
    let mut file_writer_1: FileWriter = FileWriter::open(&test_file_path).unwrap();
    let mut data_set_1 = DataSet::new();
    data_set_1.add_global_attr_string(GLOBAL_ATTR_NAME_1, "test_file_1").unwrap();
    file_writer_1.set_def(&data_set_1, Version::Classic, 0).unwrap();
    file_writer_1.close().unwrap();
    assert_eq!(true,                test_file_path.exists());

    // Reopen the same NetCDF-3 file and set an other global attributes
    let mut file_writer_2: FileWriter = FileWriter::open(&test_file_path).unwrap();
    let mut data_set_2 = DataSet::new();
    data_set_2.add_global_attr_string(GLOBAL_ATTR_NAME_2, "test_file_2").unwrap();
    file_writer_2.set_def(&data_set_2, Version::Classic, 0).unwrap();
    file_writer_2.close().unwrap();
    assert_eq!(true,                test_file_path.exists());

    // Read the outlet file
    let file_reader: FileReader = FileReader::open(&test_file_path).unwrap();
    let data_set_3: DataSet = file_reader.close().0;
    assert_eq!(1,                   data_set_3.num_global_attrs());
    assert_eq!(false,               data_set_3.has_global_attr(GLOBAL_ATTR_NAME_1));
    assert_eq!(true,                data_set_3.has_global_attr(GLOBAL_ATTR_NAME_2));
    assert_eq!("test_file_2",       data_set_3.get_global_attr_as_string(GLOBAL_ATTR_NAME_2).unwrap());

    tmp_dir.close().unwrap();
}

#[test]
fn test_file_writer_create_new() {
    const TMP_DIR_PREFIX: &str = "netcdf3_tests_";
    const TEST_FILE_NAME: &str = "example.nc";
    const GLOBAL_ATTR_NAME: &str = "comment_1";

    let tmp_dir: TempDir = TempDir::new(TMP_DIR_PREFIX).unwrap();
    let test_file_path: PathBuf = tmp_dir.path().join(TEST_FILE_NAME);
    assert_eq!(false,               test_file_path.exists());

    // First create a new NetCDF-3 file
    let mut file_writer_1: FileWriter = FileWriter::create_new(&test_file_path).unwrap();
    let mut data_set_1 = DataSet::new();
    data_set_1.add_global_attr_string(GLOBAL_ATTR_NAME, "test_file_1").unwrap();
    file_writer_1.set_def(&data_set_1, Version::Classic, 0).unwrap();  // set an empty data set
    file_writer_1.close().unwrap();
    assert_eq!(true,                test_file_path.exists());

    // Try to recreate the already existing file
    assert_eq!(
        WriteError::IOErrorKind(std::io::ErrorKind::AlreadyExists),
        FileWriter::create_new(&test_file_path).unwrap_err(),
    );
    assert_eq!(true,                test_file_path.exists());

    // The first file has not been overwritten
    let file_reader: FileReader = FileReader::open(&test_file_path).unwrap();
    let data_set_2: DataSet = file_reader.close().0;
    assert_eq!(1,                   data_set_2.num_global_attrs());
    assert_eq!(true,                data_set_2.has_global_attr(GLOBAL_ATTR_NAME));
    assert_eq!("test_file_1",       data_set_2.get_global_attr_as_string(GLOBAL_ATTR_NAME).unwrap());

    tmp_dir.close().unwrap();
}

#[test]
fn test_write_dims_list() {

    // Empty dimension list
    {
        let bytes: Vec<u8> = {
            let mut bytes: Vec<u8> = vec![];
            let _ = FileWriter::write_dims_list(&mut bytes, &[]).unwrap();
            bytes
        };

        assert_eq!(ABSENT_TAG.len(),    bytes.len());
        assert_eq!(&ABSENT_TAG[..],     &bytes[..]);
    }

    // One *fixed_size* dimension list
    {
        const DIM_NAME: &str = "dim_1";
        const DIM_SIZE: usize = 10;
        let mut cursor: Cursor<Vec<u8>> = {
            let dim_1 = Rc::new(Dimension::new_fixed_size(DIM_NAME, DIM_SIZE).unwrap());

            let mut bytes: Vec<u8> = vec![];
            let _ = FileWriter::write_dims_list(&mut bytes, &[dim_1]).unwrap();
            Cursor::new(bytes)
        };

        let mut buffer: Vec<u8> = vec![0_u8; 4];
        cursor.read(&mut buffer).unwrap();
        assert_eq!(&DIMENSION_TAG[..],  &buffer[..]);
        // the number of dimensions
        assert_eq!(1,                   cursor.read_i32::<BigEndian>().unwrap());
        // the number of useful bytes for the dim name
        assert_eq!(5,                   cursor.read_i32::<BigEndian>().unwrap());
        // the dim name bytes
        assert_eq!('d' as u8,           cursor.read_u8().unwrap());
        assert_eq!('i' as u8,           cursor.read_u8().unwrap());
        assert_eq!('m' as u8,           cursor.read_u8().unwrap());
        assert_eq!('_' as u8,           cursor.read_u8().unwrap());
        assert_eq!('1' as u8,           cursor.read_u8().unwrap());
        // the zero padding bytes
        assert_eq!(0,                   cursor.read_u8().unwrap());
        assert_eq!(0,                   cursor.read_u8().unwrap());
        assert_eq!(0,                   cursor.read_u8().unwrap());
        // the dimension size
        assert_eq!(DIM_SIZE as i32,     cursor.read_i32::<BigEndian>().unwrap());
        // no byte remaining
        assert_eq!(0,                   cursor.read_to_end(&mut buffer).unwrap());
    }

    // One *unlimited_size* dimension list
    {
        const DIM_NAME: &str = "dim_1";
        const DIM_SIZE: usize = 10;
        let mut cursor: Cursor<Vec<u8>> = {
            let dim_1 = Rc::new(Dimension::new_unlimited_size(DIM_NAME, DIM_SIZE).unwrap());

            let mut bytes: Vec<u8> = vec![];
            let _ = FileWriter::write_dims_list(&mut bytes, &[dim_1]).unwrap();
            Cursor::new(bytes)
        };

        let mut buffer: Vec<u8> = vec![0_u8; 4];
        cursor.read(&mut buffer).unwrap();
        assert_eq!(&DIMENSION_TAG[..],  &buffer[..]);
        // the number of dimensions
        assert_eq!(1,                   cursor.read_i32::<BigEndian>().unwrap());
        // the number of useful bytes for the dim name
        assert_eq!(5,                   cursor.read_i32::<BigEndian>().unwrap());
        // the dim name bytes
        assert_eq!('d' as u8,           cursor.read_u8().unwrap());
        assert_eq!('i' as u8,           cursor.read_u8().unwrap());
        assert_eq!('m' as u8,           cursor.read_u8().unwrap());
        assert_eq!('_' as u8,           cursor.read_u8().unwrap());
        assert_eq!('1' as u8,           cursor.read_u8().unwrap());
        // the zero padding bytes
        assert_eq!(0,                   cursor.read_u8().unwrap());
        assert_eq!(0,                   cursor.read_u8().unwrap());
        assert_eq!(0,                   cursor.read_u8().unwrap());
        // the dimension size: the unlimited dimension is not record here
        assert_eq!(0 as i32,     cursor.read_i32::<BigEndian>().unwrap());
        // no byte remaining
        assert_eq!(0,                   cursor.read_to_end(&mut buffer).unwrap());
    }
}

#[test]
fn test_write_name_string() {

    // 1 ASCII character string
    {
        let mut cursor: Cursor<Vec<u8>> = {
            let mut bytes: Vec<u8> = vec![];
            FileWriter::write_name_string(&mut bytes, "a").unwrap();
            Cursor::new(bytes)
        };

        assert_eq!(8,           cursor.get_ref().len());
        assert_eq!(1,           cursor.read_i32::<BigEndian>().unwrap());  // the number of useful bytes
        assert_eq!('a' as u8,   cursor.read_u8().unwrap());
        assert_eq!(0,           cursor.read_u8().unwrap());
        assert_eq!(0,           cursor.read_u8().unwrap());
        assert_eq!(0,           cursor.read_u8().unwrap());
    }

    // 4 ASCII characters string
    {
        let mut cursor: Cursor<Vec<u8>> = {
            let mut bytes: Vec<u8> = vec![];
            FileWriter::write_name_string(&mut bytes, "abcd").unwrap();
            Cursor::new(bytes)
        };

        assert_eq!(8,           cursor.get_ref().len());
        assert_eq!(4,           cursor.read_i32::<BigEndian>().unwrap());  // the number of useful bytes
        assert_eq!('a' as u8,   cursor.read_u8().unwrap());
        assert_eq!('b' as u8,   cursor.read_u8().unwrap());
        assert_eq!('c' as u8,   cursor.read_u8().unwrap());
        assert_eq!('d' as u8,   cursor.read_u8().unwrap());
    }

    // 5 ASCII characters string
    {
        let mut cursor: Cursor<Vec<u8>> = {
            let mut bytes: Vec<u8> = vec![];
            FileWriter::write_name_string(&mut bytes, "abcde").unwrap();
            Cursor::new(bytes)
        };

        assert_eq!(12,          cursor.get_ref().len());
        assert_eq!(5,           cursor.read_i32::<BigEndian>().unwrap());  // the number of useful cursor
        assert_eq!('a' as u8,   cursor.read_u8().unwrap());
        assert_eq!('b' as u8,   cursor.read_u8().unwrap());
        assert_eq!('c' as u8,   cursor.read_u8().unwrap());
        assert_eq!('d' as u8,   cursor.read_u8().unwrap());
        assert_eq!('e' as u8,   cursor.read_u8().unwrap());
        assert_eq!(0,           cursor.read_u8().unwrap());
        assert_eq!(0,           cursor.read_u8().unwrap());
        assert_eq!(0,           cursor.read_u8().unwrap());
    }

    // UTF-8 encoded string
    {
        let mut cursor: Cursor<Vec<u8>> = {
            let mut bytes: Vec<u8> = vec![];
            FileWriter::write_name_string(&mut bytes, "café").unwrap();
            Cursor::new(bytes)
        };

        assert_eq!(12,          cursor.get_ref().len());
        assert_eq!(5,           cursor.read_i32::<BigEndian>().unwrap());  // the number of useful cursor
        assert_eq!('c' as u8,   cursor.read_u8().unwrap());
        assert_eq!('a' as u8,   cursor.read_u8().unwrap());
        assert_eq!('f' as u8,   cursor.read_u8().unwrap());
        assert_eq!(0xc3,        cursor.read_u8().unwrap());
        assert_eq!(0xa9,        cursor.read_u8().unwrap());
        assert_eq!(0,           cursor.read_u8().unwrap());
        assert_eq!(0,           cursor.read_u8().unwrap());
        assert_eq!(0,           cursor.read_u8().unwrap());
    }
}