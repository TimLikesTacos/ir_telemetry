use std::error;
use std::fmt;
pub type Result<T> = std::result::Result<T, IrError>;

#[derive(Debug)]
pub enum IrError {
    MemMappingError(String),
    TypeError,
    VariableNotFound(String),
    ExceedsVariableRange,
}

impl fmt::Display for IrError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            IrError::MemMappingError(ref message) => {
                write!(f, "Failure to access memory-mapped file: {}", message)
            }
            IrError::TypeError => write!(f, "Could not convert value to requested type"),
            IrError::VariableNotFound(ref var_name) => {
                write!(f, "Variable {} not found in data", var_name)
            }
            IrError::ExceedsVariableRange => {
                write!(f, "Variable exceeds range for type")
            }
        }
    }
}

impl error::Error for IrError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}
