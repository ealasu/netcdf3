mod parse_error;
pub use parse_error::{ParseHeaderError, ParseErrorKind, InvalidBytes};
pub(crate) use parse_error::NomError;

use crate::DataType;

/// NetCDF-3 data errors.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InvalidDataSet {
    DimensionNameAlreadyExists(String),
    DimensionNotDefined(String),
    DimensionsNotDefined(Vec<String>),
    DimensionsUsedMultipleTimes(Vec<String>),
    UnlimitedDimensionNotDefined,
    UnlimitedDimensionAlreadyExists(String),
    DimensionYetUsed(Vec<String>, String),
    DimensionsIdsNotValid(Vec<usize>),
    DimensionNameNotValid(String),

    VariableAttributeAlreadyExists(String, String),
    VariableAttributeNotDefined(String, String),
    VariableAttributeNameNotValid(String, String),

    VariableNotDefined(String),
    VariableNameNotValid(String),
    VariableAlreadyExists(String),
    VariableMismatchDataType((String, DataType), DataType),
    VariableMismatchDataLength((String, usize), usize),
    VariableRequiresAtLeastOneDimension(String),
    VariableDataNotFound(String, DataType, usize),
    UnlimitedDimensionMustBeDefinedFirst(String),

    GlobalAttributeAlreadyExists(String),
    GlobalAttributeNotDefined(String),
    GlobalAttributeNameNotValid(String),
}

impl std::fmt::Display for InvalidDataSet {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for InvalidDataSet {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReadDataError {
    Unexpected,
    Read(std::io::ErrorKind),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IOError {
    ReadData(ReadDataError),
    ParseHeader(ParseHeaderError),
    DataSet(InvalidDataSet),
}

impl std::fmt::Display for IOError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for IOError {}

impl std::convert::From<InvalidDataSet> for IOError {
    fn from(err: InvalidDataSet) -> Self {
        Self::DataSet(err)
    }
}

impl std::convert::From<ParseHeaderError> for IOError {
    fn from(err: ParseHeaderError) -> Self {
        Self::ParseHeader(err)
    }
}

impl std::convert::From<ReadDataError> for IOError {
    fn from(err: ReadDataError) -> Self {
        Self::ReadData(err)
    }
}
