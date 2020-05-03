pub mod input_error;
use input_error::{ParseHeaderError, ReadDataError};

use crate::DataType;

/// NetCDF-3 data errors.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InvalidDataSet {
    DimensionAlreadyExists(String),
    DimensionNotDefined(String),
    DimensionsNotDefined{var_name: String, get_undef_dim_names: Vec<String>},
    DimensionsUsedMultipleTimes{var_name: String, get_dim_names: Vec<String>},
    UnlimitedDimensionAlreadyExists(String),
    DimensionYetUsed{var_names: Vec<String>, dim_name: String},
    DimensionsIdsNotValid(Vec<usize>),
    DimensionNameNotValid(String),

    VariableAttributeAlreadyExists{var_name: String, attr_name: String},
    VariableAttributeNotDefined{var_name: String, attr_name: String},
    VariableAttributeNameNotValid{var_name: String, attr_name: String},

    VariableNotDefined(String),
    VariableNameNotValid(String),
    VariableAlreadyExists(String),
    VariableMismatchDataType{var_name: String, req: DataType, get: DataType},
    VariableMismatchDataLength{var_name: String, req: usize, get: usize},
    UnlimitedDimensionMustBeDefinedFirst{var_name: String, unlim_dim_name: String, get_dim_names: Vec<String>},

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
