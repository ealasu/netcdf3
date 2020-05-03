mod tests;
use crate::DataType;

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum DataVector {
    I8(Vec<i8>),
    U8(Vec<u8>),
    I16(Vec<i16>),
    I32(Vec<i32>),
    F32(Vec<f32>),
    F64(Vec<f64>),
}

impl DataVector {

    /// Return the NetCDF-3 type of the data container
    pub(crate) fn data_type(&self) -> DataType {
        match self {
            DataVector::I8(_) => DataType::I8,
            DataVector::U8(_) => DataType::U8,
            DataVector::I16(_) => DataType::I16,
            DataVector::I32(_) => DataType::I32,
            DataVector::F32(_) => DataType::F32,
            DataVector::F64(_) => DataType::F64,
        }
    }

    /// Return the length (the number of elements) of the vector.
    pub(crate) fn new(data_type: DataType, length: usize) -> Self {
        match data_type {
            DataType::I8 => DataVector::I8(vec![0; length]),
            DataType::U8 => DataVector::U8(vec![0; length]),
            DataType::I16 => DataVector::I16(vec![0; length]),
            DataType::I32 => DataVector::I32(vec![0; length]),
            DataType::F32 => DataVector::F32(vec![0.0; length]),
            DataType::F64 => DataVector::F64(vec![0.0; length]),
        }
    }

    // /// Return the length (the number of elements) of the vector.
    // pub(crate) fn clear(&mut self)  {
    //     match self {
    //         DataVector::I8(data) => { data.clear(); }
    //         DataVector::U8(data) => { data.clear(); }
    //         DataVector::I16(data) => { data.clear(); }
    //         DataVector::I32(data) => { data.clear(); }
    //         DataVector::F32(data) => { data.clear(); }
    //         DataVector::F64(data) => { data.clear(); }
    //     };
    // }

    /// Return the length (the number of elements) of the vector.
    pub(crate) fn len(&self) -> usize {
        match self {
            DataVector::I8(data) => data.len(),
            DataVector::U8(data) => data.len(),
            DataVector::I16(data) => data.len(),
            DataVector::I32(data) => data.len(),
            DataVector::F32(data) => data.len(),
            DataVector::F64(data) => data.len(),
        }
    }

    /// Return the reference to the vector of *NetCDF-2 i8s*, if the type of the data is `DataType::I8`,
    /// otherwise return `None`.
    pub(crate) fn get_i8(&self) -> Option<&[i8]> {
        return match self {
            DataVector::I8(data) => Some(data),
            DataVector::U8(_) => None,
            DataVector::I16(_) => None,
            DataVector::I32(_) => None,
            DataVector::F32(_) => None,
            DataVector::F64(_) => None,
        };
    }

    pub(crate) fn get_u8(&self) -> Option<&[u8]> {
        return match self {
            DataVector::I8(_) => None,
            DataVector::U8(data) => Some(data),
            DataVector::I16(_) => None,
            DataVector::I32(_) => None,
            DataVector::F32(_) => None,
            DataVector::F64(_) => None,
        };
    }

    pub(crate) fn get_i16(&self) -> Option<&[i16]> {
        return match self {
            DataVector::I8(_) => None,
            DataVector::U8(_) => None,
            DataVector::I16(data) => Some(data),
            DataVector::I32(_) => None,
            DataVector::F32(_) => None,
            DataVector::F64(_) => None,
        };
    }

    pub(crate) fn get_i32(&self) -> Option<&[i32]> {
        return match self {
            DataVector::I8(_) => None,
            DataVector::U8(_) => None,
            DataVector::I16(_) => None,
            DataVector::I32(data) => Some(data),
            DataVector::F32(_) => None,
            DataVector::F64(_) => None,
        };
    }

    pub(crate) fn get_f32(&self) -> Option<&[f32]> {
        return match self {
            DataVector::I8(_) => None,
            DataVector::U8(_) => None,
            DataVector::I16(_) => None,
            DataVector::I32(_) => None,
            DataVector::F32(data) => Some(data),
            DataVector::F64(_) => None,
        };
    }

    pub(crate) fn get_f64(&self) -> Option<&[f64]> {
        return match self {
            DataVector::I8(_) => None,
            DataVector::U8(_) => None,
            DataVector::I16(_) => None,
            DataVector::I32(_) => None,
            DataVector::F32(_) => None,
            DataVector::F64(data) => Some(data),
        };
    }
}
