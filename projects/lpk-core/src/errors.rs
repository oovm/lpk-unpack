#[derive(Debug, Copy, Clone)]
pub enum LpkError {
    UnknownError
}

pub type Result<T> = std::result::Result<T, LpkError>;
