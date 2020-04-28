//! ## Description
//!
//! **netcdf3** is a library for read/write NetCDF-3 file written in Rust.
//!
//! # Examples
//!
//! ## Read a file
//!
//! ``` rust
//! use netcdf3::{
//!     FileReader, Version, DataSet,
//! };
//!
//! fn main()
//! {
//!
//!     // Read the data set
//!     let data_set: DataSet = {
//!         let mut file_reader = FileReader::open("netcdf3_file.nc").unwrap();
//!         let _ = file_reader.read_all_vars().unwrap();
//!         file_reader.close()
//!     };
//!
//!     println!("FILE DESCRIPTION");
//!     println!("----------------");
//!     println!("Version                          : {}", match data_set.version() {
//!         Version::Classic => "Classic",
//!         Version::Offset64Bit => "64-Bit Offset",
//!     });
//!     println!("Number of dimensions             : {}", data_set.num_dims());
//!     for dim in data_set.get_dims() {
//!         println!("    {name} = {size}{is_unlimited}", name=dim.name(), size=dim.size(), is_unlimited=
//!             if dim.is_unlimited() {
//!                 " (unlimited)"
//!             } else {
//!                 ""
//!             }
//!         );
//!     }
//!     println!("Number of global attributes      : {}", data_set.num_global_attrs());
//!     for attr in data_set.get_global_attrs() {
//!         println!("    {name}({data_type})", name=attr.name(), data_type=attr.data_type().c_api_name());
//!     }
//!
//!     println!("Number of variables              : {}", data_set.num_vars());
//!     for var in data_set.get_vars() {
//!         println!("   {} :", var.name());
//!         println!("        dimensions           : {:?}", var.get_dim_names());
//!         println!("        number of elements   : {}", var.len());
//!         println!("        data type            : {}", var.data_type().c_api_name());
//!         println!("        is a record variable : {}", var.is_record_var());
//!         println!("        number of attributes : {}", var.num_attrs());
//!         for attr in var.get_attrs() {
//!             println!("            {name}({data_type})", name=attr.name(), data_type=attr.data_type().c_api_name());
//!         }
//!     }
//! }
//! ```

pub mod error;

mod name_string;

mod data_type;
pub use data_type::DataType;

mod typed_data_vector;

mod data_set;
pub use data_set::{Attribute, DataSet, Dimension, DimensionType, Variable, Version};

mod io;
pub use io::FileReader;
