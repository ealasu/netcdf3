use std::collections::HashSet;
use std::iter::FromIterator;
use std::rc::Rc;

use crate::{name_string::is_valid_name, typed_data_vector::DataVector};
use crate::{Attribute, DataType, Dimension, error::InvalidDataSet};
use crate::io::compute_zero_padding_size;

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
        Variable::check_dims_validity(&var_dims)?;

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
    /// If the variable is a record variable then `len = num_chunks * chunks_size`.
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
            chunk_size += compute_zero_padding_size(chunk_size);
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
            return Err(InvalidDataSet::VariableAttributeAlreadyExists(
                self.name.to_string(),
                new_attr.name.to_string(),
            ));
        }
        // append the new attribute
        self.attrs.push(new_attr);
        return Ok(());
    }

    /// Append a new `i8` attribute.
    ///
    /// An error is returned if an other attribute with the same name has already been added.
    pub(in crate::data_set) fn add_attr_i8(&mut self, attr_name: &str, i8_data: Vec<i8>) -> Result<(), InvalidDataSet> {
        let attr: Attribute = Attribute::new_i8_attr(attr_name, i8_data)
            .map_err(|var_attr_name: String| InvalidDataSet::VariableAttributeNameNotValid(self.name.to_string(), var_attr_name))?;
        self.add_attr(attr)?;
        Ok(())
    }

    /// Append a new `u8` attribute.
    ///
    /// An error is returned if an other attribute with the same name has already been added.
    pub(in crate::data_set) fn add_attr_u8(&mut self, attr_name: &str, u8_data: Vec<u8>) -> Result<(), InvalidDataSet> {
        let attr: Attribute = Attribute::new_u8_attr(attr_name, u8_data)
            .map_err(|var_attr_name: String| InvalidDataSet::VariableAttributeNameNotValid(self.name.to_string(), var_attr_name))?;
        self.add_attr(attr)?;
        Ok(())
    }

    /// Append a new `i16` attribute.
    ///
    /// An error is returned if an other attribute with the same name has already been added.
    pub(in crate::data_set) fn add_attr_i16(&mut self, attr_name: &str, i16_data: Vec<i16>) -> Result<(), InvalidDataSet> {
        let attr: Attribute = Attribute::new_i16_attr(attr_name, i16_data)
            .map_err(|var_attr_name: String| InvalidDataSet::VariableAttributeNameNotValid(self.name.to_string(), var_attr_name))?;
        self.add_attr(attr)?;
        Ok(())
    }

    /// Append a new `i32` attribute.
    ///
    /// An error is returned if an other attribute with the same name has already been added.
    pub(in crate::data_set) fn add_attr_i32(&mut self, attr_name: &str, i32_data: Vec<i32>) -> Result<(), InvalidDataSet> {
        let attr: Attribute = Attribute::new_i32_attr(attr_name, i32_data)
            .map_err(|var_attr_name: String| InvalidDataSet::VariableAttributeNameNotValid(self.name.to_string(), var_attr_name))?;
        self.add_attr(attr)?;
        Ok(())
    }

    /// Append a new `f32` attribute.
    ///
    /// An error is returned if an other attribute with the same name has already been added.
    pub(in crate::data_set) fn add_attr_f32(&mut self, attr_name: &str, f32_data: Vec<f32>) -> Result<(), InvalidDataSet> {
        let attr: Attribute = Attribute::new_f32_attr(attr_name, f32_data)
            .map_err(|var_attr_name: String| InvalidDataSet::VariableAttributeNameNotValid(self.name.to_string(), var_attr_name))?;
        self.add_attr(attr)?;
        Ok(())
    }

    /// Append a new `f64` attribute.
    ///
    /// An error is returned if an other attribute with the same name has already been added.
    pub(in crate::data_set) fn add_attr_f64(&mut self, attr_name: &str, f64_data: Vec<f64>) -> Result<(), InvalidDataSet> {
        let attr: Attribute = Attribute::new_f64_attr(attr_name, f64_data)
            .map_err(|var_attr_name: String| InvalidDataSet::VariableAttributeNameNotValid(self.name.to_string(), var_attr_name))?;
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
            return Err(InvalidDataSet::VariableAttributeAlreadyExists(
                self.name.to_string(),
                new_attr_name.to_string(),
            ));
        }

        // Check that `new_attr_name`is a valid NetCDF-3 name
        let _ = Attribute::check_attr_name(new_attr_name)
            .map_err(|var_attr_name: String| InvalidDataSet::VariableAttributeNameNotValid(self.name.to_string(), var_attr_name.to_string()))?;
        let renamed_attr: &mut Attribute = &mut self.attrs[renamed_attr_index];
        renamed_attr.name = new_attr_name.to_string();
        return Ok(());
    }

    // /// Rename an existing attribute.
    // ///
    // /// An error is returned the `old_name` attribute doesn't exist.
    // pub(in crate::data_set) fn remove_attr(&mut self, attr_name: &str) -> Result<Attribute, InvalidDataSet> {
    //     // Check if an other same name attribute already exists.
    //     let removed_attr_index: usize = self.find_attr_from_name(attr_name)?.0;
    //     // Then remove the attribute
    //     let removed_attr: Attribute = self.attrs.remove(removed_attr_index);
    //     return Ok(removed_attr);
    // }

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
            .ok_or(InvalidDataSet::VariableAttributeNotDefined(self.name.to_string(), attr_name.to_string()))
    }

    pub(super) fn check_var_name(var_name: &str) -> Result<(), InvalidDataSet> {
        return match is_valid_name(var_name) {
            true => Ok(()),
            false => Err(InvalidDataSet::VariableNameNotValid(var_name.to_string())),
        };
    }

    fn check_dims_validity(dims: &Vec<Rc<Dimension>>) -> Result<(), InvalidDataSet> {
        if dims.is_empty() {
            return Ok(());
        }
        // Check that the optional unlimited dimension is defined at first
        if let Some(ref_unlim_dim) = dims.iter().skip(1).find(|ref_dim: &&Rc<Dimension>| ref_dim.is_unlimited()) {
            return Err(InvalidDataSet::UnlimitedDimensionMustBeDefinedFirst(ref_unlim_dim.name.borrow().to_string()));
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
            let repeated_dim_names = Vec::<String>::from_iter(repeated_dim_names);
            return Err(InvalidDataSet::DimensionsUsedMultipleTimes(repeated_dim_names));
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
    /// use netcdf3::{DataSet, Variable, DataType, Version};
    /// 
    /// // Initialize a data set
    /// let data_set: DataSet = {
    ///     let mut data_set: DataSet = DataSet::new(Version::Classic);
    ///     let _ = data_set.add_fixed_dim("dim_1", 1).unwrap();
    ///
    ///     // Data of `var_0` are not initialized
    ///     let _ = data_set.add_var("var_0", &vec!["dim_1"], DataType::I8).unwrap();
    /// 
    ///     // Data of `var_1` are initialized
    ///     let _ = data_set.add_var("var_1", &vec!["dim_1"], DataType::I8).unwrap();
    ///     data_set.set_var_data_i8("var_1", vec![42_i8]).unwrap();
    ///
    ///     data_set
    /// };
    ///
    /// // Check the returned data for the variable `var_0`
    /// let var_0: &Variable = data_set.get_var("var_0").unwrap();
    /// assert_eq!(DataType::I8, var_0.data_type());
    ///
    /// assert!(var_0.get_i8().is_none());
    /// assert!(var_0.get_u8().is_none());
    /// assert!(var_0.get_i16().is_none());
    /// assert!(var_0.get_i32().is_none());
    /// assert!(var_0.get_f32().is_none());
    /// assert!(var_0.get_f64().is_none());
    /// 
    /// // Check the returned data for the variable `var_1`
    /// let var_1: &Variable = data_set.get_var("var_1").unwrap();
    /// assert_eq!(DataType::I8, var_1.data_type());
    ///
    /// assert!(var_1.get_i8().is_some());
    /// assert!(var_1.get_u8().is_none());
    /// assert!(var_1.get_i16().is_none());
    /// assert!(var_1.get_i32().is_none());
    /// assert!(var_1.get_f32().is_none());
    /// assert!(var_1.get_f64().is_none());
    /// ```
    pub fn get_i8(&self) -> Option<&Vec<i8>>
    {
        match self.data {
            Some(DataVector::I8(ref data)) => Some(data),
            _ => None,
        }
    }

    /// Returns a reference to the `u8` data vector (refer to the method [get_i8](struct.Variable.html#method.get_i8)).
    pub fn get_u8(&self) -> Option<&Vec<u8>>
    {
        match self.data {
            Some(DataVector::U8(ref data)) => Some(data),
            _ => None,
        }
    }

    /// Returns a reference to the `i16` data vector (refer to the method [get_i8](struct.Variable.html#method.get_i8)).
    pub fn get_i16(&self) -> Option<&Vec<i16>>
    {
        match self.data {
            Some(DataVector::I16(ref data)) => Some(data),
            _ => None,
        }
    }

    /// Returns a reference to the `i32` data vector (refer to the method [get_i8](struct.Variable.html#method.get_i8)).
    pub fn get_i32(&self) -> Option<&Vec<i32>>
    {
        match self.data {
            Some(DataVector::I32(ref data)) => Some(data),
            _ => None,
        }
    }

    /// Returns a reference to the `f32` data vector (refer to the method [get_i8](struct.Variable.html#method.get_i8)).
    pub fn get_f32(&self) -> Option<&Vec<f32>>
    {
        match self.data {
            Some(DataVector::F32(ref data)) => Some(data),
            _ => None,
        }
    }

    /// Returns a reference to the `f64` data vector (refer to the method [get_i8](struct.Variable.html#method.get_i8)).
    pub fn get_f64(&self) -> Option<&Vec<f64>>
    {
        match self.data {
            Some(DataVector::F64(ref data)) => Some(data),
            _ => None,
        }
    }

}
