use thiserror::Error;

#[derive(Error, Debug)]
pub enum HttpError {
    #[error("Request failed, status: {0} res: {1}")]
    Status(u16, String),
}
