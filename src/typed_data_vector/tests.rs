#![cfg(test)]

use super::DataVector;

#[test]
fn test_typed_data_container_equality() {
    // Test equality between empty containers
    {
        let a = DataVector::I8(vec![]);
        let b = DataVector::I8(vec![]);
        assert_eq!(a, b)
    }

    // Test equality between 2 containers with different lengths
    {
        let a = DataVector::I8(vec![0; 0]);
        let b = DataVector::I8(vec![0; 1]);
        assert_ne!(a, b)
    }

    // Test equality between 2 i8-containers with the same length
    {
        let a = DataVector::I8(vec![1, 2, 3, 4]);
        let b = DataVector::I8(vec![1, 2, 3, 4]);
        let c = DataVector::I8(vec![1, 2, 3, 3]);
        assert_eq!(a, b);
        assert_ne!(c, a);
        assert_ne!(b, c);
    }

    // Test equality between 2 i8-containers with the same length
    {
        let a = DataVector::U8(vec![b'a', b'b', b'c', b'd']);
        let b = DataVector::U8(vec![b'a', b'b', b'c', b'd']);
        let c = DataVector::U8(vec![b'a', b'b', b'c', b'c']);
        assert_eq!(a, b);
        assert_ne!(c, a);
        assert_ne!(b, c);
    }

    // Test equality between 2 i16-containers with the same length
    {
        let a = DataVector::I16(vec![1, 2, 3, 4]);
        let b = DataVector::I16(vec![1, 2, 3, 4]);
        let c = DataVector::I16(vec![1, 2, 3, 3]);
        assert_eq!(a, b);
        assert_ne!(c, a);
        assert_ne!(b, c);
    }

    // Test equality between 2 i32-containers with the same length
    {
        let a = DataVector::I32(vec![1, 2, 3, 4]);
        let b = DataVector::I32(vec![1, 2, 3, 4]);
        let c = DataVector::I32(vec![1, 2, 3, 3]);
        assert_eq!(a, b);
        assert_ne!(c, a);
        assert_ne!(b, c);
    }

    // Test equality between 2 f32-containers with the same length
    {
        let a = DataVector::F32(vec![1.0, 2.0, 3.0, 4.0]);
        let b = DataVector::F32(vec![1.0, 2.0, 3.0, 4.0]);
        let c = DataVector::F32(vec![1.0, 2.0, 3.0, 3.0]);
        assert_eq!(a, b);
        assert_ne!(c, a);
        assert_ne!(b, c);
    }

    // Test equality between 2 f32-containers with the same length and containing NaN
    {
        let a = DataVector::F32(vec![1.0, 2.0, 3.0, std::f32::NAN]);
        let b = DataVector::F32(vec![1.0, 2.0, 3.0, std::f32::NAN]);
        let c = DataVector::F32(vec![1.0, 2.0, 3.0, 4.0]);
        assert_ne!(a, b);
        assert_ne!(c, a);
        assert_ne!(b, c);
    }

    // Test equality between 2 f64-containers with the same length
    {
        let a = DataVector::F64(vec![1.0, 2.0, 3.0, 4.0]);
        let b = DataVector::F64(vec![1.0, 2.0, 3.0, 4.0]);
        let c = DataVector::F64(vec![1.0, 2.0, 3.0, 3.0]);
        assert_eq!(a, b);
        assert_ne!(c, a);
        assert_ne!(b, c);
    }

    // Test equality between 2 f32-containers with the same length and containing NaN
    {
        let a = DataVector::F64(vec![1.0, 2.0, 3.0, std::f64::NAN]);
        let b = DataVector::F64(vec![1.0, 2.0, 3.0, std::f64::NAN]);
        let c = DataVector::F64(vec![1.0, 2.0, 3.0, 4.0]);
        assert_ne!(a, b);
        assert_ne!(c, a);
        assert_ne!(b, c);
    }

    // Test equality between 2 containers witht different data-types.
    {
        let data_i8 = DataVector::I8(vec![1, 2, 3, 4]);
        let data_u8 = DataVector::U8(vec![1, 2, 3, 4]);
        let data_i16 = DataVector::I16(vec![1, 2, 3, 4]);
        let data_i32 = DataVector::I32(vec![1, 2, 3, 4]);
        let data_f32 = DataVector::F32(vec![1.0, 2.0, 3.0, 4.0]);
        let data_f64 = DataVector::F64(vec![1.0, 2.0, 3.0, 4.0]);

        assert_ne!(data_i8, data_u8);
        assert_ne!(data_i8, data_i16);
        assert_ne!(data_i8, data_i32);
        assert_ne!(data_i8, data_f32);
        assert_ne!(data_i8, data_f64);

        assert_ne!(data_u8, data_i16);
        assert_ne!(data_u8, data_i32);
        assert_ne!(data_u8, data_f32);
        assert_ne!(data_u8, data_f64);

        assert_ne!(data_i16, data_i32);
        assert_ne!(data_i16, data_f32);
        assert_ne!(data_i16, data_f64);

        assert_ne!(data_i32, data_f32);
        assert_ne!(data_i32, data_f64);

        assert_ne!(data_f32, data_f64);
    }
}
