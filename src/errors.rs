use failure::Fail;

pub type Result<T> = std::result::Result<T, failure::Error>;

#[derive(Fail, Debug)]
pub enum ThothError {
    #[fail(display = "{} is not a valid {} code", _0, _1)]
    InvalidSubjectCode(String, String),
    #[fail(display = "Database error: {}", _0)]
    DatabaseError(String),
    #[fail(display = "Internal error: {}", _0)]
    InternalError(String),
}

impl juniper::IntoFieldError for ThothError {
    fn into_field_error(self) -> juniper::FieldError {
        match self {
            ThothError::InvalidSubjectCode { .. } => juniper::FieldError::new(
                self.to_string(),
                graphql_value!({
                    "type": "INVALID_SUBJECT_CODE"
                }),
            ),
            _ => juniper::FieldError::new(
                self.to_string(),
                graphql_value!({
                    "type": "INTERNAL_ERROR"
                }),
            ),
        }
    }
}

impl From<std::io::Error> for ThothError {
    fn from(error: std::io::Error) -> ThothError {
        ThothError::InternalError(error.to_string())
    }
}

impl From<reqwest::Error> for ThothError {
    fn from(error: reqwest::Error) -> ThothError {
        ThothError::InternalError(error.to_string())
    }
}

impl From<xml::writer::Error> for ThothError {
    fn from(error: xml::writer::Error) -> ThothError {
        ThothError::InternalError(error.to_string())
    }
}

impl From<failure::Error> for ThothError {
    fn from(error: failure::Error) -> ThothError {
        if error.downcast_ref::<ThothError>().is_some() {
            return error.downcast::<ThothError>().unwrap();
        }
        ThothError::InternalError(error.to_string())
    }
}
