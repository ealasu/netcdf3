"""
This script builds NetCDF-3 data file used by tests of the Rust crate.
"""
import os.path
import datetime
import argparse

import numpy as np

import netCDF4


def define_temperatures_dataset(ds):

    TIME_UNITS = "hours since 1970-01-01 00:00:00"

    def init_time():
        time = [
            datetime.datetime(2020, 1, 1, 12, 0, 0),
            datetime.datetime(2020, 1, 2, 12, 0, 0),
        ]
        return netCDF4.date2num(
            time,
            units=TIME_UNITS,
            calendar="gregorian",
    )

    def define_temp_attrs(temperature_var: netCDF4.Dataset):
        temperature_var.setncatts({
            "standard_name": "air_temperature",
            "long_name": "TEMPERATURE",
            "units": "Celsius"
        })

    # First define values
    latitude = np.linspace(0, 1.0, num=3, endpoint=True)
    longitude = np.linspace(0, 2.0, num=5, endpoint=True)
    time = init_time()
    num_elements = latitude.size * longitude.size * time.size
    temperature = np.reshape(
        np.arange(0, num_elements),
        newshape=(time.size, latitude.size, longitude.size)
    )

    # Define the `latitude`
    ds.createDimension("latitude", latitude.size)
    latitude_var = ds.createVariable("latitude", datatype=np.float32, dimensions="latitude")
    latitude_var.setncattr("standard_name", "latitude")
    latitude_var.setncattr("long_name", "LATITUDE")
    latitude_var.setncattr("units", "degrees_north")
    latitude_var.setncattr("axis", "Y")
    latitude_var[:] = latitude

    # Define the `longitude`
    ds.createDimension("longitude", longitude.size)
    longitude_var = ds.createVariable("longitude", datatype=np.float32, dimensions="longitude")
    longitude_var.setncattr("standard_name", "longitude")
    longitude_var.setncattr("long_name", "LONGITUDE")
    longitude_var.setncattr("units", "degrees_east")
    longitude_var.setncattr("axis", "X")
    longitude_var[:] = longitude

    # Define the `TIME`
    ds.createDimension("time")
    time_var = ds.createVariable("time", datatype=np.float32, dimensions="time")
    time_var.setncattr("standard_name", "time")
    time_var.setncattr("long_name", "TIME")
    time_var.setncattr("units", TIME_UNITS)
    time_var.setncattr("calendar", "gregorian")
    time_var.setncattr("axis", "T")
    time_var[:] = time

    # Define the variables `temparature`
    temperature_i8_var = ds.createVariable("temperature_i8", datatype=np.int8, dimensions=("time", "latitude", "longitude"))
    define_temp_attrs(temperature_i8_var)
    temperature_i8_var[:] = np.asarray(temperature, dtype=np.int8)

    temperature_u8_var = ds.createVariable("temperature_u8", datatype='c', dimensions=("time", "latitude", "longitude"))
    define_temp_attrs(temperature_u8_var)
    temperature_u8_var[0, 0, 0] = b'\x00'
    temperature_u8_var[0, 0, 1] = b'\x01'
    temperature_u8_var[0, 0, 2] = b'\x02'
    temperature_u8_var[0, 0, 3] = b'\x03'
    temperature_u8_var[0, 0, 4] = b'\x04'
    temperature_u8_var[0, 1, 0] = b'\x05'
    temperature_u8_var[0, 1, 1] = b'\x06'
    temperature_u8_var[0, 1, 2] = b'\x07'
    temperature_u8_var[0, 1, 3] = b'\x08'
    temperature_u8_var[0, 1, 4] = b'\x09'
    temperature_u8_var[0, 2, 0] = b'\x0a'
    temperature_u8_var[0, 2, 1] = b'\x0b'
    temperature_u8_var[0, 2, 2] = b'\x0c'
    temperature_u8_var[0, 2, 3] = b'\x0d'
    temperature_u8_var[0, 2, 4] = b'\x0e'
    temperature_u8_var[1, 0, 0] = b'\x0f'
    temperature_u8_var[1, 0, 1] = b'\x10'
    temperature_u8_var[1, 0, 2] = b'\x11'
    temperature_u8_var[1, 0, 3] = b'\x12'
    temperature_u8_var[1, 0, 4] = b'\x13'
    temperature_u8_var[1, 1, 0] = b'\x14'
    temperature_u8_var[1, 1, 1] = b'\x15'
    temperature_u8_var[1, 1, 2] = b'\x16'
    temperature_u8_var[1, 1, 3] = b'\x17'
    temperature_u8_var[1, 1, 4] = b'\x18'
    temperature_u8_var[1, 2, 0] = b'\x19'
    temperature_u8_var[1, 2, 1] = b'\x1a'
    temperature_u8_var[1, 2, 2] = b'\x1b'
    temperature_u8_var[1, 2, 3] = b'\x1c'
    temperature_u8_var[1, 2, 4] = b'\x1d'

    temperature_i16_var = ds.createVariable("temperature_i16", datatype=np.int16, dimensions=("time", "latitude", "longitude"))
    define_temp_attrs(temperature_i16_var)
    temperature_i16_var[:] = np.asarray(temperature, dtype=np.int16)

    temperature_i32_var = ds.createVariable("temperature_i32", datatype=np.int32, dimensions=("time", "latitude", "longitude"))
    define_temp_attrs(temperature_i32_var)
    temperature_i32_var[:] = np.asarray(temperature, dtype=np.int32)

    temperature_f32_var = ds.createVariable("temperature_f32", datatype=np.float32, dimensions=("time", "latitude", "longitude"))
    define_temp_attrs(temperature_f32_var)
    temperature_f32_var[:] = np.asarray(temperature, dtype=np.float32)

    temperature_f64_var = ds.createVariable("temperature_f64", datatype=np.float64, dimensions=("time", "latitude", "longitude"))
    define_temp_attrs(temperature_f64_var)
    temperature_f64_var[:] = np.asarray(temperature, dtype=np.float64)


# Initialze the command line parser
def init_parser():
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--output-dir", "-o",
        nargs=1,
        type=str,
        help="Path of the output data directory"
    )
    return parser

if __name__ == "__main__":
    parser = init_parser()
    args = parser.parse_args()
    output_dir = args.output_dir[0]


    # Create an empty data set file
    with netCDF4.Dataset(os.path.join(output_dir, "empty_data_set.nc"), format="NETCDF3_CLASSIC", mode="w") as ds:
        pass

    # Create a `NETCDF3_CLASSIC` data file
    with netCDF4.Dataset(os.path.join(output_dir, "temp_3D_classic.nc"), format="NETCDF3_CLASSIC", mode="w") as ds:
        # Define global attributes
        ds.setncatts({
            "comment": "NETCDF3_CLASSIC file"
        })
        define_temperatures_dataset(ds)

    # Create a `NETCDF3_64BIT_OFFSET` data file
    with netCDF4.Dataset(os.path.join(output_dir, "temp_3D_64bit_offset.nc"), format="NETCDF3_64BIT_OFFSET", mode="w") as ds:
        ds.setncatts({
            "comment": "NETCDF3_64BIT_OFFSET file",
        })
        define_temperatures_dataset(ds)

    # Create a data set containing variables without dimension and value
    with netCDF4.Dataset(os.path.join(output_dir, "empty_vars.nc"), format="NETCDF3_CLASSIC", mode="w") as ds:
        ds.createVariable("no_value_i8", datatype=np.int8, dimensions=())
        ds.createVariable("no_value_u8", datatype='c', dimensions=())
        ds.createVariable("no_value_i16", datatype=np.int16, dimensions=())
        ds.createVariable("no_value_i32", datatype=np.int32, dimensions=())
        ds.createVariable("no_value_f32", datatype=np.float32, dimensions=())
        ds.createVariable("no_value_f64", datatype=np.float64, dimensions=())

    # Create a data set containing scalar variables
    with netCDF4.Dataset(os.path.join(output_dir, "scalar_vars.nc"), format="NETCDF3_CLASSIC", mode="w") as ds:
        ds.createDimension("scalar_dim", 1)
        var = ds.createVariable("scalar_value_i8", datatype=np.int8, dimensions=("scalar_dim"))
        var[0] = np.int8(42)
        var = ds.createVariable("scalar_value_u8", datatype='c', dimensions=("scalar_dim"))
        var[0] = b'\x2a'
        var = ds.createVariable("scalar_value_i16", datatype=np.int16, dimensions=("scalar_dim"))
        var[0] = np.int16(42)
        var = ds.createVariable("scalar_value_i32", datatype=np.int32, dimensions=("scalar_dim"))
        var[0] = np.int32(42)
        var = ds.createVariable("scalar_value_f32", datatype=np.float32, dimensions=("scalar_dim"))
        var[0] = np.float32(42.0)
        var = ds.createVariable("scalar_value_f64", datatype=np.float64, dimensions=("scalar_dim"))
        var[0] = np.float64(42.0)
