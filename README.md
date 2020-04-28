# netcdf3

[![LICENSE](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Build Status](https://travis-ci.com/julienbonte/netcdf3.svg?branch=master)](https://travis-ci.com/julienbonte/netcdf3)
[![Crates.io Version](https://img.shields.io/crates/v/netcdf3.svg)](https://crates.io/crates/netcdf3)

[Documentation](https://docs.rs/netcdf3)

## Description

**netcdf3** is a library for read/write NetCDF-3 file written in Rust.

## Technical features
- [x] Read classic and 64-bit offset NetCDF-3 file :
    - [x] Open file and parse the header.
    - [x] Read all variables of a file.
    - [ ] Read a part of variables.
    - [ ] Keeping the shape of the N-dimensional arrays (using the crate [ndarray](https://github.com/rust-ndarray/ndarray))
- [ ] Write classic and 64-bit offset NetCDF-3 file.

# Examples

## Read a file

``` rust
use netcdf3::{
    FileReader, Version, DataSet,
};

fn main()
{

    // Read the data set
    let data_set: DataSet = {
        let mut file_reader = FileReader::open("netcdf3_file.nc").unwrap();
        let _ = file_reader.read_all_vars().unwrap();
        file_reader.close()
    };

    println!("FILE DESCRIPTION");
    println!("----------------");
    println!("Version                          : {}", match data_set.version() {
        Version::Classic => "Classic",
        Version::Offset64Bit => "64-Bit Offset",
    });
    println!("Number of dimensions             : {}", data_set.num_dims());
    for dim in data_set.get_dims() {
        println!("    {name} = {size}{is_unlimited}", name=dim.name(), size=dim.size(), is_unlimited=
            if dim.is_unlimited() {
                " (unlimited)"
            } else {
                ""
            }
        );
    }
    println!("Number of global attributes      : {}", data_set.num_global_attrs());
    for attr in data_set.get_global_attrs() {
        println!("    {name}({data_type})", name=attr.name(), data_type=attr.data_type().c_api_name());
    }

    println!("Number of variables              : {}", data_set.num_vars());
    for var in data_set.get_vars() {
        println!("   {} :", var.name());
        println!("        dimensions           : {:?}", var.get_dim_names());
        println!("        number of elements   : {}", var.len());
        println!("        data type            : {}", var.data_type().c_api_name());
        println!("        is a record variable : {}", var.is_record_var());
        println!("        number of attributes : {}", var.num_attrs());
        for attr in var.get_attrs() {
            println!("            {name}({data_type})", name=attr.name(), data_type=attr.data_type().c_api_name());
        }
    }
}
```

Run a similar example with the command :

``` shell
cargo run --example read_file
```

