mod tests;

use std::convert::TryFrom;
use std::rc::Rc;
use std::io::{Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};

use std::iter::*;

use byteorder::{ReadBytesExt, BigEndian};

use nom::{
    combinator::{
        verify,
        map_res,
    },
    bytes::streaming::{
        tag,
        take,
    },
    number::streaming::{
        be_i8,
        be_u8,
        be_i16,
        be_i32,
        be_f32,
        be_f64,
        be_i64,
    },
    branch::alt,
    multi::many_m_n,
};


use crate::{
    DataSet,
    DataType,
    Dimension,
    Version,
    error::{IOError, ReadDataError, ParseHeaderError, ParseErrorKind, NomError},
};
use crate::{
    typed_data_vector::DataVector,
};



const ABSENT_TAG: [u8; 8] = [0; 8];
const DIMENSION_TAG: [u8; 4] = [0, 0, 0, 0x0A];
const VARIABLE_TAG: [u8; 4] = [0, 0, 0, 0x0b];
const ATTRIBUTE_TAG: [u8; 4] = [0, 0, 0, 0x0C];

#[derive(Debug, Clone, PartialEq, Eq)]
enum Offset {
    I32(i32),
    I64(i64),
}

impl std::convert::From<Offset> for i64 {

    fn from(offset: Offset) -> Self
    {
        match offset {
            Offset::I32(value) => value as i64,
            Offset::I64(value) => value,
        }
    }
}

pub struct FileReader {
    data_set: DataSet,
    input_file_path: PathBuf,
    input_file: std::fs::File,
    vars_info: std::collections::HashMap<String, (usize, Offset)>,
}

impl FileReader {

    /// Returns the data set managed by the reader.
    pub fn data_set(&self) -> &DataSet
    {
        return &self.data_set;
    }

    /// Returns the data set managed by the reader.
    pub fn file_path(&self) -> &std::path::Path
    {
        return &self.input_file_path;
    }

    /// Opens the file and parse the header of the NetCDF-3.
    pub fn open<P: AsRef<Path>>(input_file_path: P) -> Result<Self, IOError>
    {
        // Open the file
        let input_file_path: PathBuf = {
            let mut path = PathBuf::new();
            path.push(input_file_path);
            path
        };

        let mut input_file = std::fs::File::open(input_file_path.clone()).map_err(|err: std::io::Error| {
            ReadDataError::Read(err.kind())
        })?;

        let input: Vec<u8> = {
            let mut input: Vec<u8> = vec![];
            input_file.read_to_end(&mut input).map_err(|err: std::io::Error| {
                ReadDataError::Read(err.kind())
            })?;
            input
        };
     

        let (data_set, vars_info): (DataSet, Vec<(String, (usize, Offset))>) = FileReader::parse_header(&input)?;

        return Ok(FileReader{
            data_set: data_set,
            input_file_path: input_file_path,
            input_file: input_file,
            vars_info: vars_info.into_iter().collect(),  // convert list of tuples to map
        })
    }

    // Closes the NetCDF-3 file and returns the inferred data set containing onlyt the previously read variables.
    pub fn close(self) -> DataSet {
        self.data_set
    }

    pub fn read_all_vars(&mut self) -> Result<(), IOError>
    {
        let record_size: usize = self.data_set.record_size();
        let num_records: usize = self.data_set.num_records();
        let ref mut input = self.input_file;
        for ref mut var in self.data_set.vars.iter_mut()
            {
                let begin_offset: u64 = {
                    // let info: &(usize, Offset) = .unwrap();
                    let (_var_size, begin_offset): &(usize, Offset) = self.vars_info.get(var.name()).ok_or(ReadDataError::Unexpected)?;
                    i64::from(begin_offset.clone()) as u64
                };
            let data_type: DataType = var.data_type();
            let num_elements_per_chunk: usize = var.num_elements_per_chunk();
            let padding_size: usize = {
                let num_bytes: usize = num_elements_per_chunk * data_type.size_of();
                compute_zero_padding_size(num_bytes)
            };
            input.seek(SeekFrom::Start(begin_offset)).map_err(|err: std::io::Error|{
                ReadDataError::Read(err.kind())
            })?;
            // memory allocation
            let mut vec_data = DataVector::new(data_type, var.len());
            if !var.is_record_var() {
                match vec_data {
                    DataVector::I8(ref mut data) => { input.read_i8_into(&mut data[..]) },
                    DataVector::U8(ref mut data) => { input.read_exact(&mut data[..]) },
                    DataVector::I16(ref mut data) => { input.read_i16_into::<BigEndian>(&mut data[..]) },
                    DataVector::I32(ref mut data) => { input.read_i32_into::<BigEndian>(&mut data[..]) },
                    DataVector::F32(ref mut data) => { input.read_f32_into::<BigEndian>(&mut data[..]) },
                    DataVector::F64(ref mut data) => { input.read_f64_into::<BigEndian>(&mut data[..]) },
                }.map_err(|err: std::io::Error| {
                    ReadDataError::Read(err.kind())
                })?;
                if padding_size > 0
                {
                    input.seek(SeekFrom::Current(padding_size as i64)).map_err(|err: std::io::Error| {
                        ReadDataError::Read(err.kind())
                    })?;
                }
                var.data = Some(vec_data);
            }
            else {
                let chunk_size: usize = var.chunk_size();
                let offset_size:i64 = (record_size + padding_size - chunk_size) as i64;
                for i in 0_usize..num_records
                {
                    // reader.seek(SeekFrom::)
                    let start: usize = i * num_elements_per_chunk;
                    let end: usize = (i + 1) * num_elements_per_chunk;
                    match vec_data {
                        DataVector::I8(ref mut data) => { input.read_i8_into(&mut data[start..end]) },
                        DataVector::U8(ref mut data) => { input.read_exact(&mut data[start..end]) },
                        DataVector::I16(ref mut data) => { input.read_i16_into::<BigEndian>(&mut data[start..end]) },
                        DataVector::I32(ref mut data) => { input.read_i32_into::<BigEndian>(&mut data[start..end]) },
                        DataVector::F32(ref mut data) => { input.read_f32_into::<BigEndian>(&mut data[start..end]) },
                        DataVector::F64(ref mut data) => { input.read_f64_into::<BigEndian>(&mut data[start..end]) },
                    }.map_err(|err: std::io::Error| {
                        ReadDataError::Read(err.kind())
                    })?;
                    input.seek(SeekFrom::Current(offset_size)).map_err(|err: std::io::Error| {
                        ReadDataError::Read(err.kind())
                    })?;
                }
                var.data = Some(vec_data);
            }
        }
        return Ok(());
    }

    /// Parses the NetCDF-3 header
    fn parse_header(input: &[u8]) -> Result<(DataSet, Vec<(String, (usize, Offset))>), IOError> {
        // the magic word
        let (input, _): (&[u8], &[u8]) = FileReader::parse_magic_word(input).unwrap();
        // the version number
        let (input, version) : (&[u8], Version) = FileReader::parse_version(input).unwrap();
        
        // the number of records
        let (input, num_of_records): (&[u8], usize) = FileReader::parse_as_usize(input).unwrap();
        let (input, dims_list): (&[u8], Vec<(String, usize)>) = FileReader::parse_dims_list(input).unwrap();
        let (input, global_attrs_list): (&[u8], Vec<_>) = FileReader::parse_attrs_list(input).unwrap();
        let (_input, vars_list): (&[u8], Vec<_>) = FileReader::parse_vars_list(input, version.clone()).unwrap();

        // Create a new dataset
        let mut data_set = DataSet::new(version);

        // Append it the dimensions
        for (dim_name, dim_size) in dims_list.into_iter() {
            if dim_size == 0 {
                data_set.set_unlimited_dim(dim_name, num_of_records)?;
            } else {
                data_set.add_fixed_dim(dim_name, dim_size)?;
            }
        }
        
        // Append ot the global attributes
        for (attr_name, attr_data) in global_attrs_list.into_iter() {
            use DataVector::*;
            match attr_data {
                I8(data) => {
                    data_set.add_global_attr_i8(&attr_name, data)?;
                }
                U8(data) => {
                    data_set.add_global_attr_u8(&attr_name, data)?;
                }
                I16(data) => {
                    data_set.add_global_attr_i16(&attr_name, data)?;
                }
                I32(data) => {
                    data_set.add_global_attr_i32(&attr_name, data)?;
                }
                F32(data) => {
                    data_set.add_global_attr_f32(&attr_name, data)?
                }
                F64(data) => {
                    data_set.add_global_attr_f64(&attr_name, data)?;
                }
            }
        }
        
        let mut vars_info: Vec<(String, (usize, Offset))> = vec![];
        // Append the variables
        for (var_name, var_dim_ids, var_attrs_list, var_data_type, var_size, begin_offset) in vars_list.into_iter() {
            let var_dim_refs: Vec<Rc<Dimension>> = data_set.get_dims_from_ids(&var_dim_ids)?;
            // Append the variable
            data_set.add_var_using_dim_refs(&var_name, var_dim_refs, var_data_type)?;
            // Save the size and the begin offset of the variable
            vars_info.push((
                var_name.to_string(),
                (var_size, begin_offset)
            ));
            // data_set.vars.last_mut()?.1 = Some(begin_offset);
            // Append variable attributes
            for (attr_name, attr_data) in var_attrs_list.into_iter() {
                use DataVector::*;
                match attr_data {
                    I8(data) => {
                        data_set.add_var_attr_i8(&var_name, &attr_name, data)?;
                    }
                    U8(data) => {
                        data_set.add_var_attr_u8(&var_name, &attr_name, data)?;
                    }
                    I16(data) => {
                        data_set.add_var_attr_i16(&var_name, &attr_name, data)?;
                    }
                    I32(data) => {
                        data_set.add_var_attr_i32(&var_name, &attr_name, data)?;
                    }
                    F32(data) => {
                        data_set.add_var_attr_f32(&var_name, &attr_name, data)?;
                    }
                    F64(data) => {
                        data_set.add_var_attr_f64(&var_name, &attr_name, data)?;
                    }
                }
            }
        }
        Ok((data_set, vars_info))
    }


    fn parse_magic_word(input: &[u8]) -> Result<(&[u8], &[u8]), ParseHeaderError>
    {
        let (input, tag_value): (&[u8], &[u8]) = tag(&b"CDF"[..])(input).map_err(|err: NomError|{
            ParseHeaderError::new(err, ParseErrorKind::MagicWord)
        })?;
        Ok((input, tag_value))
    }

    fn parse_version(input: &[u8]) -> Result<(&[u8], Version), ParseHeaderError>
    {
        let (input, version_number): (&[u8], u8) = verify(be_u8, |ver_num: &u8|{
            ver_num == &(Version::Classic as u8) || ver_num == &(Version::Offset64Bit as u8)
        })(input).map_err(|err: NomError|{
            ParseHeaderError::new(err, ParseErrorKind::VersionNumber)
        })?;
        let version = Version::try_from(version_number).unwrap();  // previously checked
        Ok((input, version))
    }

    /// Parses a `i32` word and checks that it is non-negative.
    fn parse_non_neg_i32(input: &[u8]) -> Result<(&[u8], i32), ParseHeaderError> {
        verify(be_i32, |number: &i32| *number >= 0_i32)(input).map_err(|err: NomError|{
            ParseHeaderError::new(err, ParseErrorKind::NonNegativeI32)
        })
    }

    /// Parses a non-negative `i32` word and converts it to a `usize`.
    fn parse_as_usize(input: &[u8]) -> Result<(&[u8], usize), ParseHeaderError> {
        let (input, number): (&[u8], i32) = FileReader::parse_non_neg_i32(input)?;
        Ok((input, number as usize))
    }

    /// Parses a non-negative `i32` word and converts it to a `u32`.
    fn parse_as_u32(input: &[u8]) -> Result<(&[u8], u32), ParseHeaderError> {
        let (input, number): (&[u8], i32) = FileReader::parse_non_neg_i32(input)?;
        Ok((input, number as u32))
    }
    /// Parses a string
    fn parse_name_string(input: &[u8]) -> Result<(&[u8], String), ParseHeaderError>
    {
        let (input, num_of_bytes): (&[u8], usize) = FileReader::parse_as_usize(input)?;
        let (input, name): (&[u8], String) = map_res(take(num_of_bytes), |bytes: &[u8]| {
            String::from_utf8(bytes.to_vec())
        })(input).map_err(|err: NomError|{
            ParseHeaderError::new(err, ParseErrorKind::Utf8)
        })?;
        // Take the zero padding bytes if necessary
        let (input, _zero_padding_bytes): (&[u8], &[u8]) = FileReader::parse_zero_padding(input, compute_zero_padding_size(num_of_bytes))?;
        Ok((input, name))
    }

    // Parses a NetCDF-3 data type.
    fn parse_data_type(input: &[u8]) -> Result<(&[u8], DataType), ParseHeaderError>
    {
        let start: &[u8] = input;
        let (input, data_type_number): (&[u8], u32) = FileReader::parse_as_u32(input)?;
        let data_type: DataType = DataType::try_from(data_type_number).map_err(|_err|{
            nom::Err::Error((&start[0..4], nom::error::ErrorKind::Verify))
        }).map_err(|err: NomError|{
            ParseHeaderError::new(err, ParseErrorKind::DataType)
        })?;
        Ok((input, data_type))
    }

    fn parse_typed_data_elements(input: &[u8], num_of_elements: usize, data_type: DataType) -> Result<(&[u8], DataVector), ParseHeaderError>
    {
        // Parsed the useful data
        let (input, typed_data_vector): (&[u8], DataVector) = match data_type {
            DataType::I8 => many_m_n(num_of_elements, num_of_elements, be_i8)(&input).map(|(input, data): (&[u8], Vec<i8>)| (input, DataVector::I8(data))),
            DataType::U8 => many_m_n(num_of_elements, num_of_elements, be_u8)(&input).map(|(input, data): (&[u8], Vec<u8>)| (input, DataVector::U8(data))),
            DataType::I16 => many_m_n(num_of_elements, num_of_elements, be_i16)(&input).map(|(input, data): (&[u8], Vec<i16>)| (input, DataVector::I16(data))),
            DataType::I32 => many_m_n(num_of_elements, num_of_elements, be_i32)(&input).map(|(input, data): (&[u8], Vec<i32>)| (input, DataVector::I32(data))),
            DataType::F32 => many_m_n(num_of_elements, num_of_elements, be_f32)(&input).map(|(input, data): (&[u8], Vec<f32>)| (input, DataVector::F32(data))),
            DataType::F64 => many_m_n(num_of_elements, num_of_elements, be_f64)(&input).map(|(input, data): (&[u8], Vec<f64>)| (input, DataVector::F64(data))),
        }.map_err(|err: NomError|{
            ParseHeaderError::new(err, ParseErrorKind::DataElements)
        })?;

        // Parse the zero padding bytes if necessary
        let num_of_bytes: usize = data_type.size_of() * num_of_elements;
        let (input, _zero_padding_bytes): (&[u8], &[u8]) = FileReader::parse_zero_padding(input, compute_zero_padding_size(num_of_bytes))?;
        Ok((input, typed_data_vector))
    }

    fn parse_zero_padding(input: &[u8], padding_size: usize) -> Result<(&[u8], &[u8]), ParseHeaderError>
    {
        verify(take(padding_size), |padding_bytes: &[u8]| {
            padding_bytes.iter().all(|byte: &u8| {
                *byte == 0_u8
            })
        })(input).map_err(|err: NomError|{
            ParseHeaderError::new(err, ParseErrorKind::ZeroPadding)
        })
    }

    // Parses the list of the dimensions from the header.
    fn parse_dims_list(input: &[u8]) -> Result<(&[u8], Vec<(String, usize)>), ParseHeaderError>
    {
        fn parse_dim(input: &[u8]) -> Result<(&[u8], (String, usize)), ParseHeaderError>
        {
            let (input, dim_name): (&[u8], String) = FileReader::parse_name_string(input)?;
            let (input, dim_size): (&[u8], usize) = FileReader::parse_as_usize(input)?;
            Ok((input, (dim_name, dim_size)))
        }
        let (input, dim_tag): (&[u8], &[u8]) = alt((tag(ABSENT_TAG), tag(DIMENSION_TAG)))(input).map_err(|err: NomError|{
            ParseHeaderError::new(err, ParseErrorKind::DimTag)
        })?;
        if dim_tag == &ABSENT_TAG {
            return Ok((input, vec![]));
        }
        let (mut input, num_of_dims): (&[u8], usize) = FileReader::parse_as_usize(input)?;
        let mut dims_list: Vec<(String, usize)> = Vec::with_capacity(num_of_dims);
        for _ in 0..num_of_dims{
            let (rem_input, dim): (&[u8], (String, usize)) = parse_dim(input)?;
            input = rem_input;
            dims_list.push(dim);
        }

        Ok((input, dims_list))
    }

    // Parses a list of attributes (global of from any variables) from the header.
    fn parse_attrs_list(input: &[u8]) -> Result<(&[u8], Vec<(String, DataVector)>), ParseHeaderError>
    {
        fn parse_attr(input: &[u8]) -> Result<(&[u8], (String, DataVector)), ParseHeaderError>
        {
            let (input, attr_name): (&[u8], String) = FileReader::parse_name_string(input)?;
            let (input, attr_data_type): (&[u8], DataType) = FileReader::parse_data_type(input)?;
            let (input, num_of_elements): (&[u8], usize) = FileReader::parse_as_usize(input)?;
            let (input, attr_data): (&[u8], DataVector) = FileReader::parse_typed_data_elements(input, num_of_elements, attr_data_type)?;
            Ok((input, (attr_name, attr_data)))
        }
        let (input, attr_tag): (&[u8], &[u8]) = alt((tag(ABSENT_TAG), tag(ATTRIBUTE_TAG)))(input).map_err(|err: NomError|{
            ParseHeaderError::new(err, ParseErrorKind::AttrTag)
        })?;
        if attr_tag == &ABSENT_TAG {
            return Ok((input, vec![]));
        }
        let (mut input, num_of_attrs): (&[u8], usize) = FileReader::parse_as_usize(input)?;
        let mut attrs_list: Vec<(String, DataVector)> = Vec::with_capacity(num_of_attrs);
        for _ in 0..num_of_attrs
        {
            let (rem_input, attr): (&[u8], (String, DataVector)) = parse_attr(input)?;
            input = rem_input;
            attrs_list.push(attr);
        }
        Ok((input, attrs_list))
    }



    // Parses a list of variables from the header.
    fn parse_vars_list(input: &[u8], version: Version) -> Result<(&[u8], Vec<(String, Vec<usize>, Vec<(String, DataVector)>, DataType, usize, Offset)>), ParseHeaderError>
    {
        fn parse_dim_ids_list(input: &[u8]) -> Result<(&[u8], Vec<usize>), ParseHeaderError>
        {
                // number of dimensions
                let (mut input, num_of_dims): (&[u8], usize) = FileReader::parse_as_usize(input)?;
                // list of the dimension ids
                let mut dim_ids_list: Vec<usize> = Vec::with_capacity(num_of_dims);
                for _ in 0..num_of_dims {
                    let(rem_input, dim_id): (&[u8], usize) = FileReader::parse_as_usize(input)?;
                    input = rem_input;
                    dim_ids_list.push(dim_id);
                }
                Ok((input, dim_ids_list))
        }

        fn parse_offset(input: &[u8], version: Version) -> Result<(&[u8], Offset), ParseHeaderError>
        {
            match version {
                Version::Classic => {
                    be_i32(input).map(|(input, num_of_bytes): (&[u8], i32)| {
                        (input, Offset::I32(num_of_bytes))
                    })
                },
                Version::Offset64Bit => {
                    be_i64(input).map(|(input, num_of_bytes): (&[u8], i64)| {
                        (input, Offset::I64(num_of_bytes))
                    })
                },
            }.map_err(|err: NomError| {
                ParseHeaderError::new(err, ParseErrorKind::Offset)
            })
        }

        fn parse_var(input: &[u8], version: Version) -> Result<(&[u8], (String, Vec<usize>, Vec<(String, DataVector)>, DataType, usize, Offset)), ParseHeaderError> {
            // Variable name
            let (input, var_name): (&[u8], String) = FileReader::parse_name_string(input)?;

            // list of the dimensions
            let (input, dim_ids_list): (&[u8], Vec<usize>) = parse_dim_ids_list(input)?;
            // list of the variable attributes
            let (input, var_attrs_list): (&[u8], Vec<(String, DataVector)>) = FileReader::parse_attrs_list(input)?;
            // data type of the variable
            let (input, var_data_type): (& [u8], DataType) = FileReader::parse_data_type(input)?;
            // size occupied in each record by the variable (number of bytes)
            let (input, var_size): (&[u8], usize) = FileReader::parse_as_usize(input)?;
            // begin offset (number of bytes)
            let (input, begin_offset): (&[u8], Offset) = parse_offset(input, version)?;

            return Ok((input, (var_name, dim_ids_list, var_attrs_list, var_data_type, var_size, begin_offset)));
        }
        let (input, var_tag): (&[u8], &[u8]) = alt((tag(ABSENT_TAG), tag(VARIABLE_TAG)))(input).map_err(|err: NomError| {
            ParseHeaderError::new(err, ParseErrorKind::VarTag)
        })?;
        if var_tag == &ABSENT_TAG {
            return Ok((input, vec![]));
        }
        let (mut input, num_of_vars): (&[u8], usize) = FileReader::parse_as_usize(input)?;
        let mut vars_list: Vec<(String, Vec<usize>, Vec<(String, DataVector)>, DataType, usize, Offset)> = vec![];
        for _ in 0..num_of_vars {
            let (temp_input, var) = parse_var(input, version.clone())?;
            input = temp_input;
            vars_list.push(var);
        }
        Ok((input, vars_list))
    }
}




#[inline]
/// Compute and return the number of bytes of the zero padding required to fill remaining bytes up.
///
/// Arguments :
/// - `number_of_bytes` : number of used bytes
pub(crate) fn compute_zero_padding_size(number_of_bytes: usize) -> usize {
    const ALIGNMENT_SIZE: usize = 4;
    return match number_of_bytes % 4 {
        0 => 0,
        n => ALIGNMENT_SIZE - n,
    };
}