#![cfg(test)]
use std::rc::Rc;

use crate::{DataSet, Dimension, DimensionType, InvalidDataSet};

#[test]
fn test_add_2_fixed_size_dims() {
    const DIM_NAME_1: &str = "dim_1";
    const DIM_SIZE_1: usize = 10;
    const DIM_NAME_2: &str = "dim_2";
    const DIM_SIZE_2: usize = 20;

    let mut data_set = DataSet::new();

    assert_eq!(0,       data_set.num_dims());
    assert_eq!(false,   data_set.has_unlimited_dim());
    assert_eq!(false,   data_set.has_dim(DIM_NAME_1));
    assert_eq!(None,    data_set.get_dim_size(DIM_NAME_1));
    assert_eq!(None,    data_set.get_dim_type(DIM_NAME_1));

    // add the *fixed-size* dimension
    data_set.add_fixed_dim(DIM_NAME_1, DIM_SIZE_1).unwrap();
    data_set.add_fixed_dim(DIM_NAME_2, DIM_SIZE_2).unwrap();

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
fn test_set_dim_unlimited_size() {
    const DIM_NAME: &str = "dim_1";
    const DIM_SIZE: usize = 10;

    let mut data_set = DataSet::new();

    assert_eq!(0,       data_set.num_dims());
    assert_eq!(false,   data_set.has_unlimited_dim());
    assert_eq!(false,   data_set.has_dim(DIM_NAME));
    assert_eq!(None,    data_set.get_dim_size(DIM_NAME));
    assert_eq!(None,    data_set.get_dim_type(DIM_NAME));

    // set the *unlimited-size* dimension
    data_set.set_unlimited_dim(DIM_NAME, DIM_SIZE).unwrap();

    assert_eq!(1,                                   data_set.num_dims());
    assert_eq!(true,                                data_set.has_unlimited_dim());
    assert_eq!(true,                                data_set.has_dim(DIM_NAME));
    assert_eq!(Some(DIM_SIZE),                      data_set.get_dim_size(DIM_NAME));
    assert_eq!(Some(DimensionType::UnlimitedSize),  data_set.get_dim_type(DIM_NAME));
}

#[test]
fn test_rename_dim_fixed_size() {
    const DIM_NAME_1: &str = "dim_1";
    const DIM_NAME_2: &str = "dim_2";
    const DIM_SIZE: usize = 10;

    let mut data_set = DataSet::new();

    assert_eq!(0,       data_set.num_dims());
    assert_eq!(false,   data_set.has_unlimited_dim());
    assert_eq!(false,   data_set.has_dim(DIM_NAME_1));
    assert_eq!(None,    data_set.get_dim_size(DIM_NAME_1));
    assert_eq!(None,    data_set.get_dim_type(DIM_NAME_1));
    assert_eq!(false,   data_set.has_dim(DIM_NAME_2));
    assert_eq!(None,    data_set.get_dim_size(DIM_NAME_2));
    assert_eq!(None,    data_set.get_dim_type(DIM_NAME_2));

    // set a *fixed-size* dimension
    data_set.add_fixed_dim(DIM_NAME_1, DIM_SIZE).unwrap();

    assert_eq!(1,                               data_set.num_dims());
    assert_eq!(false,                           data_set.has_unlimited_dim());
    assert_eq!(true,                            data_set.has_dim(DIM_NAME_1));
    assert_eq!(Some(DIM_SIZE),                  data_set.get_dim_size(DIM_NAME_1));
    assert_eq!(Some(DimensionType::FixedSize),  data_set.get_dim_type(DIM_NAME_1));
    assert_eq!(false,                           data_set.has_dim(DIM_NAME_2));
    assert_eq!(None,                            data_set.get_dim_size(DIM_NAME_2));
    assert_eq!(None,                            data_set.get_dim_type(DIM_NAME_2));

    // rename the *fixed-size* dimension
    data_set.rename_dim(DIM_NAME_1, DIM_NAME_2).unwrap();

    assert_eq!(1,                               data_set.num_dims());
    assert_eq!(false,                           data_set.has_unlimited_dim());
    assert_eq!(false,                           data_set.has_dim(DIM_NAME_1));
    assert_eq!(None,                            data_set.get_dim_size(DIM_NAME_1));
    assert_eq!(None,                            data_set.get_dim_type(DIM_NAME_1));
    assert_eq!(true,                            data_set.has_dim(DIM_NAME_2));
    assert_eq!(Some(DIM_SIZE),                  data_set.get_dim_size(DIM_NAME_2));
    assert_eq!(Some(DimensionType::FixedSize),  data_set.get_dim_type(DIM_NAME_2));
}

#[test]
fn test_rename_dim_unlimited_size() {
    const DIM_NAME_1: &str = "dim_1";
    const DIM_NAME_2: &str = "dim_2";
    const DIM_SIZE: usize = 10;

    let mut data_set = DataSet::new();

    assert_eq!(0,       data_set.num_dims());
    assert_eq!(false,   data_set.has_unlimited_dim());
    assert_eq!(false,   data_set.has_dim(DIM_NAME_1));
    assert_eq!(None,    data_set.get_dim_size(DIM_NAME_1));
    assert_eq!(None,    data_set.get_dim_type(DIM_NAME_1));
    assert_eq!(false,   data_set.has_dim(DIM_NAME_2));
    assert_eq!(None,    data_set.get_dim_size(DIM_NAME_2));
    assert_eq!(None,    data_set.get_dim_type(DIM_NAME_2));

    // set the *unlimited-size* dimension
    data_set.set_unlimited_dim(DIM_NAME_1, DIM_SIZE).unwrap();

    assert_eq!(1,                                   data_set.num_dims());
    assert_eq!(true,                                data_set.has_unlimited_dim());
    assert_eq!(true,                                data_set.has_dim(DIM_NAME_1));
    assert_eq!(Some(DIM_SIZE),                      data_set.get_dim_size(DIM_NAME_1));
    assert_eq!(Some(DimensionType::UnlimitedSize),  data_set.get_dim_type(DIM_NAME_1));
    assert_eq!(false,                               data_set.has_dim(DIM_NAME_2));
    assert_eq!(None,                                data_set.get_dim_size(DIM_NAME_2));
    assert_eq!(None,                                data_set.get_dim_type(DIM_NAME_2));

    // rename the *unlimited-size* dimension
    data_set.rename_dim(DIM_NAME_1, DIM_NAME_2).unwrap();

    assert_eq!(1,                                   data_set.num_dims());
    assert_eq!(true,                                data_set.has_unlimited_dim());
    assert_eq!(false,                               data_set.has_dim(DIM_NAME_1));
    assert_eq!(None,                                data_set.get_dim_size(DIM_NAME_1));
    assert_eq!(None,                                data_set.get_dim_type(DIM_NAME_1));
    assert_eq!(true,                                data_set.has_dim(DIM_NAME_2));
    assert_eq!(Some(DIM_SIZE),                      data_set.get_dim_size(DIM_NAME_2));
    assert_eq!(Some(DimensionType::UnlimitedSize),  data_set.get_dim_type(DIM_NAME_2));
}

#[test]
fn test_remove_dim_fixed_size() {
    const DIM_NAME: &str = "dim_1";
    const DIM_SIZE: usize = 10;

    let mut data_set = DataSet::new();

    assert_eq!(0,       data_set.num_dims());
    assert_eq!(false,   data_set.has_unlimited_dim());
    assert_eq!(false,   data_set.has_dim(DIM_NAME));
    assert_eq!(None,    data_set.get_dim_size(DIM_NAME));
    assert_eq!(None,    data_set.get_dim_type(DIM_NAME));

    // add a *fixed-size* dimension
    data_set.add_fixed_dim(DIM_NAME, DIM_SIZE).unwrap();

    assert_eq!(1,                               data_set.num_dims());
    assert_eq!(false,                           data_set.has_unlimited_dim());
    assert_eq!(true,                            data_set.has_dim(DIM_NAME));
    assert_eq!(Some(DIM_SIZE),                  data_set.get_dim_size(DIM_NAME));
    assert_eq!(Some(DimensionType::FixedSize),  data_set.get_dim_type(DIM_NAME));

    // remove the *fixed-size* dimension
    data_set.remove_dim(DIM_NAME).unwrap();

    assert_eq!(0,                               data_set.num_dims());
    assert_eq!(false,                           data_set.has_unlimited_dim());
    assert_eq!(false,                           data_set.has_dim(DIM_NAME));
    assert_eq!(None,                            data_set.get_dim_size(DIM_NAME));
    assert_eq!(None,                            data_set.get_dim_type(DIM_NAME));
}

#[test]
fn test_remove_dim_unlimited_size() {
    const DIM_NAME: &str = "dim_1";
    const DIM_SIZE: usize = 10;

    let mut data_set = DataSet::new();

    assert_eq!(0,       data_set.num_dims());
    assert_eq!(false,   data_set.has_unlimited_dim());
    assert_eq!(false,   data_set.has_dim(DIM_NAME));
    assert_eq!(None,    data_set.get_dim_size(DIM_NAME));
    assert_eq!(None,    data_set.get_dim_type(DIM_NAME));

    // set the *unlimited-size* dimension
    data_set.set_unlimited_dim(DIM_NAME, DIM_SIZE).unwrap();

    assert_eq!(1,                                   data_set.num_dims());
    assert_eq!(true,                                data_set.has_unlimited_dim());
    assert_eq!(true,                                data_set.has_dim(DIM_NAME));
    assert_eq!(Some(DIM_SIZE),                      data_set.get_dim_size(DIM_NAME));
    assert_eq!(Some(DimensionType::UnlimitedSize),  data_set.get_dim_type(DIM_NAME));

    // rename the *unlimited-size* dimension
    data_set.remove_dim(DIM_NAME).unwrap();

    assert_eq!(0,                               data_set.num_dims());
    assert_eq!(false,                           data_set.has_unlimited_dim());
    assert_eq!(false,                           data_set.has_dim(DIM_NAME));
    assert_eq!(None,                            data_set.get_dim_size(DIM_NAME));
    assert_eq!(None,                            data_set.get_dim_type(DIM_NAME));
}

#[test]
fn test_add_dim_error_dim_already_exists() {
    const DIM_NAME: &str = "dim_1";
    const DIM_SIZE_1: usize = 10;
    const DIM_SIZE_2: usize = 20;

    // Test :
    //   1. add a fixed-size dim
    //   2. add a fixed-size dim
    {
        // create the data set
        let mut data_set = DataSet::new();

        assert_eq!(0,       data_set.num_dims());
        assert_eq!(false,   data_set.has_unlimited_dim());
        assert_eq!(false,   data_set.has_dim(DIM_NAME));
        assert_eq!(None,    data_set.get_dim_size(DIM_NAME));
        assert_eq!(None,    data_set.get_dim_type(DIM_NAME));

        // add a *fixed-size* dimension
        data_set.add_fixed_dim(DIM_NAME, DIM_SIZE_1).unwrap();

        assert_eq!(1,                               data_set.num_dims());
        assert_eq!(false,                           data_set.has_unlimited_dim());
        assert_eq!(true,                            data_set.has_dim(DIM_NAME));
        assert_eq!(Some(DIM_SIZE_1),                data_set.get_dim_size(DIM_NAME));
        assert_eq!(Some(DimensionType::FixedSize),  data_set.get_dim_type(DIM_NAME));

        // Trye to add a other dimension with the same name
        assert_eq!(
            InvalidDataSet::DimensionAlreadyExists(DIM_NAME.to_string()),
            data_set.add_fixed_dim(DIM_NAME, DIM_SIZE_2).unwrap_err()
        );

        assert_eq!(1,                               data_set.num_dims());
        assert_eq!(false,                           data_set.has_unlimited_dim());
        assert_eq!(true,                            data_set.has_dim(DIM_NAME));
        assert_eq!(Some(DIM_SIZE_1),                data_set.get_dim_size(DIM_NAME));
        assert_eq!(Some(DimensionType::FixedSize),  data_set.get_dim_type(DIM_NAME));
    }
    // Test :
    //   1. add a fixed-size dim
    //   2. set the unlimited-size dim
    {
        // create the data set
        let mut data_set = DataSet::new();

        assert_eq!(0,       data_set.num_dims());
        assert_eq!(false,   data_set.has_unlimited_dim());
        assert_eq!(false,   data_set.has_dim(DIM_NAME));
        assert_eq!(None,    data_set.get_dim_size(DIM_NAME));
        assert_eq!(None,    data_set.get_dim_type(DIM_NAME));

        // add a *fixed-size* dimension
        data_set.add_fixed_dim(DIM_NAME, DIM_SIZE_1).unwrap();

        assert_eq!(1,                               data_set.num_dims());
        assert_eq!(false,                           data_set.has_unlimited_dim());
        assert_eq!(true,                            data_set.has_dim(DIM_NAME));
        assert_eq!(Some(DIM_SIZE_1),                data_set.get_dim_size(DIM_NAME));
        assert_eq!(Some(DimensionType::FixedSize),  data_set.get_dim_type(DIM_NAME));

        // Trye to add a other dimension with the same name
        assert_eq!(
            InvalidDataSet::DimensionAlreadyExists(DIM_NAME.to_string()),
            data_set.set_unlimited_dim(DIM_NAME, DIM_SIZE_2).unwrap_err()
        );

        assert_eq!(1,                               data_set.num_dims());
        assert_eq!(false,                           data_set.has_unlimited_dim());
        assert_eq!(true,                            data_set.has_dim(DIM_NAME));
        assert_eq!(Some(DIM_SIZE_1),                data_set.get_dim_size(DIM_NAME));
        assert_eq!(Some(DimensionType::FixedSize),  data_set.get_dim_type(DIM_NAME));
    }
    // Test :
    //   1. set the unlimited-size dim
    //   2. add a fixed-size dim
    {
        // create the data set
        let mut data_set = DataSet::new();

        assert_eq!(0,       data_set.num_dims());
        assert_eq!(false,   data_set.has_unlimited_dim());
        assert_eq!(false,   data_set.has_dim(DIM_NAME));
        assert_eq!(None,    data_set.get_dim_size(DIM_NAME));
        assert_eq!(None,    data_set.get_dim_type(DIM_NAME));

        // add a *fixed-size* dimension
        data_set.set_unlimited_dim(DIM_NAME, DIM_SIZE_1).unwrap();

        assert_eq!(1,                                   data_set.num_dims());
        assert_eq!(true,                                data_set.has_unlimited_dim());
        assert_eq!(true,                                data_set.has_dim(DIM_NAME));
        assert_eq!(Some(DIM_SIZE_1),                    data_set.get_dim_size(DIM_NAME));
        assert_eq!(Some(DimensionType::UnlimitedSize),  data_set.get_dim_type(DIM_NAME));

        // Trye to add a other dimension with the same name
        assert_eq!(
            InvalidDataSet::DimensionAlreadyExists(DIM_NAME.to_string()),
            data_set.add_fixed_dim(DIM_NAME, DIM_SIZE_2).unwrap_err()
        );

        assert_eq!(1,                                   data_set.num_dims());
        assert_eq!(true,                                data_set.has_unlimited_dim());
        assert_eq!(true,                                data_set.has_dim(DIM_NAME));
        assert_eq!(Some(DIM_SIZE_1),                    data_set.get_dim_size(DIM_NAME));
        assert_eq!(Some(DimensionType::UnlimitedSize),  data_set.get_dim_type(DIM_NAME));
    }
}

#[test]
fn test_rename_dim_error_dim_dot_defined()
{
    const UNDEF_DIM_NAME: &str = "undef_dim";
    const DIM_NAME_2: &str = "dim_2";

    let mut data_set = DataSet::new();

    assert_eq!(0,       data_set.num_dims());
    assert_eq!(false,   data_set.has_unlimited_dim());
    assert_eq!(false,   data_set.has_dim(UNDEF_DIM_NAME));
    assert_eq!(None,    data_set.get_dim_size(UNDEF_DIM_NAME));
    assert_eq!(None,    data_set.get_dim_type(UNDEF_DIM_NAME));
    assert_eq!(false,   data_set.has_dim(DIM_NAME_2));
    assert_eq!(None,    data_set.get_dim_size(DIM_NAME_2));
    assert_eq!(None,    data_set.get_dim_type(DIM_NAME_2));

    // Try to rename an undefined dimension
    assert_eq!(
        InvalidDataSet::DimensionNotDefined(UNDEF_DIM_NAME.to_string()),
        data_set.rename_dim(UNDEF_DIM_NAME, DIM_NAME_2).unwrap_err()
    );

    assert_eq!(0,       data_set.num_dims());
    assert_eq!(false,   data_set.has_unlimited_dim());
    assert_eq!(false,   data_set.has_dim(UNDEF_DIM_NAME));
    assert_eq!(None,    data_set.get_dim_size(UNDEF_DIM_NAME));
    assert_eq!(None,    data_set.get_dim_type(UNDEF_DIM_NAME));
    assert_eq!(false,   data_set.has_dim(DIM_NAME_2));
    assert_eq!(None,    data_set.get_dim_size(DIM_NAME_2));
    assert_eq!(None,    data_set.get_dim_type(DIM_NAME_2));
}

#[test]
fn test_remove_dim_error_dim_dot_defined()
{
    const UNDEF_DIM_NAME: &str = "undef_dim";

    let mut data_set = DataSet::new();

    assert_eq!(0,       data_set.num_dims());
    assert_eq!(false,   data_set.has_unlimited_dim());
    assert_eq!(false,   data_set.has_dim(UNDEF_DIM_NAME));
    assert_eq!(None,    data_set.get_dim_size(UNDEF_DIM_NAME));
    assert_eq!(None,    data_set.get_dim_type(UNDEF_DIM_NAME));


    // Try to remove an undefined dimension
    assert_eq!(
        InvalidDataSet::DimensionNotDefined(UNDEF_DIM_NAME.to_string()),
        data_set.remove_dim(UNDEF_DIM_NAME).unwrap_err()
    );

    assert_eq!(0,       data_set.num_dims());
    assert_eq!(false,   data_set.has_unlimited_dim());
    assert_eq!(false,   data_set.has_dim(UNDEF_DIM_NAME));
    assert_eq!(None,    data_set.get_dim_size(UNDEF_DIM_NAME));
    assert_eq!(None,    data_set.get_dim_type(UNDEF_DIM_NAME));
}

#[test]
fn test_set_unlim_dim_error_unlim_dim_already_exists() {
    const DIM_NAME_1: &str = "dim_1";
    const DIM_SIZE_1: usize = 10;
    const DIM_NAME_2: &str = "dim_2";
    const DIM_SIZE_2: usize = 20;

    let mut data_set = DataSet::new();

    assert_eq!(0,       data_set.num_dims());
    assert_eq!(false,   data_set.has_unlimited_dim());
    assert_eq!(false,   data_set.has_dim(DIM_NAME_1));
    assert_eq!(None,    data_set.get_dim_size(DIM_NAME_1));
    assert_eq!(None,    data_set.get_dim_type(DIM_NAME_1));
    assert_eq!(false,   data_set.has_dim(DIM_NAME_2));
    assert_eq!(None,    data_set.get_dim_size(DIM_NAME_2));
    assert_eq!(None,    data_set.get_dim_type(DIM_NAME_2));

    data_set.set_unlimited_dim(DIM_NAME_1, DIM_SIZE_1).unwrap();

    assert_eq!(1,                                   data_set.num_dims());
    assert_eq!(true,                                data_set.has_unlimited_dim());
    assert_eq!(true,                                data_set.has_dim(DIM_NAME_1));
    assert_eq!(Some(DIM_SIZE_1),                    data_set.get_dim_size(DIM_NAME_1));
    assert_eq!(Some(DimensionType::UnlimitedSize),  data_set.get_dim_type(DIM_NAME_1));
    assert_eq!(false,                               data_set.has_dim(DIM_NAME_2));
    assert_eq!(None,                                data_set.get_dim_size(DIM_NAME_2));
    assert_eq!(None,                                 data_set.get_dim_type(DIM_NAME_2));

    assert_eq!(
        InvalidDataSet::UnlimitedDimensionAlreadyExists(DIM_NAME_1.to_string()),
        data_set.set_unlimited_dim(DIM_NAME_2, DIM_SIZE_2).unwrap_err()
    );

    assert_eq!(1,                                   data_set.num_dims());
    assert_eq!(true,                                data_set.has_unlimited_dim());
    assert_eq!(true,                                data_set.has_dim(DIM_NAME_1));
    assert_eq!(Some(DIM_SIZE_1),                    data_set.get_dim_size(DIM_NAME_1));
    assert_eq!(Some(DimensionType::UnlimitedSize),  data_set.get_dim_type(DIM_NAME_1));
    assert_eq!(false,   data_set.has_dim(DIM_NAME_2));
    assert_eq!(None,    data_set.get_dim_size(DIM_NAME_2));
    assert_eq!(None,    data_set.get_dim_type(DIM_NAME_2));
}

#[test]
fn test_remove_dim_error_dim_already_used() {
    const VAR_NAME: &str = "var_1";
    const DIM_NAME: &str = "dim_1";
    const DIM_SIZE: usize = 10;

    let mut data_set = DataSet::new();

    assert_eq!(0,       data_set.num_vars());
    assert_eq!(false,   data_set.has_var(VAR_NAME));
    assert_eq!(0,       data_set.num_dims());
    assert_eq!(false,   data_set.has_unlimited_dim());
    assert_eq!(false,   data_set.has_dim(DIM_NAME));
    assert_eq!(None,    data_set.get_dim_size(DIM_NAME));
    assert_eq!(None,    data_set.get_dim_type(DIM_NAME));

    // add a *fixed-size* dimension
    data_set.add_fixed_dim(DIM_NAME, DIM_SIZE).unwrap();

    assert_eq!(0,                               data_set.num_vars());
    assert_eq!(false,                           data_set.has_var(VAR_NAME));
    assert_eq!(1,                               data_set.num_dims());
    assert_eq!(false,                           data_set.has_unlimited_dim());
    assert_eq!(true,                            data_set.has_dim(DIM_NAME));
    assert_eq!(Some(DIM_SIZE),                  data_set.get_dim_size(DIM_NAME));
    assert_eq!(Some(DimensionType::FixedSize),  data_set.get_dim_type(DIM_NAME));

    data_set.add_var_i8(VAR_NAME, &[DIM_NAME]).unwrap();

    assert_eq!(1,                               data_set.num_vars());
    assert_eq!(true,                            data_set.has_var(VAR_NAME));
    assert_eq!(1,                               data_set.num_dims());
    assert_eq!(false,                           data_set.has_unlimited_dim());
    assert_eq!(true,                            data_set.has_dim(DIM_NAME));
    assert_eq!(Some(DIM_SIZE),                  data_set.get_dim_size(DIM_NAME));
    assert_eq!(Some(DimensionType::FixedSize),  data_set.get_dim_type(DIM_NAME));

    // Try to remove the dimension while a variable it
    assert_eq!(
        InvalidDataSet::DimensionYetUsed{
            var_names: vec![VAR_NAME.to_string()],
            dim_name: DIM_NAME.to_string()
        },
        data_set.remove_dim(DIM_NAME).unwrap_err()
    );
    assert_eq!(1,                               data_set.num_vars());
    assert_eq!(true,                            data_set.has_var(VAR_NAME));
    assert_eq!(1,                               data_set.num_dims());
    assert_eq!(false,                           data_set.has_unlimited_dim());
    assert_eq!(true,                            data_set.has_dim(DIM_NAME));
    assert_eq!(Some(DIM_SIZE),                  data_set.get_dim_size(DIM_NAME));
    assert_eq!(Some(DimensionType::FixedSize),  data_set.get_dim_type(DIM_NAME));

    // Remove the variable before
    data_set.remove_var(VAR_NAME).unwrap();

    assert_eq!(0,                               data_set.num_vars());
    assert_eq!(false,                           data_set.has_var(VAR_NAME));
    assert_eq!(1,                               data_set.num_dims());
    assert_eq!(false,                           data_set.has_unlimited_dim());
    assert_eq!(true,                            data_set.has_dim(DIM_NAME));
    assert_eq!(Some(DIM_SIZE),                  data_set.get_dim_size(DIM_NAME));
    assert_eq!(Some(DimensionType::FixedSize),  data_set.get_dim_type(DIM_NAME));

    // And remove the dimension
    data_set.remove_dim(DIM_NAME).unwrap();

    assert_eq!(0,       data_set.num_vars());
    assert_eq!(false,   data_set.has_var(VAR_NAME));
    assert_eq!(0,       data_set.num_dims());
    assert_eq!(false,   data_set.has_unlimited_dim());
    assert_eq!(false,   data_set.has_dim(DIM_NAME));
    assert_eq!(None,    data_set.get_dim_size(DIM_NAME));
    assert_eq!(None,    data_set.get_dim_type(DIM_NAME));
}

#[test]
fn test_get_dims_from_ids() {
    const DIM_NAME_1: &str = "dim_1";
    const DIM_SIZE_1: usize = 10;
    const DIM_NAME_2: &str = "dim_2";
    const DIM_SIZE_2: usize = 20;
    const DIM_NAME_3: &str = "dim_3";
    const DIM_SIZE_3: usize = 30;
    
    let mut data_set = DataSet::new();

    assert_eq!(0,       data_set.num_dims());
    assert_eq!(false,   data_set.has_unlimited_dim());
    assert_eq!(false,   data_set.has_dim(DIM_NAME_1));
    assert_eq!(None,    data_set.get_dim_size(DIM_NAME_1));
    assert_eq!(None,    data_set.get_dim_type(DIM_NAME_1));
    assert_eq!(false,   data_set.has_dim(DIM_NAME_2));
    assert_eq!(None,    data_set.get_dim_size(DIM_NAME_2));
    assert_eq!(None,    data_set.get_dim_type(DIM_NAME_2));
    assert_eq!(false,   data_set.has_dim(DIM_NAME_3));
    assert_eq!(None,    data_set.get_dim_size(DIM_NAME_3));
    assert_eq!(None,    data_set.get_dim_type(DIM_NAME_3));

    // Add 3 dimensions
    data_set.add_fixed_dim(DIM_NAME_1, DIM_SIZE_1).unwrap();
    data_set.set_unlimited_dim(DIM_NAME_2, DIM_SIZE_2).unwrap();
    data_set.add_fixed_dim(DIM_NAME_3, DIM_SIZE_3).unwrap();

    assert_eq!(3,                                   data_set.num_dims());
    assert_eq!(true,                                data_set.has_unlimited_dim());
    assert_eq!(true,                                data_set.has_dim(DIM_NAME_1));
    assert_eq!(Some(DIM_SIZE_1),                    data_set.get_dim_size(DIM_NAME_1));
    assert_eq!(Some(DimensionType::FixedSize),      data_set.get_dim_type(DIM_NAME_1));
    assert_eq!(true,                                data_set.has_dim(DIM_NAME_2));
    assert_eq!(Some(DIM_SIZE_2),                    data_set.get_dim_size(DIM_NAME_2));
    assert_eq!(Some(DimensionType::UnlimitedSize),  data_set.get_dim_type(DIM_NAME_2));
    assert_eq!(true,                                data_set.has_dim(DIM_NAME_3));
    assert_eq!(Some(DIM_SIZE_3),                    data_set.get_dim_size(DIM_NAME_3));
    assert_eq!(Some(DimensionType::FixedSize),      data_set.get_dim_type(DIM_NAME_3));


    // Get dims from their IDs
    let dim_list: Vec<Rc<Dimension>> = data_set.get_dims_from_ids(&[1, 0, 2]).unwrap();

    // check returned dimensions
    assert_eq!(data_set.get_dim(DIM_NAME_2).unwrap(), dim_list[0]);
    assert_eq!(data_set.get_dim(DIM_NAME_1).unwrap(), dim_list[1]);
    assert_eq!(data_set.get_dim(DIM_NAME_3).unwrap(), dim_list[2]);
}

#[test]
fn test_get_dims_from_ids_error_dim_ids_not_valid()
{
    const DIM_NAME_1: &str = "dim_1";
    const DIM_SIZE_1: usize = 10;
    const DIM_NAME_2: &str = "dim_2";
    const DIM_SIZE_2: usize = 20;
    const DIM_NAME_3: &str = "dim_3";
    const DIM_SIZE_3: usize = 30;
    
    let mut data_set = DataSet::new();

    assert_eq!(0,       data_set.num_dims());
    assert_eq!(false,   data_set.has_unlimited_dim());
    assert_eq!(false,   data_set.has_dim(DIM_NAME_1));
    assert_eq!(None,    data_set.get_dim_size(DIM_NAME_1));
    assert_eq!(None,    data_set.get_dim_type(DIM_NAME_1));
    assert_eq!(false,   data_set.has_dim(DIM_NAME_2));
    assert_eq!(None,    data_set.get_dim_size(DIM_NAME_2));
    assert_eq!(None,    data_set.get_dim_type(DIM_NAME_2));
    assert_eq!(false,   data_set.has_dim(DIM_NAME_3));
    assert_eq!(None,    data_set.get_dim_size(DIM_NAME_3));
    assert_eq!(None,    data_set.get_dim_type(DIM_NAME_3));

    // Add 3 dimensions
    data_set.add_fixed_dim(DIM_NAME_1, DIM_SIZE_1).unwrap();
    data_set.set_unlimited_dim(DIM_NAME_2, DIM_SIZE_2).unwrap();
    data_set.add_fixed_dim(DIM_NAME_3, DIM_SIZE_3).unwrap();

    assert_eq!(3,                                   data_set.num_dims());
    assert_eq!(true,                                data_set.has_unlimited_dim());
    assert_eq!(true,                                data_set.has_dim(DIM_NAME_1));
    assert_eq!(Some(DIM_SIZE_1),                    data_set.get_dim_size(DIM_NAME_1));
    assert_eq!(Some(DimensionType::FixedSize),      data_set.get_dim_type(DIM_NAME_1));
    assert_eq!(true,                                data_set.has_dim(DIM_NAME_2));
    assert_eq!(Some(DIM_SIZE_2),                    data_set.get_dim_size(DIM_NAME_2));
    assert_eq!(Some(DimensionType::UnlimitedSize),  data_set.get_dim_type(DIM_NAME_2));
    assert_eq!(true,                                data_set.has_dim(DIM_NAME_3));
    assert_eq!(Some(DIM_SIZE_3),                    data_set.get_dim_size(DIM_NAME_3));
    assert_eq!(Some(DimensionType::FixedSize),      data_set.get_dim_type(DIM_NAME_3));

    // Try dims from their IDs
    assert_eq!(
        InvalidDataSet::DimensionsIdsNotValid(vec![7, 5]),
        data_set.get_dims_from_ids(&[1, 0, 2, 7, 5]).unwrap_err()
    );
}

#[test]
fn test_add_fixed_size_dim_errror_dim_name_not_valid()
{
    const VAR_NAME: &str = "var_1";
    const INVALID_DIM_NAME: &str = "!invalid_name";
    const DIM_SIZE: usize = 10;

    let mut data_set = DataSet::new();

    assert_eq!(0,       data_set.num_vars());
    assert_eq!(false,   data_set.has_var(VAR_NAME));
    assert_eq!(0,       data_set.num_dims());
    assert_eq!(false,   data_set.has_unlimited_dim());
    assert_eq!(false,   data_set.has_dim(INVALID_DIM_NAME));
    assert_eq!(None,    data_set.get_dim_size(INVALID_DIM_NAME));
    assert_eq!(None,    data_set.get_dim_type(INVALID_DIM_NAME));

    // Try to add a fixed-size dimension with an invalid name
    assert_eq!(
        InvalidDataSet::DimensionNameNotValid(INVALID_DIM_NAME.to_string()),
        data_set.add_fixed_dim(INVALID_DIM_NAME, DIM_SIZE).unwrap_err()
    );

    assert_eq!(0,       data_set.num_vars());
    assert_eq!(false,   data_set.has_var(VAR_NAME));
    assert_eq!(0,       data_set.num_dims());
    assert_eq!(false,   data_set.has_unlimited_dim());
    assert_eq!(false,   data_set.has_dim(INVALID_DIM_NAME));
    assert_eq!(None,    data_set.get_dim_size(INVALID_DIM_NAME));
    assert_eq!(None,    data_set.get_dim_type(INVALID_DIM_NAME));
}

#[test]
fn test_add_unlimited_size_dim_errror_dim_name_not_valid()
{
    const VAR_NAME: &str = "var_1";
    const INVALID_DIM_NAME: &str = "!invalid_name";
    const DIM_SIZE: usize = 10;

    let mut data_set = DataSet::new();

    assert_eq!(0,       data_set.num_vars());
    assert_eq!(false,   data_set.has_var(VAR_NAME));
    assert_eq!(0,       data_set.num_dims());
    assert_eq!(false,   data_set.has_unlimited_dim());
    assert_eq!(false,   data_set.has_dim(INVALID_DIM_NAME));
    assert_eq!(None,    data_set.get_dim_size(INVALID_DIM_NAME));
    assert_eq!(None,    data_set.get_dim_type(INVALID_DIM_NAME));

    // Try to add a fixed-size dimension with an invalid name
    assert_eq!(
        InvalidDataSet::DimensionNameNotValid(INVALID_DIM_NAME.to_string()),
        data_set.set_unlimited_dim(INVALID_DIM_NAME, DIM_SIZE).unwrap_err()
    );

    assert_eq!(0,       data_set.num_vars());
    assert_eq!(false,   data_set.has_var(VAR_NAME));
    assert_eq!(0,       data_set.num_dims());
    assert_eq!(false,   data_set.has_unlimited_dim());
    assert_eq!(false,   data_set.has_dim(INVALID_DIM_NAME));
    assert_eq!(None,    data_set.get_dim_size(INVALID_DIM_NAME));
    assert_eq!(None,    data_set.get_dim_type(INVALID_DIM_NAME));
}

