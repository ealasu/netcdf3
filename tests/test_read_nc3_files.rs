#[cfg(test)]
mod common;

use std::rc::Rc;

use netcdf3::{FileReader, DataSet, Variable, Attribute, Dimension, DataType, Version};

use common::{
    copy_bytes_to_tmp_file,
    EMPTY_DATA_SET_FILE_NAME ,EMPTY_DATA_SET_FILE_BYTES,
    NC3_CLASSIC_FILE_NAME, NC3_CLASSIC_FILE_BYTES,
    NC3_64BIT_OFFSET_FILE_NAME, NC3_64BIT_OFFSET_FILE_BYTES,
    EMPTY_VARIABLES_FILE_NAME, EMPTY_VARIABLES_FILE_BYTES,
    SCALAR_VARIABLES_FILE_NAME, SCALAR_VARIABLES_FILE_BYTES,
};

#[test]
fn test_file_empty_data_set() {
    // Copy bytes to a temporary file
    let (tmp_dir, input_data_file_path) = copy_bytes_to_tmp_file(EMPTY_DATA_SET_FILE_BYTES, EMPTY_DATA_SET_FILE_NAME);

    // Read the data set
    let data_set: DataSet = {
        let mut file_reader = FileReader::open(input_data_file_path).unwrap();
        let _ = file_reader.read_all_vars().unwrap();
        file_reader.close()
    };
    tmp_dir.close().unwrap();

    // Check the parsed header
    assert_eq!(Version::Classic, data_set.version());
    assert_eq!(0, data_set.num_dims());
    assert_eq!(0, data_set.num_global_attrs());
    assert_eq!(0, data_set.num_vars());
}

#[test]
fn test_read_file_nc3_classic() {
    // Copy bytes to a temporary file
    let (tmp_dir, input_data_file_path) = copy_bytes_to_tmp_file(NC3_CLASSIC_FILE_BYTES, NC3_CLASSIC_FILE_NAME);

    // Read the data set
    let data_set: DataSet = {
        let mut file_reader = FileReader::open(input_data_file_path).unwrap();
        let _ = file_reader.read_all_vars().unwrap();
        file_reader.close()
    };
    tmp_dir.close().unwrap();


    // Check the parsed header
    assert_eq!(Version::Classic, data_set.version());
    {
        assert_eq!(1, data_set.num_global_attrs());
        assert!(data_set.get_global_attr_i8("comment").is_none());
        assert!(data_set.get_global_attr_u8("comment").is_some());
        assert!(data_set.get_global_attr_i16("comment").is_none());
        assert!(data_set.get_global_attr_i32("comment").is_none());
        assert!(data_set.get_global_attr_f32("comment").is_none());
        assert!(data_set.get_global_attr_f64("comment").is_none());

        assert!(data_set.get_global_attr("comment").is_some());
        let attr: &Attribute = data_set.get_global_attr("comment").unwrap();
        assert!(attr.get_u8().is_some());
        let attr_data: &Vec<u8> = attr.get_u8().unwrap();
        assert_eq!(
            "NETCDF3_CLASSIC file".as_bytes(),
            &attr_data[..],
        );
    }

    // Check the parsed header of the data set
    check_temperature_dataset_header(&data_set);
    // Check the temperature parsed values
    check_temperature_dataset_values(&data_set);
}

#[test]
fn test_read_file_nc3_64bit_offset() {
    // Copy bytes to a temporary file
    let (tmp_dir, input_data_file_path) = copy_bytes_to_tmp_file(NC3_64BIT_OFFSET_FILE_BYTES, NC3_64BIT_OFFSET_FILE_NAME);

    // Read the data set
    let data_set: DataSet = {
        let mut file_reader = FileReader::open(input_data_file_path).unwrap();
        let _ = file_reader.read_all_vars().unwrap();
        file_reader.close()
    };
    tmp_dir.close().unwrap();

    // Check the parsed header
    assert_eq!(Version::Offset64Bit, data_set.version());
    {
        assert_eq!(1, data_set.num_global_attrs());
        assert!(data_set.get_global_attr_i8("comment").is_none());
        assert!(data_set.get_global_attr_u8("comment").is_some());
        assert!(data_set.get_global_attr_i16("comment").is_none());
        assert!(data_set.get_global_attr_i32("comment").is_none());
        assert!(data_set.get_global_attr_f32("comment").is_none());
        assert!(data_set.get_global_attr_f64("comment").is_none());

        assert!(data_set.get_global_attr("comment").is_some());
        let attr: &Attribute = data_set.get_global_attr("comment").unwrap();
        assert!(attr.get_u8().is_some());
        let attr_data: &Vec<u8> = attr.get_u8().unwrap();
        assert_eq!(
            "NETCDF3_64BIT_OFFSET file".as_bytes(),
            &attr_data[..],
        );
    }

    // Check the parsed header of the data set
    check_temperature_dataset_header(&data_set);
    // Check the temperature parsed values
    check_temperature_dataset_values(&data_set);
}

fn check_temperature_dataset_header(data_set: &DataSet)
{

    // Check the dimensions
    // --------------------
    {
        assert_eq!(3, data_set.num_dims());
        assert!(data_set.has_unlimited_dim());
        assert_eq!(2, data_set.num_records());

        // check the returns of undefined dimensions, global attributes and variables
        assert!(data_set.get_dim("undefined_dim").is_none());
        assert!(data_set.get_global_attr("undefined_global_attr").is_none());
        assert!(data_set.get_var("undefined_var").is_none());

        // latitude
        {
            let latitude_dim = data_set.get_dim("latitude");
            assert!(latitude_dim.is_some());
            let latitude_dim: Rc<Dimension> = latitude_dim.unwrap();
            assert_eq!("latitude", latitude_dim.name());
            assert_eq!(3, latitude_dim.size());
            assert!(latitude_dim.is_fixed());
        }

        // longitude
        {
            let longitude_dim = data_set.get_dim("longitude");
            assert!(longitude_dim.is_some());
            let longitude_dim: Rc<Dimension> = longitude_dim.unwrap();
            assert_eq!("longitude", longitude_dim.name());
            assert_eq!(5, longitude_dim.size());
            assert!(longitude_dim.is_fixed());
        }

        // time
        {
            let time_dim = data_set.get_dim("time");
            assert!(time_dim.is_some());
            let time_dim: Rc<Dimension> = time_dim.unwrap();
            assert_eq!("time", time_dim.name());
            assert_eq!(2, time_dim.size());
            assert!(time_dim.is_unlimited());
        }
    }
    // Check the variables
    // ---------------------
    assert_eq!(9, data_set.num_vars());
    // the variable `latitude`
    {
        let latitude_var: Option<&Variable> = data_set.get_var("latitude");
        assert!(latitude_var.is_some());
        let latitude_var: &Variable = latitude_var.unwrap();
        assert_eq!("latitude", latitude_var.name());
        assert_eq!(DataType::F32, latitude_var.data_type());
        assert_eq!(1, latitude_var.num_dims());
        assert!(!latitude_var.is_record_var());
        assert_eq!(3, latitude_var.len());
        assert_eq!(1, latitude_var.num_chunks());
        assert_eq!(3, latitude_var.num_elements_per_chunk());
        assert_eq!(12, latitude_var.chunk_size());
    }
    // the variable `longitude`
    {
        let longitude_var: Option<&Variable> = data_set.get_var("longitude");
        assert!(longitude_var.is_some());
        let longitude_var: &Variable = longitude_var.unwrap();
        assert_eq!("longitude", longitude_var.name());
        assert_eq!(DataType::F32, longitude_var.data_type());
        assert_eq!(1, longitude_var.num_dims());
        assert!(!longitude_var.is_record_var());
        assert_eq!(5, longitude_var.len());
        assert_eq!(1, longitude_var.num_chunks());
        assert_eq!(5, longitude_var.num_elements_per_chunk());
        assert_eq!(20, longitude_var.chunk_size());
    }
    // the variable `time`
    {
        let time_var: Option<&Variable> = data_set.get_var("time");
        assert!(time_var.is_some());
        let time_var: &Variable = time_var.unwrap();
        assert_eq!("time", time_var.name());
        assert_eq!(DataType::F32, time_var.data_type());
        assert_eq!(1, time_var.num_dims());
        assert!(time_var.is_record_var());
        assert_eq!(2, time_var.len());
        assert_eq!(2, time_var.num_chunks());
        assert_eq!(1, time_var.num_elements_per_chunk());
        assert_eq!(4, time_var.chunk_size());
    }
    // the variable `temperature_i8`
    {
        let temp_var: Option<&Variable> = data_set.get_var("temperature_i8");
        assert!(temp_var.is_some());
        let temp_var: &Variable = temp_var.unwrap();
        assert_eq!("temperature_i8", temp_var.name());
        assert_eq!(DataType::I8, temp_var.data_type());
        assert_eq!(3, temp_var.num_dims());
        assert!(temp_var.is_record_var());
        assert_eq!(30, temp_var.len());
        assert_eq!(2, temp_var.num_chunks());
        assert_eq!(15, temp_var.num_elements_per_chunk());
        assert_eq!(16, temp_var.chunk_size());

        assert!(data_set.get_var("temperature_i8").is_some());
        let temp_var: &Variable = data_set.get_var("temperature_i8").unwrap();
        assert_eq!(3, temp_var.num_attrs());
        {
            assert!(temp_var.get_attr("standard_name").is_some());
            let attr: &Attribute = temp_var.get_attr("standard_name").unwrap();
            assert_eq!(DataType::U8, attr.data_type());
            assert!(attr.get_u8().is_some());
            let attr_data: &Vec<u8> = attr.get_u8().unwrap();
            assert_eq!(
                "air_temperature".as_bytes(),
                &attr_data[..],
            )
        }
        {
            assert!(temp_var.get_attr("long_name").is_some());
            let attr: &Attribute = temp_var.get_attr("long_name").unwrap();
            assert_eq!(DataType::U8, attr.data_type());
            assert!(attr.get_u8().is_some());
            let attr_data: &Vec<u8> = attr.get_u8().unwrap();
            assert_eq!(
                "TEMPERATURE".as_bytes(),
                &attr_data[..],
            )
        }
        {
            assert!(temp_var.get_attr("units").is_some());
            let attr: &Attribute = temp_var.get_attr("units").unwrap();
            assert_eq!(DataType::U8, attr.data_type());
            assert!(attr.get_u8().is_some());
            let attr_data: &Vec<u8> = attr.get_u8().unwrap();
            assert_eq!(
                "Celsius".as_bytes(),
                &attr_data[..],
            )
        }
    }

    // the variable `temperature_u8`
    {
        let temp_var: Option<&Variable> = data_set.get_var("temperature_u8");
        assert!(temp_var.is_some());
        let temp_var: &Variable = temp_var.unwrap();
        assert_eq!("temperature_u8", temp_var.name());
        assert_eq!(DataType::U8, temp_var.data_type());
        assert_eq!(3, temp_var.num_dims());
        assert!(temp_var.is_record_var());
        assert_eq!(30, temp_var.len());
        assert_eq!(2, temp_var.num_chunks());
        assert_eq!(15, temp_var.num_elements_per_chunk());
        assert_eq!(16, temp_var.chunk_size());
    }

    // the variable `temperature_u8`
    {
        let temp_var: Option<&Variable> = data_set.get_var("temperature_i16");
        assert!(temp_var.is_some());
        let temp_var: &Variable = temp_var.unwrap();
        assert_eq!("temperature_i16", temp_var.name());
        assert_eq!(DataType::I16, temp_var.data_type());
        assert_eq!(3, temp_var.num_dims());
        assert!(temp_var.is_record_var());
        assert_eq!(30, temp_var.len());
        assert_eq!(2, temp_var.num_chunks());
        assert_eq!(15, temp_var.num_elements_per_chunk());
        assert_eq!(32, temp_var.chunk_size());
    }

    // the variable `temperature_i32`
    {
        let temp_var: Option<&Variable> = data_set.get_var("temperature_i32");
        assert!(temp_var.is_some());
        let temp_var: &Variable = temp_var.unwrap();
        assert_eq!("temperature_i32", temp_var.name());
        assert_eq!(DataType::I32, temp_var.data_type());
        assert_eq!(3, temp_var.num_dims());
        assert!(temp_var.is_record_var());
        assert_eq!(30, temp_var.len());
        assert_eq!(2, temp_var.num_chunks());
        assert_eq!(15, temp_var.num_elements_per_chunk());
        assert_eq!(60, temp_var.chunk_size());
    }

    // the variable `temperature_f32`
    {
        let temp_var: Option<&Variable> = data_set.get_var("temperature_f32");
        assert!(temp_var.is_some());
        let temp_var: &Variable = temp_var.unwrap();
        assert_eq!("temperature_f32", temp_var.name());
        assert_eq!(DataType::F32, temp_var.data_type());
        assert_eq!(3, temp_var.num_dims());
        assert!(temp_var.is_record_var());
        assert_eq!(30, temp_var.len());
        assert_eq!(2, temp_var.num_chunks());
        assert_eq!(15, temp_var.num_elements_per_chunk());
        assert_eq!(60, temp_var.chunk_size());
    }
    assert_eq!(4 + 16 + 16 + 32 + 60 + 60 + 120, data_set.record_size());

    // the variable `temperature_f64`
    {
        let temp_var: Option<&Variable> = data_set.get_var("temperature_f64");
        assert!(temp_var.is_some());
        let temp_var: &Variable = temp_var.unwrap();
        assert_eq!("temperature_f64", temp_var.name());
        assert_eq!(DataType::F64, temp_var.data_type());
        assert_eq!(3, temp_var.num_dims());
        assert!(temp_var.is_record_var());
        assert_eq!(30, temp_var.len());
        assert_eq!(2, temp_var.num_chunks());
        assert_eq!(15, temp_var.num_elements_per_chunk());
        assert_eq!(120, temp_var.chunk_size());
    }

    let expected_record_size: usize = data_set.get_vars().into_iter()
        .map(|var|{
            var
        })
        .filter(|var| {
            var.is_record_var()
        })
        .fold(0, |sum, var| {
            sum + var.chunk_size()
        });

    assert_eq!(expected_record_size, data_set.record_size());
}

fn check_temperature_dataset_values(data_set: &DataSet){
    // the variable `latitude`
    {
        assert!(data_set.get_var("latitude").is_some());
        let latitude_var: &Variable = data_set.get_var("latitude").unwrap();
        assert!(latitude_var.get_f32().is_some());
        let latitude_data: &Vec<f32> = latitude_var.get_f32().unwrap();
        assert_eq!(
            &vec![0.0, 0.5, 1.0],
            latitude_data,
        )
    }
    // the variable `longitude`
    {
        assert!(data_set.get_var("longitude").is_some());
        let longitude_var: &Variable = data_set.get_var("longitude").unwrap();
        assert!(longitude_var.get_f32().is_some());
        let longitude_data: &Vec<f32> = longitude_var.get_f32().unwrap();
        assert_eq!(
            &vec![0.0, 0.5, 1.0, 1.5, 2.0],
            longitude_data,
        );
    }
    // the variable `time`
    {
        assert!(data_set.get_var("time").is_some());
        let time_var: &Variable = data_set.get_var("time").unwrap();
        assert!(time_var.get_f32().is_some());
        let time_data: &Vec<f32> = time_var.get_f32().unwrap();
        assert_eq!(
            &vec![438_300.0, 438_324.0],
            time_data,
        );
    }
    // the variable `temperature_i8`
    {
        assert!(data_set.get_var("temperature_i8").is_some());
        let temp_var: &Variable = data_set.get_var("temperature_i8").unwrap();

        assert!(temp_var.get_i8().is_some());
        assert!(temp_var.get_u8().is_none());
        assert!(temp_var.get_i16().is_none());
        assert!(temp_var.get_i32().is_none());
        assert!(temp_var.get_f32().is_none());
        assert!(temp_var.get_f64().is_none());

        let expected_temp_data: Vec<i8> = (0_i8..30).collect();
        let temp_data: &Vec<i8> = temp_var.get_i8().unwrap();
        assert_eq!(
            &expected_temp_data,
            temp_data,
        );
    }
    // the variable `temperature_u8`
    {
        assert!(data_set.get_var("temperature_u8").is_some());
        let temp_var: &Variable = data_set.get_var("temperature_u8").unwrap();

        assert!(temp_var.get_i8().is_none());
        assert!(temp_var.get_u8().is_some());
        assert!(temp_var.get_i16().is_none());
        assert!(temp_var.get_i32().is_none());
        assert!(temp_var.get_f32().is_none());
        assert!(temp_var.get_f64().is_none());

        let expected_temp_data: Vec<u8> = (0_u8..30).collect();
        let temp_data: &Vec<u8> = temp_var.get_u8().unwrap();
        assert_eq!(
            &expected_temp_data,
            temp_data,
        );
    }
    // the variable `temperature_i16`
    {
        assert!(data_set.get_var("temperature_i16").is_some());
        let temp_var: &Variable = data_set.get_var("temperature_i16").unwrap();

        assert!(temp_var.get_i8().is_none());
        assert!(temp_var.get_u8().is_none());
        assert!(temp_var.get_i16().is_some());
        assert!(temp_var.get_i32().is_none());
        assert!(temp_var.get_f32().is_none());
        assert!(temp_var.get_f64().is_none());

        let expected_temp_data: Vec<i16> = (0_i16..30).collect();
        let temp_data: &Vec<i16> = temp_var.get_i16().unwrap();
        assert_eq!(
            &expected_temp_data,
            temp_data,
        );
    }
    // the variable `temperature_i32`
    {
        assert!(data_set.get_var("temperature_i32").is_some());
        let temp_var: &Variable = data_set.get_var("temperature_i32").unwrap();

        assert!(temp_var.get_i8().is_none());
        assert!(temp_var.get_u8().is_none());
        assert!(temp_var.get_i16().is_none());
        assert!(temp_var.get_i32().is_some());
        assert!(temp_var.get_f32().is_none());
        assert!(temp_var.get_f64().is_none());

        let expected_temp_data: Vec<i32> = (0_i32..30).collect();
        let temp_data: &Vec<i32> = temp_var.get_i32().unwrap();
        assert_eq!(
            &expected_temp_data,
            temp_data,
        );
    }
    // the variable `temperature_f32`
    {
        assert!(data_set.get_var("temperature_f32").is_some());
        let temp_var: &Variable = data_set.get_var("temperature_f32").unwrap();

        assert!(temp_var.get_i8().is_none());
        assert!(temp_var.get_u8().is_none());
        assert!(temp_var.get_i16().is_none());
        assert!(temp_var.get_i32().is_none());
        assert!(temp_var.get_f32().is_some());
        assert!(temp_var.get_f64().is_none());

        let expected_temp_data: Vec<f32> = (0_i32..30).map(|i: i32| i as f32).collect();
        let temp_data: &Vec<f32> = temp_var.get_f32().unwrap();
        assert_eq!(
            &expected_temp_data,
            temp_data,
        );
    }
    // the variable `temperature_f64`
    {
        assert!(data_set.get_var("temperature_f64").is_some());
        let temp_var: &Variable = data_set.get_var("temperature_f64").unwrap();

        assert!(temp_var.get_i8().is_none());
        assert!(temp_var.get_u8().is_none());
        assert!(temp_var.get_i16().is_none());
        assert!(temp_var.get_i32().is_none());
        assert!(temp_var.get_f32().is_none());
        assert!(temp_var.get_f64().is_some());

        let expected_temp_data: Vec<f64> = (0_i32..30).map(|i: i32| i as f64).collect();
        let temp_data: &Vec<f64> = temp_var.get_f64().unwrap();
        assert_eq!(
            &expected_temp_data,
            temp_data,
        );
    }

}

#[test]
fn test_read_file_empty_variables() {
    // Copy bytes to a temporary file
    let (tmp_dir, input_data_file_path) = copy_bytes_to_tmp_file(EMPTY_VARIABLES_FILE_BYTES, EMPTY_VARIABLES_FILE_NAME);

    // Read the data set
    let data_set: DataSet = {
        let mut file_reader = FileReader::open(input_data_file_path).unwrap();
        let _ = file_reader.read_all_vars().unwrap();
        file_reader.close()
    };
    tmp_dir.close().unwrap();

    // Check the parsed header
    assert!(data_set.get_var("no_value_i8").is_some());
    assert!(data_set.get_var("no_value_u8").is_some());
    assert!(data_set.get_var("no_value_i16").is_some());
    assert!(data_set.get_var("no_value_i32").is_some());
    assert!(data_set.get_var("no_value_f32").is_some());
    assert!(data_set.get_var("no_value_f64").is_some());


    // Check the variable `no_value_i8`
    {
        let var: &Variable = data_set.get_var("no_value_i8").unwrap();
        assert!(var.get_i8().is_some());
        assert!(var.get_u8().is_none());
        assert!(var.get_i16().is_none());
        assert!(var.get_i32().is_none());
        assert!(var.get_f32().is_none());
        assert!(var.get_f64().is_none());

        let data: &Vec<i8> = var.get_i8().unwrap();
        assert!(data.is_empty());
    }
    // Check the variable `no_value_u8`
    {
        let var: &Variable = data_set.get_var("no_value_u8").unwrap();
        assert!(var.get_i8().is_none());
        assert!(var.get_u8().is_some());
        assert!(var.get_i16().is_none());
        assert!(var.get_i32().is_none());
        assert!(var.get_f32().is_none());
        assert!(var.get_f64().is_none());

        let data: &Vec<u8> = var.get_u8().unwrap();
        assert!(data.is_empty());
    }
    // Check the variable `no_value_i16`
    {
        let var: &Variable = data_set.get_var("no_value_i16").unwrap();
        assert!(var.get_i8().is_none());
        assert!(var.get_u8().is_none());
        assert!(var.get_i16().is_some());
        assert!(var.get_i32().is_none());
        assert!(var.get_f32().is_none());
        assert!(var.get_f64().is_none());

        let data: &Vec<i16> = var.get_i16().unwrap();
        assert!(data.is_empty());
    }
    // Check the variable `no_value_i32`
    {
        let var: &Variable = data_set.get_var("no_value_i32").unwrap();
        assert!(var.get_i8().is_none());
        assert!(var.get_u8().is_none());
        assert!(var.get_i16().is_none());
        assert!(var.get_i32().is_some());
        assert!(var.get_f32().is_none());
        assert!(var.get_f64().is_none());

        let data: &Vec<i32> = var.get_i32().unwrap();
        assert!(data.is_empty());
    }
    // Check the variable `no_value_f32`
    {
        let var: &Variable = data_set.get_var("no_value_f32").unwrap();
        assert!(var.get_i8().is_none());
        assert!(var.get_u8().is_none());
        assert!(var.get_i16().is_none());
        assert!(var.get_i32().is_none());
        assert!(var.get_f32().is_some());
        assert!(var.get_f64().is_none());

        let data: &Vec<f32> = var.get_f32().unwrap();
        assert!(data.is_empty());
    }
    // Check the variable `no_value_f64`
    {
        let var: &Variable = data_set.get_var("no_value_f64").unwrap();
        assert!(var.get_i8().is_none());
        assert!(var.get_u8().is_none());
        assert!(var.get_i16().is_none());
        assert!(var.get_i32().is_none());
        assert!(var.get_f32().is_none());
        assert!(var.get_f64().is_some());

        let data: &Vec<f64> = var.get_f64().unwrap();
        assert!(data.is_empty());
    }
}


#[test]
fn test_read_file_scalar_variables() {
    // Copy bytes to a temporary file
    let (tmp_dir, input_data_file_path) = copy_bytes_to_tmp_file(SCALAR_VARIABLES_FILE_BYTES, SCALAR_VARIABLES_FILE_NAME);

    // Read the data set
    let data_set: DataSet = {
        let mut file_reader = FileReader::open(input_data_file_path).unwrap();
        let _ = file_reader.read_all_vars().unwrap();
        file_reader.close()
    };
    tmp_dir.close().unwrap();

    // Check the parsed header
    assert!(data_set.get_var("scalar_value_i8").is_some());
    assert!(data_set.get_var("scalar_value_u8").is_some());
    assert!(data_set.get_var("scalar_value_i16").is_some());
    assert!(data_set.get_var("scalar_value_i32").is_some());
    assert!(data_set.get_var("scalar_value_f32").is_some());
    assert!(data_set.get_var("scalar_value_f64").is_some());


    // Check the variable `scalar_value_i8`
    {
        let var: &Variable = data_set.get_var("scalar_value_i8").unwrap();
        assert!(var.get_i8().is_some());
        assert!(var.get_u8().is_none());
        assert!(var.get_i16().is_none());
        assert!(var.get_i32().is_none());
        assert!(var.get_f32().is_none());
        assert!(var.get_f64().is_none());

       let data: &Vec<i8> = var.get_i8().unwrap();
       assert_eq!(1, data.len());
       assert_eq!(&vec![42_i8], data);
    }
    // Check the variable `scalar_value_u8`
    {
        let var: &Variable = data_set.get_var("scalar_value_u8").unwrap();
        assert!(var.get_i8().is_none());
        assert!(var.get_u8().is_some());
        assert!(var.get_i16().is_none());
        assert!(var.get_i32().is_none());
        assert!(var.get_f32().is_none());
        assert!(var.get_f64().is_none());

        let data: &Vec<u8> = var.get_u8().unwrap();
        assert_eq!(1, data.len());
        assert_eq!(&vec![42_u8], data);
    }
    // Check the variable `scalar_value_i16`
    {
        let var: &Variable = data_set.get_var("scalar_value_i16").unwrap();
        assert!(var.get_i8().is_none());
        assert!(var.get_u8().is_none());
        assert!(var.get_i16().is_some());
        assert!(var.get_i32().is_none());
        assert!(var.get_f32().is_none());
        assert!(var.get_f64().is_none());

        let data: &Vec<i16> = var.get_i16().unwrap();
        assert_eq!(1, data.len());
        assert_eq!(&vec![42_i16], data);
    }
    // Check the variable `scalar_value_i32`
    {
        let var: &Variable = data_set.get_var("scalar_value_i32").unwrap();
        assert!(var.get_i8().is_none());
        assert!(var.get_u8().is_none());
        assert!(var.get_i16().is_none());
        assert!(var.get_i32().is_some());
        assert!(var.get_f32().is_none());
        assert!(var.get_f64().is_none());

        let data: &Vec<i32> = var.get_i32().unwrap();
        assert_eq!(1, data.len());
        assert_eq!(&vec![42_i32], data);
    }
    // Check the variable `scalar_value_f32`
    {
        let var: &Variable = data_set.get_var("scalar_value_f32").unwrap();
        assert!(var.get_i8().is_none());
        assert!(var.get_u8().is_none());
        assert!(var.get_i16().is_none());
        assert!(var.get_i32().is_none());
        assert!(var.get_f32().is_some());
        assert!(var.get_f64().is_none());

        let data: &Vec<f32> = var.get_f32().unwrap();
        assert_eq!(1, data.len());
        assert_eq!(&vec![42.0_f32], data);
    }
    // Check the variable `scalar_value_f64`
    {
        let var: &Variable = data_set.get_var("scalar_value_f64").unwrap();
        assert!(var.get_i8().is_none());
        assert!(var.get_u8().is_none());
        assert!(var.get_i16().is_none());
        assert!(var.get_i32().is_none());
        assert!(var.get_f32().is_none());
        assert!(var.get_f64().is_some());

        let data: &Vec<f64> = var.get_f64().unwrap();
        assert_eq!(1, data.len());
        assert_eq!(&vec![42.0_f64], data);
    }
}