#![cfg(test)]

use std::rc::Rc;

use crate::{DataSet, Dimension, DimensionType, error::InvalidDataSet, Version};

#[test]
fn test_add_fixed_dim() -> Result<(), InvalidDataSet> {
    let mut data_set = DataSet::new(Version::Classic);

    // test empty dims
    assert_eq!(data_set.num_dims(), 0);

    // add the first dimension
    assert!(data_set.get_dim("latitude").is_none());
    data_set.add_fixed_dim("latitude", 180)?;
    assert_eq!(1, data_set.num_dims());
    assert!(data_set.get_dim("latitude").is_some());
    {
        let latitude_dim: Rc<Dimension> = data_set.get_dim("latitude").unwrap();
        assert_eq!("latitude", latitude_dim.name());
        assert_eq!(180, latitude_dim.size());
        assert_eq!(DimensionType::FixedSize, latitude_dim.dim_type());
    }

    // try to add an other dimnesion with the same name
    match data_set.add_fixed_dim("latitude", 180) {
        Err(InvalidDataSet::DimensionNameAlreadyExists(_)) => { /* do nothing */ }
        _ => {
            panic!("Expect an error of type `InvalidDataSet::DimensionNameAlreadyExists`.");
        }
    }
    assert_eq!(1, data_set.num_dims());

    // Add a second dimension: the longitude
    assert!(data_set.get_dim("longitude").is_none());
    data_set.add_fixed_dim("longitude", 360)?;
    assert!(data_set.get_dim("longitude").is_some());
    assert_eq!(2, data_set.num_dims());
    {
        let longitude_dim: Rc<Dimension> = data_set.get_dim("longitude").unwrap();
        assert_eq!("longitude", longitude_dim.name());
        assert_eq!(360, longitude_dim.size());
        assert_eq!(DimensionType::FixedSize, longitude_dim.dim_type());
    }

    // Rename the first dimension : set upper case
    assert!(data_set.rename_dim("latitude", "LATITUDE").is_ok());
    assert!(data_set.get_dim("latitude").is_none());
    assert!(data_set.get_dim("LATITUDE").is_some());
    assert_eq!(2, data_set.dims.len());
    {
        let latitude_dim: Rc<Dimension> = data_set.get_dim("LATITUDE").unwrap();
        assert_eq!("LATITUDE", latitude_dim.name());
        assert_eq!(180, latitude_dim.size());
        assert_eq!(DimensionType::FixedSize, latitude_dim.dim_type());
    }

    // removed the second dim
    assert_eq!(2, data_set.dims.len());
    let _longitude_dim: Rc<Dimension> = data_set.remove_dim("longitude")?;
    assert_eq!(1, data_set.dims.len());
    assert!(data_set.get_dim("longitude").is_none());

    // check the remained dimension
    assert_eq!(data_set.dims.len(), 1);
    {
        let latitude_dim: Rc<Dimension> = data_set.get_dim("LATITUDE").unwrap();
        assert_eq!("LATITUDE", latitude_dim.name());
        assert_eq!(180, latitude_dim.size());
        assert_eq!(DimensionType::FixedSize, latitude_dim.dim_type());
    }

    Ok(())
}

#[test]
fn test_set_unlimited_dim() -> Result<(), InvalidDataSet> {
    let mut data_set = DataSet::new(Version::Classic);

    assert_eq!(0, data_set.num_dims());
    assert!(!data_set.has_unlimited_dim());

    // add a *fixed-size* dimension
    data_set.add_fixed_dim("latitude", 180)?;
    assert_eq!(1, data_set.num_dims());
    assert!(!data_set.has_unlimited_dim());

    // set the *unlimited-fixed* size dimension
    data_set.set_unlimited_dim("time", 1440)?;
    assert_eq!(2, data_set.num_dims());
    assert!(data_set.has_unlimited_dim());
    {
        let time_dim: Rc<Dimension> = data_set.get_dim("time").unwrap();
        assert_eq!("time", time_dim.name());
        assert_eq!(1440, time_dim.size());
        assert_eq!(DimensionType::UnlimitedSize, time_dim.dim_type());
    }

    let _time_dim: Rc<Dimension> = data_set.remove_dim("time")?;

    Ok(())
}
