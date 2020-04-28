use netcdf3::{
    FileReader, Version, DataSet,
};

// #![allow(dead_code)]
use std::io::Write;
use tempdir::TempDir;

/// NetCDF-3 (classic version) test file name.
pub(crate) static NC3_CLASSIC_FILE_NAME: &'static str = "temp_3D_classic.nc";
pub(crate) static NC3_CLASSIC_FILE_BYTES: &'static[u8] = include_bytes!("../data/temp_3D_classic.nc");

pub fn copy_bytes_to_tmp_file(bytes: &[u8], file_name: &str) -> (TempDir, std::path::PathBuf)
{
    // Crete the temporary directory
    let tmp_dir: TempDir = TempDir::new("netcdf3_test_data").unwrap();
    // Crete the temporary file
    let tmp_file_path = std::path::PathBuf::from(tmp_dir.path()).join(file_name);
    let mut tmp_file = std::fs::File::create(tmp_file_path.clone()).unwrap();
    // Copy all bytes
    let _ = tmp_file.write_all(bytes).unwrap();

    return (tmp_dir, tmp_file_path);
}


fn main()
{
    // Copy bytes to a temporary file
    let (tmp_dir, input_data_file_path) = copy_bytes_to_tmp_file(NC3_CLASSIC_FILE_BYTES, NC3_CLASSIC_FILE_NAME);

    // Read the data set
    let data_set: DataSet = {
        let mut file_reader = FileReader::open(input_data_file_path.clone()).unwrap();
        let _ = file_reader.read_all_vars().unwrap();
        file_reader.close()
    };
    tmp_dir.close().unwrap();


    // Print miscellaneous values
    println!("FILE PATH");
    println!("---------");
    println!("NetCDF-3 file path               : {}", input_data_file_path.display());

    println!();
    println!("FILE DESCRIPTION");
    println!("----------------");
    println!("Version                          : {}", match data_set.version() {
        Version::Classic => "Classic",
        Version::Offset64Bit => "64-Bit Offset",
    });
    println!("Number of dimensions             : {}", data_set.num_dims());
    for dim in data_set.get_dims() {
        println!("    {name} = {size}{is_unlimited}",
            name=dim.name(),
            size=dim.size(),
            is_unlimited=if dim.is_unlimited() {
                " (unlimited)"
            } else {
                ""
            }
        );
    }
    println!("Number of global attributes      : {}", data_set.num_global_attrs());
    for attr in data_set.get_global_attrs() {
        println!("    {name}({data_type})",
            name=attr.name(),
            data_type=attr.data_type().c_api_name(),
        );
    }

    println!("Number of variables              : {}", data_set.num_vars());
    for var in data_set.get_vars() {
        println!(
"   {name} :
        dimensions           : {dim_list:?}
        number of elements   : {var_len},
        data type            : {data_type}
        is a record variable : {is_record}
        number of attributes : {num_attrs}",
            name=var.name(),
            var_len=var.len(),
            data_type=var.data_type().c_api_name(),
            dim_list=var.get_dim_names(),
            is_record=var.is_record_var(),
            num_attrs=var.num_attrs(),
        );
        for attr in var.get_attrs() {
            println!("            {name}({data_type})",
                name=attr.name(),
                data_type=attr.data_type().c_api_name(),
            );
        }

    }
    println!();
    println!("EXAMPLE OF ATTRIBUTE");
    println!("---------------------");
    println!(
        "The global attribute `comment` : {}",
        String::from_utf8(data_set.get_global_attr("comment").unwrap().get_u8().unwrap().clone()).unwrap()
    );

    println!();
    println!("EXAMPLE OF VARIABLE");
    println!("-------------------");
    println!(
        "The values of the variable `temperature_f32` : {:?}",
        data_set.get_var("temperature_f32").unwrap().get_f32().unwrap().clone()
    );
}