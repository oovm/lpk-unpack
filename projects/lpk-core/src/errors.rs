use std::fmt::{Display, Formatter};
use thiserror::__private::AsDisplay;

#[derive(Debug, Clone)]
pub enum LpkError {
    IoError(String),

    ZipError(String),

    JsonError(String),

    ConfigMissing,

    UnsupportedLpkType(String),

    DecryptionFailed(String),

    UnknownError,
}

impl Display for LpkError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LpkError::IoError(_0) => match (_0.as_display(),) {
                (__display0,) => f.write_fmt(format_args!("IO错误: {__display0}", __display0 = __display0)),
            },
            LpkError::ZipError(_0) => match (_0.as_display(),) {
                (__display0,) => f.write_fmt(format_args!("ZIP错误: {__display0}", __display0 = __display0)),
            },
            LpkError::JsonError(_0) => match (_0.as_display(),) {
                (__display0,) => f.write_fmt(format_args!("JSON解析错误: {__display0}", __display0 = __display0)),
            },
            LpkError::ConfigMissing {} => f.write_str("配置文件缺失"),
            LpkError::UnsupportedLpkType(_0) => match (_0.as_display(),) {
                (__display0,) => f.write_fmt(format_args!("不支持的LPK类型: {__display0}", __display0 = __display0)),
            },
            LpkError::DecryptionFailed(_0) => match (_0.as_display(),) {
                (__display0,) => f.write_fmt(format_args!("解密失败: {__display0}", __display0 = __display0)),
            },
            LpkError::UnknownError {} => f.write_str("未知错误"),
        }
    }
}

impl From<std::io::Error> for LpkError {
    fn from(err: std::io::Error) -> Self {
        LpkError::IoError(err.to_string())
    }
}

impl From<zip::result::ZipError> for LpkError {
    fn from(err: zip::result::ZipError) -> Self {
        LpkError::ZipError(err.to_string())
    }
}

impl From<serde_json::Error> for LpkError {
    fn from(err: serde_json::Error) -> Self {
        LpkError::JsonError(err.to_string())
    }
}

pub type Result<T> = std::result::Result<T, LpkError>;
