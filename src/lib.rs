//! ## Description
//!
//! A pure Rust library for reading and writing NetCDF-3 files.
//!
//! # Examples
//!
//! - Read a file with [`FileReader`](struct.FileReader.html).
//! - Manage NetCDF-3 data set structures : dimensions, attributes and variables with [`Dataset`](struct.DataSet.html).
//! - Create, read, rename and remove variables ([examples](struct.Variable.html#examples)).
//! - Create, read, rename and remove global attributes ([examples](struct.Attribute.html#global-attributes)).
//! - Create, read, rename and remove variable attributes ([examples](struct.Attribute.html#variable-attributes)).
//! - Create, read, rename and remove dimensions ([examples](struct.Dimension.html#examples))
//!
pub mod error;
pub use error::{IOError, InvalidDataSet};

mod name_string;
pub use name_string::is_valid_name;

mod data_type;
pub use data_type::DataType;

mod data_vector;

mod data_set;
pub use data_set::{Attribute, DataSet, Dimension, DimensionType, Variable};

mod io;
pub use io::FileReader;

mod version;
pub use version::Version;