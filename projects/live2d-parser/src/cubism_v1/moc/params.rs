use crate::{
    cubism_v1::moc::{MocObject, MocReader},
    L2Error,
};
use integer_encoding::VarInt;
use serde::de::Error;
use tracing::debug;

#[derive(Debug)]
pub struct ParameterList {
    items: Vec<u32>,
}

#[derive(Debug)]
pub struct Parameter {
    pub _align: [u8; 3],
    /// Parameter name
    pub name: String,
    pub min_value: f32,
    pub max_value: f32,
    /// Default value
    pub default_value: f32,
}

impl MocObject for Vec<Parameter> {
    unsafe fn read_object(r: &'i MocReader<'i>) -> Result<Vec<Parameter<'i>>, L2Error>
    where
        Self: Sized,
    {
        let count = r.read_var()?;
        let mut params = Vec::with_capacity(count);
        debug!("Find parameters: {}", count);
        for _ in 0..count {
            params.push(r.read()?)
        }
        Ok(params)
    }
}

impl<'i> MocObject<'i> for Parameter<'i> {
    unsafe fn read_object(r: &'i MocReader) -> Result<Parameter<'i>, L2Error>
    where
        Self: Sized,
    {
        let align = r.read::<[u8; 3]>()?;
        let max_value = r.read()?;
        let min_value = r.read()?;
        let default_value = r.read()?;
        let name = r.read_str()?;
        Ok(Parameter { _align: align, name, min_value, max_value, default_value })
    }
}

impl<'i> MocObject<'i> for f32 {
    unsafe fn read_object(r: &MocReader) -> Result<Self, L2Error>
    where
        Self: Sized,
    {
        let float = std::ptr::read(r.rest().as_ptr() as *const f32);
        r.advance(4);
        Ok(float)
    }
}

impl<'i, const N: usize> MocObject<'i> for [u8; N] {
    unsafe fn read_object(r: &MocReader) -> Result<Self, L2Error>
    where
        Self: Sized,
    {
        if r.rest().len() < N {
            return Err(L2Error::OutOfBounds { rest: r.rest().len(), request: N });
        }
        let array = std::ptr::read(r.rest().as_ptr() as *const [u8; N]);
        r.advance(N);
        Ok(array)
    }
}
