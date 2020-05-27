# netcdf3

![Crates.io](https://img.shields.io/crates/l/netcdf3)
[![Documentation](https://docs.rs/netcdf3/badge.svg)](https://docs.rs/netcdf3)
[![Build Status](https://travis-ci.com/julienbonte/netcdf3.svg?branch=master)](https://travis-ci.com/julienbonte/netcdf3)
[![Crates.io Version](https://img.shields.io/crates/v/netcdf3.svg)](https://crates.io/crates/netcdf3)
[![Minimum rustc version](https://img.shields.io/badge/rustc-1.40.0+-lightgray.svg)](#rust-version-requirements)

## Description

A pure Rust library for reading and writing NetCDF-3 files.

## Technical features

- [X] Read a NetCDF-3 file :
    - [X] Read a variable data.
    - [ ] Read a subset of a variable data.
    - [ ] Read a variable data into a N-dimensional array (using the crate [ndarray](https://github.com/rust-ndarray/ndarray)).
- [X] Write a NetCDF-3 file :
    - [X] Write a variable data.
    - [ ] Write a subset of a variable data.
    - [ ] Write a variable data from a N-dimensional array (using the crate [ndarray](https://github.com/rust-ndarray/ndarray)).

# Notes

- If the number of records `numrecs` is greater than `std::i32::MAX` then this value is considered as indeterminated (`numrecs = 2^32 - 1`) (see the [File Format Specifications][File_Format_Specs]).
- If the chunk size of a given variable `vsize` is greater the `std::i32::MAX` then its value is considered as indeterminated (`vsize = 2^32 - 1`) (see the [File Format Specifications][File_Format_Specs]).
- For the file writing, binary comparisons between the crate outcomes and files produced by the Python library ([netCDF4](https://github.com/Unidata/netcdf4-python)) are done while the test suite (see the Python script `pyscripts/create_test_nc3_files.py` and the Rust test file `tests/tests_write_nc3_files.rs`.)

## Known limitations

- Cannot read/write too large NetCDF-3 files. Cannot read/write a subset of a variable data yet.
- Cannot rewrite the NetCDF-3 files (append/remove records, edit dimensions/attributes/variables, ...).


## Examples

Various examples are available [here](https://docs.rs/netcdf3).

[File_Format_Specs]: https://www.unidata.ucar.edu/software/netcdf/docs/file_format_specifications.html
