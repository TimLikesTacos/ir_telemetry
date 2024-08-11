use crate::mapped_file::var_header::VarHeaderGeneric;
use crate::types::IrValue;
use num_derive::FromPrimitive;
use std::fmt::Display;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, FromPrimitive)]
pub(crate) enum DataVarType {
    Char = 0,
    Bool = 1,
    Int = 2,
    BitField = 3,
    Float = 4,
    Double = 5,
    ETCount = 6,
}

impl DataVarType {
    pub fn amount_of_bytes(&self) -> usize {
        match self {
            DataVarType::Char => 1,
            DataVarType::Bool => 1,
            DataVarType::Int => 4,
            DataVarType::BitField => 4,
            DataVarType::Float => 4,
            DataVarType::Double => 8,
            DataVarType::ETCount => std::mem::size_of::<usize>(),
        }
    }
}

impl Display for DataVarType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DataVarType::Char => write!(f, "Char"),
            DataVarType::Bool => write!(f, "Bool"),
            DataVarType::Int => write!(f, "Int"),
            DataVarType::BitField => write!(f, "BitField"),
            DataVarType::Float => write!(f, "Float"),
            DataVarType::Double => write!(f, "Double"),
            DataVarType::ETCount => write!(f, "ETCount"),
        }
    }
}

#[inline]
fn value_from_bytes<T>(ptr: *const T) -> T {
    unsafe { std::ptr::read_unaligned(ptr) }
}

fn value_from_bytes_ptr_offset_count<T>(
    buffer: &[u8],
    offset: usize,
    additional_offset: usize,
) -> T {
    let ptr = unsafe { buffer.as_ptr().add(offset + additional_offset) as *const T };
    value_from_bytes(ptr)
}

impl<'a, S, D> From<(&'a VarHeaderGeneric<S, D>, &'a [u8])> for IrValue {
    fn from((var_header, buffer): (&VarHeaderGeneric<S, D>, &[u8])) -> Self {
        let count = var_header.count as usize;
        let offset = var_header.offset as usize;
        let size = var_header._type.amount_of_bytes();
        debug_assert!(
            offset + size < buffer.len(),
            "Offset larger than buffer length"
        );
        if count == 1 {
            match var_header._type {
                DataVarType::Char => {
                    IrValue::Char(value_from_bytes_ptr_offset_count(buffer, offset, 0))
                }
                DataVarType::Bool => {
                    IrValue::Bool(value_from_bytes_ptr_offset_count(buffer, offset, 0))
                }
                DataVarType::Int => {
                    IrValue::Int(value_from_bytes_ptr_offset_count(buffer, offset, 0))
                }
                DataVarType::BitField => {
                    IrValue::BitField(value_from_bytes_ptr_offset_count(buffer, offset, 0))
                }
                DataVarType::Float => {
                    IrValue::Float(value_from_bytes_ptr_offset_count(buffer, offset, 0))
                }
                DataVarType::Double => {
                    IrValue::Double(value_from_bytes_ptr_offset_count(buffer, offset, 0))
                }
                DataVarType::ETCount => IrValue::ETCount,
            }
        } else {
            let mut array = Vec::with_capacity(count);
            for i in 0..count {
                match var_header._type {
                    DataVarType::Char => array.push(IrValue::Char(
                        value_from_bytes_ptr_offset_count(buffer, offset, size * i),
                    )),
                    DataVarType::Bool => array.push(IrValue::Bool(
                        value_from_bytes_ptr_offset_count(buffer, offset, size * i),
                    )),
                    DataVarType::Int => array.push(IrValue::Int(
                        value_from_bytes_ptr_offset_count(buffer, offset, size * i),
                    )),
                    DataVarType::BitField => array.push(IrValue::BitField(
                        value_from_bytes_ptr_offset_count(buffer, offset, size * i),
                    )),
                    DataVarType::Float => array.push(IrValue::Float(
                        value_from_bytes_ptr_offset_count(buffer, offset, size * i),
                    )),
                    DataVarType::Double => array.push(IrValue::Double(
                        value_from_bytes_ptr_offset_count(buffer, offset, size * i),
                    )),
                    DataVarType::ETCount => array.push(IrValue::ETCount),
                }
            }
            IrValue::Array(array)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value_from_bytes() {
        let numbers: [u8; 4] = [0x78, 0x56, 0x34, 0x12];
        let ptr = numbers.as_ptr() as *const u32;
        let value: u32 = value_from_bytes(ptr);
        assert_eq!(value, 0x12345678);
    }

    #[test]
    fn test_value_from_bytes_ptr_offset_count() {
        let numbers: Vec<u8> = vec![
            0, 0, 0, 0, // padding
            0x78, 0x56, 0x34, 0x12,
        ]; // target u32
        let value: u32 = value_from_bytes_ptr_offset_count(&numbers, 1, 3);
        assert_eq!(value, 0x12345678);
    }
}
