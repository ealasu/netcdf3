# netcdf3

![Crates.io](https://img.shields.io/crates/l/netcdf3)
[![Documentation](https://docs.rs/netcdf3/badge.svg)](https://docs.rs/netcdf3)
[![Build Status](https://travis-ci.com/julienbonte/netcdf3.svg?branch=master)](https://travis-ci.com/julienbonte/netcdf3)
[![Crates.io Version](https://img.shields.io/crates/v/netcdf3.svg)](https://crates.io/crates/netcdf3)
[![Minimum rustc version](https://img.shields.io/badge/rustc-1.40.0+-lightgray.svg)](#rust-version-requirements)

## Description

A pure Rust library for reading and writing NetCDF-3 files.

## Technical features

- [X] Read the classic and 64-bit offset NetCDF-3 files :
    - [X] Open the files and parse the headers.
    - [X] Read the variables data from a file.
    - [X] Read the attribute `u8` values as UTF-8 string.
    - [ ] Read the variable data as N-dimensional arrays (using the crate [ndarray](https://github.com/rust-ndarray/ndarray)).
- [X] Manage the NetCDF-3 structures : dimensions, attributes and variables (create, read, rename, remove).
- [X] Write the classic and 64-bit offset NetCDF-3 files:
    - Binary comparisons are done between the crate outcomes and files produced with the Python library ([netCDF4](https://github.com/Unidata/netcdf4-python)).
    - Also see the Python script `pyscripts/create_test_nc3_files.py` and the Rust test file `tests/tests_write_nc3_files.rs`.

## Known limitations

- Cannot read and write too large NetCDF-3 files (the slicing arrays is not implemented yet).
- Don't manage yet the special case under which the `numrecs` value is indeterminate (`numrecs = 2^32 - 1`) (see the [File Format Specifications][File_Format_Specs]).
- Don't manage yet the special case under which a `vsize` value is indeterminate (`vsize = 2^32 - 1`) (see the [File Format Specifications][File_Format_Specs]).


## Examples

Various examples are available [here](https://docs.rs/netcdf3).

[File_Format_Specs]: https://www.unidata.ucar.edu/software/netcdf/docs/file_format_specifications.html
