use novax::errors::NovaXError;
use crate::errors::xexchange::XExchangeError;

#[derive(PartialEq, Clone, Debug)]
pub enum AppError {
    XExchange(XExchangeError),
    NovaXError(NovaXError)
}

impl From<NovaXError> for AppError {
    fn from(value: NovaXError) -> Self {
        AppError::NovaXError(value)
    }
}