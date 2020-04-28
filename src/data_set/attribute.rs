use crate::name_string::is_valid_name;
use crate::typed_data_vector::DataVector;
use crate::DataType;

/// Represents a NetCDF-3 attribute.
///
/// # Example
///
/// ```
/// # use netcdf3::{DataType, Attribute, DataSet, Version};
///
/// let mut data_set = DataSet::new(Version::Classic);
/// let _ = data_set.add_global_attr_i8("attr_1", vec![0, 1, 2, 3]).unwrap();
///
/// let attr: &Attribute = data_set.get_global_attr("attr_1").unwrap();
///
/// assert_eq!(DataType::I8, attr.data_type());
/// assert!(attr.get_i8().is_some());
/// assert_eq!(&vec![0_i8, 1, 2, 3], attr.get_i8().unwrap());
///
/// assert_eq!(None, attr.get_u8());
/// assert_eq!(None, attr.get_i16());
/// assert_eq!(None, attr.get_i32());
/// assert_eq!(None, attr.get_f32());
/// assert_eq!(None, attr.get_f64());
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Attribute {
    pub(crate) name: String,
    pub(crate) data: DataVector,
}

impl Attribute {
    /// Create a new attribute from a `DataVector`.
    pub(crate) fn new(name: &str, data: DataVector) -> Result<Attribute, String> {
        Attribute::check_attr_name(name)?;
        Ok(Attribute {
            name: name.to_string(),
            data: data,
        })
    }
    /// Create a new attribute containing i8 data.
    pub(in crate::data_set) fn new_i8_attr(name: &str, data: Vec<i8>) -> Result<Attribute, String> {
        let data = DataVector::I8(data);
        Attribute::new(name, data)
    }

    /// Create a new attribute containing *u8* data.
    pub(in crate::data_set) fn new_u8_attr(name: &str, data: Vec<u8>) -> Result<Attribute, String> {
        let data = DataVector::U8(data);
        Attribute::new(name, data)
    }

    /// Create a new attribute containing *i16* data.
    pub(in crate::data_set) fn new_i16_attr(name: &str, data: Vec<i16>) -> Result<Attribute, String> {
        let data = DataVector::I16(data);
        Attribute::new(name, data)
    }

    /// Create a new attribute containing *i32* data.
    pub(crate) fn new_i32_attr(name: &str, data: Vec<i32>) -> Result<Attribute, String> {
        let data = DataVector::I32(data);
        Attribute::new(name, data)
    }

    /// Create a new attribute containing *f32* data.
    pub(crate) fn new_f32_attr(name: &str, data: Vec<f32>) -> Result<Attribute, String> {
        let data = DataVector::F32(data);
        Attribute::new(name, data)
    }

    /// Create a new attribute containing *f64* data.
    pub(crate) fn new_f64_attr(name: &str, data: Vec<f64>) -> Result<Attribute, String> {
        let data = DataVector::F64(data);
        Attribute::new(name, data)
    }
    /// Return the name of the attribute.
    pub fn name(&self) -> &str {
        &self.name
    }
    /// Return the NetCDF-3 data type of the attribute : *i8*, *u8*, ...
    pub fn data_type(&self) -> DataType {
        self.data.data_type()
    }

    /// Return the number of elements (the length) of the attribute.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Returns a reference of the data vector, if it contains `i8` elements,
    /// otherwise returns `None`.
    pub fn get_i8(&self) -> Option<&Vec<i8>> {
        self.data.get_i8()
    }

    pub fn get_u8(&self) -> Option<&Vec<u8>> {
        self.data.get_u8()
    }

    pub fn get_i16(&self) -> Option<&Vec<i16>> {
        self.data.get_i16()
    }

    pub fn get_i32(&self) -> Option<&Vec<i32>> {
        self.data.get_i32()
    }

    pub fn get_f32(&self) -> Option<&Vec<f32>> {
        self.data.get_f32()
    }

    pub fn get_f64(&self) -> Option<&Vec<f64>> {
        self.data.get_f64()
    }

    pub(crate) fn check_attr_name(attr_name: &str) -> Result<(), String> {
        match is_valid_name(attr_name) {
            true => Ok(()),
            false => Err(attr_name.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Attribute, DataType};
    #[test]
    fn test_new_i8_attr() {
        let attr = Attribute::new_i8_attr("attr1", vec![0, 1, 2, 3]).unwrap();

        assert_eq!(DataType::I8, attr.data_type());
        assert!(attr.get_i8().is_some());
        assert_eq!(&vec![0_i8, 1, 2, 3], attr.get_i8().unwrap());

        assert_eq!(None, attr.get_u8());
        assert_eq!(None, attr.get_i16());
        assert_eq!(None, attr.get_i32());
        assert_eq!(None, attr.get_f32());
        assert_eq!(None, attr.get_f64());
    }
}
