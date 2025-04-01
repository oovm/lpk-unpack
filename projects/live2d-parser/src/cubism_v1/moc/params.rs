use crate::cubism_v1::moc::Moc;
use integer_encoding::VarInt;
use serde::de::Error;

#[derive(Debug)]
pub struct Parameter {
    /// Parameter name
    pub name: String,
    /// Default value
    pub default_value: f32,
}

#[derive(Debug)]
pub struct ParamList {
    params: Vec<Parameter>,
}

impl ParamList {
    /// Parse parameter list from moc data
    ///
    /// ## Safety
    /// The input data must be a valid moc file
    pub unsafe fn parse(data: &[u8], offset: usize) -> Result<Self, serde_json::Error> {
        let mut params = Vec::new();
        let mut current_offset = 0x00000027;

        // Read parameter count using variable-length encoding
        let out = u32::decode_var(&data[0x00000019..]);
        println!("count: {:?}", out);
        println!("count: {:?}", read_str(&data[0x00000019..])?.0);
        let out = i32::decode_var(&data[0x00000027..]);
        println!("count: {:?}", out);

        // current_offset += 1;
        let min_value = std::ptr::read(data.as_ptr().add(current_offset) as *const i32);
        let max_value = std::ptr::read(data.as_ptr().add(current_offset + 4) as *const f32);
        let default_value = std::ptr::read(data.as_ptr().add(current_offset + 8) as *const f32);
        let dd_value = std::ptr::read(data.as_ptr().add(current_offset + 12) as *const f32);

        println!("min_value: {}, max_value: {}, default_value: {}, {dd_value}", min_value, max_value, default_value);

        Ok(Self { params })
    }

    /// Get all parameters
    pub fn parameters(&self) -> &[Parameter] {
        &self.params
    }
}

unsafe fn read_str(bytes: &[u8]) -> Result<(&str, &[u8]), serde_json::Error> {
    let (length, delta) = match u64::decode_var(bytes) {
        Some(s) => s,
        None => Err(serde_json::Error::custom("Invalid string length"))?,
    };
    let end = delta + length as usize;
    let str = std::str::from_utf8_unchecked(bytes.get_unchecked(delta..end));
    let rest = bytes.get_unchecked(end..);
    Ok((str, rest))
}
