//! # Description
//!
//! A pure Rust library for reading and writing NetCDF-3 files.
//!
//! # Examples
//!
//! - Read the NetCDF-3 files with the struct [`FileReader`](struct.FileReader.html).
//! - Write the NetCDF-3 files with the struct [`FileWriter`](struct.FileWriter.html).
//! - Manage the NetCDF-3 data set defintions : dimensions, attributes and variables with [`Dataset`](struct.DataSet.html)
//!     - Create, read, rename and remove variables (examples [here](struct.Variable.html#examples)).
//!     - Create, read, rename and remove global attributes (examples [here](struct.Attribute.html#global-attributes)).
//!     - Create, read, rename and remove variable attributes (examples [here](struct.Attribute.html#variable-attributes)).
//!     - Create, read, rename and remove dimensions (examples [here](struct.Dimension.html#examples)).
//!
//! # Notes
//!
//! - If the number of records `numrecs` is greater than `std::i32::MAX` then this value is considered as indeterminated (`numrecs = 2^32 - 1`) (see the [File Format Specifications][File_Format_Specs]).
//! - If the chunk size of a given variable `vsize` is greater the `std::i32::MAX` then its value is considered as indeterminated (`vsize = 2^32 - 1`) (see the [File Format Specifications][File_Format_Specs]).
//! - For the file writing, binary comparisons between the crate outcomes and files produced by the Python library ([netCDF4](https://github.com/Unidata/netcdf4-python)) are done while the test suite (see the Python script `pyscripts/create_test_nc3_files.py` and the Rust test file `tests/tests_write_nc3_files.rs`.)
//!
//! ## Known limitations
//!
//! - Cannot read/write too large NetCDF-3 files. Cannot read/write a subset of a variable data yet.
//! - Cannot rewrite the NetCDF-3 files (append/remove data, attribute, ...).
//!
//! [File_Format_Specs]: https://www.unidata.ucar.edu/software/netcdf/docs/file_format_specifications.html
pub mod error;
pub use error::{ReadError, WriteError, InvalidDataSet};

mod name_string;
pub use name_string::is_valid_name;
pub use name_string::NC_MAX_NAME_SIZE;

mod data_type;
pub use data_type::DataType;

mod data_vector;
pub use data_vector::DataVector;

mod data_set;
pub use data_set::{Attribute, DataSet, Dimension, DimensionType, Variable};
pub use data_set::NC_FILL_I8;
pub use data_set::NC_FILL_U8;
pub use data_set::NC_FILL_I16;
pub use data_set::NC_FILL_I32;
pub use data_set::NC_FILL_F32;
pub use data_set::NC_FILL_F64;
pub use data_set::NC_MAX_DIM_SIZE;
pub use data_set::NC_MAX_VAR_DIMS;

mod io;
pub use io::{FileReader, FileWriter};

mod version;
pub use version::Version;