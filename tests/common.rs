#![allow(dead_code)]
use std::io::Write;
use tempdir::TempDir;

/// Empty data set test file name.
pub(crate) static EMPTY_DATA_SET_FILE_NAME: &'static str = "empty_data_set";
pub(crate) static EMPTY_DATA_SET_FILE_BYTES: &'static[u8] = include_bytes!("../data/empty_data_set.nc");

/// NetCDF-3 (classic version) test file name.
pub(crate) static NC3_CLASSIC_FILE_NAME: &'static str = "temp_3D_classic.nc";
pub(crate) static NC3_CLASSIC_FILE_BYTES: &'static[u8] = include_bytes!("../data/temp_3D_classic.nc");

/// NetCDF-3 (64-bit offset version) test file name.
pub(crate) static NC3_64BIT_OFFSET_FILE_NAME: &'static str = "temp_3D_64bit_offset.nc";
pub(crate) static NC3_64BIT_OFFSET_FILE_BYTES: &'static[u8] = include_bytes!("../data/temp_3D_64bit_offset.nc");

/// Empty variables test file name.
pub(crate) static EMPTY_VARIABLES_FILE_NAME: &'static str = "empty_vars.nc";
pub(crate) static EMPTY_VARIABLES_FILE_BYTES: &'static[u8] = include_bytes!("../data/empty_vars.nc");

/// Scalar variables test file name.
pub(crate) static SCALAR_VARIABLES_FILE_NAME: &'static str = "scalar_vars.nc";
pub(crate) static SCALAR_VARIABLES_FILE_BYTES: &'static[u8] = include_bytes!("../data/scalar_vars.nc");


pub fn copy_bytes_to_tmp_file(bytes: &[u8], file_name: &str) -> (TempDir, std::path::PathBuf)
{
    // Crete the temporary directory
    let tmp_dir: TempDir = TempDir::new("netcdf3_test_data").unwrap();
    // Crete the temporary file
    let tmp_file_path = std::path::PathBuf::from(tmp_dir.path()).join(file_name);
    let mut tmp_file = std::fs::File::create(tmp_file_path.clone()).unwrap();
    // Copy all bytes
    let _ = tmp_file.write_all(bytes).unwrap();

    return (tmp_dir, tmp_file_path);
}

