use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub enum LpkError {
    IoError { path: String, message: String },

    ZipError(String),

    DecodeError { format: String, message: String },

    ConfigMissing,

    UnsupportedLpkType(String),

    DecryptionFailed(String),

    UnknownError,
}

impl Display for LpkError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LpkError::IoError { path, message } => {
                write!(f, "IO错误: {path} {message}", path = path, message = message)
            }
            LpkError::ZipError(e) => {
                write!(f, "Zip错误: {e}", e = e)
            }
            LpkError::DecodeError { format, message } => {
                write!(f, "解码错误: {format} {message}", format = format, message = message)
            }
            LpkError::ConfigMissing {} => f.write_str("配置文件缺失"),
            LpkError::UnsupportedLpkType(e) => {
                write!(f, "不支持的LPK类型: {e}", e = e)
            }
            LpkError::DecryptionFailed(e) => {
                write!(f, "解密失败: {e}", e = e)
            }
            LpkError::UnknownError {} => f.write_str("未知错误"),
        }
    }
}

impl From<std::io::Error> for LpkError {
    #[track_caller]
    fn from(e: std::io::Error) -> Self {
        let loc = std::panic::Location::caller();
        LpkError::IoError { path: loc.to_string(), message: e.to_string() }
    }
}

impl From<zip::result::ZipError> for LpkError {
    fn from(err: zip::result::ZipError) -> Self {
        LpkError::ZipError(err.to_string())
    }
}

impl From<serde_json::Error> for LpkError {
    fn from(e: serde_json::Error) -> Self {
        LpkError::DecodeError { format: "json".to_string(), message: e.to_string() }
    }
}

pub type Result<T> = std::result::Result<T, LpkError>;
