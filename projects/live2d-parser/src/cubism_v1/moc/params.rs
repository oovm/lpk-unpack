use crate::cubism_v1::moc::read_str;
use integer_encoding::VarInt;
use serde::de::Error;

#[derive(Debug)]
pub struct ParameterList {
    items: Vec<u32>,
}

#[derive(Debug)]
pub struct Parameter<'i> {
    pub _align: [u8; 3],
    /// Parameter name
    pub name: &'i str,
    pub min_value: f32,
    pub max_value: f32,
    /// Default value
    pub default_value: f32,
}

impl<'i> Parameter<'i> {
    pub unsafe fn parse_many(data: &[u8]) -> Result<(Vec<Self>, &[u8]), serde_json::Error> {
        let mut params = Vec::new();
        let (count, delta) = match u64::decode_var(data) {
            Some(s) => s,
            None => Err(serde_json::Error::custom("Invalid string length"))?,
        };
        let mut rest = data.get_unchecked(delta..);
        for _ in 0..count {
            let out = Parameter::parse_one(rest)?;
            println!("{:#?}", out.0);
            rest = out.1;
        }
        Ok((params, rest))
    }
    pub unsafe fn parse_one(data: &'i [u8]) -> Result<(Parameter<'i>, &'i [u8]), serde_json::Error> {
        let align = std::ptr::read(data.as_ptr().add(0x0) as *const [u8; 3]);
        let min_value = std::ptr::read(data.as_ptr().add(0x0 + 3) as *const f32);
        let max_value = std::ptr::read(data.as_ptr().add(0x4 + 3) as *const f32);
        let default_value = std::ptr::read(data.as_ptr().add(0x8 + 3) as *const f32);
        let (name, rest) = read_str(data.get_unchecked(0xC + 3..))?;
        Ok((Self { _align: align, name, min_value, max_value, default_value }, rest))
    }
}
