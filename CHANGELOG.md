# Changelog

## 0.4.0 - 2020-05-27

### Added

- Manage the indeterminated value of the number of records (`numrecs = 2^32 - 1`) while the reading and the writing ([netCDF4](https://github.com/Unidata/netcdf4-python)).
- Manage the indeterminated value of the chunk size for each variable (`vsize = 2^32 - 1`) while the reading and the writing ([netCDF4](https://github.com/Unidata/netcdf4-python)).
- Set the maximum size of the NetCDF-3 names (`NC_MAX_NAME_SIZE = 256`).

[File_Format_Specs]: https://www.unidata.ucar.edu/software/netcdf/docs/file_format_specifications.html

## 0.3.1 - 2020-05-22

### Changed

- Correct the file `README.md`.

## 0.3.0 - 2020-05-22

### Changed

- Add The library is under the licenses `MIT OR Apache-2.0`.
- Change the `struct DataSet`. It does not contain the variable data.
- Change the error `enum`s.
- Set the maximum size of the *fixed-size* dimensions (`NC_MAX_DIM_SIZE = 2_147_483_644`).
- Set the maximum number of dimensions per variable (`NC_MAX_VAR_DIMS = 1_024`).

### Added

- Add the `struct FileWriter`. It allows to write the NetCDF-3 file.

## 0.2.0 - 2020-05-04

### Changed

- Change the error `enum`s.
- Change the `struct DataSet`.
- Change the `struct FileReader`.

## 0.1.0 - 2020-04-28 [YANKED]

Initial release
