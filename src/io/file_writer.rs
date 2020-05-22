mod tests_file_writer;

use std::io::{Write, Seek, SeekFrom};
use std::rc::Rc;
use std::path::{Path, PathBuf};
use std::convert::TryFrom;

use crate::{DataSet, Version, Dimension, Attribute, DataType, Variable};
use crate::io::Offset;
use crate::data_set::DimensionSize;
use crate::data_vector::DataVector;
use crate::error::WriteError;

use crate::io::{
    ABSENT_TAG, DIMENSION_TAG, VARIABLE_TAG, ATTRIBUTE_TAG,
    compute_padding_size,
};

use crate::{
    NC_FILL_I8,
    NC_FILL_U8,
    NC_FILL_I16,
    NC_FILL_I32,
    NC_FILL_F32,
    NC_FILL_F64,
};

/// Allows to write NetCDF-3 files (the *classic* and the *64-bit offset* versions).
///
/// # Example
///
/// ```
/// use std::path::PathBuf;
/// use std::io::Read;
/// use copy_to_tmp_file::NC3_BASIC_CLASSIC_FILE_BYTES;
/// # use tempdir::TempDir;
///
/// use netcdf3::{FileWriter, DataSet, Version};
/// const LATITUDE_DIM_NAME: &str = "latitude";
/// const LATITUDE_VAR_NAME: &str = LATITUDE_DIM_NAME;
/// const LATITUDE_VAR_DATA: [f32; 3] = [0.0, 1.0, 2.0];
/// const LATITUDE_VAR_LEN: usize = LATITUDE_VAR_DATA.len();
///
/// const LONGITUDE_DIM_NAME: &str = "longitude";
/// const LONGITUDE_VAR_NAME: &str = LONGITUDE_DIM_NAME;
/// const LONGITUDE_VAR_DATA: [f32; 4] = [0.0, 1.0, 2.0, 3.0];
/// const LONGITUDE_VAR_LEN: usize = LONGITUDE_VAR_DATA.len();
///
/// const TEMPERATURE_VAR_NAME: &str = "temperature";
/// const TEMPERATURE_DATA: [f64; 12] = [0., 1., 2., 3., 4., 5., 6., 7., 8., 9., 10., 11.];
/// const TEMPERATURE_VAR_LEN: usize = TEMPERATURE_DATA.len();
///
/// // Create the NetCDF-3 definition
/// // ------------------------------
/// let data_set: DataSet = {
///     let mut data_set: DataSet = DataSet::new();
///     // Define the dimensions
///     data_set.add_fixed_dim(LATITUDE_DIM_NAME, LATITUDE_VAR_LEN).unwrap();
///     data_set.add_fixed_dim(LONGITUDE_DIM_NAME, LONGITUDE_VAR_LEN).unwrap();
///     // Define the variable
///     data_set.add_var_f32(LATITUDE_VAR_NAME, &[LATITUDE_DIM_NAME]).unwrap();
///     data_set.add_var_f32(LONGITUDE_VAR_NAME, &[LONGITUDE_VAR_NAME]).unwrap();
///     data_set.add_var_f64(TEMPERATURE_VAR_NAME, &[LATITUDE_DIM_NAME, LONGITUDE_VAR_NAME]).unwrap();
///
///     data_set
/// };
///
/// // ...
/// # let (tmp_dir, output_file_path): (TempDir, PathBuf) = {
/// #     const TMP_DIR_PREFIX: &str = "tests_netcdf_3";
/// #     const OUTPUT_FILE_NAME: &str = "example.nc";
/// #     let mut tmp_dir: TempDir = TempDir::new(TMP_DIR_PREFIX).unwrap();
/// #     let output_file_path = tmp_dir.path().join(OUTPUT_FILE_NAME);
/// #     (tmp_dir, output_file_path)
/// # };
///
/// // Create and write the NetCDF-3 file
/// // ----------------------------------
/// assert_eq!(false,                           output_file_path.exists());
/// let mut file_writer: FileWriter = FileWriter::create_new(&output_file_path).unwrap();
/// // Set the NetCDF-3 definition
/// file_writer.set_def(&data_set, Version::Classic, 0).unwrap();
/// assert_eq!(TEMPERATURE_VAR_LEN,             LATITUDE_VAR_LEN * LONGITUDE_VAR_LEN);
/// file_writer.write_var_f32(LATITUDE_VAR_NAME, &LATITUDE_VAR_DATA[..]).unwrap();
/// file_writer.write_var_f32(LONGITUDE_VAR_NAME, &LONGITUDE_VAR_DATA[..]).unwrap();
/// file_writer.write_var_f64(TEMPERATURE_VAR_NAME, &TEMPERATURE_DATA[..]).unwrap();
/// file_writer.close().unwrap();
/// assert_eq!(true,                            output_file_path.exists());
///
/// // Binary comparaison with the "same" NeTCDF-3 file created with the [Python library netCDF4](https://github.com/Unidata/netcdf4-python).
/// // -------------------------------------------------------------------------------------------------------------------------------------------------------
/// let nc3_file_bytes: Vec<u8> = {
///     let mut written_bytes: Vec<u8> = vec![];
///     let mut written_file: std::fs::File = std::fs::File::open(&output_file_path).unwrap();
///     written_file.read_to_end(&mut written_bytes).unwrap();
///     written_bytes
/// };
/// # tmp_dir.close().unwrap();
/// assert_eq!(NC3_BASIC_CLASSIC_FILE_BYTES.len(),   nc3_file_bytes.len());
/// assert_eq!(NC3_BASIC_CLASSIC_FILE_BYTES,         &nc3_file_bytes[..]);
/// ```
#[derive(Debug)]
pub struct FileWriter<'a> {
    output_file_path: PathBuf,
    output_file: std::fs::File,
    header_def: Option<HeaderDefinition<'a>>,
    written_vars: Vec<&'a Variable>,
}

impl<'a> FileWriter<'a> {

    /// Opens and overwrites an existing NetCDF-3 file or creates one.
     pub fn open<P: std::convert::AsRef<Path>>(output_file_path: P) -> Result<FileWriter<'a>, WriteError> {
        let output_file_path: PathBuf = {
            let mut path = PathBuf::new();
            path.push(output_file_path);
            path
        };
        let output_file: std::fs::File = std::fs::OpenOptions::new()
            .read(false)
            .write(true)
            .create(true)
            .create_new(false)
            .truncate(true)
            .append(false)
            .open(output_file_path.clone())?;
        Ok(FileWriter{
            output_file: output_file,
            output_file_path: output_file_path,
            header_def: None,
            written_vars: vec![],
        })
    }

    /// Creates a new NetCDF-3 file.
    ///
    /// # Error
    ///
    /// An error occures if the NetCDF-3 file already exists.
    pub fn create_new<P: std::convert::AsRef<Path>>(output_file_path: P) -> Result<FileWriter<'a>, WriteError> {
        let output_file_path: PathBuf = {
            let mut path = PathBuf::new();
            path.push(output_file_path);
            path
        };
        let output_file: std::fs::File = std::fs::OpenOptions::new()
            .read(false)
            .write(true)
            .create_new(true)
            .open(output_file_path.clone())?;
        Ok(FileWriter{
            output_file: output_file,
            output_file_path: output_file_path,
            header_def: None,
            written_vars: vec![],
        })
    }

    pub fn file_path(&self) -> &Path {
        return &self.output_file_path;
    }

    /// Set the NetCDF-3 definition.
    ///
    /// # Arguments
    ///
    /// - `data_set`: the NetCDF-3 defintion set (also see [`DataSet`](struct.DataSet.html)).
    /// - `version`: the NetCDF-3 version (also see [`Version`](enum.Version.html)).
    /// - `header_min_size`: the mininum number of bytes reserved for header of the NetCDF-3 file.
    ///
    /// # Example
    ///
    /// ```
    /// use std::path::PathBuf;
    /// use netcdf3::{FileWriter, DataSet, Version};
    /// use tempdir::TempDir;
    ///
    /// const TMP_DIR_PREFIX: &str = "netcdf3_tests_";
    /// const FILE_NAME_1: &str = "empty_data_set_1.nc";
    /// const FILE_NAME_2: &str = "empty_data_set_2.nc";
    /// const HEADER_MIN_SIZE_1: usize = 0;
    /// const HEADER_MIN_SIZE_2: usize = 1024;
    ///
    /// let tmp_dir: TempDir = TempDir::new(TMP_DIR_PREFIX).unwrap();
    ///
    /// // Create 2 NetCDF-3 files containing empty data sets but with different `header_min_size`
    /// // ---------------------------------------------------------------------------------------
    /// let empty_data_set: DataSet = DataSet::new();
    /// let file_path_1: PathBuf = tmp_dir.path().join(FILE_NAME_1);
    /// {
    ///     let mut file_writer_1: FileWriter = FileWriter::create_new(&file_path_1).unwrap();
    ///     file_writer_1.set_def(&empty_data_set, Version::Classic, HEADER_MIN_SIZE_1).unwrap();
    ///     file_writer_1.close();
    /// }
    /// let file_path_2: PathBuf = tmp_dir.path().join(FILE_NAME_2);
    /// {
    ///     let mut file_writer_1: FileWriter = FileWriter::create_new(&file_path_2).unwrap();
    ///     file_writer_1.set_def(&empty_data_set, Version::Classic, HEADER_MIN_SIZE_2).unwrap();
    ///     file_writer_1.close();
    /// }
    ///
    /// // Compare the size beetween the 2 NetCDF-3 files
    /// // ----------------------------------------------
    /// assert_eq!(32,                  std::fs::metadata(&file_path_1).unwrap().len());
    /// assert_eq!(1024,                std::fs::metadata(&file_path_2).unwrap().len());
    /// ```
    pub fn set_def(&mut self, data_set: &'a DataSet, version: Version, header_min_size: usize) -> Result<(), WriteError> {
        match &self.header_def {
            Some(_) => return Err(WriteError::HeaderAlreadyDefined),
            None => self.header_def = Some(HeaderDefinition::new(data_set, version, header_min_size)?),
        }
        let _ = self.write_header()?;
        Ok(())
    }

    pub fn header_is_defined(&self) -> bool {
        return self.header_def.is_some();
    }

    pub fn data_set(&self) -> Option<&'a DataSet> {
        return self.header_def.as_ref().map(|header_def| header_def.data_set);
    }

    pub fn version(&self) -> Option<Version> {
        return self.header_def.as_ref().map(|header_def| header_def.version.clone());
    }

    pub fn header_min_size(&self) -> Option<usize> {
        return self.header_def.as_ref().map(|header_def| header_def.header_min_size);
    }


    /// Fills the unwritten data, and closes the NetCDF-3 file.
    pub fn close(mut self) -> Result<(), WriteError>
    {
        let header_def: &HeaderDefinition = match self.header_def {
            None => return Ok(()),
            Some(ref header_def) => header_def,
        };
        let not_written_vars: Vec<&'a Variable> = header_def.data_set.vars.iter().filter(|var: &&'a Variable| !self.written_vars.contains(var)).collect();
        let record_size: usize = header_def.data_set.record_size().unwrap_or(0);
        for var in not_written_vars.into_iter() {
            let num_chunks: usize = var.num_chunks();
            let chunk_len: usize = var.chunk_len();
            let var_metadata: &ComputedVariableMetadata = header_def.get_var_metadata(var)?;
            let begin_offset: usize = i64::from(var_metadata.begin_offset.clone()) as usize;
            for i in 0..num_chunks {
                let position: usize = begin_offset + (i * record_size);
                self.output_file.seek(SeekFrom::Start(position as u64))?;
                let _num_bytes: usize = match var.data_type() {
                    DataType::I8 => FileWriter::write_chunk_nc_fill_i8(&mut self.output_file, chunk_len),
                    DataType::U8 => FileWriter::write_chunk_nc_fill_u8(&mut self.output_file, chunk_len),
                    DataType::I16 => FileWriter::write_chunk_nc_fill_i16(&mut self.output_file, chunk_len),
                    DataType::I32 => FileWriter::write_chunk_nc_fill_i32(&mut self.output_file, chunk_len),
                    DataType::F32 => FileWriter::write_chunk_nc_fill_f32(&mut self.output_file, chunk_len),
                    DataType::F64 => FileWriter::write_chunk_nc_fill_f64(&mut self.output_file, chunk_len),
                }?;
            }
        }
        Ok(())
    }

    pub fn write_var_i8(&mut self, var_name: &str, data: &[i8]) -> Result<(), WriteError> {
        let header_def: &HeaderDefinition = self.header_def.as_ref().ok_or(WriteError::HeaderNotDefined)?;
        let var: &Variable = header_def.data_set.find_var_from_name(var_name).map_err(|_err| WriteError::VariableNotDefined(var_name.to_owned()))?.1;
        if var.data_type != DataType::I8 {
            return Err(WriteError::VariableMismatchDataType{var_name: var_name.to_owned(), req:var.data_type(), get: DataType::I8});
        }
        if var.len() != data.len() {
            return Err(WriteError::VariableMismatchDataLength{var_name: var_name.to_owned(), req:var.len(), get: data.len()});
        }
        let var_metadata: &ComputedVariableMetadata = header_def.get_var_metadata(var)?;

        // Write the `i8` data
        let begin_offset: u64 = i64::from(var_metadata.begin_offset.clone()) as u64;
        match header_def.data_set.record_size() {
            None => {  // fixed-size variable
                self.output_file.seek(SeekFrom::Start(begin_offset))?;
                let _chunk_size: usize = FileWriter::write_chunk_i8(&mut self.output_file, data)?;
            },
            Some(record_size) => {  // record variable
                let num_chunks: usize = var.num_chunks();
                let chunk_len: usize = var.chunk_len();
                // Loop over data chunks
                for i in 0..num_chunks {
                    let start: usize = i * chunk_len;
                    let end: usize = (i + 1) * chunk_len;
                    let chunk_slice: &[i8] = &data[start..end];
                    let position: u64 = begin_offset + ((i * record_size) as u64);
                    self.output_file.seek(SeekFrom::Start(position))?;
                    let _chunk_size: usize = FileWriter::write_chunk_i8(&mut self.output_file, chunk_slice)?;
                }
            }
        }
        self.written_vars.push(var);
        Ok(())
    }

    pub fn write_var_u8(&mut self, var_name: &str, data: &[u8]) -> Result<(), WriteError> {
        let header_def: &HeaderDefinition = self.header_def.as_ref().ok_or(WriteError::HeaderNotDefined)?;
        let var: &Variable = header_def.data_set.find_var_from_name(var_name).map_err(|_err| WriteError::VariableNotDefined(var_name.to_owned()))?.1;
        if var.data_type != DataType::U8 {
            return Err(WriteError::VariableMismatchDataType{var_name: var_name.to_owned(), req:var.data_type(), get: DataType::U8});
        }
        if var.len() != data.len() {
            return Err(WriteError::VariableMismatchDataLength{var_name: var_name.to_owned(), req:var.len(), get: data.len()});
        }
        let var_metadata: &ComputedVariableMetadata = header_def.get_var_metadata(var)?;

        // Write the `u8` data
        let begin_offset: u64 = i64::from(var_metadata.begin_offset.clone()) as u64;
        match header_def.data_set.record_size() {
            None => {  // fixed-size variable
                self.output_file.seek(SeekFrom::Start(begin_offset))?;
                let _chunk_size: usize = FileWriter::write_chunk_u8(&mut self.output_file, data)?;
            },
            Some(record_size) => {  // record variable
                let num_chunks: usize = var.num_chunks();
                let chunk_len: usize = var.chunk_len();
                // Loop over data chunks
                for i in 0..num_chunks {
                    let start: usize = i * chunk_len;
                    let end: usize = (i + 1) * chunk_len;
                    let chunk_slice: &[u8] = &data[start..end];
                    let position: u64 = begin_offset + ((i * record_size) as u64);
                    self.output_file.seek(SeekFrom::Start(position))?;
                    let _chunk_size: usize = FileWriter::write_chunk_u8(&mut self.output_file, chunk_slice)?;
                }
            }
        }
        self.written_vars.push(var);
        Ok(())
    }

    pub fn write_var_i16(&mut self, var_name: &str, data: &[i16]) -> Result<(), WriteError> {
        let header_def: &HeaderDefinition = self.header_def.as_ref().ok_or(WriteError::HeaderNotDefined)?;
        let var: &Variable = header_def.data_set.find_var_from_name(var_name).map_err(|_err| WriteError::VariableNotDefined(var_name.to_owned()))?.1;
        if var.data_type != DataType::I16 {
            return Err(WriteError::VariableMismatchDataType{var_name: var_name.to_owned(), req:var.data_type(), get: DataType::I16});
        }
        if var.len() != data.len() {
            return Err(WriteError::VariableMismatchDataLength{var_name: var_name.to_owned(), req:var.len(), get: data.len()});
        }
        let var_metadata: &ComputedVariableMetadata = header_def.get_var_metadata(var)?;

        // Write the `i16` data
        let begin_offset: u64 = i64::from(var_metadata.begin_offset.clone()) as u64;
        match header_def.data_set.record_size() {
            None => {  // fixed-size variable
                self.output_file.seek(SeekFrom::Start(begin_offset))?;
                let _chunk_size: usize = FileWriter::write_chunk_i16(&mut self.output_file, data)?;
            },
            Some(record_size) => {  // record variable
                let num_chunks: usize = var.num_chunks();
                let chunk_len: usize = var.chunk_len();
                // Loop over data chunks
                for i in 0..num_chunks {
                    let start: usize = i * chunk_len;
                    let end: usize = (i + 1) * chunk_len;
                    let chunk_slice: &[i16] = &data[start..end];
                    let position: u64 = begin_offset + ((i * record_size) as u64);
                    self.output_file.seek(SeekFrom::Start(position))?;
                    let _chunk_size: usize = FileWriter::write_chunk_i16(&mut self.output_file, chunk_slice)?;
                }
            }
        }
        self.written_vars.push(var);
        Ok(())
    }

    pub fn write_var_i32(&mut self, var_name: &str, data: &[i32]) -> Result<(), WriteError> {
        let header_def: &HeaderDefinition = self.header_def.as_ref().ok_or(WriteError::HeaderNotDefined)?;
        let var: &Variable = header_def.data_set.find_var_from_name(var_name).map_err(|_err| WriteError::VariableNotDefined(var_name.to_owned()))?.1;
        if var.data_type != DataType::I32 {
            return Err(WriteError::VariableMismatchDataType{var_name: var_name.to_owned(), req:var.data_type(), get: DataType::I32});
        }
        if var.len() != data.len() {
            return Err(WriteError::VariableMismatchDataLength{var_name: var_name.to_owned(), req:var.len(), get: data.len()});
        }
        let var_metadata: &ComputedVariableMetadata = header_def.get_var_metadata(var)?;

        // Write the `i32` data
        let begin_offset: u64 = i64::from(var_metadata.begin_offset.clone()) as u64;
        match header_def.data_set.record_size() {
            None => {  // fixed-size variable
                self.output_file.seek(SeekFrom::Start(begin_offset))?;
                let _chunk_size: usize = FileWriter::write_chunk_i32(&mut self.output_file, data)?;
            },
            Some(record_size) => {  // record variable
                let num_chunks: usize = var.num_chunks();
                let chunk_len: usize = var.chunk_len();
                // Loop over data chunks
                for i in 0..num_chunks {
                    let start: usize = i * chunk_len;
                    let end: usize = (i + 1) * chunk_len;
                    let chunk_slice: &[i32] = &data[start..end];
                    let position: u64 = begin_offset + ((i * record_size) as u64);
                    self.output_file.seek(SeekFrom::Start(position))?;
                    let _chunk_size: usize = FileWriter::write_chunk_i32(&mut self.output_file, chunk_slice)?;
                }
            }
        }
        self.written_vars.push(var);
        Ok(())
    }

    pub fn write_var_f32(&mut self, var_name: &str, data: &[f32]) -> Result<(), WriteError> {
        let header_def: &HeaderDefinition = self.header_def.as_ref().ok_or(WriteError::HeaderNotDefined)?;
        let var: &Variable = header_def.data_set.find_var_from_name(var_name).map_err(|_err| WriteError::VariableNotDefined(var_name.to_owned()))?.1;
        if var.data_type != DataType::F32 {
            return Err(WriteError::VariableMismatchDataType{var_name: var_name.to_owned(), req:var.data_type(), get: DataType::F32});
        }
        if var.len() != data.len() {
            return Err(WriteError::VariableMismatchDataLength{var_name: var_name.to_owned(), req:var.len(), get: data.len()});
        }
        let var_metadata: &ComputedVariableMetadata = header_def.get_var_metadata(var)?;

        // Write the `f32` data
        let begin_offset: u64 = i64::from(var_metadata.begin_offset.clone()) as u64;
        match header_def.data_set.record_size() {
            None => {  // fixed-size variable
                self.output_file.seek(SeekFrom::Start(begin_offset))?;
                let _chunk_size: usize = FileWriter::write_chunk_f32(&mut self.output_file, data)?;
            },
            Some(record_size) => {  // record variable
                let num_chunks: usize = var.num_chunks();
                let chunk_len: usize = var.chunk_len();
                // Loop over data chunks
                for i in 0..num_chunks {
                    let start: usize = i * chunk_len;
                    let end: usize = (i + 1) * chunk_len;
                    let chunk_slice: &[f32] = &data[start..end];
                    let position: u64 = begin_offset + ((i * record_size) as u64);
                    self.output_file.seek(SeekFrom::Start(position))?;
                    let _chunk_size: usize = FileWriter::write_chunk_f32(&mut self.output_file, chunk_slice)?;
                }
            }
        }
        self.written_vars.push(var);
        Ok(())
    }

    pub fn write_var_f64(&mut self, var_name: &str, data: &[f64]) -> Result<(), WriteError> {
        let header_def: &HeaderDefinition = self.header_def.as_ref().ok_or(WriteError::HeaderNotDefined)?;
        let var: &Variable = header_def.data_set.find_var_from_name(var_name).map_err(|_err| WriteError::VariableNotDefined(var_name.to_owned()))?.1;
        if var.data_type != DataType::F64 {
            return Err(WriteError::VariableMismatchDataType{var_name: var_name.to_owned(), req:var.data_type(), get: DataType::F64});
        }
        if var.len() != data.len() {
            return Err(WriteError::VariableMismatchDataLength{var_name: var_name.to_owned(), req:var.len(), get: data.len()});
        }
        let var_metadata: &ComputedVariableMetadata = header_def.get_var_metadata(var)?;

        // Write the `f64` data
        let begin_offset: u64 = i64::from(var_metadata.begin_offset.clone()) as u64;
        match header_def.data_set.record_size() {
            None => {  // fixed-size variable
                self.output_file.seek(SeekFrom::Start(begin_offset))?;
                let _chunk_size: usize = FileWriter::write_chunk_f64(&mut self.output_file, data)?;
            },
            Some(record_size) => {  // record variable
                let num_chunks: usize = var.num_chunks();
                let chunk_len: usize = var.chunk_len();
                // Loop over data chunks
                for i in 0..num_chunks {
                    let start: usize = i * chunk_len;
                    let end: usize = (i + 1) * chunk_len;
                    let chunk_slice: &[f64] = &data[start..end];
                    let position: u64 = begin_offset + ((i * record_size) as u64);
                    self.output_file.seek(SeekFrom::Start(position))?;
                    let _chunk_size: usize = FileWriter::write_chunk_f64(&mut self.output_file, chunk_slice)?;
                }
            }
        }
        self.written_vars.push(var);
        Ok(())
    }


    fn write_header(&mut self) -> Result<usize, WriteError>{
        let header_def: &HeaderDefinition = self.header_def.as_ref().ok_or(WriteError::HeaderNotDefined)?;
        self.output_file.seek(SeekFrom::Start(0))?;
        let mut num_bytes = 0;
        // the magic word
        num_bytes += self.output_file.write("CDF".as_bytes())?;
        //the version number
        num_bytes += self.output_file.write(&[header_def.version.clone() as u8])?;
        // the size of the *unlimited-size* dimension
        let unlim_dim_size: usize = header_def.data_set.unlimited_dim.as_ref()
            .map(|unlim_dim: &Rc<Dimension>| unlim_dim.size())
            .unwrap_or(0);
        let bytes: [u8; 4] = (unlim_dim_size as i32).to_be_bytes();
        num_bytes += self.output_file.write(&bytes)?;
        // the list of the dimensions
        num_bytes += FileWriter::write_dims_list(&mut self.output_file, &header_def.data_set.dims)?;
        // the list of the global attributes
        num_bytes += FileWriter::write_attrs_list(&mut self.output_file, &header_def.data_set.attrs)?;

        // the list of the variables
        // -------------------------
        // compute the number of bytes *begin-offset* for each variable of the dataset
        let data_set_metadata: &ComputedDataSetMetadata = &header_def.data_set_metadata;
        num_bytes += FileWriter::write_vars_list(&mut self.output_file, &data_set_metadata.vars_metadata)?;
        let zero_padding_size: &usize = &data_set_metadata.header_zero_padding_size;
        for _ in 0..*zero_padding_size {
            num_bytes +=  self.output_file.write(&[0_u8])?;
        }
        Ok(num_bytes)
    }

    /// Fill the output stream  with the default value [`NC_FILL_I8`](constant.NC_FILL_I8.html).
    fn write_chunk_nc_fill_i8<T: Write>(out_stream: &mut T, num_values: usize) -> Result<usize, std::io::Error>
    {
        // Write the useful bytes
        let bytes: [u8; 1] = NC_FILL_I8.to_be_bytes();
        for _ in 0..num_values {
            out_stream.write_all(&bytes)?;
        }
        let mut num_bytes: usize = num_values * std::mem::size_of::<i8>();

        // Write the padding bytes if necessary
        let padding_size: usize = compute_padding_size(num_bytes);
        if padding_size > 0 {
            let nc_fill_byte: [u8; 1] = NC_FILL_I8.to_be_bytes();
            let padding_bytes: Vec<u8> = vec![nc_fill_byte[0]; padding_size];
            out_stream.write_all(&padding_bytes)?;
            num_bytes += padding_size;
        }

        // Return the number of written bytes
        Ok(num_bytes)
    }

    /// Fill the output stream  with the default value [`NC_FILL_U8`](constant.NC_FILL_U8.html).
    fn write_chunk_nc_fill_u8<T: Write>(out_stream: &mut T, num_values: usize) -> Result<usize, std::io::Error>
    {
        // Write the useful bytes
        let bytes: [u8; 1] = NC_FILL_U8.to_be_bytes();
        for _ in 0..num_values {
            out_stream.write_all(&bytes)?;
        }
        let mut num_bytes: usize = num_values * std::mem::size_of::<u8>();

        // Write the padding bytes if necessary
        let padding_size: usize = compute_padding_size(num_bytes);
        if padding_size > 0 {
            let nc_fill_byte: [u8; 1] = NC_FILL_U8.to_be_bytes();
            let padding_bytes: Vec<u8> = vec![nc_fill_byte[0]; padding_size];
            out_stream.write_all(&padding_bytes)?;
            num_bytes += padding_size;
        }

        // Return the number of written bytes
        Ok(num_bytes)
    }

    /// Fill the output stream  with the default value [`NC_FILL_I16`](constant.NC_FILL_I16.html).
    fn write_chunk_nc_fill_i16<T: Write>(out_stream: &mut T, num_values: usize) -> Result<usize, std::io::Error>
    {
        // Write the useful bytes
        let bytes: [u8; 2] = NC_FILL_I16.to_be_bytes();
        for _ in 0..num_values {
            out_stream.write_all(&bytes)?;
        }
        let mut num_bytes: usize = num_values * std::mem::size_of::<i16>();

        // Write the padding bytes if necessary
        let padding_size: usize = compute_padding_size(num_bytes);
        if padding_size == 2 {
            let nc_fill_bytes: [u8; 2] = NC_FILL_I16.to_be_bytes();
            let padding_bytes: Vec<u8> = nc_fill_bytes.to_vec().into_iter().cycle().take(padding_size).collect();
            out_stream.write_all(&padding_bytes)?;
            num_bytes += padding_size;
        }


        // Return the number of written bytes
        Ok(num_bytes)
    }

    /// Fill the output stream  with the default value [`NC_FILL_I32`](constant.NC_FILL_I32.html).
    fn write_chunk_nc_fill_i32<T: Write>(out_stream: &mut T, num_values: usize) -> Result<usize, std::io::Error>
    {
        // Write the useful bytes
        let bytes: [u8; 4] = NC_FILL_I32.to_be_bytes();
        for _ in 0..num_values {
            out_stream.write_all(&bytes)?;
        }
        let num_bytes: usize = num_values * std::mem::size_of::<i32>();

        // Return the number of written bytes
        Ok(num_bytes)
    }

    /// Fill the output stream  with the default value [`NC_FILL_F32`](constant.NC_FILL_F32.html).
    fn write_chunk_nc_fill_f32<T: Write>(out_stream: &mut T, num_values: usize) -> Result<usize, std::io::Error>
    {
        // Write the useful bytes
        let bytes: [u8; 4] = NC_FILL_F32.to_be_bytes();
        for _ in 0..num_values {
            out_stream.write_all(&bytes)?;
        }
        let num_bytes: usize = num_values * std::mem::size_of::<f32>();

        // Return the number of written bytes
        Ok(num_bytes)
    }

    /// Fill the output stream with the default value [`NC_FILL_F64`](constant.NC_FILL_F64.html).
    fn write_chunk_nc_fill_f64<T: Write>(out_stream: &mut T, num_values: usize) -> Result<usize, std::io::Error>
    {
        // Write the useful bytes
        let bytes: [u8; 8] = NC_FILL_F64.to_be_bytes();
        for _ in 0..num_values {
            out_stream.write_all(&bytes)?;
        }
        let num_bytes: usize = num_values * std::mem::size_of::<f64>();

        // Return the number of written bytes
        Ok(num_bytes)
    }

    /// Write the `i8` slice into the output stream.
    fn write_chunk_i8<T: Write>(out_stream: &mut T, slice: &[i8]) -> Result<usize, std::io::Error>
    {
        // Write the useful bytes
        let mut bytes: [u8; 1];
        for value in slice.iter() {
            bytes = value.to_be_bytes();
            out_stream.write_all(&bytes)?;
        }
        let mut num_bytes: usize = slice.len() * std::mem::size_of::<i8>();

        // Write the padding bytes if necessary
        let padding_size: usize = compute_padding_size(num_bytes);
        if padding_size > 0 {
            let nc_fill_byte: [u8; 1] = NC_FILL_I8.to_be_bytes();
            let padding_bytes: Vec<u8> = vec![nc_fill_byte[0]; padding_size];
            out_stream.write_all(&padding_bytes)?;
            num_bytes += padding_size;
        }

        // Return the number of written bytes
        Ok(num_bytes)
    }

    /// Write the `u8` slice into the output stream.
    fn write_chunk_u8<T: Write>(out_stream: &mut T, slice: &[u8]) -> Result<usize, std::io::Error>
    {
        // Write the useful bytes
        let mut bytes: [u8; 1];
        for value in slice.iter() {
            bytes = value.to_be_bytes();
            out_stream.write_all(&bytes)?;
        }
        let mut num_bytes: usize = slice.len() * std::mem::size_of::<u8>();

        // Write the padding bytes if necessary
        let padding_size: usize = compute_padding_size(num_bytes);
        if padding_size > 0 {
            let nc_fill_byte: [u8; 1] = NC_FILL_U8.to_be_bytes();
            let padding_bytes: Vec<u8> = vec![nc_fill_byte[0]; padding_size];
            out_stream.write_all(&padding_bytes)?;
            num_bytes += padding_size;
        }

        // Return the number of written bytes
        Ok(num_bytes)
    }

    /// Write the `i16` slice into the output stream.
    fn write_chunk_i16<T: Write>(out_stream: &mut T, slice: &[i16]) -> Result<usize, std::io::Error>
    {
        // Write the useful bytes
        let mut bytes: [u8; 2];
        for value in slice.iter() {
            bytes = value.to_be_bytes();
            out_stream.write_all(&bytes)?;
        }
        let mut num_bytes: usize = slice.len() * std::mem::size_of::<i16>();

        // Write the padding bytes if necessary
        let padding_size: usize = compute_padding_size(num_bytes);
        if padding_size == 2 {
            let nc_fill_bytes: [u8; 2] = NC_FILL_I16.to_be_bytes();
            let padding_bytes: Vec<u8> = nc_fill_bytes.to_vec().into_iter().cycle().take(padding_size).collect();
            out_stream.write_all(&padding_bytes)?;
            num_bytes += padding_size;
        }

        // Return the number of written bytes
        Ok(num_bytes)
    }

    /// Write the `i32` slice into the output stream.
    fn write_chunk_i32<T: Write>(out_stream: &mut T, slice: &[i32]) -> Result<usize, std::io::Error>
    {
        // Write the useful bytes
        let mut bytes: [u8; 4];
        for value in slice.iter() {
            bytes = value.to_be_bytes();
            out_stream.write_all(&bytes)?;
        }
        let num_bytes: usize = slice.len() * std::mem::size_of::<i32>();

        // Return the number of written bytes
        Ok(num_bytes)
    }

    /// Write the `f32` slice into the output stream.
    fn write_chunk_f32<T: Write>(out_stream: &mut T, slice: &[f32]) -> Result<usize, std::io::Error>
    {
        // Write the useful bytes
        let mut bytes: [u8; 4];
        for value in slice.iter() {
            bytes = value.to_be_bytes();
            out_stream.write_all(&bytes)?;
        }
        let num_bytes: usize = slice.len() * std::mem::size_of::<f32>();

        // Return the number of written bytes
        Ok(num_bytes)
    }

    /// Write the `f64` slice into the output stream.
    fn write_chunk_f64<T: Write>(out_stream: &mut T, slice: &[f64]) -> Result<usize, std::io::Error>
    {
        // Write the useful bytes
        let mut bytes: [u8; 8];
        for value in slice.iter() {
            bytes = value.to_be_bytes();
            out_stream.write_all(&bytes)?;
        }
        let num_bytes: usize = slice.len() * std::mem::size_of::<f64>();

        // Return the number of written bytes
        Ok(num_bytes)
    }


    fn write_name_string<T: Write>(out_stream: &mut T, name: &str) -> Result<usize, std::io::Error> {
        let name_bytes: &[u8] = name.as_bytes();
        let zero_padding_size = compute_padding_size(name_bytes.len());
        let mut num_bytes = 0;

        // Write the number of useful bytes
        let bytes: [u8; 4] = (name_bytes.len() as i32).to_be_bytes();
        num_bytes += out_stream.write(&bytes)?;
        // Write the name
        num_bytes += out_stream.write(name_bytes)?;
        // Write the zero padding bytes
        if zero_padding_size > 0 {
            num_bytes += out_stream.write(&vec![0_u8; zero_padding_size])?;
        }

        Ok(num_bytes)
    }

    fn write_data_type<T: Write>(out_stream: &mut T, data_type: DataType) -> Result<usize, std::io::Error> {
        let bytes: [u8; 4] = (data_type as i32).to_be_bytes();
        let num_bytes: usize = out_stream.write(&bytes)?;
        Ok(num_bytes)
    }

    fn write_dims_list<T: Write>(out_stream: &mut T, dims_list: &[Rc<Dimension>]) -> Result<usize, std::io::Error> {
        fn write_dim<T: Write>(out_stream: &mut T, dim: &Rc<Dimension>) -> Result<usize, std::io::Error> {
            // First write the dimension name
            let mut num_bytes = FileWriter::write_name_string(out_stream, dim.name().as_ref())?;
            // Then write the dimension size
            let dim_size: usize = match dim.size {
                DimensionSize::Unlimited(_) => 0,  // the unlimited-size is recorded as 0
                DimensionSize::Fixed(fixed_size) => fixed_size,
            };
            let bytes: [u8; 4] = (dim_size as i32).to_be_bytes();
            num_bytes += out_stream.write(&bytes)?;

            Ok(num_bytes)
        }
        let mut num_bytes: usize = 0;
        if dims_list.is_empty() {
            // Write the ABSENT_TAG
            num_bytes += out_stream.write(&ABSENT_TAG)?;
        }
        else {
            // Write the DIENSION_TAG
            num_bytes += out_stream.write(&DIMENSION_TAG)?;

            // Write the number of dimensions
            let num_dims: usize = dims_list.len();
            let bytes: [u8; 4] = (num_dims as i32).to_be_bytes();
            num_bytes += out_stream.write(&bytes)?;

            // Write each dimension of the list
            for dim in dims_list {
                num_bytes += write_dim(out_stream, dim)?;
            }
        }
        Ok(num_bytes)
    }

    fn write_attrs_list<T: Write>(out_stream: &mut T, attrs_list: &[Attribute]) -> Result<usize, std::io::Error> {
        fn write_attr<T: Write>(out_stream: &mut T, attr: &Attribute) -> Result<usize, std::io::Error> {
            // The name of the attribute
            let mut num_bytes = FileWriter::write_name_string(out_stream, &attr.name)?;
            // The data type of the attribute
            num_bytes += FileWriter::write_data_type(out_stream, attr.data_type())?;
            // The number of elements
            let num_elements: usize = attr.len();
            let bytes: [u8; 4] = (num_elements as i32).to_be_bytes();
            num_bytes += out_stream.write(&bytes)?;
            // The data of the attribute
            num_bytes += match &attr.data {
                DataVector::I8(slice) => FileWriter::write_chunk_i8(out_stream, slice)?,
                DataVector::U8(slice) => FileWriter::write_chunk_u8(out_stream, slice)?,
                DataVector::I16(slice) => FileWriter::write_chunk_i16(out_stream, slice)?,
                DataVector::I32(slice) => FileWriter::write_chunk_i32(out_stream, slice)?,
                DataVector::F32(slice) => FileWriter::write_chunk_f32(out_stream, slice)?,
                DataVector::F64(slice) => FileWriter::write_chunk_f64(out_stream, slice)?,
            };

            Ok(num_bytes)
        }
        // The number of bytes recorded into the output stream
        let mut num_bytes: usize = 0;

        if attrs_list.is_empty() {
            // Write the ABSENT_TAG
            num_bytes += out_stream.write(&ABSENT_TAG)?;
        }
        else {
            // Write the ATTRIBUTE_TAG
            num_bytes += out_stream.write(&ATTRIBUTE_TAG)?;
            // Write the number of attributes
            let num_attrs: usize = attrs_list.len();
            let bytes: [u8; 4] = (num_attrs as i32).to_be_bytes();
            num_bytes += out_stream.write(&bytes)?;

            // Write for each attribute:  its name, data type and data
            for attr in attrs_list {
                num_bytes += write_attr(out_stream, attr)?;
            }
        }
        Ok(num_bytes)
    }

    fn write_vars_list<T: Write>(out_stream: &mut T, vars_metadata_list: &[(&Variable, ComputedVariableMetadata)]) -> Result<usize, WriteError> {
        fn write_var<T: Write>(out_stream: &mut T, var: &Variable, var_metadata: &ComputedVariableMetadata) -> Result<usize, WriteError> {
            // Write the name of the variable
            let mut num_bytes: usize = FileWriter::write_name_string(out_stream, &var.name)?;
            // Write the number of dimensions
            let num_dims = var.num_dims();
            let mut bytes: [u8; 4] = (num_dims as i32).to_be_bytes();
            num_bytes += out_stream.write(&bytes)?;
            // Write each variable dimension ID
            for dim_id in var_metadata.dim_ids.iter() {
                bytes = (*dim_id as i32).to_be_bytes();
                num_bytes += out_stream.write(&bytes)?;
            }
            // Write variable attributes
            num_bytes += FileWriter::write_attrs_list(out_stream, &var.attrs)?;
            // Write the variable data type
            num_bytes += FileWriter::write_data_type(out_stream, var.data_type.clone())?;
            // Write the `var_size` the number of bytes used per chunk (including the zero padding bytes)
            bytes = (var_metadata.chunk_size as i32).to_be_bytes();
            num_bytes += out_stream.write(&bytes)?;
            // Write the `begin_offset`
            match var_metadata.begin_offset {
                Offset::I32(begin_offset) => {
                    let bytes: [u8; 4] = begin_offset.to_be_bytes();
                    num_bytes += out_stream.write(&bytes)?;
                },
                Offset::I64(begin_offset) => {
                    let bytes: [u8; 8] = begin_offset.to_be_bytes();
                    num_bytes += out_stream.write(&bytes)?;
                },
            }
            Ok(num_bytes)
        }
        // The number of bytes recorded into the output stream
        let mut num_bytes: usize = 0;
        if vars_metadata_list.is_empty() {
            // Write the ABSENT_TAG
            num_bytes += out_stream.write(&ABSENT_TAG)?;
        }
        else {
            // Write the VARIABLE_TAG
            num_bytes += out_stream.write(&VARIABLE_TAG)?;

            // Write the number of variables
            let num_vars: usize = vars_metadata_list.len();
            let bytes: [u8; 4] = (num_vars as i32).to_be_bytes();
            num_bytes += out_stream.write(&bytes)?;

            // Write for each variable :  its name, data type, ...
            for (var, var_metadata) in vars_metadata_list.iter() {
                num_bytes += write_var(out_stream, var, var_metadata)?;
            }
        }
        Ok(num_bytes)
    }
}

#[derive(Debug)]
struct HeaderDefinition<'a> {
    /// A reference to the written data set
    data_set: &'a DataSet,
    /// NetCDF-3 version of the output
    version: Version,
    /// Minimum size (number of bytes) of the header
    header_min_size: usize,
    /// Computed data set meta data
    data_set_metadata: ComputedDataSetMetadata<'a>,
}

impl <'a> HeaderDefinition<'a> {
    fn new(data_set: &'a DataSet, version: Version, header_min_size: usize) -> Result<HeaderDefinition, WriteError> {
        Ok(HeaderDefinition{
            data_set: data_set,
            version: version.clone(),
            header_min_size: header_min_size,
            data_set_metadata: ComputedDataSetMetadata::new(data_set, version, header_min_size)?,
        })
    }

    fn get_var_metadata(&self, var: &'a Variable) -> Result<&ComputedVariableMetadata, WriteError> {
        self.data_set_metadata.vars_metadata.iter()
            .find(|(var_2, _var_metadata): &&(&Variable,  ComputedVariableMetadata)| var == *var_2)
            .map(|(_var, var_metadata): &(&Variable,  ComputedVariableMetadata)| var_metadata)
            .ok_or(WriteError::Unexpected)
    }
}

#[derive(Debug)]
struct  ComputedDataSetMetadata<'a> {
    /// The number of bytes required for the header (containing useful bytes)
    header_required_size: usize,
    /// The number of the bytes of the zero padding append to the header
    header_zero_padding_size: usize,
    /// Metadata computed for each variable
    vars_metadata: Vec<(&'a Variable, ComputedVariableMetadata)>
}



#[derive(Debug)]
struct ComputedVariableMetadata {
    /// The dimension IDs of the variable
    dim_ids: Vec<usize>,
    /// The number of bytes required to build each chunk of the variable
    chunk_size: usize,
    /// The offset (number of bytes) of the first chunck from the begin offset.
    begin_offset: Offset,
}

impl<'a> ComputedDataSetMetadata<'a> {

    /// Computes and returns all metadata required for each variable, namely :
    ///
    /// 0. The position of the variables stored in the *data part* (a `usize` instance).
    /// 1. The header metadata of each variable :
    ///     0. A reference to the variable (a `&Variable` instance).
    ///     1. The IDs of its dimensions (a `Vec<usize>` instance)
    ///     2. The `data_offset` to located the first chunck of the variable **from the begining of the data part** (a`usize` instance).
    fn new(data_set: &'a DataSet, version: Version, header_min_size: usize) -> Result<ComputedDataSetMetadata, WriteError> {
        // Create a partition of variables to distinguish :
        // 1. Fist the *fixed-size* variables.
        // 2. Then the *record* variables.
        let (record_vars, non_record_vars): (Vec<(usize, &Variable)>, Vec<(usize, &Variable)>) = data_set.vars.iter()
            .enumerate()  // keep the original positions of the variables in the header
            .partition(|(_var_pos, var): &(usize, &Variable)|{
                var.is_record_var()
            });
        let partitioned_vars: Vec<(usize, &Variable)> = non_record_vars.into_iter().chain(record_vars).collect();

        // Compute the actual header size
        let header_required_size: usize = ComputedDataSetMetadata::compute_header_required_size(data_set, version.clone());
        let header_size: usize = {
            let mut header_size: usize = std::cmp::max(header_min_size, header_required_size);
            header_size += compute_padding_size(header_size);
            header_size
        };

        // Compute the metadata for each variable
        let mut begin_offset: usize = header_size;
        let mut vars_metadata: Vec<(usize, (&Variable, ComputedVariableMetadata))> = vec![];
        for (header_part_pos, var) in partitioned_vars.into_iter() {
            let chunk_size: usize = var.chunk_size();
            vars_metadata.push((
                header_part_pos,
                (
                    var,
                    ComputedVariableMetadata{
                        dim_ids: data_set.get_var_dim_ids(&var.name).unwrap(),
                        chunk_size: chunk_size,
                        begin_offset: match &version{
                            Version::Classic => {
                                let offset: i32 = i32::try_from(begin_offset).map_err(|_err| WriteError::ClassicVersionNotPossible)?;
                                Offset::I32(offset)
                            },
                            Version::Offset64Bit => {
                                Offset::I64(begin_offset as i64)
                            }
                        },
                    }
                )
            ));
            begin_offset += chunk_size;
        }

        // Retrieve the original position
        vars_metadata.sort_by_key(|(header_part_pos, (_var, _var_metadata)): &(usize, (&Variable, ComputedVariableMetadata))| *header_part_pos);
        // Remove the header positions of the variables
        let vars_metadata: Vec<(&'a Variable, ComputedVariableMetadata)> = vars_metadata.into_iter().map(|x| x.1).collect();

        // Returns the meta data only
        Ok(ComputedDataSetMetadata{
            header_required_size: header_required_size,
            header_zero_padding_size: header_size - header_required_size,
            vars_metadata: vars_metadata,
        })
    }

    /// Computes and returns the size (number of bytes) needed to write the file header.
    fn compute_header_required_size(data_set: &'a DataSet, version: Version) -> usize{
        fn compute_name_string_size(name: &str) -> usize {
            let mut num_bytes: usize = 0;
            // the number bytes for the name
            num_bytes += std::mem::size_of::<i32>();
            // the bytes of the name
            let num_bytes_name = name.as_bytes().len();
            num_bytes += num_bytes_name;
            // the bytes of the zero-padding
            num_bytes += compute_padding_size(num_bytes_name);

            return num_bytes;
        }
        fn compute_attrs_list_size(attrs_list: &[Attribute]) -> usize {
            let mut num_bytes: usize = 0;
            // the global attributes
            if attrs_list.is_empty() {
                num_bytes += ABSENT_TAG.len();
            }
            else {
                // the tag `ATTRIBUTE_TAG`
                num_bytes += ATTRIBUTE_TAG.len();
                // the number of attributes
                num_bytes += std::mem::size_of::<i32>();
                for attr in attrs_list.iter() {
                    // the name of the attributes
                    num_bytes += compute_name_string_size(&attr.name);
                    // the attribute data type
                    num_bytes += std::mem::size_of::<i32>();
                    // the number of elements
                    num_bytes += std::mem::size_of::<i32>();
                    // the ttribute data
                    let num_useful_bytes = attr.len() * attr.data_type().size_of();
                    num_bytes += num_useful_bytes;
                    // Zero-passing
                    num_bytes += compute_padding_size(num_useful_bytes);
                }
            }
            return num_bytes;
        }
        let mut num_bytes = 0;
        // the magic word `"CDF"`
        num_bytes += 3;
        // the version number
        num_bytes += std::mem::size_of::<u8>();
        // the length of the *unlimited-size* dimension
        num_bytes += std::mem::size_of::<i32>();
        // the dimensions list
        if data_set.dims.is_empty() {
            // the tag `ABSENT_TAG`
            num_bytes += ABSENT_TAG.len();
        }
        else {
            // the tag `DIMENSION_TAG`
            num_bytes += DIMENSION_TAG.len();
            // the number of dimensions
            num_bytes += std::mem::size_of::<i32>();
            for dim in data_set.dims.iter() {
                // the name of the dimension
                num_bytes += compute_name_string_size(&dim.name.borrow());
                // the size og the dimension
                num_bytes += std::mem::size_of::<i32>();
            }
        }
        // the global attributes
        num_bytes += compute_attrs_list_size(&data_set.attrs);
        // the variables list
        if data_set.vars.is_empty() {
            num_bytes += ABSENT_TAG.len();
        }
        else {
            num_bytes += VARIABLE_TAG.len();
            // the number of variables
            num_bytes += std::mem::size_of::<i32>();
            for var in data_set.vars.iter() {
                // the variable name
                num_bytes += compute_name_string_size(&var.name);
                // the number of dimensions
                num_bytes += std::mem::size_of::<i32>();
                // the ID of each dimension of the variable
                num_bytes += var.num_dims() * std::mem::size_of::<i32>();
                // the list of variable attributes
                num_bytes += compute_attrs_list_size(&var.attrs);
                // the variables data type
                num_bytes += std::mem::size_of::<i32>();
                // the number of bytes required each chunck
                num_bytes += std::mem::size_of::<i32>();
                // the begin offset depends of the NetCDF-3 version
                num_bytes += match version {
                    Version::Classic => std::mem::size_of::<i32>(),
                    Version::Offset64Bit => std::mem::size_of::<i64>(),
                }
            }
        }
        return num_bytes;
    }
}