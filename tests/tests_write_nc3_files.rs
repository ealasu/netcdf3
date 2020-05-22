#![cfg(test)]
//! This test module checks the binary diff beetween 2 NetCDF-3 files.
//!
//! One of theses file has been produced with the Rust crate `netcdf3`
//! while the other file has been produced with the Python package [`netCDF4`](https://github.com/Unidata/netcdf4-python).
use std::io::Read;
use std::path::Path;

use tempdir::TempDir;

use netcdf3::{FileWriter, DataSet, Variable, Version};

use copy_to_tmp_file::{
    NC3_FILL_VALUES_FILE_NAME, NC3_FILL_VALUES_FILE_BYTES,
    SCALAR_VARIABLES_FILE_NAME, SCALAR_VARIABLES_FILE_BYTES,
    EMPTY_DATA_SET_FILE_NAME, EMPTY_DATA_SET_FILE_BYTES,
    NC3_CLASSIC_FILE_NAME, NC3_CLASSIC_FILE_BYTES,
    NC3_64BIT_OFFSET_FILE_NAME, NC3_64BIT_OFFSET_FILE_BYTES,
};

/// Prefix of the temporary output directories
const TMP_DIR_PREFIX: &str = "netcdf3_tests_";

#[test]
fn test_write_file_nc_fill_values() {
    fn write_file_nc_fill_values<P: AsRef<Path>>(file_path: P) {
        let data_set: DataSet = {
            let mut data_set: DataSet = DataSet::new();

            data_set.add_var_i8::<&str>("nc_fill_value_i8", &[]).unwrap();
            data_set.add_var_u8::<&str>("nc_fill_value_u8", &[]).unwrap();
            data_set.add_var_i16::<&str>("nc_fill_value_i16", &[]).unwrap();
            data_set.add_var_i32::<&str>("nc_fill_value_i32", &[]).unwrap();
            data_set.add_var_f32::<&str>("nc_fill_value_f32", &[]).unwrap();
            data_set.add_var_f64::<&str>("nc_fill_value_f64", &[]).unwrap();

            data_set
        };

        let mut file_writer: FileWriter = FileWriter::open(file_path).unwrap();
        file_writer.set_def(&data_set, Version::Classic, 0).unwrap();
        file_writer.close().unwrap();
    }

    // Write the NetCDF-3 file
    let tmp_dir: TempDir = TempDir::new(TMP_DIR_PREFIX).unwrap();
    let output_file_path = tmp_dir.path().join(NC3_FILL_VALUES_FILE_NAME);
    write_file_nc_fill_values(&output_file_path);

    // Compare the written file with the test data file
    let written_bytes: Vec<u8> = {
        let mut written_bytes: Vec<u8> = vec![];
        let mut written_file: std::fs::File = std::fs::File::open(&output_file_path).unwrap();
        written_file.read_to_end(&mut written_bytes).unwrap();
        written_bytes
    };
    tmp_dir.close().unwrap();

    assert_eq!(NC3_FILL_VALUES_FILE_BYTES.len(),     written_bytes.len());
    assert_eq!(NC3_FILL_VALUES_FILE_BYTES,           &written_bytes[..]);
}

#[test]
fn test_write_file_scalar_vars() {
    fn write_file_scalar_vars<P: AsRef<Path>>(file_path: P) {
        const SCALAR_VAR_I8_NAME: &str = "scalar_value_i8";
        const SCALAR_VAR_U8_NAME: &str = "scalar_value_u8";
        const SCALAR_VAR_I16_NAME: &str = "scalar_value_i16";
        const SCALAR_VAR_I32_NAME: &str = "scalar_value_i32";
        const SCALAR_VAR_F32_NAME: &str = "scalar_value_f32";
        const SCALAR_VAR_F64_NAME: &str = "scalar_value_f64";
        let data_set: DataSet = {
            let mut data_set: DataSet = DataSet::new();
            data_set.add_var_i8::<&str>(SCALAR_VAR_I8_NAME, &[]).unwrap();
            data_set.add_var_u8::<&str>(SCALAR_VAR_U8_NAME, &[]).unwrap();
            data_set.add_var_i16::<&str>(SCALAR_VAR_I16_NAME, &[]).unwrap();
            data_set.add_var_i32::<&str>(SCALAR_VAR_I32_NAME, &[]).unwrap();
            data_set.add_var_f32::<&str>(SCALAR_VAR_F32_NAME, &[]).unwrap();
            data_set.add_var_f64::<&str>(SCALAR_VAR_F64_NAME, &[]).unwrap();
            data_set
        };

        let mut file_writer: FileWriter = FileWriter::open(file_path).unwrap();
        file_writer.set_def(&data_set, Version::Classic, 0).unwrap();
        file_writer.write_var_i8(SCALAR_VAR_I8_NAME, &[42][..]).unwrap();
        file_writer.write_var_u8(SCALAR_VAR_U8_NAME, &[42][..]).unwrap();
        file_writer.write_var_i16(SCALAR_VAR_I16_NAME, &[42][..]).unwrap();
        file_writer.write_var_i32(SCALAR_VAR_I32_NAME, &[42][..]).unwrap();
        file_writer.write_var_f32(SCALAR_VAR_F32_NAME, &[42.0][..]).unwrap();
        file_writer.write_var_f64(SCALAR_VAR_F64_NAME, &[42.0][..]).unwrap();

        file_writer.close().unwrap();
    }

    // Write the NetCDF-3 file
    let tmp_dir: TempDir = TempDir::new(TMP_DIR_PREFIX).unwrap();
    let output_file_path = tmp_dir.path().join(SCALAR_VARIABLES_FILE_NAME);
    write_file_scalar_vars(&output_file_path);

    // Compare the written file with the test data file
    let written_bytes: Vec<u8> = {
        let mut written_bytes: Vec<u8> = vec![];
        let mut written_file: std::fs::File = std::fs::File::open(&output_file_path).unwrap();
        written_file.read_to_end(&mut written_bytes).unwrap();
        written_bytes
    };
    tmp_dir.close().unwrap();

    assert_eq!(SCALAR_VARIABLES_FILE_BYTES.len(),   written_bytes.len());
    assert_eq!(SCALAR_VARIABLES_FILE_BYTES,         &written_bytes[..]);
}

#[test]
fn test_write_file_empty_data_set() {
    fn write_file_empty_data_set<P: AsRef<Path>>(file_path: P) {
        let data_set: DataSet = DataSet::new();

        let mut file_writer: FileWriter = FileWriter::open(file_path).unwrap();
        file_writer.set_def(&data_set, Version::Classic, 1 <<12).unwrap();
        file_writer.close().unwrap();
    }

    // Write the NetCDF-3 file
    let tmp_dir: TempDir = TempDir::new(TMP_DIR_PREFIX).unwrap();
    let output_file_path = tmp_dir.path().join(EMPTY_DATA_SET_FILE_NAME);
    write_file_empty_data_set(&output_file_path);

    // Compare the written file with the test data file
    let written_bytes: Vec<u8> = {
        let mut written_bytes: Vec<u8> = vec![];
        let mut written_file: std::fs::File = std::fs::File::open(&output_file_path).unwrap();
        written_file.read_to_end(&mut written_bytes).unwrap();
        written_bytes
    };
    tmp_dir.close().unwrap();

    assert_eq!(EMPTY_DATA_SET_FILE_BYTES.len(),   written_bytes.len());
    assert_eq!(EMPTY_DATA_SET_FILE_BYTES,         &written_bytes[..]);
}

const LATITUDE_DIM_NAME: &str = "latitude";
const LATITUDE_VAR_NAME: &str = LATITUDE_DIM_NAME;
const LATITUDE_VAR_DATA: [f32; 3] = [0.0, 0.5, 1.0];
const LATITUDE_VAR_LEN: usize = LATITUDE_VAR_DATA.len();

const LONGITUDE_DIM_NAME: &str = "longitude";
const LONGITUDE_VAR_NAME: &str = LONGITUDE_DIM_NAME;
const LONGITUDE_VAR_DATA: [f32; 5] = [0.0, 0.5, 1.0, 1.5, 2.0];
const LONGITUDE_VAR_LEN: usize = LONGITUDE_VAR_DATA.len();

const TIME_DIM_NAME: &str = "time";
const TIME_VAR_NAME: &str = TIME_DIM_NAME;
const TIME_VAR_DATA: [f32; 2] = [438_300.0, 438_324.0];
const TIME_VAR_LEN: usize = TIME_VAR_DATA.len();

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

fn init_temperatures_definition() -> DataSet {


    let mut data_set: DataSet = DataSet::new();

    // First define the dimensions
    data_set.add_fixed_dim(LATITUDE_DIM_NAME, LATITUDE_VAR_LEN).unwrap();
    data_set.add_fixed_dim(LONGITUDE_DIM_NAME, LONGITUDE_VAR_LEN).unwrap();
    data_set.set_unlimited_dim(TIME_DIM_NAME, TIME_VAR_LEN).unwrap();

    // Second define the variables, their attributes and set their data
    data_set.add_var_f32(LATITUDE_VAR_NAME, &[LATITUDE_DIM_NAME]).unwrap();
    {
        // Add the variable attributes
        let var: &mut Variable = data_set.get_var_mut(LATITUDE_VAR_NAME).unwrap();

        var.add_attr_string("standard_name", "latitude").unwrap();
        var.add_attr_string("long_name", "LATITUDE").unwrap();
        var.add_attr_string("units", "degrees_north").unwrap();
        var.add_attr_string("axis", "Y").unwrap();
    }

    data_set.add_var_f32(LONGITUDE_VAR_NAME, &[LONGITUDE_DIM_NAME]).unwrap();
    {
        // Add the variable attributes
        let var: &mut Variable = data_set.get_var_mut(LONGITUDE_VAR_NAME).unwrap();

        var.add_attr_string("standard_name", "longitude").unwrap();
        var.add_attr_string("long_name", "LONGITUDE").unwrap();
        var.add_attr_string("units", "degrees_east").unwrap();
        var.add_attr_string("axis", "X").unwrap();
    }
    data_set.add_var_f32(TIME_DIM_NAME, &[TIME_DIM_NAME]).unwrap();
    {
        // Add the variable attributes
        let var: &mut Variable = data_set.get_var_mut(TIME_VAR_NAME).unwrap();

        var.add_attr_string("standard_name", "time").unwrap();
        var.add_attr_string("long_name", "TIME").unwrap();
        var.add_attr_string("units", "hours since 1970-01-01 00:00:00").unwrap();
        var.add_attr_string("calendar", "gregorian").unwrap();
        var.add_attr_string("axis", "T").unwrap();
    }
    data_set.add_var_i8(TEMP_I8_VAR_NAME, &[TIME_DIM_NAME, LATITUDE_DIM_NAME, LONGITUDE_DIM_NAME]).unwrap();
    {
        // Add the variable attributes
        let var: &mut Variable = data_set.get_var_mut(TEMP_I8_VAR_NAME).unwrap();

        var.add_attr_string("standard_name", "air_temperature").unwrap();
        var.add_attr_string("long_name", "TEMPERATURE").unwrap();
        var.add_attr_string("units", "Celsius").unwrap();
    }
    data_set.add_var_u8(TEMP_U8_VAR_NAME, &[TIME_DIM_NAME, LATITUDE_DIM_NAME, LONGITUDE_DIM_NAME]).unwrap();
    {
        // Add the variable attributes
        let var: &mut Variable = data_set.get_var_mut(TEMP_U8_VAR_NAME).unwrap();

        var.add_attr_string("standard_name", "air_temperature").unwrap();
        var.add_attr_string("long_name", "TEMPERATURE").unwrap();
        var.add_attr_string("units", "Celsius").unwrap();
    }
    data_set.add_var_i16(TEMP_I16_VAR_NAME, &[TIME_DIM_NAME, LATITUDE_DIM_NAME, LONGITUDE_DIM_NAME]).unwrap();
    {
        // Add the variable attributes
        let var: &mut Variable = data_set.get_var_mut(TEMP_I16_VAR_NAME).unwrap();

        var.add_attr_string("standard_name", "air_temperature").unwrap();
        var.add_attr_string("long_name", "TEMPERATURE").unwrap();
        var.add_attr_string("units", "Celsius").unwrap();
    }
    data_set.add_var_i32(TEMP_I32_VAR_NAME, &[TIME_DIM_NAME, LATITUDE_DIM_NAME, LONGITUDE_DIM_NAME]).unwrap();
    {
        // Add the variable attributes
        let var: &mut Variable = data_set.get_var_mut(TEMP_I32_VAR_NAME).unwrap();

        var.add_attr_string("standard_name", "air_temperature").unwrap();
        var.add_attr_string("long_name", "TEMPERATURE").unwrap();
        var.add_attr_string("units", "Celsius").unwrap();
    }
    data_set.add_var_f32(TEMP_F32_VAR_NAME, &[TIME_DIM_NAME, LATITUDE_DIM_NAME, LONGITUDE_DIM_NAME]).unwrap();
    {
        // Add the variable attributes
        let var: &mut Variable = data_set.get_var_mut(TEMP_F32_VAR_NAME).unwrap();

        var.add_attr_string("standard_name", "air_temperature").unwrap();
        var.add_attr_string("long_name", "TEMPERATURE").unwrap();
        var.add_attr_string("units", "Celsius").unwrap();
    }
    data_set.add_var_f64(TEMP_F64_VAR_NAME, &[TIME_DIM_NAME, LATITUDE_DIM_NAME, LONGITUDE_DIM_NAME]).unwrap();
    {
        // Add the variable attributes
        let var: &mut Variable = data_set.get_var_mut(TEMP_F64_VAR_NAME).unwrap();

        var.add_attr_string("standard_name", "air_temperature").unwrap();
        var.add_attr_string("long_name", "TEMPERATURE").unwrap();
        var.add_attr_string("units", "Celsius").unwrap();
    }
    data_set
}

/// Write data of each variable
fn write_temperatures_data(file_writer: &mut FileWriter) {
        file_writer.write_var_f32(LATITUDE_VAR_NAME, &LATITUDE_VAR_DATA).unwrap();
        file_writer.write_var_f32(LONGITUDE_VAR_NAME, &LONGITUDE_VAR_DATA).unwrap();
        file_writer.write_var_f32(TIME_DIM_NAME, &TIME_VAR_DATA).unwrap();

        file_writer.write_var_i8(TEMP_I8_VAR_NAME, &TEMP_I8_VAR_DATA).unwrap();
        file_writer.write_var_u8(TEMP_U8_VAR_NAME, &TEMP_U8_VAR_DATA).unwrap();
        file_writer.write_var_i16(TEMP_I16_VAR_NAME, &TEMP_I16_VAR_DATA).unwrap();
        file_writer.write_var_i32(TEMP_I32_VAR_NAME, &TEMP_I32_VAR_DATA).unwrap();
        file_writer.write_var_f32(TEMP_F32_VAR_NAME, &TEMP_F32_VAR_DATA).unwrap();
        file_writer.write_var_f64(TEMP_F64_VAR_NAME, &TEMP_F64_VAR_DATA).unwrap();
}

#[test]
fn test_write_file_nc3_classic() {
    fn write_file_nc3_classic<P: AsRef<Path>>(file_path: P) {
        let data_set: DataSet = {
            let mut data_set: DataSet = init_temperatures_definition();
            data_set.add_global_attr_string("title", "Example of NETCDF3_CLASSIC file").unwrap();
            data_set.add_global_attr_string("Conventions", "CF-1.8").unwrap();
            data_set
        };

        let mut file_writer: FileWriter = FileWriter::open(file_path).unwrap();
        file_writer.set_def(&data_set, Version::Classic, 0).unwrap();
        write_temperatures_data(&mut file_writer);
        file_writer.close().unwrap();
    }

    // Write the NetCDF-3 file
    let tmp_dir: TempDir = TempDir::new(TMP_DIR_PREFIX).unwrap();
    let output_file_path = tmp_dir.path().join(NC3_CLASSIC_FILE_NAME);
    write_file_nc3_classic(&output_file_path);

    // Compare the written file with the test data file
    let written_bytes: Vec<u8> = {
        let mut written_bytes: Vec<u8> = vec![];
        let mut written_file: std::fs::File = std::fs::File::open(&output_file_path).unwrap();
        written_file.read_to_end(&mut written_bytes).unwrap();
        written_bytes
    };
    tmp_dir.close().unwrap();

    assert_eq!(NC3_CLASSIC_FILE_BYTES.len(),   written_bytes.len());
    assert_eq!(NC3_CLASSIC_FILE_BYTES,         &written_bytes[..]);
}

#[test]
fn test_write_file_nc3_64bit_offset() {
    fn write_file_nc3_64bit_offset<P: AsRef<Path>>(file_path: P) {
        let data_set: DataSet = {
            let mut data_set: DataSet = init_temperatures_definition();
            data_set.add_global_attr_string("title", "Example of NETCDF3_64BIT_OFFSET file").unwrap();
            data_set.add_global_attr_string("Conventions", "CF-1.8").unwrap();
            data_set
        };

        let mut file_writer: FileWriter = FileWriter::open(file_path).unwrap();
        file_writer.set_def(&data_set, Version::Offset64Bit, 0).unwrap();
        write_temperatures_data(&mut file_writer);
        file_writer.close().unwrap();
    }

    // Write the NetCDF-3 file
    let tmp_dir: TempDir = TempDir::new(TMP_DIR_PREFIX).unwrap();
    let output_file_path = tmp_dir.path().join(NC3_64BIT_OFFSET_FILE_NAME);
    write_file_nc3_64bit_offset(&output_file_path);

    // Compare the written file with the test data file
    let written_bytes: Vec<u8> = {
        let mut written_bytes: Vec<u8> = vec![];
        let mut written_file: std::fs::File = std::fs::File::open(&output_file_path).unwrap();
        written_file.read_to_end(&mut written_bytes).unwrap();
        written_bytes
    };
    tmp_dir.close().unwrap();

    assert_eq!(NC3_64BIT_OFFSET_FILE_BYTES.len(),   written_bytes.len());
    assert_eq!(NC3_64BIT_OFFSET_FILE_BYTES,         &written_bytes[..]);
}