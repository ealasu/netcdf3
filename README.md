# netcdf3

[![LICENSE](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Documentation](https://docs.rs/netcdf3/badge.svg)](https://docs.rs/netcdf3)
[![Build Status](https://travis-ci.com/julienbonte/netcdf3.svg?branch=master)](https://travis-ci.com/julienbonte/netcdf3)
[![Crates.io Version](https://img.shields.io/crates/v/netcdf3.svg)](https://crates.io/crates/netcdf3)
[![Minimum rustc version](https://img.shields.io/badge/rustc-1.39.0+-lightgray.svg)](#rust-version-requirements)

## Description

A pure Rust library for reading and writing NetCDF-3 files.

## Technical features

- [x] Read classic and 64-bit offset NetCDF-3 file :
    - [x] Open file and parse the header.
    - [x] Read variables data from a file.
    - [ ] Get `NC_CHAR` array as UTF-8 String.
    - [ ] Get variable data as a N-dimensional array (using the crate [ndarray](https://github.com/rust-ndarray/ndarray)).
- [X] Manage NetCDF-3 data set structures : dimensions, attributes and variables (create, read, rename, remove).
- [ ] Write classic and 64-bit offset NetCDF-3 file.


## Examples

Examples are available in the documentation of the project [here](https://docs.rs/netcdf3).