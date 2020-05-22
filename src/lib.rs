//! # Description
//!
//! A pure Rust library for reading and writing NetCDF-3 files.
//!
//! # Examples
//!
//! - Read the NetCDF-3 files with [`FileReader`](struct.FileReader.html).
//! - Write the NetCDF-3 files with [`FileWriter`](struct.FileWriter.html).
//! - Manage the NetCDF-3 data set defintions : dimensions, attributes and variables with [`Dataset`](struct.DataSet.html)
//!     - Create, read, rename and remove variables ([examples](struct.Variable.html#examples)).
//!     - Create, read, rename and remove global attributes ([examples](struct.Attribute.html#global-attributes)).
//!     - Create, read, rename and remove variable attributes ([examples](struct.Attribute.html#variable-attributes)).
//!     - Create, read, rename and remove dimensions ([examples](struct.Dimension.html#examples)).
//!
//! # Known limitations
//!
//! - Cannot read and write too large NetCDF-3 files yet (cannot slice data array into subsets).
//! - Don't manage the special case under which the `numrecs` value is indeterminate (`numrecs = 2^32 - 1`) yet (see the [File Format Specifications][File_Format_Specs]).
//! - Don't manage the special case under which a `vsize` value is indeterminate (`vsize = 2^32 - 1`) yet (see the [File Format Specifications][File_Format_Specs]).
//!
//! [File_Format_Specs]: https://www.unidata.ucar.edu/software/netcdf/docs/file_format_specifications.html
pub mod error;
pub use error::{ReadError, WriteError, InvalidDataSet};

mod name_string;
pub use name_string::is_valid_name;

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