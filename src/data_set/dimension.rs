use crate::error::InvalidDataSet;

use crate::name_string::is_valid_name;

use std::cell::RefCell;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dimension {
    pub(in crate::data_set) name: RefCell<String>,
    pub(in crate::data_set) size: DimensionSize,
}

/// Internal representation of the size of a dimension.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::data_set) enum DimensionSize {
    /// *Unlimited-size* dimension, the unlimited size can be modifed by the NetCDF-3 dataset.
    Unlimited(RefCell<usize>),
    /// *Fixed-size* dimension
    Fixed(usize),
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum DimensionType {
    UnlimitedSize = 0,
    FixedSize = 1,
}

impl DimensionSize {
    /// Create a new *unlimited* or *fixed* size.
    pub(in crate::data_set) fn new(size: usize, r#type: DimensionType) -> DimensionSize {
        return match r#type {
            DimensionType::FixedSize => DimensionSize::Fixed(size),
            DimensionType::UnlimitedSize => DimensionSize::Unlimited(RefCell::new(size)),
        };
    }

    #[inline]
    /// Return the size of the dimension.
    pub(in crate::data_set) fn size(&self) -> usize {
        return match self {
            DimensionSize::Unlimited(size) => size.borrow().clone(),
            DimensionSize::Fixed(size) => size.clone(),
        };
    }

    #[inline]
    /// Return the size of the dimension.
    pub(in crate::data_set) fn r#type(&self) -> DimensionType {
        return match self {
            DimensionSize::Unlimited(_) => DimensionType::UnlimitedSize,
            DimensionSize::Fixed(_) => DimensionType::FixedSize,
        };
    }
}

impl Dimension {

    /// Creates a new *fixed size* NetCDF-3 dimension.
    pub(in crate::data_set) fn new_fixed_size(name: &str, size: usize) -> Result<Dimension, InvalidDataSet> {
        Dimension::check_dim_name(name)?;
        return Ok(Dimension {
            name: RefCell::new(name.to_string()),
            size: DimensionSize::new(size, DimensionType::FixedSize),
        });
    }

    /// Creates a new *unlimited size* NetCDF-3 dimension.
    pub(in crate::data_set) fn new_unlimited_size(name: &str, size: usize) -> Result<Dimension, InvalidDataSet> {
        Dimension::check_dim_name(name)?;
        return Ok(Dimension {
            name: RefCell::new(name.to_string()),
            size: DimensionSize::new(size, DimensionType::UnlimitedSize),
        });
    }

    /// Returns the name of the NetCDF-3 dimension.
    pub fn name(&self) -> String {
        return self.name.borrow().clone();
    }

    /// Returns the size of the NetCDF-3 dimension.
    pub fn size(&self) -> usize {
        return self.size.size();
    }

    /// Returns the dimension type (*fixed size* ou *unlimited size*) of the NetCDF-3 dimension.
    pub fn dim_type(&self) -> DimensionType {
        return self.size.r#type();
    }

    /// Returns `true` if the dimension is a *unlimited size* dimension, otherwise return `false`.
    pub fn is_unlimited(&self) -> bool {
        return self.dim_type() == DimensionType::UnlimitedSize;
    }

    /// Returns `true` if the dimension is a *fixed size* dimension, otherwise return `false`.
    pub fn is_fixed(&self) -> bool {
        return self.dim_type() == DimensionType::FixedSize;
    }

    pub(in crate::data_set) fn check_dim_name(dim_name: &str) -> Result<(), InvalidDataSet> {
        return match is_valid_name(dim_name) {
            true => Ok(()),
            false => Err(InvalidDataSet::DimensionNameNotValid(dim_name.to_string())),
        };
    }
}

#[cfg(test)]
mod tests {
    use super::{Dimension, DimensionType};
    use std::rc::Rc;

    #[test]
    fn test_new_fixed_size_dimension() {
        let latitude_dim = Dimension::new_fixed_size("latitude", 360).unwrap();
        assert_eq!("latitude", latitude_dim.name());
        assert_eq!(360_usize, latitude_dim.size());
        assert_eq!(DimensionType::FixedSize, latitude_dim.dim_type());
        assert!(latitude_dim.is_fixed());
        assert!(!latitude_dim.is_unlimited());
    }

    #[test]
    fn test_new_unlimited_size_dimension() {
        let time_dim = Dimension::new_unlimited_size("time", 1440).unwrap();
        assert_eq!("time", time_dim.name());
        assert_eq!(1440_usize, time_dim.size());
        assert_eq!(DimensionType::UnlimitedSize, time_dim.dim_type());
        assert!(!time_dim.is_fixed());
        assert!(time_dim.is_unlimited());
    }

    #[test]
    fn test_dimension_equality() {
        // test equality between 2 fixed-size dimension
        {
            let dim_a: Dimension = Dimension::new_fixed_size("latitude", 180).unwrap();
            let dim_b: Dimension = Dimension::new_fixed_size("latitude", 180).unwrap();
            assert_eq!(dim_a, dim_b);
        }

        // test equality between 2 fixed-size dimension with different sizes
        {
            let dim_a: Dimension = Dimension::new_fixed_size("latitude", 90).unwrap();
            let dim_b: Dimension = Dimension::new_fixed_size("latitude", 180).unwrap();
            assert_ne!(dim_a, dim_b);
        }

        // test equality between 2 fixed-size dimension with different names
        {
            let dim_a: Dimension = Dimension::new_fixed_size("latitude", 180).unwrap();
            let dim_b: Dimension = Dimension::new_fixed_size("longitude", 180).unwrap();
            assert_ne!(dim_a, dim_b);
        }

        // test equality between 2 unlimited-size dimension
        {
            let dim_a: Dimension = Dimension::new_unlimited_size("latitude", 180).unwrap();
            let dim_b: Dimension = Dimension::new_unlimited_size("latitude", 180).unwrap();
            assert_eq!(dim_a, dim_b);
        }

        // test equality between 2 unlimited-size dimension with different sizes
        {
            let dim_a: Dimension = Dimension::new_unlimited_size("latitude", 90).unwrap();
            let dim_b: Dimension = Dimension::new_unlimited_size("latitude", 180).unwrap();
            assert_ne!(dim_a, dim_b);
        }

        // test equality between 2 unlimited-size dimension with different names
        {
            let dim_a: Dimension = Dimension::new_unlimited_size("latitude", 180).unwrap();
            let dim_b: Dimension = Dimension::new_unlimited_size("longitude", 180).unwrap();
            assert_ne!(dim_a, dim_b);
        }

        // test equality between 1 unlimited-size dimension and 1 fixed-size dimension
        {
            let dim_a: Dimension = Dimension::new_fixed_size("latitude", 180).unwrap();
            let dim_b: Dimension = Dimension::new_unlimited_size("latitude", 180).unwrap();
            assert_ne!(dim_a, dim_b);
        }
    }

    #[test]
    fn test_rc_dimension_equality() {
        // test equality between 2 fixed-size dimension
        {
            let dim_a: Rc<Dimension> = Rc::new(Dimension::new_fixed_size("latitude", 180).unwrap());
            let dim_b: Rc<Dimension> = Rc::new(Dimension::new_fixed_size("latitude", 180).unwrap());

            assert_eq!(dim_a, dim_b);
            assert!(!Rc::ptr_eq(&dim_a, &dim_b));

            let dim_c: Rc<Dimension> = Rc::clone(&dim_a);
            assert_eq!(dim_a, dim_c);
            assert_eq!(dim_b, dim_c);
            assert!(Rc::ptr_eq(&dim_a, &dim_c));
            assert!(!Rc::ptr_eq(&dim_b, &dim_c));
        }
    }
}
