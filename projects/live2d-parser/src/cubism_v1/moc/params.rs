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
pub struct Parameter<'i> {
    pub _align: [u8; 3],
    /// Parameter name
    pub name: &'i str,
    pub min_value: f32,
    pub max_value: f32,
    /// Default value
    pub default_value: f32,
}

impl<'i> Parameter<'i> {}

impl<'i> MocReader<'i> {
    pub unsafe fn read_parameters(&'i self) -> Result<Vec<Parameter<'i>>, L2Error> {
        let mut params = Vec::new();
        let count = self.read_var()?;
        debug!("Find parameters: {}", count);
        for _ in 0..count {
            params.push(self.read::<Parameter>()?)
        }
        Ok(params)
    }
}

impl<'i> MocObject<'i> for Parameter<'i> {
    unsafe fn read_object(r: &'i MocReader) -> Result<Self, L2Error>
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
