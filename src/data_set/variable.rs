use std::collections::HashSet;
use std::iter::FromIterator;
use std::rc::Rc;

use crate::{name_string::is_valid_name, data_vector::DataVector};
use crate::{Attribute, DataType, Dimension, InvalidDataSet};
use crate::io::compute_num_bytes_zero_padding;

/// NetCDF-3 variable
///
/// `Variable` instances are managed by a [`DataSet`](struct.DataSet.html).
///
/// `DataSet`s allow to create, read, remove and rename `Variable`s.
///
/// # Examples
///
/// ## Create a variable
///
/// ```
/// use netcdf3::{DataSet, Variable, DataType, DimensionType};
///
/// const VAR_NAME: &str = "var_1";
/// const DIM_NAME_1: &str = "dim_1";
/// const DIM_NAME_2: &str = "dim_2";
/// const DIM_SIZE_1: usize = 2;
/// const DIM_SIZE_2: usize = 3;
/// const DATA_F32: &'static [f32; 6] = &[0.0, 1.0, 2.0, 3.0, 4.0, 5.0];
/// const DATA_F32_LEN: usize = DATA_F32.len();
///
/// assert_eq!(DATA_F32_LEN, DIM_SIZE_1 * DIM_SIZE_2);
///
/// // Create a data set
/// let mut data_set: DataSet = DataSet::new();
/// // Define 2 dimensions
/// data_set.set_unlimited_dim(DIM_NAME_1, DIM_SIZE_1).unwrap();
/// data_set.add_fixed_dim(DIM_NAME_2, DIM_SIZE_2).unwrap();
/// // Define a `f32` variable
/// data_set.add_var_f32(VAR_NAME, &[DIM_NAME_1, DIM_NAME_2]).unwrap();
/// // Set a data vector
/// data_set.set_var_f32(VAR_NAME, DATA_F32.to_vec()).unwrap();
///
/// // Get values stored in the file header
/// assert_eq!(1,                                   data_set.num_vars());
/// assert_eq!(2,                                   data_set.num_dims());
/// assert_eq!(Some(DIM_SIZE_1),                    data_set.get_dim_size(DIM_NAME_1));
/// assert_eq!(true,                                data_set.has_unlimited_dim());
/// assert_eq!(true,                                data_set.has_dim(DIM_NAME_1));
/// assert_eq!(Some(DimensionType::UnlimitedSize),  data_set.get_dim_type(DIM_NAME_1));
/// assert_eq!(true,                                data_set.has_dim(DIM_NAME_2));
/// assert_eq!(Some(DIM_SIZE_2),                    data_set.get_dim_size(DIM_NAME_2));
/// assert_eq!(Some(DimensionType::FixedSize),      data_set.get_dim_type(DIM_NAME_2));
/// assert_eq!(true,                                data_set.has_var(VAR_NAME));
/// assert_eq!(Some(0),                             data_set.num_var_attrs(VAR_NAME));
/// assert_eq!(Some(DATA_F32_LEN),                  data_set.get_var_len(VAR_NAME));
/// assert_eq!(Some(DataType::F32),                 data_set.get_var_data_type(VAR_NAME));
///
/// // Get `f32` data thought the data set
/// assert_eq!(None,                data_set.get_var_i8(VAR_NAME));
/// assert_eq!(None,                data_set.get_var_u8(VAR_NAME));
/// assert_eq!(None,                data_set.get_var_i16(VAR_NAME));
/// assert_eq!(None,                data_set.get_var_i32(VAR_NAME));
/// assert_eq!(Some(&DATA_F32[..]), data_set.get_var_f32(VAR_NAME));
/// assert_eq!(None,                data_set.get_var_f64(VAR_NAME));
/// 
/// // Or through a reference to the variable
/// let var: &Variable = data_set.get_var(VAR_NAME).unwrap();
/// assert_eq!(VAR_NAME,                        var.name());
/// assert_eq!(true,                            var.is_record_var());
/// assert_eq!(2,                               var.num_dims());
/// assert_eq!(0,                               var.num_attrs());
/// assert_eq!(vec![DIM_NAME_1, DIM_NAME_2],    var.get_dim_names());
/// assert_eq!(DATA_F32_LEN,                    var.len());
/// assert_eq!(DataType::F32,                   var.data_type());
///
/// assert_eq!(None,                            var.get_i8());
/// assert_eq!(None,                            var.get_u8());
/// assert_eq!(None,                            var.get_i16());
/// assert_eq!(None,                            var.get_i32());
/// assert_eq!(Some(&DATA_F32[..]),             var.get_f32());
/// assert_eq!(None,                            var.get_f64());
/// ```
/// ## Get a reference to a `Variable`
///
/// ```
/// # use netcdf3::{DataSet, Variable, DataType};
/// const VAR_NAME: &str = "var_1";
/// # // Create a new data set
/// # let mut data_set: DataSet = DataSet::new();
/// # // Create a new (empty) variable
/// # data_set.add_var::<&str>(VAR_NAME, &[], DataType::F32).unwrap();
///
/// // Get a reference to a `Variable`
/// let var: &Variable = data_set.get_var(VAR_NAME).unwrap();
///
/// // Get a mutable reference to a `Variable`
/// let var: &mut Variable = data_set.get_var_mut(VAR_NAME).unwrap();
/// ```
///
/// ## Rename a variable
///
/// ```
/// use netcdf3::{DataSet, DataType};
/// const VAR_NAME_1: &str = "var_1";
/// const VAR_NAME_2: &str = "var_2";
/// const DIM_NAME: &str = "dim_1";
/// const VAR_DATA: [i32; 4] = [1, 2, 3, 4];
/// const VAR_DATA_LEN: usize = VAR_DATA.len();
///
/// // Create a data set and a variable
/// let mut data_set: DataSet = DataSet::new();
/// data_set.add_fixed_dim(DIM_NAME, VAR_DATA_LEN).unwrap();
/// data_set.add_var_i32::<&str>(VAR_NAME_1, &[DIM_NAME]).unwrap();
/// data_set.set_var_i32(VAR_NAME_1, VAR_DATA.to_vec()).unwrap();
///
/// assert_eq!(1,                   data_set.num_vars());
/// assert_eq!(true,                data_set.has_var(VAR_NAME_1));
/// assert_eq!(Some(VAR_DATA_LEN),  data_set.get_var_len(VAR_NAME_1));
/// assert_eq!(Some(DataType::I32), data_set.get_var_data_type(VAR_NAME_1));
/// assert_eq!(Some(&VAR_DATA[..]), data_set.get_var_i32(VAR_NAME_1));
/// assert_eq!(false,               data_set.has_var(VAR_NAME_2));
/// assert_eq!(None,                data_set.get_var_len(VAR_NAME_2));
/// assert_eq!(None,                data_set.get_var_data_type(VAR_NAME_2));
/// assert_eq!(None,                data_set.get_var_i32(VAR_NAME_2));
///
/// // Rename the variable
/// data_set.rename_var(VAR_NAME_1, VAR_NAME_2).unwrap();
///
/// assert_eq!(1,                   data_set.num_vars());
/// assert_eq!(false,               data_set.has_var(VAR_NAME_1));
/// assert_eq!(None,                data_set.get_var_len(VAR_NAME_1));
/// assert_eq!(None,                data_set.get_var_data_type(VAR_NAME_1));
/// assert_eq!(None,                data_set.get_var_i32(VAR_NAME_1));
/// assert_eq!(true,                data_set.has_var(VAR_NAME_2));
/// assert_eq!(Some(VAR_DATA_LEN),  data_set.get_var_len(VAR_NAME_2));
/// assert_eq!(Some(DataType::I32), data_set.get_var_data_type(VAR_NAME_2));
/// assert_eq!(Some(&VAR_DATA[..]), data_set.get_var_i32(VAR_NAME_2));
/// ```
///
/// ## Remove a variable
///
/// ```
/// use netcdf3::{DataSet, DataType};
///
/// const DIM_NAME: &str = "dim_1";
/// const VAR_NAME: &str = "var_1";
/// const VAR_DATA: [i32; 4] = [1, 2, 3, 4];
/// const VAR_DATA_LEN: usize = VAR_DATA.len();
///
/// // Create a data set and a variable
/// let mut data_set: DataSet = DataSet::new();
///
/// data_set.add_fixed_dim(DIM_NAME, VAR_DATA_LEN).unwrap();
/// data_set.add_var_i32::<&str>(VAR_NAME, &[DIM_NAME]).unwrap();
/// data_set.set_var_i32(VAR_NAME, VAR_DATA.to_vec()).unwrap();
///
/// assert_eq!(1,                   data_set.num_vars());
/// assert_eq!(true,                data_set.has_var(VAR_NAME));
/// assert_eq!(Some(VAR_DATA_LEN),  data_set.get_var_len(VAR_NAME));
/// assert_eq!(Some(DataType::I32), data_set.get_var_data_type(VAR_NAME));
/// assert_eq!(Some(&VAR_DATA[..]), data_set.get_var_i32(VAR_NAME));
///
/// // Remove the variable
/// data_set.remove_var(VAR_NAME).unwrap();
///
/// assert_eq!(0,       data_set.num_vars());
/// assert_eq!(false,   data_set.has_var(VAR_NAME));
/// assert_eq!(None,    data_set.get_var_len(VAR_NAME));
/// assert_eq!(None,    data_set.get_var_data_type(VAR_NAME));
/// assert_eq!(None,    data_set.get_var_i32(VAR_NAME));
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Variable {
    pub(in crate::data_set) name: String,
    pub(in crate::data_set) unlimited_dim: Option<Rc<Dimension>>,
    pub(in crate::data_set) dims: Vec<Rc<Dimension>>,
    pub(in crate::data_set) attrs: Vec<Attribute>,
    pub(in crate::data_set) data_type: DataType,
    pub(crate) data: Option<DataVector>,
}

impl Variable {
    pub(in crate::data_set) fn new(var_name: &str, var_dims: Vec<Rc<Dimension>>, data_type: DataType) -> Result<Variable, InvalidDataSet> {
        // Check if the name of the variable is a valid NetCDF-3 name.
        let _ = Variable::check_var_name(var_name)?;

        let unlimited_dim: Option<Rc<Dimension>> = match var_dims.first() {
            None => None,
            Some(ref first_dim) => match first_dim.is_unlimited() {
                false => None,
                true => Some(Rc::clone(first_dim)),
            },
        };
        Variable::check_dims_validity(var_name, &var_dims)?;

        Ok(Variable {
            name: var_name.to_string(),
            unlimited_dim: unlimited_dim,
            dims: var_dims,
            attrs: vec![],
            data_type: data_type,
            data: None,
        })
    }

    /// Return the name of the variable.
    pub fn name(&self) -> &str {
        return &self.name;
    }

    /// Returns the name of the variable.
    pub fn data_type(&self) -> DataType {
        return self.data_type.clone();
    }

    /// Returns the total number of elements.
    ///
    /// If the variable is a record variable then `len = num_chunks * num_elements_per_chunk`.
    pub fn len(&self) -> usize {
        return self.num_chunks() * self.num_elements_per_chunk();
    }

    pub fn use_dim(&self, dim_name: &str) -> bool {
        return self.dims.iter().position(|dim| *dim.name.borrow() == dim_name).is_some();
    }

    /// Returns the number of dimensions.
    pub fn num_dims(&self) -> usize {
        return self.dims.len();
    }

    /// Returns the list of the dimensions
    pub fn get_dims(&self) -> Vec<Rc<Dimension>>
    {
        self.dims.clone()
    }

    /// Returns the list of the dimension names
    pub fn get_dim_names(&self) -> Vec<String>
    {
        self.dims.iter().map(|dim: &Rc<Dimension>| {
            dim.name().to_string()
        }).collect()
    }

    /// Returns :
    ///
    /// - `true` if the variable is defined over the *unlimited size* dimension, then has several records
    /// - `false` otherwise
    pub fn is_record_var(&self) -> bool {
        if self.dims.is_empty()
        {
            return false;
        }
        else
        {
            return self.dims[0].is_unlimited();
        }
    }

    /// Returns the number of attributes.
    pub fn num_attrs(&self) -> usize {
        return self.attrs.len();
    }

    /// Returns :
    ///
    /// - `true` if the variable has the attribute
    /// - `false` if not
    pub fn has_attr(&self, attr_name: &str) -> bool {
        return self.find_attr_from_name(attr_name).is_ok();
    }

    // Returns the number of elements per chunk.
    pub fn num_elements_per_chunk(&self) -> usize
    {
        let skip_len: usize = if self.is_record_var() { 1 } else { 0 };
        self.dims.iter().skip(skip_len).fold(1, |product, dim| {
            product * dim.size()
        })
    }

    /// Returns the size of each chunk (the number of bytes) including *zero-padding* bytes.
    pub fn chunk_size(&self) -> usize {
        if self.dims.len() == 0
        {
            return 0;
        }
        else
        {
            let mut chunk_size = self.num_elements_per_chunk() * self.data_type.size_of();
            // append the bytes of the zero padding, if necessary
            chunk_size += compute_num_bytes_zero_padding(chunk_size);
            return chunk_size
        }
    }

    /// Returns the number of chunks.
    pub fn num_chunks(&self) -> usize {
        if self.dims.is_empty()
        {
            0
        }
        else {
            match &self.unlimited_dim {
                None => 1_usize,
                Some(unlimited_dim) => unlimited_dim.size() as usize,
            }
        }
    }

    /// Returns all attributs defined in the dataset or in the variable.
    pub fn get_attrs(&self) -> Vec<&Attribute> {
        return self.attrs.iter().collect();
    }

    /// Returns all attributs defined in the dataset or in the variable.
    pub fn get_attr_names(&self) -> Vec<String> {
        return self.attrs.iter().map(|attr: &Attribute| {
            attr.name().to_string()
        }).collect();
    }

    /// Returns a reference counter to the named attribute, return an error if
    /// the attribute is not already defined.
    pub fn get_attr(&self, attr_name: &str) -> Option<&Attribute> {
        return self.find_attr_from_name(attr_name).map(|result: (usize, &Attribute)|{
            result.1
        }).ok();
    }

    /// Appends a new attribute.
    ///
    /// An error is returned if an other attribute with the same name has already been added.
    fn add_attr(&mut self, new_attr: Attribute) -> Result<(), InvalidDataSet> {
        // Check if an other same name attribute already exists.
        if self.find_attr_from_name(&new_attr.name).is_ok() {
            return Err(InvalidDataSet::VariableAttributeAlreadyExists{
                var_name: self.name.to_string(),
                attr_name: new_attr.name.to_string(),
            });
        }
        // append the new attribute
        self.attrs.push(new_attr);
        return Ok(());
    }

    /// Append a new `i8` attribute.
    ///
    /// An error is returned if an other attribute with the same name has already been added.
    pub(in crate::data_set) fn add_attr_i8(&mut self, attr_name: &str, i8_data: Vec<i8>) -> Result<(), InvalidDataSet> {
        let attr: Attribute = Attribute::new_i8(attr_name, i8_data)
            .map_err(|var_attr_name: String| InvalidDataSet::VariableAttributeNameNotValid{
                var_name: self.name.to_string(),
                attr_name: var_attr_name,
            })?;
        self.add_attr(attr)?;
        Ok(())
    }

    /// Append a new `u8` attribute.
    ///
    /// An error is returned if an other attribute with the same name has already been added.
    pub(in crate::data_set) fn add_attr_u8(&mut self, attr_name: &str, u8_data: Vec<u8>) -> Result<(), InvalidDataSet> {
        let attr: Attribute = Attribute::new_u8(attr_name, u8_data)
            .map_err(|var_attr_name: String| InvalidDataSet::VariableAttributeNameNotValid{
                var_name: self.name.to_string(),
                attr_name: var_attr_name,
            })?;
        self.add_attr(attr)?;
        Ok(())
    }

    /// Append a new `i16` attribute.
    ///
    /// An error is returned if an other attribute with the same name has already been added.
    pub(in crate::data_set) fn add_attr_i16(&mut self, attr_name: &str, i16_data: Vec<i16>) -> Result<(), InvalidDataSet> {
        let attr: Attribute = Attribute::new_i16(attr_name, i16_data)
            .map_err(|var_attr_name: String| InvalidDataSet::VariableAttributeNameNotValid{
                var_name: self.name.to_string(),
                attr_name: var_attr_name,
            })?;
        self.add_attr(attr)?;
        Ok(())
    }

    /// Append a new `i32` attribute.
    ///
    /// An error is returned if an other attribute with the same name has already been added.
    pub(in crate::data_set) fn add_attr_i32(&mut self, attr_name: &str, i32_data: Vec<i32>) -> Result<(), InvalidDataSet> {
        let attr: Attribute = Attribute::new_i32(attr_name, i32_data)
            .map_err(|var_attr_name: String| InvalidDataSet::VariableAttributeNameNotValid{
                var_name: self.name.to_string(),
                attr_name: var_attr_name,
            })?;
        self.add_attr(attr)?;
        Ok(())
    }

    /// Append a new `f32` attribute.
    ///
    /// An error is returned if an other attribute with the same name has already been added.
    pub(in crate::data_set) fn add_attr_f32(&mut self, attr_name: &str, f32_data: Vec<f32>) -> Result<(), InvalidDataSet> {
        let attr: Attribute = Attribute::new_f32(attr_name, f32_data)
            .map_err(|var_attr_name: String| InvalidDataSet::VariableAttributeNameNotValid{
                var_name: self.name.to_string(),
                attr_name: var_attr_name,
            })?;
        self.add_attr(attr)?;
        Ok(())
    }

    /// Append a new `f64` attribute.
    ///
    /// An error is returned if an other attribute with the same name has already been added.
    pub(in crate::data_set) fn add_attr_f64(&mut self, attr_name: &str, f64_data: Vec<f64>) -> Result<(), InvalidDataSet> {
        let attr: Attribute = Attribute::new_f64(attr_name, f64_data)
            .map_err(|var_attr_name: String| InvalidDataSet::VariableAttributeNameNotValid{
                var_name: self.name.to_string(),
                attr_name: var_attr_name,
            })?;
        self.add_attr(attr)?;
        Ok(())
    }

    /// Rename an existing attribute.
    ///
    /// An error is returned :
    ///  - the `old_attr_name`is not a valid NetCDF-3 name
    ///  - the `old_attr_name` attribute doesn't exist
    ///  - an other `new_attr_name` attribute already exist
    pub(in crate::data_set) fn rename_attr(&mut self, old_attr_name: &str, new_attr_name: &str) -> Result<(), InvalidDataSet> {
        if old_attr_name == new_attr_name {
            return Ok(());
        }
        // Check if the `old_attr_name` attribute exists
        let renamed_attr_index: usize = self.find_attr_from_name(old_attr_name)?.0;
        // Check if an other `new_attr_name` attribute already exist
        if self.find_attr_from_name(new_attr_name).is_ok() {
            return Err(InvalidDataSet::VariableAttributeAlreadyExists{
                var_name: self.name.to_string(),
                attr_name: new_attr_name.to_string(),
            });
        }

        // Check that `new_attr_name`is a valid NetCDF-3 name
        Attribute::check_attr_name(new_attr_name)
            .map_err(|var_attr_name: String| InvalidDataSet::VariableAttributeNameNotValid{
                var_name: self.name.to_string(),
                attr_name:var_attr_name.to_string()
            })?;
        let renamed_attr: &mut Attribute = &mut self.attrs[renamed_attr_index];
        renamed_attr.name = new_attr_name.to_string();
        return Ok(());
    }

    // Remove the attribute.
    pub fn remove_attr(&mut self, attr_name: &str) -> Result<Attribute, InvalidDataSet> {
        let removed_attr_index: usize = self.find_attr_from_name(attr_name)?.0;
        let removed_attr: Attribute = self.attrs.remove(removed_attr_index);
        return Ok(removed_attr);
    }

    /// Find a dataset's attribute from is name.
    pub(in crate::data_set) fn find_attr_from_name(&self, attr_name: &str) -> Result<(usize, &Attribute), InvalidDataSet> {
        self.attrs
            .iter()
            .position(|attr| {
                // First find the position
                attr.name() == attr_name
            })
            .map(|index| {
                // Then get the referance to the attribute
                return (index, &self.attrs[index]);
            })
            .ok_or(InvalidDataSet::VariableAttributeNotDefined{
                var_name: self.name.to_string(),
                attr_name: attr_name.to_string(),
            })
    }

    pub(super) fn check_var_name(var_name: &str) -> Result<(), InvalidDataSet> {
        return match is_valid_name(var_name) {
            true => Ok(()),
            false => Err(InvalidDataSet::VariableNameNotValid(var_name.to_string())),
        };
    }

    fn check_dims_validity(var_name: &str, dims: &Vec<Rc<Dimension>>) -> Result<(), InvalidDataSet> {
        if dims.is_empty() {
            return Ok(());
        }
        // Check that the optional unlimited dimension is defined at first
        if let Some(unlim_dim) = dims.iter().skip(1).find(|dim: &&Rc<Dimension>| dim.is_unlimited()) {
            let dim_names: Vec<String> = dims.iter().map(|dim: &Rc<Dimension>| {
                dim.name()
            }).collect();
            return Err(InvalidDataSet::UnlimitedDimensionMustBeDefinedFirst{
                var_name: var_name.to_string(),
                unlim_dim_name: unlim_dim.name(),
                get_dim_names: dim_names,
            });
        }
        // Check that the same dimension is not used multiple times by the variable
        let mut repeated_dim_names: Vec<String> = vec![];
        for (i, ref_dim_1) in dims.iter().enumerate().skip(1) {
            let i32ernal_repeated_dim_names: Vec<String> = dims
                .iter()
                .take(i)
                .filter(|ref_dim_2: &&Rc<Dimension>| Rc::ptr_eq(ref_dim_1, ref_dim_2))
                .map(|ref_dim_2: &Rc<Dimension>| ref_dim_2.name())
                .collect();
            repeated_dim_names.extend(i32ernal_repeated_dim_names.into_iter());
        }
        let repeated_dim_names = HashSet::<String>::from_iter(repeated_dim_names.into_iter());
        if !repeated_dim_names.is_empty() {
            let dim_names: Vec<String> = dims.iter().map(|dim: &Rc<Dimension>| {
                dim.name()
            }).collect();
            return Err(InvalidDataSet::DimensionsUsedMultipleTimes{
                var_name: var_name.to_string(),
                get_dim_names: dim_names,
            });
        }
        Ok(())
    }


    /// Returns a reference to the `i8` data vector.
    ///
    /// Returns `None` if :
    ///
    /// - The variable is not a `i8` data variable.
    /// - The data are not initialized or not be loaded while the file reading.
    ///
    /// # Example
    /// ```
    /// use netcdf3::{DataSet, DataType, InvalidDataSet};
    ///
    /// const DIM_NAME: &str = "dim_1";
    ///
    /// const VAR_I32_NAME: &str = "var_i8";
    /// const DATA_I32: [i32; 3] = [1, 2, 3];
    /// const DATA_I32_LEN: usize = DATA_I32.len();
    ///
    /// // Create a new data set, one dimension
    /// let mut data_set = DataSet::new();
    /// data_set.add_fixed_dim(DIM_NAME, DATA_I32_LEN).unwrap();
    ///
    /// assert_eq!(0,       data_set.num_vars());
    /// assert_eq!(false,   data_set.has_var(VAR_I32_NAME));
    /// assert_eq!(None,    data_set.get_var_len(VAR_I32_NAME));
    /// assert_eq!(None,    data_set.get_var_data_type(VAR_I32_NAME));
    /// assert_eq!(None,    data_set.get_var_i8(VAR_I32_NAME));
    /// assert_eq!(None,    data_set.get_var_u8(VAR_I32_NAME));
    /// assert_eq!(None,    data_set.get_var_i16(VAR_I32_NAME));
    /// assert_eq!(None,    data_set.get_var_i32(VAR_I32_NAME));
    /// assert_eq!(None,    data_set.get_var_f32(VAR_I32_NAME));
    /// assert_eq!(None,    data_set.get_var_f64(VAR_I32_NAME));
    ///
    /// // Create a `i32` variable but don't set its values.
    /// data_set.add_var_i32(VAR_I32_NAME, &[DIM_NAME]).unwrap();
    ///
    /// assert_eq!(1,                   data_set.num_vars());
    /// assert_eq!(true,                data_set.has_var(VAR_I32_NAME));
    /// assert_eq!(Some(DATA_I32_LEN),  data_set.get_var_len(VAR_I32_NAME));
    /// assert_eq!(Some(DataType::I32), data_set.get_var_data_type(VAR_I32_NAME));
    /// assert_eq!(None,                data_set.get_var_i8(VAR_I32_NAME));
    /// assert_eq!(None,                data_set.get_var_u8(VAR_I32_NAME));
    /// assert_eq!(None,                data_set.get_var_i16(VAR_I32_NAME));
    /// assert_eq!(None,                data_set.get_var_i32(VAR_I32_NAME));
    /// assert_eq!(None,                data_set.get_var_f32(VAR_I32_NAME));
    /// assert_eq!(None,                data_set.get_var_f64(VAR_I32_NAME));
    ///
    /// // Set a data vector in the `i32` variable
    /// data_set.set_var_i32(VAR_I32_NAME, DATA_I32.to_vec()).unwrap();
    ///
    /// assert_eq!(1,                   data_set.num_vars());
    /// assert_eq!(true,                data_set.has_var(VAR_I32_NAME));
    /// assert_eq!(Some(DATA_I32_LEN),  data_set.get_var_len(VAR_I32_NAME));
    /// assert_eq!(Some(DataType::I32), data_set.get_var_data_type(VAR_I32_NAME));
    /// assert_eq!(None,                data_set.get_var_i8(VAR_I32_NAME));
    /// assert_eq!(None,                data_set.get_var_u8(VAR_I32_NAME));
    /// assert_eq!(None,                data_set.get_var_i16(VAR_I32_NAME));
    /// assert_eq!(Some(&DATA_I32[..]), data_set.get_var_i32(VAR_I32_NAME));
    /// assert_eq!(None,                data_set.get_var_f32(VAR_I32_NAME));
    /// assert_eq!(None,                data_set.get_var_f64(VAR_I32_NAME));
    ///
    /// ```
    pub fn get_i8(&self) -> Option<&[i8]>
    {
        match self.data {
            Some(DataVector::I8(ref data)) => Some(data),
            _ => None,
        }
    }

    /// Returns a reference to the `u8` data vector (refer to the method [get_i8](struct.Variable.html#method.get_i8)).
    pub fn get_u8(&self) -> Option<&[u8]>
    {
        match self.data {
            Some(DataVector::U8(ref data)) => Some(data),
            _ => None,
        }
    }

    /// Returns a reference to the `i16` data vector (refer to the method [get_i8](struct.Variable.html#method.get_i8)).
    pub fn get_i16(&self) -> Option<&[i16]>
    {
        match self.data {
            Some(DataVector::I16(ref data)) => Some(data),
            _ => None,
        }
    }

    /// Returns a reference to the `i32` data vector (refer to the method [get_i8](struct.Variable.html#method.get_i8)).
    pub fn get_i32(&self) -> Option<&[i32]>
    {
        match self.data {
            Some(DataVector::I32(ref data)) => Some(data),
            _ => None,
        }
    }

    /// Returns a reference to the `f32` data vector (refer to the method [get_i8](struct.Variable.html#method.get_i8)).
    pub fn get_f32(&self) -> Option<&[f32]>
    {
        match self.data {
            Some(DataVector::F32(ref data)) => Some(data),
            _ => None,
        }
    }

    /// Returns a reference to the `f64` data vector (refer to the method [get_i8](struct.Variable.html#method.get_i8)).
    pub fn get_f64(&self) -> Option<&[f64]>
    {
        match self.data {
            Some(DataVector::F64(ref data)) => Some(data),
            _ => None,
        }
    }

    /// Set `i8` data
    ///
    /// Returns an error if :
    ///
    ///  - The length of the data vector is not equal to the variable length.
    ///  - The data type is not the same.
    ///
    /// # Example
    ///
    /// ```
    /// use netcdf3::{DataSet, DataType, InvalidDataSet};
    ///
    /// const DIM_NAME: &str = "dim_1";
    ///
    /// const VAR_I32_NAME: &str = "var_i8";
    /// const DATA_I32: [i32; 3] = [1, 2, 3];
    /// const DATA_I32_LEN: usize = DATA_I32.len();
    ///
    /// const DATA_F32: [f32; 3] = [1.0, 2.0, 3.0];
    /// const DATA_F32_LEN: usize = DATA_F32.len();
    ///
    /// assert_eq!(DATA_I32_LEN, DATA_F32_LEN);
    ///
    /// // Create a new data set, one dimension, and one variable
    /// let mut data_set = DataSet::new();
    /// data_set.add_fixed_dim(DIM_NAME, DATA_I32_LEN).unwrap();
    /// data_set.add_var_i32(VAR_I32_NAME, &[DIM_NAME]).unwrap();
    ///
    /// assert_eq!(1,                   data_set.num_vars());
    /// assert_eq!(true,                data_set.has_var(VAR_I32_NAME));
    /// assert_eq!(Some(DATA_I32_LEN),  data_set.get_var_len(VAR_I32_NAME));
    /// assert_eq!(Some(DataType::I32), data_set.get_var_data_type(VAR_I32_NAME));
    /// assert_eq!(None,                data_set.get_var_i32(VAR_I32_NAME));
    ///
    /// // Try to set a data vector with a wrong number of elements
    /// assert_eq!(
    ///     InvalidDataSet::VariableMismatchDataLength{
    ///         var_name: String::from(VAR_I32_NAME),
    ///         req: DATA_I32_LEN,
    ///         get: DATA_I32_LEN - 1,
    ///     },
    ///     data_set.set_var_i32(VAR_I32_NAME, DATA_I32[0..(DATA_I32_LEN - 1)].to_vec()).unwrap_err()
    /// );
    ///
    /// assert_eq!(1,                   data_set.num_vars());
    /// assert_eq!(true,                data_set.has_var(VAR_I32_NAME));
    /// assert_eq!(Some(DATA_I32_LEN),  data_set.get_var_len(VAR_I32_NAME));
    /// assert_eq!(Some(DataType::I32), data_set.get_var_data_type(VAR_I32_NAME));
    /// assert_eq!(None,                data_set.get_var_i32(VAR_I32_NAME));
    ///
    /// /// Try to set a data vector with a wrong datat type
    /// assert_eq!(
    ///     InvalidDataSet::VariableMismatchDataType{
    ///         var_name: String::from(VAR_I32_NAME),
    ///         req: DataType::I32,
    ///         get: DataType::F32,
    ///     },
    ///     data_set.set_var_f32(VAR_I32_NAME, DATA_F32.to_vec()).unwrap_err()
    /// );
    ///
    /// assert_eq!(1,                   data_set.num_vars());
    /// assert_eq!(true,                data_set.has_var(VAR_I32_NAME));
    /// assert_eq!(Some(DATA_I32_LEN),  data_set.get_var_len(VAR_I32_NAME));
    /// assert_eq!(Some(DataType::I32), data_set.get_var_data_type(VAR_I32_NAME));
    /// assert_eq!(None,                data_set.get_var_i32(VAR_I32_NAME));
    ///
    /// // Set a data vector with the valid number of elements and the valid data type
    /// data_set.set_var_i32(VAR_I32_NAME, DATA_I32.to_vec()).unwrap();
    ///
    /// assert_eq!(1,                       data_set.num_vars());
    /// assert_eq!(true,                    data_set.has_var(VAR_I32_NAME));
    /// assert_eq!(Some(DATA_I32_LEN),      data_set.get_var_len(VAR_I32_NAME));
    /// assert_eq!(Some(DataType::I32),     data_set.get_var_data_type(VAR_I32_NAME));
    /// assert_eq!(Some(&DATA_I32[..]),    data_set.get_var_i32(VAR_I32_NAME));
    /// ```
    pub fn set_i8(&mut self, data: Vec<i8>) -> Result<(), InvalidDataSet>
    {
        // Check that the variable is a `i8` type variable
        if self.data_type != DataType::I8 {
            return Err(InvalidDataSet::VariableMismatchDataType{
                var_name: self.name.to_string(),
                req: self.data_type.clone(),
                get: DataType::I8,
            });
        }

        // Check that the variable has the same length the the input vector
        let var_len: usize = self.len();
        if var_len != data.len() {
            return Err(InvalidDataSet::VariableMismatchDataLength{
                var_name: self.name.to_string(),
                req: var_len,
                get: data.len()
            });
        }

        // Set the data
        self.data = Some(DataVector::I8(data));
        Ok(())
    }

    /// Set `u8` data (refer to the method [set_i8](struct.Variable.html#method.set_i8)).
    pub fn set_u8(&mut self, data: Vec<u8>) -> Result<(), InvalidDataSet>
    {
        // Check that the variable is a `u8` type variable
        if self.data_type != DataType::U8 {
            return Err(InvalidDataSet::VariableMismatchDataType{
                var_name: self.name.to_string(),
                req: self.data_type.clone(),
                get: DataType::U8,
            });
        }

        // Check that the variable has the same length the the input vector
        let var_len: usize = self.len();
        if var_len != data.len() {
            return Err(InvalidDataSet::VariableMismatchDataLength{
                var_name: self.name.to_string(),
                req: var_len,
                get: data.len()
            });
        }

        // Set the data
        self.data = Some(DataVector::U8(data));
        Ok(())
    }

    /// Set `i16` data (refer to the method [set_i8](struct.Variable.html#method.set_i8)).
    pub fn set_i16(&mut self, data: Vec<i16>) -> Result<(), InvalidDataSet>
    {
        // Check that the variable is a `i16` type variable
        if self.data_type != DataType::I16 {
            return Err(InvalidDataSet::VariableMismatchDataType{
                var_name: self.name.to_string(),
                req: self.data_type.clone(),
                get: DataType::I16,
            });
        }

        // Check that the variable has the same length the the input vector
        let var_len: usize = self.len();
        if var_len != data.len() {
            return Err(InvalidDataSet::VariableMismatchDataLength{
                var_name: self.name.to_string(),
                req: var_len,
                get: data.len()
            });
        }

        // Set the data
        self.data = Some(DataVector::I16(data));
        Ok(())
    }

    /// Set `i32` data (refer to the method [set_i8](struct.Variable.html#method.set_i8)).
    pub fn set_i32(&mut self, data: Vec<i32>) -> Result<(), InvalidDataSet>
    {
        // Check that the variable is a `i32` type variable
        if self.data_type != DataType::I32 {
            return Err(InvalidDataSet::VariableMismatchDataType{
                var_name: self.name.to_string(),
                req: self.data_type.clone(),
                get: DataType::I32,
            });
        }

        // Check that the variable has the same length the the input vector
        let var_len: usize = self.len();
        if var_len != data.len() {
            return Err(InvalidDataSet::VariableMismatchDataLength{
                var_name: self.name.to_string(),
                req: var_len,
                get: data.len()
            });
        }

        // Set the data
        self.data = Some(DataVector::I32(data));
        Ok(())
    }

    /// Set `f32` data (refer to the method [set_i8](struct.Variable.html#method.set_i8)).
    pub fn set_f32(&mut self, data: Vec<f32>) -> Result<(), InvalidDataSet>
    {
        // Check that the variable is a `f32` type variable
        if self.data_type != DataType::F32 {
            return Err(InvalidDataSet::VariableMismatchDataType{
                var_name: self.name.to_string(),
                req: self.data_type.clone(),
                get: DataType::F32,
            });
        }

        // Check that the variable has the same length the the input vector
        let var_len: usize = self.len();
        if var_len != data.len() {
            return Err(InvalidDataSet::VariableMismatchDataLength{
                var_name: self.name.to_string(),
                req: var_len,
                get: data.len()
            });
        }

        // Set the data
        self.data = Some(DataVector::F32(data));
        Ok(())
    }

    /// Set `f64` data (refer to the method [set_i8](struct.Variable.html#method.set_i8)).
    pub fn set_f64(&mut self, data: Vec<f64>) -> Result<(), InvalidDataSet>
    {
        // Check that the variable is a `f64` type variable
        if self.data_type != DataType::F64 {
            return Err(InvalidDataSet::VariableMismatchDataType{
                var_name: self.name.to_string(),
                req: self.data_type.clone(),
                get: DataType::F64,
            });
        }

        // Check that the variable has the same length the the input vector
        let var_len: usize = self.len();
        if var_len != data.len() {
            return Err(InvalidDataSet::VariableMismatchDataLength{
                var_name: self.name.to_string(),
                req: var_len,
                get: data.len()
            });
        }

        // Set the data
        self.data = Some(DataVector::F64(data));
        Ok(())
    }

}
