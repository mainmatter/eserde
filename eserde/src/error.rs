#[derive(Debug)]
pub enum DeserializationError {
    MissingField { field_name: &'static str },
    Custom { message: String },
}

impl std::fmt::Display for DeserializationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DeserializationError::MissingField { field_name } => {
                write!(f, "missing field `{}`", field_name)
            }
            DeserializationError::Custom { message } => write!(f, "{}", message),
        }
    }
}
