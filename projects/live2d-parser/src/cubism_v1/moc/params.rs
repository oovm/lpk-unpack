use crate::{
    cubism_v1::moc::{MocObject, MocReader},
    L2Error,
};
use tracing::debug;

#[derive(Debug)]
pub struct ParameterList {
    items: Vec<u32>,
}

#[derive(Debug)]
pub struct Parameter {
    pub _align: [u8; 2],
    /// Parameter name
    pub id: String,
    pub min_value: f32,
    pub max_value: f32,
    /// Default value
    pub default_value: f32,
}

impl MocObject for Vec<Parameter> {
    unsafe fn read_object(r: &MocReader) -> Result<Vec<Parameter>, L2Error>
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

impl MocObject for Parameter {
    unsafe fn read_object(r: &MocReader) -> Result<Parameter, L2Error>
    where
        Self: Sized,
    {
        let align = r.read()?;
        let max_value = r.read()?;
        let min_value = r.read()?;
        let default_value = r.read()?;
        let name = r.read()?;
        Ok(Parameter { _align: align, id: name, min_value, max_value, default_value })
    }
}

