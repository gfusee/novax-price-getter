use crate::errors::app::AppError;

#[derive(Debug)]
pub enum XExchangeError {
    PairNotFound { first_token_identifier: String, second_token_identifier: String }
}

impl From<XExchangeError> for AppError {
    fn from(value: XExchangeError) -> Self {
        AppError::XExchange(value)
    }
}