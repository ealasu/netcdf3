

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReadError {
    UnexpectedError,
    IOError(std::io::ErrorKind),
    VariableNotDefined(String),
}