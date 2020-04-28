mod dimension;
pub use dimension::{Dimension, DimensionType};

mod attribute;
pub use attribute::Attribute;

mod variable;
pub use variable::Variable;

mod tests;

use std::{cell::RefMut, ops::Deref, rc::Rc};

use crate::{typed_data_vector::DataVector, DataType, error::InvalidDataSet};

#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Eq)]
/// NetCDF-3 versions
pub enum Version {
    /// Classic format (using a 32-bit offset integer)
    Classic = 1,
    /// 64-bit offset format (using a 64-bit offset integer)
    Offset64Bit = 2,
}

impl std::convert::TryFrom<u8> for Version {

    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1_u8 => Ok(Version::Classic),
            2_u8 => Ok(Version::Offset64Bit),
            _ => Err("Invalid data type number."),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct DataSet {
    unlimited_dim: Option<Rc<Dimension>>,
    pub(crate) dims: Vec<Rc<Dimension>>,
    attrs: Vec<Attribute>,
    pub(crate) vars: Vec<Variable>,
    pub(crate) version: Version,
}

impl DataSet {
    // Creates an new empty NetCDF-3 dataset.
    pub fn new(version: Version) -> DataSet {
        DataSet {
            unlimited_dim: None,
            dims: vec![],
            attrs: vec![],
            vars: vec![],
            version: version,
        }
    }

    /// Returns the version of the NetCDF-3 file.
    pub fn version(&self) -> Version
    {
        return self.version.clone();
    }


    /// Modifies the version of the NetCDF-3 file.
    pub fn set_version(&mut self, version: Version)
    {
        self.version = version;
    }
    
    /// Appends a new *fixed size* dimension in the dataset.
    ///
    /// Returns a error if an other dimension with the same name is already defined.
    pub fn add_fixed_dim<T: std::convert::AsRef<str>>(&mut self, dim_name: T, dim_size: usize) -> Result<(), InvalidDataSet> {
        let dim_name: &str = dim_name.as_ref();
        if self.dims.iter().position(|dim| *dim.name.borrow() == dim_name).is_some() {
            return Err(InvalidDataSet::DimensionNameAlreadyExists(dim_name.to_string()));
        }
        let new_fixed_size_dim = Rc::new(Dimension::new_fixed_size(dim_name, dim_size)?);
        self.dims.push(new_fixed_size_dim);
        return Ok(());
    }

    /// Initializes the *unlimited size* dimension of the dataset.
    ///
    /// Returns a error if :
    ///  1. the *unlimited size* is already defined
    ///  2. if an other dimension with the same name is already defined
    pub fn set_unlimited_dim<T: std::convert::AsRef<str>>(&mut self, dim_name: T, dim_size: usize) -> Result<(), InvalidDataSet> {
        let dim_name: &str = dim_name.as_ref();
        if let Some(unlimited_dim) = &self.unlimited_dim {
            return Err(InvalidDataSet::UnlimitedDimensionAlreadyExists(unlimited_dim.name()));
        }
        if self.dims.iter().position(|dim| *dim.name.borrow() == dim_name).is_some() {
            return Err(InvalidDataSet::DimensionNameAlreadyExists(dim_name.to_string()));
        }
        let new_unlimited_dim = Rc::new(Dimension::new_unlimited_size(dim_name, dim_size)?);
        self.dims.push(Rc::clone(&new_unlimited_dim));
        self.unlimited_dim = Some(new_unlimited_dim);
        return Ok(());
    }

    /// Returns the number of dimensions defined in the data set.
    pub fn num_dims(&self) -> usize {
        return self.dims.len();
    }

    /// Returns :
    ///  - `true` if the dimension is defined.
    ///  - `false` otherwise.
    pub fn has_dim(&self, dim_name: &str) -> bool {
        return self.find_dim_from_name(dim_name).is_some();
    }

    /// Returns a reference to the dimension.
    ///
    /// Returns `None` if the dimension is not defined.
    pub fn get_dim(&self, dim_name: &str) -> Option<Rc<Dimension>> {
        self.find_dim_from_name(dim_name)
            .map(|(_dim_index, dim): (usize, &Rc<Dimension>)| Rc::clone(dim))
    }

    /// Returns the references of all the dimensions defined in the dataset.
    pub fn get_dims(&self) -> Vec<Rc<Dimension>> {
        return self.dims.iter().map(|dim: &Rc<Dimension>| Rc::clone(dim)).collect();
    }

    /// Returns the names all the dimensions defined in the dataset.
    pub fn get_dim_names(&self) -> Vec<String>
    {
        self.dims.iter().map(|dim| {
            dim.name().to_string()
        }).collect()
    }

    /// Returns `true` if the *unlimited-size* dimension is defined, otherwise return `false`.
    pub fn has_unlimited_dim(&self) -> bool {
        return self.unlimited_dim.is_some();
    }

    /// Returns the *unlimited-size* dimension if it is defined, otherwise return `None`.
    pub fn get_unlimited_dim(&self) -> Option<Rc<Dimension>> {
        return self.unlimited_dim.as_ref().map(|rc_dim: &Rc<Dimension>| Rc::clone(rc_dim));
    }

    /// Removes and returns the dimension.
    ///
    /// Returns an error if:
    ///
    /// - the dimension is not already defined
    /// - the dimension is yet used by a variable of the dataset
    pub fn remove_dim(&mut self, dim_name: &str) -> Result<Rc<Dimension>, InvalidDataSet> {
        let removed_dim_index: usize = match self.find_dim_from_name(dim_name) {
            None => {
                return Err(InvalidDataSet::DimensionNotDefined(dim_name.to_string()));
            }
            Some((index, _)) => index,
        };
        let mut variables_using_removed_dim: Vec<String> = vec![];
        for current_var in self.vars.iter() {
            if current_var.use_dim(dim_name) {
                variables_using_removed_dim.push(current_var.name.clone());
            }
        }
        if !variables_using_removed_dim.is_empty() {
            return Err(InvalidDataSet::DimensionYetUsed(variables_using_removed_dim, dim_name.to_string()));
        }
        let removed_dim: Rc<Dimension> = self.dims.remove(removed_dim_index);
        return Ok(removed_dim);
    }

    /// Rename the dimension or return en error if :
    /// - no dimension named `old_dim_name` already exists
    /// - an other dimension named `new_dim_name` already exists
    /// - the `new_dim_name` is not a NetCDF-3 valid name
    ///
    /// **Nothing is done if `old_dim_name` and `new_dim_name` are the same.**
    pub fn rename_dim(&mut self, old_dim_name: &str, new_dim_name: &str) -> Result<(), InvalidDataSet> {
        if old_dim_name == new_dim_name {
            // nothing is done
            return Ok(());
        }

        let (_dim_position, renamed_dim): (usize, &Rc<Dimension>) = match self.find_dim_from_name(old_dim_name) {
            None => {
                return Err(InvalidDataSet::DimensionNotDefined(old_dim_name.to_string()));
            }
            Some(rc_dim) => rc_dim,
        };

        if self.find_dim_from_name(new_dim_name).is_some() {
            return Err(InvalidDataSet::DimensionNameAlreadyExists(new_dim_name.to_string()));
        }

        Dimension::check_dim_name(new_dim_name)?;

        let mut dim_name: RefMut<String> = renamed_dim.name.borrow_mut();
        *dim_name = new_dim_name.to_string();
        return Ok(());
    }

    /// Find a dataset's dimension from is name.
    fn find_dim_from_name(&self, dim_name: &str) -> Option<(usize, &Rc<Dimension>)> {
        return self
            .dims
            .iter()
            .position(|dim| {
                return dim.name.borrow().deref() == dim_name;
            })
            .map(|index| {
                return (index, &self.dims[index]);
            });
    }

    pub fn get_dims_from_ids(&self, dim_ids: &Vec<usize>) -> Result<Vec<Rc<Dimension>>, InvalidDataSet> {
        let invalid_dim_ids: Vec<usize> = dim_ids
            .iter()
            .filter(|dim_id: &&usize| self.dims.get(**dim_id).is_none())
            .map(|i| i.clone())
            .collect();
        if !invalid_dim_ids.is_empty() {
            return Err(InvalidDataSet::DimensionsIdsNotValid(invalid_dim_ids));
        }
        Ok(dim_ids.iter().map(|dim_id: &usize| Rc::clone(&self.dims[*dim_id])).collect())
    }

    /// Add a new variable in the dataset defined over named dimensions.
    ///
    /// # Examples
    ///
    /// Add a variable
    ///
    /// ```
    /// use netcdf3::{DataSet, DataType, Version};
    ///
    /// let mut data_set = DataSet::new(Version::Classic);
    /// let _ = data_set.add_fixed_dim("latitude", 181).unwrap();
    /// let _ = data_set.add_fixed_dim("longitude", 361).unwrap();
    /// let _ = data_set.set_unlimited_dim("time", 2).unwrap();
    ///
    /// assert_eq!(0, data_set.num_vars());
    /// let _ = data_set.add_var("sea_level_temperature", &["latitude", "longitude"], DataType::F64).unwrap();
    /// assert_eq!(1, data_set.num_vars());
    /// ```
    ///
    /// Add an empty variable
    ///
    /// ```
    /// use netcdf3::{DataSet, DataType, Version};
    ///
    /// let mut data_set = DataSet::new(Version::Classic);
    ///
    /// assert_eq!(0, data_set.num_vars());
    /// let _ = data_set.add_var("empty_variable", &[] as &[&str] /* no dimensions*/, DataType::U8).unwrap();
    /// assert_eq!(1, data_set.num_vars());
    /// ```
    pub fn add_var<T: std::convert::AsRef<str>>(&mut self, var_name: &str, dims_name: &[T], data_type: DataType) -> Result<(), InvalidDataSet> {

        let var_dims: Vec<&Rc<Dimension>> = {
            let mut var_dims: Vec<&Rc<Dimension>> = vec![];
            let mut undefined_dims: Vec<String> = vec![];
            for dim_name in dims_name.iter() {
                let dim_name: &str = dim_name.as_ref();
                match self.find_dim_from_name(dim_name) {
                    None => {
                        undefined_dims.push(dim_name.to_string());
                    }
                    Some((_index, dim)) => {
                        var_dims.push(dim);
                    }
                }
            }
            if !undefined_dims.is_empty() {
                return Err(InvalidDataSet::DimensionsNotDefined(undefined_dims));
            }
            var_dims
        };
        let var_dims: Vec<Rc<Dimension>> = var_dims.into_iter().map(|ref dim| Rc::clone(dim)).collect();
        self.add_var_using_dim_refs(var_name, var_dims, data_type.clone())?;
        Ok(())
    }

    pub(crate) fn add_var_using_dim_refs(&mut self, var_name: &str, var_dims: Vec<Rc<Dimension>>, data_type: DataType) -> Result<(), InvalidDataSet> {
        let _ = self.vars.push(Variable::new(var_name, var_dims, data_type)?);
        Ok(())
    }

    /// Add a new `i8` type variable  defined over named dimensions (see the [add_var](struct.DataSet.html#method.add_var) method).
    pub fn add_var_i8<T: std::convert::AsRef<str>>(&mut self, var_name: &str, dims_name: &[T]) -> Result<(), InvalidDataSet> {
        self.add_var(var_name, dims_name, DataType::I8)
    }

    /// Add a new `u8` type variable  defined over named dimensions (see the [add_var](struct.DataSet.html#method.add_var) method).
    pub fn add_var_u8<T: std::convert::AsRef<str>>(&mut self, var_name: &str, dims_name: &[T]) -> Result<(), InvalidDataSet> {
        self.add_var(var_name, dims_name, DataType::U8)
    }

    /// Add a new `i16` type variable  defined over named dimensions (see the [add_var](struct.DataSet.html#method.add_var) method).
    pub fn add_var_i16<T: std::convert::AsRef<str>>(&mut self, var_name: &str, dims_name: &[T]) -> Result<(), InvalidDataSet> {
        self.add_var(var_name, dims_name, DataType::I16)
    }

    /// Add a new `i32` type variable  defined over named dimensions (see the [add_var](struct.DataSet.html#method.add_var) method).
    pub fn add_var_i32<T: std::convert::AsRef<str>>(&mut self, var_name: &str, dims_name: &[T]) -> Result<(), InvalidDataSet> {
        self.add_var(var_name, dims_name, DataType::I32)
    }

    /// Add a new `f32` type variable  defined over named dimensions (see the [add_var](struct.DataSet.html#method.add_var) method).
    pub fn add_var_f32<T: std::convert::AsRef<str>>(&mut self, var_name: &str, dims_name: &[T]) -> Result<(), InvalidDataSet> {
        self.add_var(var_name, dims_name, DataType::F32)
    }

    /// Add a new `f64` type variable  defined over named dimensions (see the [add_var](struct.DataSet.html#method.add_var) method).
    pub fn add_var_f64<T: std::convert::AsRef<str>>(&mut self, var_name: &str, dims_name: &[T]) -> Result<(), InvalidDataSet> {
        self.add_var(var_name, dims_name, DataType::F64)
    }

    /// Returns the number of defined variables.
    pub fn num_vars(&self) -> usize {
        self.vars.len()
    }

    /// Returns :
    ///  - `true` if the variable is defined.
    ///  - `false` otherwise.
    pub fn has_var(&self, var_name: &str) -> bool {
        return self.find_var_from_name(var_name).is_ok();
    }

    // Returns the named variables defined in the dataset.
    pub fn get_var(&self, var_name: &str) -> Option<&Variable> {
        return self
            .find_var_from_name(var_name)
            .map(|(_var_index, var): (usize, &Variable)| var)
            .ok();
    }

    /// Returns the references all the variables defined in the dataset.
    pub fn get_vars(&self) -> Vec<&Variable> {
        return self.vars.iter().collect();
    }

    /// Returns the names all the variables defined in the dataset.
    pub fn get_var_names(&self) -> Vec<String>
    {
        return self.vars.iter().map(|var: &Variable|{
            var.name().to_string()
        }).collect();
    }

    /// Renames a variable.
    ///
    /// Nothing is do if `old_var_name` and `new_var_name` the same.
    ///
    /// Returns an error if :
    /// - no variable `old_var_name` exists
    /// - an other variable `new_var_name` already exists
    /// - `new_var_name` is a NetCDF-3 valid name
    pub fn rename_var(&mut self, old_var_name: &str, new_var_name: &str) -> Result<(), InvalidDataSet> {
        // If the names are same then nothing of done
        if old_var_name == new_var_name {
            return Ok(());
        }
        // Get the index of the renamed variable
        let renamed_var_index: usize = self.find_var_from_name(old_var_name)?.0;

        // Check that an other variable has already been defined with `new_var_name`
        if self.find_var_from_name(new_var_name).is_ok() {
            return Err(InvalidDataSet::VariableAlreadyExists(new_var_name.to_string()));
        }
        // Check the validity of the new name
        let _ = Variable::check_var_name(new_var_name)?;

        // Then rename the variable
        self.vars[renamed_var_index].name = new_var_name.to_string();

        return Ok(());
    }

    /// Remove a variable.
    ///
    /// Return the removed variable if it exists.
    /// Returns an error if the variable named `var_name` does not exists.
    pub fn remove_var(&mut self, var_name: &str) -> Result<Variable, InvalidDataSet> {
        let var_index: usize = self.find_var_from_name(var_name)?.0;
        let removed_var: Variable = self.vars.remove(var_index);
        return Ok(removed_var);
    }


    /// Find a dataset's variable from is name, and return its position and a reference to it.
    pub(crate) fn find_var_from_name(&self, var_name: &str) -> Result<(usize, &Variable), InvalidDataSet> {
        return self
            .vars
            .iter()
            .position(|var: &Variable| var.name == var_name)
            .map(|var_index| (var_index, &self.vars[var_index]))
            .ok_or(InvalidDataSet::VariableNotDefined(var_name.to_string()));
    }

    /// Set data of a named `i8` type variable.
    pub fn set_var_data_i8(&mut self, var_name: &str, data: Vec<i8>) -> Result<(), InvalidDataSet> {
        // Search the variable, must be already defined
        let var_index: usize = self.find_var_from_name(var_name)?.0;
        let ref_mut_var: &mut Variable = &mut self.vars[var_index];

        // Check that the variable is a `i8` type variable
        let var_data_type: DataType = ref_mut_var.data_type();
        if var_data_type != DataType::I8 {
            return Err(InvalidDataSet::VariableMismatchDataType(
                (var_name.to_string(), var_data_type.clone()),
                DataType::I8,
            ));
        }

        // Check that the variable has the same length the the input vector
        let var_data_length: usize = ref_mut_var.len();
        if var_data_length != data.len() {
            return Err(InvalidDataSet::VariableMismatchDataLength(
                (var_name.to_string(), var_data_length),
                data.len(),
            ));
        }

        // Set the data
        ref_mut_var.data = Some(DataVector::I8(data));

        Ok(())
    }

    /// Set data of a named `u8` type variable.
    pub fn set_var_data_u8(&mut self, var_name: &str, data: Vec<u8>) -> Result<(), InvalidDataSet> {
        // Search the variable, must be already defined
        let var_index: usize = self.find_var_from_name(var_name)?.0;
        let ref_mut_var: &mut Variable = &mut self.vars[var_index];

        // Check that the variable is a `u8` type variable
        let var_data_type: DataType = ref_mut_var.data_type();
        if var_data_type != DataType::U8 {
            return Err(InvalidDataSet::VariableMismatchDataType(
                (var_name.to_string(), var_data_type.clone()),
                DataType::U8,
            ));
        }

        // Check that the variable has the same length the the input vector
        let var_data_length: usize = ref_mut_var.len();
        if var_data_length != data.len() {
            return Err(InvalidDataSet::VariableMismatchDataLength(
                (var_name.to_string(), var_data_length),
                data.len(),
            ));
        }

        // Set the data
        ref_mut_var.data = Some(DataVector::U8(data));

        Ok(())
    }

    /// Set data of a named `i16` type variable.
    pub fn set_var_data_i16(&mut self, var_name: &str, data: Vec<i16>) -> Result<(), InvalidDataSet> {
        // Search the variable, must be already defined
        let var_index: usize = self.find_var_from_name(var_name)?.0;
        let ref_mut_var: &mut Variable = &mut self.vars[var_index];

        // Check that the variable is a `i16` type variable
        let var_data_type: DataType = ref_mut_var.data_type();
        if var_data_type != DataType::I16 {
            return Err(InvalidDataSet::VariableMismatchDataType(
                (var_name.to_string(), var_data_type.clone()),
                DataType::I16,
            ));
        }

        // Check that the variable has the same length the the input vector
        let var_data_length: usize = ref_mut_var.len();
        if var_data_length != data.len() {
            return Err(InvalidDataSet::VariableMismatchDataLength(
                (var_name.to_string(), var_data_length),
                data.len(),
            ));
        }

        // Set the data
        ref_mut_var.data = Some(DataVector::I16(data));

        Ok(())
    }

    /// Set data of a named `i32` type variable.
    pub fn set_var_data_i32(&mut self, var_name: &str, data: Vec<i32>) -> Result<(), InvalidDataSet> {
        // Search the variable, must be already defined
        let var_index: usize = self.find_var_from_name(var_name)?.0;
        let ref_mut_var: &mut Variable = &mut self.vars[var_index];

        // Check that the variable is a `i32` type variable
        let var_data_type: DataType = ref_mut_var.data_type();
        if var_data_type != DataType::I32 {
            return Err(InvalidDataSet::VariableMismatchDataType(
                (var_name.to_string(), var_data_type.clone()),
                DataType::I32,
            ));
        }

        // Check that the variable has the same length the the input vector
        let var_data_length: usize = ref_mut_var.len();
        if var_data_length != data.len() {
            return Err(InvalidDataSet::VariableMismatchDataLength(
                (var_name.to_string(), var_data_length),
                data.len(),
            ));
        }

        // Set the data
        ref_mut_var.data = Some(DataVector::I32(data));

        Ok(())
    }

    /// Set data of a named `f32` type variable.
    pub fn set_var_data_f32(&mut self, var_name: &str, data: Vec<f32>) -> Result<(), InvalidDataSet> {
        // Search the variable, must be already defined
        let var_index: usize = self.find_var_from_name(var_name)?.0;
        let ref_mut_var: &mut Variable = &mut self.vars[var_index];

        // Check that the variable is a `i16` type variable
        let var_data_type: DataType = ref_mut_var.data_type();
        if var_data_type != DataType::I16 {
            return Err(InvalidDataSet::VariableMismatchDataType(
                (var_name.to_string(), var_data_type.clone()),
                DataType::I16,
            ));
        }

        // Check that the variable has the same length the the input vector
        let var_data_length: usize = ref_mut_var.len();
        if var_data_length != data.len() {
            return Err(InvalidDataSet::VariableMismatchDataLength(
                (var_name.to_string(), var_data_length),
                data.len(),
            ));
        }

        // Set the data
        ref_mut_var.data = Some(DataVector::F32(data));

        Ok(())
    }

    /// Set data of a named `f64` type variable.
    pub fn set_var_data_f64(&mut self, var_name: &str, data: Vec<f64>) -> Result<(), InvalidDataSet> {
        // Search the variable, must be already defined
        let var_index: usize = self.find_var_from_name(var_name)?.0;
        let ref_mut_var: &mut Variable = &mut self.vars[var_index];

        // Check that the variable is a `f64` type variable
        let var_data_type: DataType = ref_mut_var.data_type();
        if var_data_type != DataType::F64 {
            return Err(InvalidDataSet::VariableMismatchDataType(
                (var_name.to_string(), var_data_type.clone()),
                DataType::F64,
            ));
        }

        // Check that the variable has the same length the the input vector
        let var_data_length: usize = ref_mut_var.len();
        if var_data_length != data.len() {
            return Err(InvalidDataSet::VariableMismatchDataLength(
                (var_name.to_string(), var_data_length),
                data.len(),
            ));
        }

        // Set the data
        ref_mut_var.data = Some(DataVector::F64(data));

        Ok(())
    }

    // Add a `i8` type attribute in a variable.
    pub fn add_var_attr_i8(&mut self, var_name: &str, var_attr_name: &str, var_attr_value: Vec<i8>) -> Result<(), InvalidDataSet> {
        // Check that the variable is defined
        let var_index: usize = self.find_var_from_name(var_name)?.0;
        // Append the new attribute
        let ref_mut_var: &mut Variable = &mut self.vars[var_index];
        ref_mut_var.add_attr_i8(var_attr_name, var_attr_value)?;
        Ok(())
    }

    // Add a `u8` type attribute in a variable.
    pub fn add_var_attr_u8(&mut self, var_name: &str, var_attr_name: &str, var_attr_value: Vec<u8>) -> Result<(), InvalidDataSet> {
        // Check that the variable is defined
        let var_index: usize = self.find_var_from_name(var_name)?.0;
        // Append the new attribute
        let ref_mut_var: &mut Variable = &mut self.vars[var_index];
        ref_mut_var.add_attr_u8(var_attr_name, var_attr_value)?;
        Ok(())
    }

    // Add a `i16` type attribute in a variable.
    pub fn add_var_attr_i16(&mut self, var_name: &str, var_attr_name: &str, var_attr_value: Vec<i16>) -> Result<(), InvalidDataSet> {
        // Check that the variable is defined
        let var_index: usize = self.find_var_from_name(var_name)?.0;
        // Append the new attribute
        let ref_mut_var: &mut Variable = &mut self.vars[var_index];
        ref_mut_var.add_attr_i16(var_attr_name, var_attr_value)?;
        Ok(())
    }

    // Add a `i32` type attribute in a variable.
    pub fn add_var_attr_i32(&mut self, var_name: &str, var_attr_name: &str, var_attr_value: Vec<i32>) -> Result<(), InvalidDataSet> {
        // Check that the variable is defined
        let var_index: usize = self.find_var_from_name(var_name)?.0;
        // Append the new attribute
        let ref_mut_var: &mut Variable = &mut self.vars[var_index];
        ref_mut_var.add_attr_i32(var_attr_name, var_attr_value)?;
        Ok(())
    }


    // Add a `f32` type attribute in a variable.
    pub fn add_var_attr_f32(&mut self, var_name: &str, var_attr_name: &str, var_attr_value: Vec<f32>) -> Result<(), InvalidDataSet> {
        // Check that the variable is defined
        let var_index: usize = self.find_var_from_name(var_name)?.0;
        // Append the new attribute
        let ref_mut_var: &mut Variable = &mut self.vars[var_index];
        ref_mut_var.add_attr_f32(var_attr_name, var_attr_value)?;
        Ok(())
    }

    // Add a `f64` type attribute in a variable.
    pub fn add_var_attr_f64(&mut self, var_name: &str, var_attr_name: &str, var_attr_value: Vec<f64>) -> Result<(), InvalidDataSet> {
        // Check that the variable is defined
        let var_index: usize = self.find_var_from_name(var_name)?.0;
        // Append the new attribute
        let ref_mut_var: &mut Variable = &mut self.vars[var_index];
        ref_mut_var.add_attr_f64(var_attr_name, var_attr_value)?;
        Ok(())
    }

    /// Remove a attribute from a named variable.
    pub fn remove_var_attr(&mut self, var_name: &str, var_attr_name: &str) -> Result<Attribute, InvalidDataSet> {
        let ((var_index, _), (removed_var_attr_index, _)) = self.find_var_attr_from_name(var_name, var_attr_name)?;
        let ref_mut_var: &mut Variable = &mut self.vars[var_index];
        Ok(ref_mut_var.attrs.remove(removed_var_attr_index))
    }

    /// Returns a reference of variable attribute.
    pub fn get_var_attr(&self, var_name: &str, var_attr_name: &str) -> Option<&Attribute> {
        return self.find_var_attr_from_name(var_name, var_attr_name).map(
            |((_var_index, _ref_var), (_attr_index, ref_var)): ((usize, &Variable), (usize, &Attribute))| {
                ref_var
            }
        ).ok();
    }

    /// Returns all attributes of a variable.
    ///
    /// Returns `None` if the variable is not defined.
    /// 
    pub fn get_var_attrs(&self, var_name: &str) -> Option<Vec<&Attribute>> {
        return self.find_var_from_name(var_name).map(|(_var_index, ref_var): (usize, &Variable)|{
            ref_var
        }).ok().map(|ref_var: &Variable| {
            ref_var.get_attrs()
        })
    }

    pub fn rename_var_attr(&mut self, var_name: &str, old_var_attr_name: &str, new_var_attr_name: &str) -> Result<(), InvalidDataSet> {
        let var_index = self.find_var_from_name(var_name)?.0;
        let ref_mut_var: &mut Variable = &mut self.vars[var_index];
        ref_mut_var.rename_attr(old_var_attr_name, new_var_attr_name)?;
        Ok(())
    }

    fn find_var_attr_from_name(&self, var_name: &str, var_attr_name: &str) -> Result<((usize, &Variable), (usize, &Attribute)), InvalidDataSet> {
        // Check that the variable is defined
        let (var_index, ref_var): (usize, &Variable) = self.find_var_from_name(var_name)?;
        let (var_attr_index, ref_var_attr): (usize, &Attribute) = ref_var.find_attr_from_name(var_attr_name)?;

        Ok(((var_index, ref_var), (var_attr_index, ref_var_attr)))
    }

    fn find_global_attr_from_name(&self, attr_name: &str) -> Result<(usize, &Attribute), InvalidDataSet> {
        self.attrs
            .iter()
            .position(|ref_attr: &Attribute| ref_attr.name == attr_name)
            .map(|attr_index: usize| (attr_index, &self.attrs[attr_index]))
            .ok_or(InvalidDataSet::GlobalAttributeNotDefined(attr_name.to_string()))
    }

    /// Returns a reference of a named global attribute.
    pub fn get_global_attr(&self, attr_name: &str) -> Option<&Attribute> {
        self.find_global_attr_from_name(attr_name)
            .ok()
            .map(|(_attr_index, ref_attr)| ref_attr)
    }

    /// Returns a references of all global attributes.
    pub fn get_global_attrs(&self) -> Vec<&Attribute> {
        self.attrs.iter().collect()
    }

    /// Returns the number of global attributes.
    pub fn num_global_attrs(&self) -> usize {
        self.attrs.len()
    }

    pub fn has_global_attr(&self, attr_name:&str) -> bool {
        self.find_global_attr_from_name(attr_name).is_ok()
    }

    /// Returns the number of global attributes.
    pub fn get_global_attr_names(&self) -> Vec<String> {
        self.attrs.iter().map(|attr: &Attribute| {
            attr.name().to_string()
        }).collect()
    }

    /// Adds a global `i8` type attribute in the data set.
    pub fn add_global_attr_i8(&mut self, attr_name: &str, attr_data: Vec<i8>) -> Result<(), InvalidDataSet> {
        if self.find_global_attr_from_name(attr_name).is_ok() {
            return Err(InvalidDataSet::GlobalAttributeAlreadyExists(attr_name.to_string()));
        }
        let _ = Attribute::check_attr_name(attr_name)
            .map_err(|invalid_attr_name: String| InvalidDataSet::GlobalAttributeNameNotValid(invalid_attr_name))?;
        self.attrs.push(Attribute {
            name: attr_name.to_string(),
            data: DataVector::I8(attr_data),
        });
        Ok(())
    }

    /// Adds a global `u8` type attribute in the data set.
    pub fn add_global_attr_u8(&mut self, attr_name: &str, attr_data: Vec<u8>) -> Result<(), InvalidDataSet> {
        if self.find_global_attr_from_name(attr_name).is_ok() {
            return Err(InvalidDataSet::GlobalAttributeAlreadyExists(attr_name.to_string()));
        }
        let _ = Attribute::check_attr_name(attr_name)
            .map_err(|invalid_attr_name: String| InvalidDataSet::GlobalAttributeNameNotValid(invalid_attr_name))?;
        self.attrs.push(Attribute {
            name: attr_name.to_string(),
            data: DataVector::U8(attr_data),
        });
        Ok(())
    }

    /// Adds a global `i16` type attribute in the data set.
    pub fn add_global_attr_i16(&mut self, attr_name: &str, attr_data: Vec<i16>) -> Result<(), InvalidDataSet> {
        if self.find_global_attr_from_name(attr_name).is_ok() {
            return Err(InvalidDataSet::GlobalAttributeAlreadyExists(attr_name.to_string()));
        }
        let _ = Attribute::check_attr_name(attr_name)
            .map_err(|invalid_attr_name: String| InvalidDataSet::GlobalAttributeNameNotValid(invalid_attr_name))?;
        self.attrs.push(Attribute {
            name: attr_name.to_string(),
            data: DataVector::I16(attr_data),
        });
        Ok(())
    }

    /// Adds a global `i32` type attribute in the data set.
    pub fn add_global_attr_i32(&mut self, attr_name: &str, attr_data: Vec<i32>) -> Result<(), InvalidDataSet> {
        if self.find_global_attr_from_name(attr_name).is_ok() {
            return Err(InvalidDataSet::GlobalAttributeAlreadyExists(attr_name.to_string()));
        }
        let _ = Attribute::check_attr_name(attr_name)
            .map_err(|invalid_attr_name: String| InvalidDataSet::GlobalAttributeNameNotValid(invalid_attr_name))?;
        self.attrs.push(Attribute {
            name: attr_name.to_string(),
            data: DataVector::I32(attr_data),
        });
        Ok(())
    }

    /// Adds a global `f32` type attribute in the data set.
    pub fn add_global_attr_f32(&mut self, attr_name: &str, attr_data: Vec<f32>) -> Result<(), InvalidDataSet> {
        if self.find_global_attr_from_name(attr_name).is_ok() {
            return Err(InvalidDataSet::GlobalAttributeAlreadyExists(attr_name.to_string()));
        }
        let _ = Attribute::check_attr_name(attr_name)
            .map_err(|invalid_attr_name: String| InvalidDataSet::GlobalAttributeNameNotValid(invalid_attr_name))?;
        self.attrs.push(Attribute {
            name: attr_name.to_string(),
            data: DataVector::F32(attr_data),
        });
        Ok(())
    }

    /// Add a global `f64` type attribute in the data set.
    pub fn add_global_attr_f64(&mut self, attr_name: &str, attr_data: Vec<f64>) -> Result<(), InvalidDataSet> {
        if self.find_global_attr_from_name(attr_name).is_ok() {
            return Err(InvalidDataSet::GlobalAttributeAlreadyExists(attr_name.to_string()));
        }
        let _ = Attribute::check_attr_name(attr_name)
            .map_err(|invalid_attr_name: String| InvalidDataSet::GlobalAttributeNameNotValid(invalid_attr_name))?;
        self.attrs.push(Attribute {
            name: attr_name.to_string(),
            data: DataVector::F64(attr_data),
        });
        Ok(())
    }

    pub fn rename_global_attr(&mut self, old_attr_name: &str, new_attr_name: &str) -> Result<(), InvalidDataSet> {
        // Check that both names are different
        if old_attr_name == new_attr_name {
            // nothing to do
        }

        // Check that the `old_attr_name` attribute has been defined
        let renamed_attr_index = self.find_global_attr_from_name(old_attr_name)?.0;

        // Check that the `new_attr_name` attribute has not already benn defined
        if self.find_global_attr_from_name(new_attr_name).is_ok() {
            return Err(InvalidDataSet::GlobalAttributeAlreadyExists(new_attr_name.to_string()));
        }

        // Check that the new name is a NetCDF-3 valid name
        let _ = Attribute::check_attr_name(new_attr_name)
            .map_err(|invalid_attr_name: String| InvalidDataSet::GlobalAttributeNameNotValid(invalid_attr_name))?;

        // Update the attribute name
        self.attrs[renamed_attr_index].name = new_attr_name.to_string();

        Ok(())
    }

    pub fn remove_global_attr(&mut self, attr_name: &str) -> Result<Attribute, InvalidDataSet> {
        // Check that the `attr_name` attribute has been defined
        let removed_attr_index = self.find_global_attr_from_name(attr_name)?.0;

        Ok(self.attrs.remove(removed_attr_index))
    }

    /// Returns a **reference** of the value the global attribute.
    ///
    /// Returns `None` if:
    /// - the global attribute has not been defined in the dateset
    /// - the global attribute is not a `i8` type attribute
    pub fn get_ref_global_attr_i8(&self, attr_name: &str) -> Option<&Vec<i8>> {
        let ref_attr: Option<(usize, &Attribute)> = self.find_global_attr_from_name(attr_name).ok();
        match ref_attr {
            Some((_, ref_attr)) => {
                use DataVector::*;
                match &ref_attr.data {
                    I8(attr_data) => Some(&attr_data),
                    // if the global attribute is not a `i8` type attribute
                    _ => None,
                }
            }
            // if the named global attribute has not been defined
            None => None,
        }
    }

    /// Returns a **copy** of the value the global attribute.
    ///
    /// Returns `None` if:
    /// - the global attribute has not been defined in the dateset
    /// - the global attribute is not a `i8` type attribute
    pub fn get_global_attr_i8(&self, attr_name: &str) -> Option<Vec<i8>> {
        self.get_ref_global_attr_i8(attr_name).map(|ref_attr: &Vec<i8>|{
            ref_attr.clone()
        })
    }

    /// Returns a **reference** of the value the global attribute.
    ///
    /// Returns `None` if:
    /// - the global attribute has not been defined in the dateset
    /// - the global attribute is not a `u8` type attribute
    pub fn get_ref_global_attr_u8(&self, attr_name: &str) -> Option<&Vec<u8>> {
        let ref_attr: Option<(usize, &Attribute)> = self.find_global_attr_from_name(attr_name).ok();
        match ref_attr {
            Some((_, ref_attr)) => {
                use DataVector::*;
                match &ref_attr.data {
                    U8(attr_data) => Some(&attr_data),
                    // if the global attribute is not a `u8` type attribute
                    _ => None,
                }
            }
            // if the named global attribute has not been defined
            None => None,
        }
    }

    /// Returns a **copy** of the value the global attribute.
    ///
    /// Returns `None` if:
    /// - the global attribute has not been defined in the dateset
    /// - the global attribute is not a `u8` type attribute
    pub fn get_global_attr_u8(&self, attr_name: &str) -> Option<Vec<u8>> {
        self.get_ref_global_attr_u8(attr_name).map(|ref_attr: &Vec<u8>|{
            ref_attr.clone()
        })
    }

    /// Returns a **reference** of the value the global attribute.
    ///
    /// Returns `None` if:
    /// - the global attribute has not been defined in the dateset
    /// - the global attribute is not a `i16` type attribute
    pub fn get_ref_global_attr_i16(&self, attr_name: &str) -> Option<&Vec<i16>> {
        let ref_attr: Option<(usize, &Attribute)> = self.find_global_attr_from_name(attr_name).ok();
        match ref_attr {
            Some((_, ref_attr)) => {
                use DataVector::*;
                match &ref_attr.data {
                    I16(attr_data) => Some(&attr_data),
                    // if the global attribute is not a `u8` type attribute
                    _ => None,
                }
            }
            // if the named global attribute has not been defined
            None => None,
        }
    }

    /// Returns a **copy** of the value the global attribute.
    ///
    /// Returns `None` if:
    /// - the global attribute has not been defined in the dateset
    /// - the global attribute is not a `i16` type attribute
    pub fn get_global_attr_i16(&self, attr_name: &str) -> Option<Vec<i16>> {
        self.get_ref_global_attr_i16(attr_name).map(|ref_attr: &Vec<i16>|{
            ref_attr.clone()
        })
    }

    /// Returns a **reference** of the value the global attribute.
    ///
    /// Returns `None` if:
    /// - the global attribute has not been defined in the dateset
    /// - the global attribute is not a `i32` type attribute
    pub fn get_ref_global_attr_i32(&self, attr_name: &str) -> Option<&Vec<i32>> {
        let ref_attr: Option<(usize, &Attribute)> = self.find_global_attr_from_name(attr_name).ok();
        match ref_attr {
            Some((_, ref_attr)) => {
                use DataVector::*;
                match &ref_attr.data {
                    I32(attr_data) => Some(&attr_data),
                    // if the global attribute is not a `u8` type attribute
                    _ => None,
                }
            }
            // if the named global attribute has not been defined
            None => None,
        }
    }

    /// Returns a **copy** of the value the global attribute.
    ///
    /// Returns `None` if:
    /// - the global attribute has not been defined in the dateset
    /// - the global attribute is not a `i32` type attribute
    pub fn get_global_attr_i32(&self, attr_name: &str) -> Option<Vec<i32>> {
        self.get_ref_global_attr_i32(attr_name).map(|ref_attr: &Vec<i32>|{
            ref_attr.clone()
        })
    }

    /// Returns a **reference** of the value the global attribute.
    ///
    /// Returns `None` if:
    /// - the global attribute has not been defined in the dateset
    /// - the global attribute is not a `f32` type attribute
    pub fn get_ref_global_attr_f32(&self, attr_name: &str) -> Option<&Vec<f32>> {
        let ref_attr: Option<(usize, &Attribute)> = self.find_global_attr_from_name(attr_name).ok();
        match ref_attr {
            Some((_, ref_attr)) => {
                use DataVector::*;
                match &ref_attr.data {
                    F32(attr_data) => Some(&attr_data),
                    // if the global attribute is not a `u8` type attribute
                    _ => None,
                }
            }
            // if the named global attribute has not been defined
            None => None,
        }
    }

    /// Returns a **copy** of the value the global attribute.
    ///
    /// Returns `None` if:
    /// - the global attribute has not been defined in the dateset
    /// - the global attribute is not a `f32` type attribute
    pub fn get_global_attr_f32(&self, attr_name: &str) -> Option<Vec<f32>> {
        self.get_ref_global_attr_f32(attr_name).map(|ref_attr: &Vec<f32>|{
            ref_attr.clone()
        })
    }

    /// Returns a **reference** of the value the global attribute.
    ///
    /// Returns `None` if:
    /// - the global attribute has not been defined in the dateset
    /// - the global attribute is not a `f64` type attribute
    pub fn get_ref_global_attr_f64(&self, attr_name: &str) -> Option<&Vec<f64>> {
        let ref_attr: Option<(usize, &Attribute)> = self.find_global_attr_from_name(attr_name).ok();
        match ref_attr {
            Some((_, ref_attr)) => {
                use DataVector::*;
                match &ref_attr.data {
                    F64(attr_data) => Some(&attr_data),
                    // if the global attribute is not a `u8` type attribute
                    _ => None,
                }
            }
            // if the named global attribute has not been defined
            None => None,
        }
    }

    /// Returns a **copy** of the value the global attribute.
    ///
    /// Returns `None` if:
    /// - the global attribute has not been defined in the dateset
    /// - the global attribute is not a `f64` type attribute
    pub fn get_global_attr_f64(&self, attr_name: &str) -> Option<Vec<f64>> {
        self.get_ref_global_attr_f64(attr_name).map(|ref_attr: &Vec<f64>|{
            ref_attr.clone()
        })
    }

    // Returns the size (number of bytes) required each record stored in the data file.
    pub fn record_size(&self) -> usize
    {
        return self.vars.iter()
            .filter(|var| {  // keep only the record-variables
                var.is_record_var()
            }).map(|var| {
                var.chunk_size()
            }).fold(0, |sum, chunk_size| {
                sum + chunk_size
            });
    }

    // Returns the number records stored in data file.
    pub fn num_records(&self) -> usize {
        match &self.unlimited_dim {
            None => 0,
            Some(dim) => dim.size()
        }
    }

}


