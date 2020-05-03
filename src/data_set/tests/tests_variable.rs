use crate::{DataSet, InvalidDataSet, DataType, DimensionType};

#[test]
fn test_add_var_error_invalid_name() {
    const INVALID_VAR_NAME: &str = "!invalid_name";

    let mut data_set: DataSet = DataSet::new();

    assert_eq!(0,       data_set.num_vars());
    assert_eq!(false,   data_set.has_var(INVALID_VAR_NAME));
    assert_eq!(None,    data_set.get_var_len(INVALID_VAR_NAME));
    assert_eq!(None,    data_set.get_var_data_type(INVALID_VAR_NAME));

    assert_eq!(
        InvalidDataSet::VariableNameNotValid(INVALID_VAR_NAME.to_string()),
        data_set.add_var_i8::<&str>(INVALID_VAR_NAME, &vec![]).unwrap_err()
    );

    assert_eq!(0,       data_set.num_vars());
    assert_eq!(false,   data_set.has_var(INVALID_VAR_NAME));
    assert_eq!(None,    data_set.get_var_len(INVALID_VAR_NAME));
    assert_eq!(None,    data_set.get_var_data_type(INVALID_VAR_NAME));
}

#[test]
fn test_add_var_error_already_exists() {
    const VAR_NAME: &str = "var_1";

    let mut data_set: DataSet = DataSet::new();

    assert_eq!(0,       data_set.num_vars());
    assert_eq!(false,   data_set.has_var(VAR_NAME));
    assert_eq!(None,    data_set.get_var_len(VAR_NAME));
    assert_eq!(None,    data_set.get_var_data_type(VAR_NAME));


    data_set.add_var_i8::<String>(VAR_NAME, &vec![]).unwrap();

    assert_eq!(1,                   data_set.num_vars());
    assert_eq!(true,                data_set.has_var(VAR_NAME));
    assert_eq!(Some(0),             data_set.get_var_len(VAR_NAME));
    assert_eq!(Some(DataType::I8),  data_set.get_var_data_type(VAR_NAME));

    // Try to a `i32`  variable with the same name
    assert_eq!(
        InvalidDataSet::VariableAlreadyExists(VAR_NAME.to_string()),
        data_set.add_var_i32::<String>(VAR_NAME, &vec![]).unwrap_err()
    );

    assert_eq!(1,                   data_set.num_vars());
    assert_eq!(true,                data_set.has_var(VAR_NAME));
    assert_eq!(Some(0),             data_set.get_var_len(VAR_NAME));
    assert_eq!(Some(DataType::I8),  data_set.get_var_data_type(VAR_NAME));
}

#[test]
fn test_add_var_error_unlim_dim_first() {
    // If a variable is a record-variable, then the unlimited dimension must be the first dimension of this one
    const VAR_NAME: &str = "var_1";
    const FIXED_DIM_NAME: &str = "fixed_dim";
    const FIXED_DIM_SIZE: usize = 3;
    const UNLIMITED_DIM_NAME: &str = "unlimited_dim";
    const UNLIMITED_DIM_SIZE: usize = 2;
    const INVALID_DIM_LIST: [&str; 2] = [FIXED_DIM_NAME, UNLIMITED_DIM_NAME];

    let mut data_set: DataSet = DataSet::new();

    assert_eq!(0,       data_set.num_vars());
    assert_eq!(false,   data_set.has_var(VAR_NAME));
    assert_eq!(None,    data_set.get_var_len(VAR_NAME));
    assert_eq!(None,    data_set.get_var_data_type(VAR_NAME));
    assert_eq!(0,       data_set.num_dims());
    assert_eq!(false,   data_set.has_unlimited_dim());
    assert_eq!(false,   data_set.has_dim(FIXED_DIM_NAME));
    assert_eq!(None,    data_set.get_dim_size(FIXED_DIM_NAME));
    assert_eq!(None,    data_set.get_dim_type(FIXED_DIM_NAME));
    assert_eq!(false,   data_set.has_dim(UNLIMITED_DIM_NAME));
    assert_eq!(None,    data_set.get_dim_size(UNLIMITED_DIM_NAME));
    assert_eq!(None,    data_set.get_dim_type(UNLIMITED_DIM_NAME));

    data_set.add_fixed_dim(FIXED_DIM_NAME, FIXED_DIM_SIZE).unwrap();
    data_set.set_unlimited_dim(UNLIMITED_DIM_NAME, UNLIMITED_DIM_SIZE).unwrap();

    assert_eq!(0,                                   data_set.num_vars());
    assert_eq!(false,                               data_set.has_var(VAR_NAME));
    assert_eq!(None,                                data_set.get_var_len(VAR_NAME));
    assert_eq!(None,                                data_set.get_var_data_type(VAR_NAME));
    assert_eq!(2,                                   data_set.num_dims());
    assert_eq!(true,                                data_set.has_unlimited_dim());
    assert_eq!(true,                                data_set.has_dim(FIXED_DIM_NAME));
    assert_eq!(Some(FIXED_DIM_SIZE),                data_set.get_dim_size(FIXED_DIM_NAME));
    assert_eq!(Some(DimensionType::FixedSize),      data_set.get_dim_type(FIXED_DIM_NAME));
    assert_eq!(true,                                data_set.has_dim(UNLIMITED_DIM_NAME));
    assert_eq!(Some(UNLIMITED_DIM_SIZE),            data_set.get_dim_size(UNLIMITED_DIM_NAME));
    assert_eq!(Some(DimensionType::UnlimitedSize),  data_set.get_dim_type(UNLIMITED_DIM_NAME));

    assert_eq!(
        InvalidDataSet::UnlimitedDimensionMustBeDefinedFirst{
            var_name: VAR_NAME.to_string(),
            unlim_dim_name: UNLIMITED_DIM_NAME.to_string(),
            get_dim_names: INVALID_DIM_LIST.iter().map(|s: &&str| String::from(*s)).collect()
        },
        data_set.add_var_i8(VAR_NAME, &INVALID_DIM_LIST[..]).unwrap_err()
    );

    assert_eq!(0,                                   data_set.num_vars());
    assert_eq!(false,                               data_set.has_var(VAR_NAME));
    assert_eq!(None,                                data_set.get_var_len(VAR_NAME));
    assert_eq!(None,                                data_set.get_var_data_type(VAR_NAME));
    assert_eq!(2,                                   data_set.num_dims());
    assert_eq!(true,                                data_set.has_unlimited_dim());
    assert_eq!(true,                                data_set.has_dim(FIXED_DIM_NAME));
    assert_eq!(Some(FIXED_DIM_SIZE),                data_set.get_dim_size(FIXED_DIM_NAME));
    assert_eq!(Some(DimensionType::FixedSize),      data_set.get_dim_type(FIXED_DIM_NAME));
    assert_eq!(true,                                data_set.has_dim(UNLIMITED_DIM_NAME));
    assert_eq!(Some(UNLIMITED_DIM_SIZE),            data_set.get_dim_size(UNLIMITED_DIM_NAME));
    assert_eq!(Some(DimensionType::UnlimitedSize),  data_set.get_dim_type(UNLIMITED_DIM_NAME));
}

#[test]
fn test_add_var_error_fixed_dim_used_multiple_times_with() {
    // If a variable is a record-variable, then the unlimited dimension must be the first dimnesion of this one
    const VAR_NAME: &str = "var_1";
    const DIM_NAME_1: &str = "dim_1";
    const DIM_NAME_2: &str = "dim_2";
    const DIM_SIZE_1: usize = 10;
    const DIM_SIZE_2: usize = 20;
    const INVALID_DIM_LIST: [&str; 3] = [DIM_NAME_1, DIM_NAME_2, DIM_NAME_1];

    let mut data_set: DataSet = DataSet::new();

    assert_eq!(0,       data_set.num_vars());
    assert_eq!(false,   data_set.has_var(VAR_NAME));
    assert_eq!(None,    data_set.get_var_len(VAR_NAME));
    assert_eq!(None,    data_set.get_var_data_type(VAR_NAME));
    assert_eq!(0,       data_set.num_dims());
    assert_eq!(false,   data_set.has_unlimited_dim());
    assert_eq!(false,   data_set.has_dim(DIM_NAME_1));
    assert_eq!(None,    data_set.get_dim_size(DIM_NAME_1));
    assert_eq!(None,    data_set.get_dim_type(DIM_NAME_1));
    assert_eq!(false,   data_set.has_dim(DIM_NAME_2));
    assert_eq!(None,    data_set.get_dim_size(DIM_NAME_2));
    assert_eq!(None,    data_set.get_dim_type(DIM_NAME_2));

    data_set.add_fixed_dim(DIM_NAME_1, DIM_SIZE_1).unwrap();
    data_set.add_fixed_dim(DIM_NAME_2, DIM_SIZE_2).unwrap();

    assert_eq!(0,                               data_set.num_vars());
    assert_eq!(false,                           data_set.has_var(VAR_NAME));
    assert_eq!(None,                            data_set.get_var_len(VAR_NAME));
    assert_eq!(None,                            data_set.get_var_data_type(VAR_NAME));
    assert_eq!(2,                               data_set.num_dims());
    assert_eq!(false,                           data_set.has_unlimited_dim());
    assert_eq!(true,                            data_set.has_dim(DIM_NAME_1));
    assert_eq!(Some(DIM_SIZE_1),                data_set.get_dim_size(DIM_NAME_1));
    assert_eq!(Some(DimensionType::FixedSize),  data_set.get_dim_type(DIM_NAME_1));
    assert_eq!(true,                            data_set.has_dim(DIM_NAME_2));
    assert_eq!(Some(DIM_SIZE_2),                data_set.get_dim_size(DIM_NAME_2));
    assert_eq!(Some(DimensionType::FixedSize),  data_set.get_dim_type(DIM_NAME_2));

    assert_eq!(
        InvalidDataSet::DimensionsUsedMultipleTimes{
            var_name: VAR_NAME.to_string(),
            get_dim_names: INVALID_DIM_LIST.iter().map(|s: &&str| String::from(*s)).collect()
        },
        data_set.add_var_i8(VAR_NAME, &INVALID_DIM_LIST).unwrap_err()
    );

    assert_eq!(0,                               data_set.num_vars());
    assert_eq!(false,                           data_set.has_var(VAR_NAME));
    assert_eq!(None,                            data_set.get_var_len(VAR_NAME));
    assert_eq!(None,                            data_set.get_var_data_type(VAR_NAME));
    assert_eq!(2,                               data_set.num_dims());
    assert_eq!(false,                           data_set.has_unlimited_dim());
    assert_eq!(true,                            data_set.has_dim(DIM_NAME_1));
    assert_eq!(Some(DIM_SIZE_1),                data_set.get_dim_size(DIM_NAME_1));
    assert_eq!(Some(DimensionType::FixedSize),  data_set.get_dim_type(DIM_NAME_1));
    assert_eq!(true,                            data_set.has_dim(DIM_NAME_2));
    assert_eq!(Some(DIM_SIZE_2),                data_set.get_dim_size(DIM_NAME_2));
    assert_eq!(Some(DimensionType::FixedSize),  data_set.get_dim_type(DIM_NAME_2));
}

#[test]
fn test_add_var_error_undef_dim() {
    // A variable can't be defined over undefined dimensions.
    const VAR_NAME: &str = "var_1";
    const DIM_NAME_1: &str = "dim_1";
    const DIM_NAME_2: &str = "dim_2";
    const UNDEF_DIM_NAME_1: &str = "undef_dim_1";
    const UNDEF_DIM_NAME_2: &str = "undef_dim_2";
    const DIM_SIZE_1: usize = 10;
    const DIM_SIZE_2: usize = 20;
    const INVALID_DIM_LIST: [&str; 4] = [UNDEF_DIM_NAME_1, DIM_NAME_1, DIM_NAME_2, UNDEF_DIM_NAME_2];

    let mut data_set: DataSet = DataSet::new();

    assert_eq!(0,       data_set.num_vars());
    assert_eq!(false,   data_set.has_var(VAR_NAME));
    assert_eq!(None,    data_set.get_var_len(VAR_NAME));
    assert_eq!(None,    data_set.get_var_data_type(VAR_NAME));
    assert_eq!(0,       data_set.num_dims());
    assert_eq!(false,   data_set.has_unlimited_dim());
    assert_eq!(false,   data_set.has_dim(DIM_NAME_1));
    assert_eq!(None,    data_set.get_dim_size(DIM_NAME_1));
    assert_eq!(None,    data_set.get_dim_type(DIM_NAME_1));
    assert_eq!(false,   data_set.has_dim(DIM_NAME_2));
    assert_eq!(None,    data_set.get_dim_size(DIM_NAME_2));
    assert_eq!(None,    data_set.get_dim_type(DIM_NAME_2));
    assert_eq!(false,   data_set.has_dim(UNDEF_DIM_NAME_1));
    assert_eq!(None,    data_set.get_dim_size(UNDEF_DIM_NAME_1));
    assert_eq!(None,    data_set.get_dim_type(UNDEF_DIM_NAME_1));
    assert_eq!(false,   data_set.has_dim(UNDEF_DIM_NAME_2));
    assert_eq!(None,    data_set.get_dim_size(UNDEF_DIM_NAME_2));
    assert_eq!(None,    data_set.get_dim_type(UNDEF_DIM_NAME_2));

    data_set.add_fixed_dim(DIM_NAME_1, DIM_SIZE_1).unwrap();
    data_set.add_fixed_dim(DIM_NAME_2, DIM_SIZE_2).unwrap();

    assert_eq!(0,                               data_set.num_vars());
    assert_eq!(false,                           data_set.has_var(VAR_NAME));
    assert_eq!(None,                            data_set.get_var_len(VAR_NAME));
    assert_eq!(None,                            data_set.get_var_data_type(VAR_NAME));
    assert_eq!(2,                               data_set.num_dims());
    assert_eq!(false,                           data_set.has_unlimited_dim());
    assert_eq!(true,                            data_set.has_dim(DIM_NAME_1));
    assert_eq!(Some(DIM_SIZE_1),                data_set.get_dim_size(DIM_NAME_1));
    assert_eq!(Some(DimensionType::FixedSize),  data_set.get_dim_type(DIM_NAME_1));
    assert_eq!(true,                            data_set.has_dim(DIM_NAME_2));
    assert_eq!(Some(DIM_SIZE_2),                data_set.get_dim_size(DIM_NAME_2));
    assert_eq!(Some(DimensionType::FixedSize),  data_set.get_dim_type(DIM_NAME_2));
    assert_eq!(false,                           data_set.has_dim(UNDEF_DIM_NAME_1));
    assert_eq!(None,                            data_set.get_dim_size(UNDEF_DIM_NAME_1));
    assert_eq!(None,                            data_set.get_dim_type(UNDEF_DIM_NAME_1));
    assert_eq!(false,                           data_set.has_dim(UNDEF_DIM_NAME_2));
    assert_eq!(None,                            data_set.get_dim_size(UNDEF_DIM_NAME_2));
    assert_eq!(None,                            data_set.get_dim_type(UNDEF_DIM_NAME_2));

    assert_eq!(
        InvalidDataSet::DimensionsNotDefined{
            var_name: VAR_NAME.to_string(),
            get_undef_dim_names: vec![
                String::from(UNDEF_DIM_NAME_1),
                String::from(UNDEF_DIM_NAME_2),
            ]
        },
        data_set.add_var_i8(VAR_NAME, &INVALID_DIM_LIST).unwrap_err()
    );

    assert_eq!(0,                               data_set.num_vars());
    assert_eq!(false,                           data_set.has_var(VAR_NAME));
    assert_eq!(None,                            data_set.get_var_len(VAR_NAME));
    assert_eq!(None,                            data_set.get_var_data_type(VAR_NAME));
    assert_eq!(2,                               data_set.num_dims());
    assert_eq!(false,                           data_set.has_unlimited_dim());
    assert_eq!(true,                            data_set.has_dim(DIM_NAME_1));
    assert_eq!(Some(DIM_SIZE_1),                data_set.get_dim_size(DIM_NAME_1));
    assert_eq!(Some(DimensionType::FixedSize),  data_set.get_dim_type(DIM_NAME_1));
    assert_eq!(true,                            data_set.has_dim(DIM_NAME_2));
    assert_eq!(Some(DIM_SIZE_2),                data_set.get_dim_size(DIM_NAME_2));
    assert_eq!(Some(DimensionType::FixedSize),  data_set.get_dim_type(DIM_NAME_2));
    assert_eq!(false,                           data_set.has_dim(UNDEF_DIM_NAME_1));
    assert_eq!(None,                            data_set.get_dim_size(UNDEF_DIM_NAME_1));
    assert_eq!(None,                            data_set.get_dim_type(UNDEF_DIM_NAME_1));
    assert_eq!(false,                           data_set.has_dim(UNDEF_DIM_NAME_2));
    assert_eq!(None,                            data_set.get_dim_size(UNDEF_DIM_NAME_2));
    assert_eq!(None,                            data_set.get_dim_type(UNDEF_DIM_NAME_2));
}

#[test]
fn test_set_var_data_error_undef_var() {
    const UNDEF_VAR_NAME: &str = "undef_var";


    let mut data_set: DataSet = DataSet::new();
    assert_eq!(0,       data_set.num_vars());
    assert_eq!(false,   data_set.has_var(UNDEF_VAR_NAME));
    assert_eq!(false,   data_set.has_var(UNDEF_VAR_NAME));
    assert_eq!(None,    data_set.get_var_len(UNDEF_VAR_NAME));

    assert_eq!(
        InvalidDataSet::VariableNotDefined(String::from(UNDEF_VAR_NAME)),
        data_set.set_var_i8(UNDEF_VAR_NAME, vec![0, 1, 2]).unwrap_err()
    );

    assert_eq!(0,       data_set.num_vars());
    assert_eq!(false,   data_set.has_var(UNDEF_VAR_NAME));
    assert_eq!(false,   data_set.has_var(UNDEF_VAR_NAME));
    assert_eq!(None,    data_set.get_var_len(UNDEF_VAR_NAME));
}

const DATA_I8: &'static [i8; 6] = &[0, 1, 2, 3, 4, 5];
const DATA_U8: &'static [u8; 6] = &[0, 1, 2, 3, 4, 5];
const DATA_I16: &'static [i16; 6] = &[0, 1, 2, 3, 4, 5];
const DATA_I32: &'static [i32; 6] = &[0, 1, 2, 3, 4, 5];
const DATA_F32: &'static [f32; 6] = &[0.0, 1.0, 2.0, 3.0, 4.0, 5.0];
const DATA_F64: &'static [f64; 6] = &[0.0, 1.0, 2.0, 3.0, 4.0, 5.0];

#[test]
fn test_var_i8() {
    const VAR_NAME: &str = "var_i8";
    const DIM_NAME_1: &str = "dim_1";
    const DIM_NAME_2: &str = "dim_2";
    const DIM_SIZE_1: usize = 2;
    const DIM_SIZE_2: usize = 3;
    const DATA_I8_LEN: usize = DATA_I8.len();
    assert_eq!(DATA_I8_LEN, DIM_SIZE_1 * DIM_SIZE_2);

    let mut data_set: DataSet = DataSet::new();

    assert_eq!(0,       data_set.num_vars());
    assert_eq!(false,   data_set.has_var(VAR_NAME));
    assert_eq!(None,    data_set.get_var_len(VAR_NAME));
    assert_eq!(None,    data_set.get_var_data_type(VAR_NAME));
    assert_eq!(0,       data_set.num_dims());
    assert_eq!(false,   data_set.has_unlimited_dim());
    assert_eq!(false,   data_set.has_dim(DIM_NAME_1));
    assert_eq!(None,    data_set.get_dim_size(DIM_NAME_1));
    assert_eq!(None,    data_set.get_dim_type(DIM_NAME_1));
    assert_eq!(false,   data_set.has_dim(DIM_NAME_2));
    assert_eq!(None,    data_set.get_dim_size(DIM_NAME_2));
    assert_eq!(None,    data_set.get_dim_type(DIM_NAME_2));

    // Define new dimensions
    data_set.set_unlimited_dim(DIM_NAME_1, DIM_SIZE_1).unwrap();
    data_set.add_fixed_dim(DIM_NAME_2, DIM_SIZE_2).unwrap();

    assert_eq!(0,                                   data_set.num_vars());
    assert_eq!(false,                               data_set.has_var(VAR_NAME));
    assert_eq!(None,                                data_set.get_var_len(VAR_NAME));
    assert_eq!(None,                                data_set.get_var_data_type(VAR_NAME));
    assert_eq!(2,                                   data_set.num_dims());
    assert_eq!(Some(DIM_SIZE_1),                    data_set.get_dim_size(DIM_NAME_1));
    assert_eq!(true,                                data_set.has_unlimited_dim());
    assert_eq!(true,                                data_set.has_dim(DIM_NAME_1));
    assert_eq!(Some(DimensionType::UnlimitedSize),  data_set.get_dim_type(DIM_NAME_1));
    assert_eq!(true,                                data_set.has_dim(DIM_NAME_2));
    assert_eq!(Some(DIM_SIZE_2),                    data_set.get_dim_size(DIM_NAME_2));
    assert_eq!(Some(DimensionType::FixedSize),      data_set.get_dim_type(DIM_NAME_2));

    // Define a new variable
    data_set.add_var_i8(VAR_NAME, &[DIM_NAME_1, DIM_NAME_2]).unwrap();

    assert_eq!(1,                                   data_set.num_vars());
    assert_eq!(true,                                data_set.has_var(VAR_NAME));
    assert_eq!(Some(DATA_I8_LEN),                   data_set.get_var_len(VAR_NAME));
    assert_eq!(Some(DataType::I8),                  data_set.get_var_data_type(VAR_NAME));
    assert_eq!(2,                                   data_set.num_dims());
    assert_eq!(Some(DIM_SIZE_1),                    data_set.get_dim_size(DIM_NAME_1));
    assert_eq!(true,                                data_set.has_unlimited_dim());
    assert_eq!(true,                                data_set.has_dim(DIM_NAME_1));
    assert_eq!(Some(DimensionType::UnlimitedSize),  data_set.get_dim_type(DIM_NAME_1));
    assert_eq!(true,                                data_set.has_dim(DIM_NAME_2));
    assert_eq!(Some(DIM_SIZE_2),                    data_set.get_dim_size(DIM_NAME_2));
    assert_eq!(Some(DimensionType::FixedSize),      data_set.get_dim_type(DIM_NAME_2));

    // Here, no data has been yet set
    assert_eq!(None, data_set.get_var_i8(VAR_NAME));
    assert_eq!(None, data_set.get_var_u8(VAR_NAME));
    assert_eq!(None, data_set.get_var_i16(VAR_NAME));
    assert_eq!(None, data_set.get_var_i32(VAR_NAME));
    assert_eq!(None, data_set.get_var_f32(VAR_NAME));
    assert_eq!(None, data_set.get_var_f64(VAR_NAME));

    // Try to set wrong typed data
    {
        // Try to set `u8` data
        assert_eq!(
            InvalidDataSet::VariableMismatchDataType{var_name: String::from(VAR_NAME), req: DataType::I8, get: DataType::U8},
            data_set.set_var_u8(VAR_NAME, DATA_U8.to_vec()).unwrap_err()
        );
        // Try to set `i16` data
        assert_eq!(
            InvalidDataSet::VariableMismatchDataType{var_name: String::from(VAR_NAME), req: DataType::I8, get: DataType::I16},
            data_set.set_var_i16(VAR_NAME, DATA_I16.to_vec()).unwrap_err()
        );
        // Try to set `i32` data
        assert_eq!(
            InvalidDataSet::VariableMismatchDataType{var_name: String::from(VAR_NAME), req: DataType::I8, get: DataType::I32},
            data_set.set_var_i32(VAR_NAME, DATA_I32.to_vec()).unwrap_err()
        );
        // Try to set `f32` data
        assert_eq!(
            InvalidDataSet::VariableMismatchDataType{var_name: String::from(VAR_NAME), req: DataType::I8, get: DataType::F32},
            data_set.set_var_f32(VAR_NAME, DATA_F32.to_vec()).unwrap_err()
        );
        // Try to set `f64` data
        assert_eq!(
            InvalidDataSet::VariableMismatchDataType{var_name: String::from(VAR_NAME), req: DataType::I8, get: DataType::F64},
            data_set.set_var_f64(VAR_NAME, DATA_F64.to_vec()).unwrap_err()
        );
    }
    // Try to set data with a wrong number of elements
    {
        assert_eq!(
            InvalidDataSet::VariableMismatchDataLength{var_name: String::from(VAR_NAME), req: DATA_I8_LEN, get: DATA_I8_LEN - 1},
            data_set.set_var_i8(VAR_NAME, DATA_I8[0..DATA_I8_LEN-1].to_vec()).unwrap_err()
        );
    }
    // Here, no data has been yet set
    assert_eq!(None, data_set.get_var_i8(VAR_NAME));
    assert_eq!(None, data_set.get_var_u8(VAR_NAME));
    assert_eq!(None, data_set.get_var_i16(VAR_NAME));
    assert_eq!(None, data_set.get_var_i32(VAR_NAME));
    assert_eq!(None, data_set.get_var_f32(VAR_NAME));
    assert_eq!(None, data_set.get_var_f64(VAR_NAME));

    // Set a valid data vector
    data_set.set_var_i8(VAR_NAME, DATA_I8.to_vec()).unwrap();

    // Check the stored vector
    assert_eq!(Some(&DATA_I8[..]),  data_set.get_var_i8(VAR_NAME));
    assert_eq!(None,                data_set.get_var_u8(VAR_NAME));
    assert_eq!(None,                data_set.get_var_i16(VAR_NAME));
    assert_eq!(None,                data_set.get_var_i32(VAR_NAME));
    assert_eq!(None,                data_set.get_var_f32(VAR_NAME));
    assert_eq!(None,                data_set.get_var_f64(VAR_NAME))
}

#[test]
fn test_set_var_u8() {
    const VAR_NAME: &str = "var_u8";
    const DIM_NAME_1: &str = "dim_1";
    const DIM_NAME_2: &str = "dim_2";
    const DIM_SIZE_1: usize = 2;
    const DIM_SIZE_2: usize = 3;
    const DATA_U8_LEN: usize = DATA_U8.len();
    assert_eq!(DATA_U8_LEN, DIM_SIZE_1 * DIM_SIZE_2);

    let mut data_set: DataSet = DataSet::new();

    assert_eq!(0,       data_set.num_vars());
    assert_eq!(false,   data_set.has_var(VAR_NAME));
    assert_eq!(None,    data_set.get_var_len(VAR_NAME));
    assert_eq!(None,    data_set.get_var_data_type(VAR_NAME));
    assert_eq!(0,       data_set.num_dims());
    assert_eq!(false,   data_set.has_unlimited_dim());
    assert_eq!(false,   data_set.has_dim(DIM_NAME_1));
    assert_eq!(None,    data_set.get_dim_size(DIM_NAME_1));
    assert_eq!(None,    data_set.get_dim_type(DIM_NAME_1));
    assert_eq!(false,   data_set.has_dim(DIM_NAME_2));
    assert_eq!(None,    data_set.get_dim_size(DIM_NAME_2));
    assert_eq!(None,    data_set.get_dim_type(DIM_NAME_2));

    // Define new dimensions
    data_set.set_unlimited_dim(DIM_NAME_1, DIM_SIZE_1).unwrap();
    data_set.add_fixed_dim(DIM_NAME_2, DIM_SIZE_2).unwrap();

    assert_eq!(0,                                   data_set.num_vars());
    assert_eq!(false,                               data_set.has_var(VAR_NAME));
    assert_eq!(None,                                data_set.get_var_len(VAR_NAME));
    assert_eq!(None,                                data_set.get_var_data_type(VAR_NAME));
    assert_eq!(2,                                   data_set.num_dims());
    assert_eq!(Some(DIM_SIZE_1),                    data_set.get_dim_size(DIM_NAME_1));
    assert_eq!(true,                                data_set.has_unlimited_dim());
    assert_eq!(true,                                data_set.has_dim(DIM_NAME_1));
    assert_eq!(Some(DimensionType::UnlimitedSize),  data_set.get_dim_type(DIM_NAME_1));
    assert_eq!(true,                                data_set.has_dim(DIM_NAME_2));
    assert_eq!(Some(DIM_SIZE_2),                    data_set.get_dim_size(DIM_NAME_2));
    assert_eq!(Some(DimensionType::FixedSize),      data_set.get_dim_type(DIM_NAME_2));

    // Define a new variable
    data_set.add_var_u8(VAR_NAME, &[DIM_NAME_1, DIM_NAME_2]).unwrap();


    assert_eq!(1,                                   data_set.num_vars());
    assert_eq!(true,                                data_set.has_var(VAR_NAME));
    assert_eq!(Some(DATA_U8_LEN),                   data_set.get_var_len(VAR_NAME));
    assert_eq!(Some(DataType::U8),                  data_set.get_var_data_type(VAR_NAME));
    assert_eq!(2,                                   data_set.num_dims());
    assert_eq!(Some(DIM_SIZE_1),                    data_set.get_dim_size(DIM_NAME_1));
    assert_eq!(true,                                data_set.has_unlimited_dim());
    assert_eq!(true,                                data_set.has_dim(DIM_NAME_1));
    assert_eq!(Some(DimensionType::UnlimitedSize),  data_set.get_dim_type(DIM_NAME_1));
    assert_eq!(true,                                data_set.has_dim(DIM_NAME_2));
    assert_eq!(Some(DIM_SIZE_2),                    data_set.get_dim_size(DIM_NAME_2));
    assert_eq!(Some(DimensionType::FixedSize),      data_set.get_dim_type(DIM_NAME_2));

    // Here, no data has been yet set
    assert_eq!(None, data_set.get_var_i8(VAR_NAME));
    assert_eq!(None, data_set.get_var_u8(VAR_NAME));
    assert_eq!(None, data_set.get_var_i16(VAR_NAME));
    assert_eq!(None, data_set.get_var_i32(VAR_NAME));
    assert_eq!(None, data_set.get_var_f32(VAR_NAME));
    assert_eq!(None, data_set.get_var_f64(VAR_NAME));

    // Try to set wrong typed data
    {
        // Try to set `i8` data
        assert_eq!(
            InvalidDataSet::VariableMismatchDataType{var_name: String::from(VAR_NAME), req: DataType::U8, get: DataType::I8},
            data_set.set_var_i8(VAR_NAME, DATA_I8.to_vec()).unwrap_err()
        );
        // Try to set `i16` data
        assert_eq!(
            InvalidDataSet::VariableMismatchDataType{var_name: String::from(VAR_NAME), req: DataType::U8, get: DataType::I16},
            data_set.set_var_i16(VAR_NAME, DATA_I16.to_vec()).unwrap_err()
        );
        // Try to set `i32` data
        assert_eq!(
            InvalidDataSet::VariableMismatchDataType{var_name: String::from(VAR_NAME), req: DataType::U8, get: DataType::I32},
            data_set.set_var_i32(VAR_NAME, DATA_I32.to_vec()).unwrap_err()
        );
        // Try to set `f32` data
        assert_eq!(
            InvalidDataSet::VariableMismatchDataType{var_name: String::from(VAR_NAME), req: DataType::U8, get: DataType::F32},
            data_set.set_var_f32(VAR_NAME, DATA_F32.to_vec()).unwrap_err()
        );
        // Try to set `f64` data
        assert_eq!(
            InvalidDataSet::VariableMismatchDataType{var_name: String::from(VAR_NAME), req: DataType::U8, get: DataType::F64},
            data_set.set_var_f64(VAR_NAME, DATA_F64.to_vec()).unwrap_err()
        );
    }
    // Try to set data with a wrong number of elements
    {
        assert_eq!(
            InvalidDataSet::VariableMismatchDataLength{var_name: String::from(VAR_NAME), req: DATA_U8_LEN, get: DATA_U8_LEN - 1},
            data_set.set_var_u8(VAR_NAME, DATA_U8[0..DATA_U8_LEN-1].to_vec()).unwrap_err()
        );
    }
    // Here, no data has been yet set
    assert_eq!(None, data_set.get_var_i8(VAR_NAME));
    assert_eq!(None, data_set.get_var_u8(VAR_NAME));
    assert_eq!(None, data_set.get_var_i16(VAR_NAME));
    assert_eq!(None, data_set.get_var_i32(VAR_NAME));
    assert_eq!(None, data_set.get_var_f32(VAR_NAME));
    assert_eq!(None, data_set.get_var_f64(VAR_NAME));

    // Set a valid data vector
    data_set.set_var_u8(VAR_NAME, DATA_U8.to_vec()).unwrap();

    // Check the stored vector
    assert_eq!(None,                data_set.get_var_i8(VAR_NAME));
    assert_eq!(Some(&DATA_U8[..]),  data_set.get_var_u8(VAR_NAME));
    assert_eq!(None,                data_set.get_var_i16(VAR_NAME));
    assert_eq!(None,                data_set.get_var_i32(VAR_NAME));
    assert_eq!(None,                data_set.get_var_f32(VAR_NAME));
    assert_eq!(None,                data_set.get_var_f64(VAR_NAME));
}

#[test]
fn test_set_var_i16() {
    const VAR_NAME: &str = "var_i16";
    const DIM_NAME_1: &str = "dim_1";
    const DIM_NAME_2: &str = "dim_2";
    const DIM_SIZE_1: usize = 2;
    const DIM_SIZE_2: usize = 3;
    const DATA_I16_LEN: usize = DATA_I16.len();
    assert_eq!(DATA_I16_LEN, DIM_SIZE_1 * DIM_SIZE_2);

    let mut data_set: DataSet = DataSet::new();

    assert_eq!(0,       data_set.num_vars());
    assert_eq!(false,   data_set.has_var(VAR_NAME));
    assert_eq!(None,    data_set.get_var_len(VAR_NAME));
    assert_eq!(None,    data_set.get_var_data_type(VAR_NAME));
    assert_eq!(0,       data_set.num_dims());
    assert_eq!(false,   data_set.has_unlimited_dim());
    assert_eq!(false,   data_set.has_dim(DIM_NAME_1));
    assert_eq!(None,    data_set.get_dim_size(DIM_NAME_1));
    assert_eq!(None,    data_set.get_dim_type(DIM_NAME_1));
    assert_eq!(false,   data_set.has_dim(DIM_NAME_2));
    assert_eq!(None,    data_set.get_dim_size(DIM_NAME_2));
    assert_eq!(None,    data_set.get_dim_type(DIM_NAME_2));

    // Define new dimensions
    data_set.set_unlimited_dim(DIM_NAME_1, DIM_SIZE_1).unwrap();
    data_set.add_fixed_dim(DIM_NAME_2, DIM_SIZE_2).unwrap();

    assert_eq!(0,                                   data_set.num_vars());
    assert_eq!(false,                               data_set.has_var(VAR_NAME));
    assert_eq!(None,                                data_set.get_var_len(VAR_NAME));
    assert_eq!(None,                                data_set.get_var_data_type(VAR_NAME));
    assert_eq!(2,                                   data_set.num_dims());
    assert_eq!(Some(DIM_SIZE_1),                    data_set.get_dim_size(DIM_NAME_1));
    assert_eq!(true,                                data_set.has_unlimited_dim());
    assert_eq!(true,                                data_set.has_dim(DIM_NAME_1));
    assert_eq!(Some(DimensionType::UnlimitedSize),  data_set.get_dim_type(DIM_NAME_1));
    assert_eq!(true,                                data_set.has_dim(DIM_NAME_2));
    assert_eq!(Some(DIM_SIZE_2),                    data_set.get_dim_size(DIM_NAME_2));
    assert_eq!(Some(DimensionType::FixedSize),      data_set.get_dim_type(DIM_NAME_2));

    // Define a new variable
    data_set.add_var_i16(VAR_NAME, &[DIM_NAME_1, DIM_NAME_2]).unwrap();


    assert_eq!(1,                                   data_set.num_vars());
    assert_eq!(true,                                data_set.has_var(VAR_NAME));
    assert_eq!(Some(DATA_I16_LEN),                  data_set.get_var_len(VAR_NAME));
    assert_eq!(Some(DataType::I16),                 data_set.get_var_data_type(VAR_NAME));
    assert_eq!(2,                                   data_set.num_dims());
    assert_eq!(Some(DIM_SIZE_1),                    data_set.get_dim_size(DIM_NAME_1));
    assert_eq!(true,                                data_set.has_unlimited_dim());
    assert_eq!(true,                                data_set.has_dim(DIM_NAME_1));
    assert_eq!(Some(DimensionType::UnlimitedSize),  data_set.get_dim_type(DIM_NAME_1));
    assert_eq!(true,                                data_set.has_dim(DIM_NAME_2));
    assert_eq!(Some(DIM_SIZE_2),                    data_set.get_dim_size(DIM_NAME_2));
    assert_eq!(Some(DimensionType::FixedSize),      data_set.get_dim_type(DIM_NAME_2));

    // Here, no data has been yet set
    assert_eq!(None, data_set.get_var_i8(VAR_NAME));
    assert_eq!(None, data_set.get_var_u8(VAR_NAME));
    assert_eq!(None, data_set.get_var_i16(VAR_NAME));
    assert_eq!(None, data_set.get_var_i32(VAR_NAME));
    assert_eq!(None, data_set.get_var_f32(VAR_NAME));
    assert_eq!(None, data_set.get_var_f64(VAR_NAME));

    // Try to set wrong typed data
    {
        // Try to set `i8` data
        assert_eq!(
            InvalidDataSet::VariableMismatchDataType{var_name: String::from(VAR_NAME), req: DataType::I16, get: DataType::I8},
            data_set.set_var_i8(VAR_NAME, DATA_I8.to_vec()).unwrap_err()
        );
        // Try to set `u8` data
        assert_eq!(
            InvalidDataSet::VariableMismatchDataType{var_name: String::from(VAR_NAME), req: DataType::I16, get: DataType::U8},
            data_set.set_var_u8(VAR_NAME, DATA_U8.to_vec()).unwrap_err()
        );
        // Try to set `i32` data
        assert_eq!(
            InvalidDataSet::VariableMismatchDataType{var_name: String::from(VAR_NAME), req: DataType::I16, get: DataType::I32},
            data_set.set_var_i32(VAR_NAME, DATA_I32.to_vec()).unwrap_err()
        );
        // Try to set `f32` data
        assert_eq!(
            InvalidDataSet::VariableMismatchDataType{var_name: String::from(VAR_NAME), req: DataType::I16, get: DataType::F32},
            data_set.set_var_f32(VAR_NAME, DATA_F32.to_vec()).unwrap_err()
        );
        // Try to set `f64` data
        assert_eq!(
            InvalidDataSet::VariableMismatchDataType{var_name: String::from(VAR_NAME), req: DataType::I16, get: DataType::F64},
            data_set.set_var_f64(VAR_NAME, DATA_F64.to_vec()).unwrap_err()
        );
    }
    // Try to set data with a wrong number of elements
    {
        assert_eq!(
            InvalidDataSet::VariableMismatchDataLength{var_name: String::from(VAR_NAME), req: DATA_I16_LEN, get: DATA_I16_LEN - 1},
            data_set.set_var_i16(VAR_NAME, DATA_I16[0..DATA_I16_LEN-1].to_vec()).unwrap_err()
        );
    }
    // Here, no data has been yet set
    assert_eq!(None, data_set.get_var_i8(VAR_NAME));
    assert_eq!(None, data_set.get_var_u8(VAR_NAME));
    assert_eq!(None, data_set.get_var_i16(VAR_NAME));
    assert_eq!(None, data_set.get_var_i32(VAR_NAME));
    assert_eq!(None, data_set.get_var_f32(VAR_NAME));
    assert_eq!(None, data_set.get_var_f64(VAR_NAME));

    // Set a valid data vector
    data_set.set_var_i16(VAR_NAME, DATA_I16.to_vec()).unwrap();

    // Check the stored vector
    assert_eq!(None,                data_set.get_var_i8(VAR_NAME));
    assert_eq!(None,                data_set.get_var_u8(VAR_NAME));
    assert_eq!(Some(&DATA_I16[..]), data_set.get_var_i16(VAR_NAME));
    assert_eq!(None,                data_set.get_var_i32(VAR_NAME));
    assert_eq!(None,                data_set.get_var_f32(VAR_NAME));
    assert_eq!(None,                data_set.get_var_f64(VAR_NAME));
}

#[test]
fn test_set_var_i32() {
    const VAR_NAME: &str = "var_i32";
    const DIM_NAME_1: &str = "dim_1";
    const DIM_NAME_2: &str = "dim_2";
    const DIM_SIZE_1: usize = 2;
    const DIM_SIZE_2: usize = 3;
    const DATA_I32_LEN: usize = DATA_I32.len();
    assert_eq!(DATA_I32_LEN, DIM_SIZE_1 * DIM_SIZE_2);

    let mut data_set: DataSet = DataSet::new();

    assert_eq!(0,       data_set.num_vars());
    assert_eq!(false,   data_set.has_var(VAR_NAME));
    assert_eq!(None,    data_set.get_var_len(VAR_NAME));
    assert_eq!(None,    data_set.get_var_data_type(VAR_NAME));
    assert_eq!(0,       data_set.num_dims());
    assert_eq!(false,   data_set.has_unlimited_dim());
    assert_eq!(false,   data_set.has_dim(DIM_NAME_1));
    assert_eq!(None,    data_set.get_dim_size(DIM_NAME_1));
    assert_eq!(None,    data_set.get_dim_type(DIM_NAME_1));
    assert_eq!(false,   data_set.has_dim(DIM_NAME_2));
    assert_eq!(None,    data_set.get_dim_size(DIM_NAME_2));
    assert_eq!(None,    data_set.get_dim_type(DIM_NAME_2));

    // Define new dimensions
    data_set.set_unlimited_dim(DIM_NAME_1, DIM_SIZE_1).unwrap();
    data_set.add_fixed_dim(DIM_NAME_2, DIM_SIZE_2).unwrap();

    assert_eq!(0,                                   data_set.num_vars());
    assert_eq!(false,                               data_set.has_var(VAR_NAME));
    assert_eq!(None,                                data_set.get_var_len(VAR_NAME));
    assert_eq!(None,                                data_set.get_var_data_type(VAR_NAME));
    assert_eq!(2,                                   data_set.num_dims());
    assert_eq!(Some(DIM_SIZE_1),                    data_set.get_dim_size(DIM_NAME_1));
    assert_eq!(true,                                data_set.has_unlimited_dim());
    assert_eq!(true,                                data_set.has_dim(DIM_NAME_1));
    assert_eq!(Some(DimensionType::UnlimitedSize),  data_set.get_dim_type(DIM_NAME_1));
    assert_eq!(true,                                data_set.has_dim(DIM_NAME_2));
    assert_eq!(Some(DIM_SIZE_2),                    data_set.get_dim_size(DIM_NAME_2));
    assert_eq!(Some(DimensionType::FixedSize),      data_set.get_dim_type(DIM_NAME_2));

    // Define a new variable
    data_set.add_var_i32(VAR_NAME, &[DIM_NAME_1, DIM_NAME_2]).unwrap();

    assert_eq!(1,                                   data_set.num_vars());
    assert_eq!(true,                                data_set.has_var(VAR_NAME));
    assert_eq!(Some(DATA_I32_LEN),                  data_set.get_var_len(VAR_NAME));
    assert_eq!(Some(DataType::I32),                 data_set.get_var_data_type(VAR_NAME));
    assert_eq!(2,                                   data_set.num_dims());
    assert_eq!(Some(DIM_SIZE_1),                    data_set.get_dim_size(DIM_NAME_1));
    assert_eq!(true,                                data_set.has_unlimited_dim());
    assert_eq!(true,                                data_set.has_dim(DIM_NAME_1));
    assert_eq!(Some(DimensionType::UnlimitedSize),  data_set.get_dim_type(DIM_NAME_1));
    assert_eq!(true,                                data_set.has_dim(DIM_NAME_2));
    assert_eq!(Some(DIM_SIZE_2),                    data_set.get_dim_size(DIM_NAME_2));
    assert_eq!(Some(DimensionType::FixedSize),      data_set.get_dim_type(DIM_NAME_2));

    // Here, no data has been yet set
    assert_eq!(None, data_set.get_var_i8(VAR_NAME));
    assert_eq!(None, data_set.get_var_u8(VAR_NAME));
    assert_eq!(None, data_set.get_var_i16(VAR_NAME));
    assert_eq!(None, data_set.get_var_i32(VAR_NAME));
    assert_eq!(None, data_set.get_var_f32(VAR_NAME));
    assert_eq!(None, data_set.get_var_f64(VAR_NAME));

    // Try to set wrong typed data
    {
        // Try to set `i8` data
        assert_eq!(
            InvalidDataSet::VariableMismatchDataType{var_name: String::from(VAR_NAME), req: DataType::I32, get: DataType::I8},
            data_set.set_var_i8(VAR_NAME, DATA_I8.to_vec()).unwrap_err()
        );
        // Try to set `u8` data
        assert_eq!(
            InvalidDataSet::VariableMismatchDataType{var_name: String::from(VAR_NAME), req: DataType::I32, get: DataType::U8},
            data_set.set_var_u8(VAR_NAME, DATA_U8.to_vec()).unwrap_err()
        );
        // Try to set `i16` data
        assert_eq!(
            InvalidDataSet::VariableMismatchDataType{var_name: String::from(VAR_NAME), req: DataType::I32, get: DataType::I16},
            data_set.set_var_i16(VAR_NAME, DATA_I16.to_vec()).unwrap_err()
        );
        // Try to set `f32` data
        assert_eq!(
            InvalidDataSet::VariableMismatchDataType{var_name: String::from(VAR_NAME), req: DataType::I32, get: DataType::F32},
            data_set.set_var_f32(VAR_NAME, DATA_F32.to_vec()).unwrap_err()
        );
        // Try to set `f64` data
        assert_eq!(
            InvalidDataSet::VariableMismatchDataType{var_name: String::from(VAR_NAME), req: DataType::I32, get: DataType::F64},
            data_set.set_var_f64(VAR_NAME, DATA_F64.to_vec()).unwrap_err()
        );
    }
    // Try to set data with a wrong number of elements
    {
        assert_eq!(
            InvalidDataSet::VariableMismatchDataLength{var_name: String::from(VAR_NAME), req: DATA_I32_LEN, get: DATA_I32_LEN - 1},
            data_set.set_var_i32(VAR_NAME, DATA_I32[0..DATA_I32_LEN-1].to_vec()).unwrap_err()
        );
    }
    // Here, no data has been yet set
    assert_eq!(None, data_set.get_var_i8(VAR_NAME));
    assert_eq!(None, data_set.get_var_u8(VAR_NAME));
    assert_eq!(None, data_set.get_var_i16(VAR_NAME));
    assert_eq!(None, data_set.get_var_i32(VAR_NAME));
    assert_eq!(None, data_set.get_var_f32(VAR_NAME));
    assert_eq!(None, data_set.get_var_f64(VAR_NAME));

    // Set a valid data vector
    data_set.set_var_i32(VAR_NAME, DATA_I32.to_vec()).unwrap();

    // Check the stored vector
    assert_eq!(None,                data_set.get_var_i8(VAR_NAME));
    assert_eq!(None,                data_set.get_var_u8(VAR_NAME));
    assert_eq!(None,                data_set.get_var_i16(VAR_NAME));
    assert_eq!(Some(&DATA_I32[..]), data_set.get_var_i32(VAR_NAME));
    assert_eq!(None,                data_set.get_var_f32(VAR_NAME));
    assert_eq!(None,                data_set.get_var_f64(VAR_NAME));
}

#[test]
fn test_set_var_f32() {
    const VAR_NAME: &str = "var_f32";
    const DIM_NAME_1: &str = "dim_1";
    const DIM_NAME_2: &str = "dim_2";
    const DIM_SIZE_1: usize = 2;
    const DIM_SIZE_2: usize = 3;
    const DATA_F32_LEN: usize = DATA_F32.len();
    assert_eq!(DATA_F32_LEN, DIM_SIZE_1 * DIM_SIZE_2);

    let mut data_set: DataSet = DataSet::new();

    assert_eq!(0,       data_set.num_vars());
    assert_eq!(false,   data_set.has_var(VAR_NAME));
    assert_eq!(None,    data_set.get_var_len(VAR_NAME));
    assert_eq!(None,    data_set.get_var_data_type(VAR_NAME));
    assert_eq!(0,       data_set.num_dims());
    assert_eq!(false,   data_set.has_unlimited_dim());
    assert_eq!(false,   data_set.has_dim(DIM_NAME_1));
    assert_eq!(None,    data_set.get_dim_size(DIM_NAME_1));
    assert_eq!(None,    data_set.get_dim_type(DIM_NAME_1));
    assert_eq!(false,   data_set.has_dim(DIM_NAME_2));
    assert_eq!(None,    data_set.get_dim_size(DIM_NAME_2));
    assert_eq!(None,    data_set.get_dim_type(DIM_NAME_2));

    // Define new dimensions
    data_set.set_unlimited_dim(DIM_NAME_1, DIM_SIZE_1).unwrap();
    data_set.add_fixed_dim(DIM_NAME_2, DIM_SIZE_2).unwrap();

    assert_eq!(0,                                   data_set.num_vars());
    assert_eq!(false,                               data_set.has_var(VAR_NAME));
    assert_eq!(None,                                data_set.get_var_len(VAR_NAME));
    assert_eq!(None,                                data_set.get_var_data_type(VAR_NAME));
    assert_eq!(2,                                   data_set.num_dims());
    assert_eq!(Some(DIM_SIZE_1),                    data_set.get_dim_size(DIM_NAME_1));
    assert_eq!(true,                                data_set.has_unlimited_dim());
    assert_eq!(true,                                data_set.has_dim(DIM_NAME_1));
    assert_eq!(Some(DimensionType::UnlimitedSize),  data_set.get_dim_type(DIM_NAME_1));
    assert_eq!(true,                                data_set.has_dim(DIM_NAME_2));
    assert_eq!(Some(DIM_SIZE_2),                    data_set.get_dim_size(DIM_NAME_2));
    assert_eq!(Some(DimensionType::FixedSize),      data_set.get_dim_type(DIM_NAME_2));

    // Define a new variable
    data_set.add_var_f32(VAR_NAME, &[DIM_NAME_1, DIM_NAME_2]).unwrap();

    assert_eq!(1,                                   data_set.num_vars());
    assert_eq!(true,                                data_set.has_var(VAR_NAME));
    assert_eq!(Some(DATA_F32_LEN),                  data_set.get_var_len(VAR_NAME));
    assert_eq!(Some(DataType::F32),                 data_set.get_var_data_type(VAR_NAME));
    assert_eq!(2,                                   data_set.num_dims());
    assert_eq!(Some(DIM_SIZE_1),                    data_set.get_dim_size(DIM_NAME_1));
    assert_eq!(true,                                data_set.has_unlimited_dim());
    assert_eq!(true,                                data_set.has_dim(DIM_NAME_1));
    assert_eq!(Some(DimensionType::UnlimitedSize),  data_set.get_dim_type(DIM_NAME_1));
    assert_eq!(true,                                data_set.has_dim(DIM_NAME_2));
    assert_eq!(Some(DIM_SIZE_2),                    data_set.get_dim_size(DIM_NAME_2));
    assert_eq!(Some(DimensionType::FixedSize),      data_set.get_dim_type(DIM_NAME_2));

    // Here, no data has been yet set
    assert_eq!(None, data_set.get_var_i8(VAR_NAME));
    assert_eq!(None, data_set.get_var_u8(VAR_NAME));
    assert_eq!(None, data_set.get_var_i16(VAR_NAME));
    assert_eq!(None, data_set.get_var_i32(VAR_NAME));
    assert_eq!(None, data_set.get_var_f32(VAR_NAME));
    assert_eq!(None, data_set.get_var_f64(VAR_NAME));

    // Try to set wrong typed data
    {
        // Try to set `i8` data
        assert_eq!(
            InvalidDataSet::VariableMismatchDataType{var_name: String::from(VAR_NAME), req: DataType::F32, get: DataType::I8},
            data_set.set_var_i8(VAR_NAME, DATA_I8.to_vec()).unwrap_err()
        );
        // Try to set `u8` data
        assert_eq!(
            InvalidDataSet::VariableMismatchDataType{var_name: String::from(VAR_NAME), req: DataType::F32, get: DataType::U8},
            data_set.set_var_u8(VAR_NAME, DATA_U8.to_vec()).unwrap_err()
        );
        // Try to set `i16` data
        assert_eq!(
            InvalidDataSet::VariableMismatchDataType{var_name: String::from(VAR_NAME), req: DataType::F32, get: DataType::I16},
            data_set.set_var_i16(VAR_NAME, DATA_I16.to_vec()).unwrap_err()
        );
        // Try to set `i32` data
        assert_eq!(
            InvalidDataSet::VariableMismatchDataType{var_name: String::from(VAR_NAME), req: DataType::F32, get: DataType::I32},
            data_set.set_var_i32(VAR_NAME, DATA_I32.to_vec()).unwrap_err()
        );
        // Try to set `f64` data
        assert_eq!(
            InvalidDataSet::VariableMismatchDataType{var_name: String::from(VAR_NAME), req: DataType::F32, get: DataType::F64},
            data_set.set_var_f64(VAR_NAME, DATA_F64.to_vec()).unwrap_err()
        );
    }
    // Try to set data with a wrong number of elements
    {
        assert_eq!(
            InvalidDataSet::VariableMismatchDataLength{var_name: String::from(VAR_NAME), req: DATA_F32_LEN, get: DATA_F32_LEN - 1},
            data_set.set_var_f32(VAR_NAME, DATA_F32[0..DATA_F32_LEN-1].to_vec()).unwrap_err()
        );
    }
    // Here, no data has been yet set
    assert_eq!(None, data_set.get_var_i8(VAR_NAME));
    assert_eq!(None, data_set.get_var_u8(VAR_NAME));
    assert_eq!(None, data_set.get_var_i16(VAR_NAME));
    assert_eq!(None, data_set.get_var_i32(VAR_NAME));
    assert_eq!(None, data_set.get_var_f32(VAR_NAME));
    assert_eq!(None, data_set.get_var_f64(VAR_NAME));

    // Set a valid data vector
    data_set.set_var_f32(VAR_NAME, DATA_F32.to_vec()).unwrap();

    // Check the stored vector
    assert_eq!(None,                data_set.get_var_i8(VAR_NAME));
    assert_eq!(None,                data_set.get_var_u8(VAR_NAME));
    assert_eq!(None,                data_set.get_var_i16(VAR_NAME));
    assert_eq!(None,                data_set.get_var_i32(VAR_NAME));
    assert_eq!(Some(&DATA_F32[..]), data_set.get_var_f32(VAR_NAME));
    assert_eq!(None,                data_set.get_var_f64(VAR_NAME));
}

#[test]
fn test_set_var_f64() {
    const VAR_NAME: &str = "var_f64";
    const DIM_NAME_1: &str = "dim_1";
    const DIM_NAME_2: &str = "dim_2";
    const DIM_SIZE_1: usize = 2;
    const DIM_SIZE_2: usize = 3;
    const DATA_F64_LEN: usize = DATA_F64.len();
    assert_eq!(DATA_F64_LEN, DIM_SIZE_1 * DIM_SIZE_2);

    let mut data_set: DataSet = DataSet::new();

    assert_eq!(0,       data_set.num_vars());
    assert_eq!(false,   data_set.has_var(VAR_NAME));
    assert_eq!(None,    data_set.get_var_len(VAR_NAME));
    assert_eq!(None,    data_set.get_var_data_type(VAR_NAME));
    assert_eq!(0,       data_set.num_dims());
    assert_eq!(false,   data_set.has_unlimited_dim());
    assert_eq!(false,   data_set.has_dim(DIM_NAME_1));
    assert_eq!(None,    data_set.get_dim_size(DIM_NAME_1));
    assert_eq!(None,    data_set.get_dim_type(DIM_NAME_1));
    assert_eq!(false,   data_set.has_dim(DIM_NAME_2));
    assert_eq!(None,    data_set.get_dim_size(DIM_NAME_2));
    assert_eq!(None,    data_set.get_dim_type(DIM_NAME_2));

    // Define new dimensions
    data_set.set_unlimited_dim(DIM_NAME_1, DIM_SIZE_1).unwrap();
    data_set.add_fixed_dim(DIM_NAME_2, DIM_SIZE_2).unwrap();

    assert_eq!(0,                                   data_set.num_vars());
    assert_eq!(false,                               data_set.has_var(VAR_NAME));
    assert_eq!(None,                                data_set.get_var_len(VAR_NAME));
    assert_eq!(None,                                data_set.get_var_data_type(VAR_NAME));
    assert_eq!(2,                                   data_set.num_dims());
    assert_eq!(Some(DIM_SIZE_1),                    data_set.get_dim_size(DIM_NAME_1));
    assert_eq!(true,                                data_set.has_unlimited_dim());
    assert_eq!(true,                                data_set.has_dim(DIM_NAME_1));
    assert_eq!(Some(DimensionType::UnlimitedSize),  data_set.get_dim_type(DIM_NAME_1));
    assert_eq!(true,                                data_set.has_dim(DIM_NAME_2));
    assert_eq!(Some(DIM_SIZE_2),                    data_set.get_dim_size(DIM_NAME_2));
    assert_eq!(Some(DimensionType::FixedSize),      data_set.get_dim_type(DIM_NAME_2));

    // Define a new variable
    data_set.add_var_f64(VAR_NAME, &[DIM_NAME_1, DIM_NAME_2]).unwrap();

    assert_eq!(1,                                   data_set.num_vars());
    assert_eq!(true,                                data_set.has_var(VAR_NAME));
    assert_eq!(Some(DATA_F64_LEN),                  data_set.get_var_len(VAR_NAME));
    assert_eq!(Some(DataType::F64),                 data_set.get_var_data_type(VAR_NAME));
    assert_eq!(2,                                   data_set.num_dims());
    assert_eq!(Some(DIM_SIZE_1),                    data_set.get_dim_size(DIM_NAME_1));
    assert_eq!(true,                                data_set.has_unlimited_dim());
    assert_eq!(true,                                data_set.has_dim(DIM_NAME_1));
    assert_eq!(Some(DimensionType::UnlimitedSize),  data_set.get_dim_type(DIM_NAME_1));
    assert_eq!(true,                                data_set.has_dim(DIM_NAME_2));
    assert_eq!(Some(DIM_SIZE_2),                    data_set.get_dim_size(DIM_NAME_2));
    assert_eq!(Some(DimensionType::FixedSize),      data_set.get_dim_type(DIM_NAME_2));

    // Here, no data has been yet set
    assert_eq!(None, data_set.get_var_i8(VAR_NAME));
    assert_eq!(None, data_set.get_var_u8(VAR_NAME));
    assert_eq!(None, data_set.get_var_i16(VAR_NAME));
    assert_eq!(None, data_set.get_var_i32(VAR_NAME));
    assert_eq!(None, data_set.get_var_f32(VAR_NAME));
    assert_eq!(None, data_set.get_var_f64(VAR_NAME));

    // Try to set wrong typed data
    {
        // Try to set `i8` data
        assert_eq!(
            InvalidDataSet::VariableMismatchDataType{var_name: String::from(VAR_NAME), req: DataType::F64, get: DataType::I8},
            data_set.set_var_i8(VAR_NAME, DATA_I8.to_vec()).unwrap_err()
        );
        // Try to set `u8` data
        assert_eq!(
            InvalidDataSet::VariableMismatchDataType{var_name: String::from(VAR_NAME), req: DataType::F64, get: DataType::U8},
            data_set.set_var_u8(VAR_NAME, DATA_U8.to_vec()).unwrap_err()
        );
        // Try to set `i16` data
        assert_eq!(
            InvalidDataSet::VariableMismatchDataType{var_name: String::from(VAR_NAME), req: DataType::F64, get: DataType::I16},
            data_set.set_var_i16(VAR_NAME, DATA_I16.to_vec()).unwrap_err()
        );
        // Try to set `i32` data
        assert_eq!(
            InvalidDataSet::VariableMismatchDataType{var_name: String::from(VAR_NAME), req: DataType::F64, get: DataType::I32},
            data_set.set_var_i32(VAR_NAME, DATA_I32.to_vec()).unwrap_err()
        );
        // Try to set `f64` data
        assert_eq!(
            InvalidDataSet::VariableMismatchDataType{var_name: String::from(VAR_NAME), req: DataType::F64, get: DataType::F32},
            data_set.set_var_f32(VAR_NAME, DATA_F32.to_vec()).unwrap_err()
        );
    }
    // Try to set data with a wrong number of elements
    {
        assert_eq!(
            InvalidDataSet::VariableMismatchDataLength{var_name: String::from(VAR_NAME), req: DATA_F64_LEN, get: DATA_F64_LEN - 1},
            data_set.set_var_f64(VAR_NAME, DATA_F64[0..DATA_F64_LEN-1].to_vec()).unwrap_err()
        );
    }
    // Here, no data has been yet set
    assert_eq!(None, data_set.get_var_i8(VAR_NAME));
    assert_eq!(None, data_set.get_var_u8(VAR_NAME));
    assert_eq!(None, data_set.get_var_i16(VAR_NAME));
    assert_eq!(None, data_set.get_var_i32(VAR_NAME));
    assert_eq!(None, data_set.get_var_f32(VAR_NAME));
    assert_eq!(None, data_set.get_var_f64(VAR_NAME));

    // Set a valid data vector
    data_set.set_var_f64(VAR_NAME, DATA_F64.to_vec()).unwrap();

    // Check the stored vector
    assert_eq!(None,                data_set.get_var_i8(VAR_NAME));
    assert_eq!(None,                data_set.get_var_u8(VAR_NAME));
    assert_eq!(None,                data_set.get_var_i16(VAR_NAME));
    assert_eq!(None,                data_set.get_var_i32(VAR_NAME));
    assert_eq!(None,                data_set.get_var_f32(VAR_NAME));
    assert_eq!(Some(&DATA_F64[..]), data_set.get_var_f64(VAR_NAME));
}

#[test]
fn test_rename_var() {
    const VAR_NAME_1: &str = "var_1";
    const VAR_NAME_2: &str = "var_2";
    const DIM_NAME: &str = "dim_1";
    const VAR_DATA: [i32; 4] = [1, 2, 3, 4];
    const VAR_DATA_LEN: usize = VAR_DATA.len();

    let mut data_set: DataSet = DataSet::new();

    assert_eq!(0,       data_set.num_vars());
    assert_eq!(false,   data_set.has_var(VAR_NAME_1));
    assert_eq!(None,    data_set.get_var_len(VAR_NAME_1));
    assert_eq!(None,    data_set.get_var_data_type(VAR_NAME_1));
    assert_eq!(None,    data_set.get_var_i32(VAR_NAME_1));
    assert_eq!(false,   data_set.has_var(VAR_NAME_2));
    assert_eq!(None,    data_set.get_var_len(VAR_NAME_2));
    assert_eq!(None,    data_set.get_var_data_type(VAR_NAME_2));
    assert_eq!(None,    data_set.get_var_i32(VAR_NAME_2));

    data_set.add_fixed_dim(DIM_NAME, VAR_DATA_LEN).unwrap();
    data_set.add_var_i32::<&str>(VAR_NAME_1, &[DIM_NAME]).unwrap();
    data_set.set_var_i32(VAR_NAME_1, VAR_DATA.to_vec()).unwrap();

    assert_eq!(1,                   data_set.num_vars());
    assert_eq!(true,                data_set.has_var(VAR_NAME_1));
    assert_eq!(Some(VAR_DATA_LEN),  data_set.get_var_len(VAR_NAME_1));
    assert_eq!(Some(DataType::I32), data_set.get_var_data_type(VAR_NAME_1));
    assert_eq!(Some(&VAR_DATA[..]), data_set.get_var_i32(VAR_NAME_1));
    assert_eq!(false,               data_set.has_var(VAR_NAME_2));
    assert_eq!(None,                data_set.get_var_len(VAR_NAME_2));
    assert_eq!(None,                data_set.get_var_data_type(VAR_NAME_2));
    assert_eq!(None,                data_set.get_var_i32(VAR_NAME_2));

    data_set.rename_var(VAR_NAME_1, VAR_NAME_2).unwrap();

    assert_eq!(1,                   data_set.num_vars());
    assert_eq!(false,               data_set.has_var(VAR_NAME_1));
    assert_eq!(None,                data_set.get_var_len(VAR_NAME_1));
    assert_eq!(None,                data_set.get_var_data_type(VAR_NAME_1));
    assert_eq!(None,                data_set.get_var_i32(VAR_NAME_1));
    assert_eq!(true,                data_set.has_var(VAR_NAME_2));
    assert_eq!(Some(VAR_DATA_LEN),  data_set.get_var_len(VAR_NAME_2));
    assert_eq!(Some(DataType::I32), data_set.get_var_data_type(VAR_NAME_2));
    assert_eq!(Some(&VAR_DATA[..]), data_set.get_var_i32(VAR_NAME_2));
}

#[test]
fn test_rename_var_error_already_exists() {
    const DIM_NAME_1: &str = "dim_1";
    const VAR_NAME_1: &str = "var_1";
    const VAR_DATA_1: [i32; 4] = [1, 2, 3, 4];
    const VAR_DATA_1_LEN: usize = VAR_DATA_1.len();

    const DIM_NAME_2: &str = "dim_2";
    const VAR_NAME_2: &str = "var_2";
    const VAR_DATA_2: [i32; 3] = [5, 6, 7];
    const VAR_DATA_2_LEN: usize = VAR_DATA_2.len();

    let mut data_set: DataSet = DataSet::new();

    assert_eq!(0,       data_set.num_vars());
    assert_eq!(false,   data_set.has_var(VAR_NAME_1));
    assert_eq!(None,    data_set.get_var_len(VAR_NAME_1));
    assert_eq!(None,    data_set.get_var_data_type(VAR_NAME_1));
    assert_eq!(None,    data_set.get_var_i32(VAR_NAME_1));
    assert_eq!(false,   data_set.has_var(VAR_NAME_2));
    assert_eq!(None,    data_set.get_var_len(VAR_NAME_2));
    assert_eq!(None,    data_set.get_var_data_type(VAR_NAME_2));
    assert_eq!(None,    data_set.get_var_i32(VAR_NAME_2));

    data_set.add_fixed_dim(DIM_NAME_1, VAR_DATA_1_LEN).unwrap();
    data_set.add_var_i32::<&str>(VAR_NAME_1, &[DIM_NAME_1]).unwrap();
    data_set.set_var_i32(VAR_NAME_1, VAR_DATA_1.to_vec()).unwrap();
    data_set.add_fixed_dim(DIM_NAME_2, VAR_DATA_2_LEN).unwrap();
    data_set.add_var_i32::<&str>(VAR_NAME_2, &[DIM_NAME_2]).unwrap();
    data_set.set_var_i32(VAR_NAME_2, VAR_DATA_2.to_vec()).unwrap();

    assert_eq!(2,                       data_set.num_vars());
    assert_eq!(true,                    data_set.has_var(VAR_NAME_1));
    assert_eq!(Some(VAR_DATA_1_LEN),    data_set.get_var_len(VAR_NAME_1));
    assert_eq!(Some(DataType::I32),     data_set.get_var_data_type(VAR_NAME_1));
    assert_eq!(Some(&VAR_DATA_1[..]),   data_set.get_var_i32(VAR_NAME_1));
    assert_eq!(true,                    data_set.has_var(VAR_NAME_2));
    assert_eq!(Some(VAR_DATA_2_LEN),    data_set.get_var_len(VAR_NAME_2));
    assert_eq!(Some(DataType::I32),     data_set.get_var_data_type(VAR_NAME_2));
    assert_eq!(Some(&VAR_DATA_2[..]),   data_set.get_var_i32(VAR_NAME_2));

    assert_eq!(
        InvalidDataSet::VariableAlreadyExists(VAR_NAME_2.to_string()),
        data_set.rename_var(VAR_NAME_1, VAR_NAME_2).unwrap_err()
    );

    assert_eq!(2,                       data_set.num_vars());
    assert_eq!(true,                    data_set.has_var(VAR_NAME_1));
    assert_eq!(Some(VAR_DATA_1_LEN),    data_set.get_var_len(VAR_NAME_1));
    assert_eq!(Some(DataType::I32),     data_set.get_var_data_type(VAR_NAME_1));
    assert_eq!(Some(&VAR_DATA_1[..]),   data_set.get_var_i32(VAR_NAME_1));
    assert_eq!(true,                    data_set.has_var(VAR_NAME_2));
    assert_eq!(Some(VAR_DATA_2_LEN),    data_set.get_var_len(VAR_NAME_2));
    assert_eq!(Some(DataType::I32),     data_set.get_var_data_type(VAR_NAME_2));
    assert_eq!(Some(&VAR_DATA_2[..]),   data_set.get_var_i32(VAR_NAME_2));
}

#[test]
fn test_rename_var_error_invalid_name() {
    const DIM_NAME: &str = "dim_1";
    const VAR_NAME: &str = "var_1";
    const VAR_DATA: [i32; 4] = [1, 2, 3, 4];
    const VAR_DATA_LEN: usize = VAR_DATA.len();
    const INVALID_VAR_NAME: &str = "!invalid_name";

    let mut data_set: DataSet = DataSet::new();

    assert_eq!(0,       data_set.num_vars());
    assert_eq!(false,   data_set.has_var(VAR_NAME));
    assert_eq!(None,    data_set.get_var_len(VAR_NAME));
    assert_eq!(None,    data_set.get_var_data_type(VAR_NAME));
    assert_eq!(None,    data_set.get_var_i32(VAR_NAME));
    assert_eq!(false,   data_set.has_var(INVALID_VAR_NAME));
    assert_eq!(None,    data_set.get_var_len(INVALID_VAR_NAME));
    assert_eq!(None,    data_set.get_var_data_type(INVALID_VAR_NAME));
    assert_eq!(None,    data_set.get_var_i32(INVALID_VAR_NAME));

    data_set.add_fixed_dim(DIM_NAME, VAR_DATA_LEN).unwrap();
    data_set.add_var_i32::<&str>(VAR_NAME, &[DIM_NAME]).unwrap();
    data_set.set_var_i32(VAR_NAME, VAR_DATA.to_vec()).unwrap();


    assert_eq!(1,                   data_set.num_vars());
    assert_eq!(true,                data_set.has_var(VAR_NAME));
    assert_eq!(Some(VAR_DATA_LEN),  data_set.get_var_len(VAR_NAME));
    assert_eq!(Some(DataType::I32), data_set.get_var_data_type(VAR_NAME));
    assert_eq!(Some(&VAR_DATA[..]), data_set.get_var_i32(VAR_NAME));
    assert_eq!(false,               data_set.has_var(INVALID_VAR_NAME));
    assert_eq!(None,                data_set.get_var_len(INVALID_VAR_NAME));
    assert_eq!(None,                data_set.get_var_data_type(INVALID_VAR_NAME));
    assert_eq!(None,                data_set.get_var_i32(INVALID_VAR_NAME));

    assert_eq!(
        InvalidDataSet::VariableNameNotValid(INVALID_VAR_NAME.to_string()),
        data_set.rename_var(VAR_NAME, INVALID_VAR_NAME).unwrap_err()
    );

    assert_eq!(1,                   data_set.num_vars());
    assert_eq!(true,                data_set.has_var(VAR_NAME));
    assert_eq!(Some(VAR_DATA_LEN),  data_set.get_var_len(VAR_NAME));
    assert_eq!(Some(DataType::I32), data_set.get_var_data_type(VAR_NAME));
    assert_eq!(Some(&VAR_DATA[..]), data_set.get_var_i32(VAR_NAME));
    assert_eq!(false,               data_set.has_var(INVALID_VAR_NAME));
    assert_eq!(None,                data_set.get_var_len(INVALID_VAR_NAME));
    assert_eq!(None,                data_set.get_var_data_type(INVALID_VAR_NAME));
    assert_eq!(None,                data_set.get_var_i32(INVALID_VAR_NAME));
}

#[test]
fn test_remove_var() {
    const DIM_NAME: &str = "dim_1";
    const VAR_NAME: &str = "var_1";
    const VAR_DATA: [i32; 4] = [1, 2, 3, 4];
    const VAR_DATA_LEN: usize = VAR_DATA.len();

    let mut data_set: DataSet = DataSet::new();

    assert_eq!(0,       data_set.num_vars());
    assert_eq!(false,   data_set.has_var(VAR_NAME));
    assert_eq!(None,    data_set.get_var_len(VAR_NAME));
    assert_eq!(None,    data_set.get_var_data_type(VAR_NAME));
    assert_eq!(None,    data_set.get_var_i32(VAR_NAME));

    data_set.add_fixed_dim(DIM_NAME, VAR_DATA_LEN).unwrap();
    data_set.add_var_i32::<&str>(VAR_NAME, &[DIM_NAME]).unwrap();
    data_set.set_var_i32(VAR_NAME, VAR_DATA.to_vec()).unwrap();

    assert_eq!(1,                   data_set.num_vars());
    assert_eq!(true,                data_set.has_var(VAR_NAME));
    assert_eq!(Some(VAR_DATA_LEN),  data_set.get_var_len(VAR_NAME));
    assert_eq!(Some(DataType::I32), data_set.get_var_data_type(VAR_NAME));
    assert_eq!(Some(&VAR_DATA[..]), data_set.get_var_i32(VAR_NAME));

    data_set.remove_var(VAR_NAME).unwrap();

    assert_eq!(0,       data_set.num_vars());
    assert_eq!(false,   data_set.has_var(VAR_NAME));
    assert_eq!(None,    data_set.get_var_len(VAR_NAME));
    assert_eq!(None,    data_set.get_var_data_type(VAR_NAME));
    assert_eq!(None,    data_set.get_var_i32(VAR_NAME));
}


#[test]
fn test_remove_var_error_not_defined() {

    const VAR_NAME: &str = "var_1";

    let mut data_set: DataSet = DataSet::new();

    assert_eq!(0,       data_set.num_vars());
    assert_eq!(false,   data_set.has_var(VAR_NAME));
    assert_eq!(None,    data_set.get_var_len(VAR_NAME));
    assert_eq!(None,    data_set.get_var_data_type(VAR_NAME));

    assert_eq!(
        InvalidDataSet::VariableNotDefined(VAR_NAME.to_string()),
        data_set.remove_var(VAR_NAME).unwrap_err()
    );

    assert_eq!(0,       data_set.num_vars());
    assert_eq!(false,   data_set.has_var(VAR_NAME));
    assert_eq!(None,    data_set.get_var_len(VAR_NAME));
    assert_eq!(None,    data_set.get_var_data_type(VAR_NAME));
}