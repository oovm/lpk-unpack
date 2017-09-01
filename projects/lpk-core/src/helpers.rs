use std::path::Path;

/// 计算字符串的MD5哈希值，返回十六进制字符串
pub fn hashed_filename(s: &str) -> String {
    let hash = md5::compute(s);
    format!("{:x}", hash)
}

/// 安全地创建目录，如果目录已存在则不会报错
pub fn safe_mkdir<P: AsRef<Path>>(path: P) -> std::io::Result<()> {
    std::fs::create_dir_all(path)
}

/// 生成解密密钥
pub fn make_key(s: &str) -> i128 {
    let mut ret = 0;
    for c in s.chars() {
        ret = (ret * 31 + c as i128) & 0xffffffff;
    }
    if ret & 0x80000000 != 0 {
        ret |= 0xffffffff00000000;
    }
    ret
}

/// 解密数据
pub fn decrypt(key: i128, data: &[u8]) -> Vec<u8> {
    let mut ret = Vec::with_capacity(data.len());

    // 按1024字节分片处理
    for chunk in data.chunks(1024) {
        let mut k = key;
        for &byte in chunk {
            k = (65535 & 2531011 + 214013 * k >> 16) & 0xffffffff;
            ret.push((k & 0xff) as u8 ^ byte);
        }
    }

    ret
}

/// 检查文件名是否是加密文件（32位十六进制字符串，可能以.bin或.bin3结尾）
pub fn is_encrypted_file(s: &str) -> bool {
    if s.len() < 32 {
        return false;
    }

    let hex_part = if s.ends_with(".bin") || s.ends_with(".bin3") { &s[..s.len() - 4] } else { s };

    if hex_part.len() != 32 {
        return false;
    }

    hex_part.chars().all(|c| c.is_ascii_hexdigit())
}

/// 在字符串中查找加密文件名
pub fn find_encrypted_file(s: &str) -> Option<String> {
    // 简化实现，实际应该使用正则表达式
    for word in s.split_whitespace() {
        if is_encrypted_file(word) {
            return Some(word.to_string());
        }
    }
    None
}

/// 从命令字符串中获取加密文件名
pub fn get_encrypted_file(s: &str) -> Option<String> {
    if s.starts_with("change_cos ") {
        let filename = &s["change_cos ".len()..];
        if is_encrypted_file(filename) {
            return Some(filename.to_string());
        }
    }
    else if is_encrypted_file(s) {
        return Some(s.to_string());
    }
    None
}
