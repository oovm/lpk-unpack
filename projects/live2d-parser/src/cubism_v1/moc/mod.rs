mod params;
mod parts;

use self::parts::Part;
use crate::{cubism_v1::moc::params::Parameter, L2Error};
use integer_encoding::VarInt;
use serde::de::Error;
use std::{cell::RefCell, ops::AddAssign, slice::SliceIndex};

pub struct Moc<'i> {
    /// The version of the moc file
    version: u8,
    /// Parameter list
    params: Vec<Parameter<'i>>,
    /// Parts list
    parts: Vec<Part<'i>>,
    /// Canvas width
    canvas_width: i32,
    /// Canvas height
    canvas_height: i32,
}

#[derive(Debug)]
pub enum ObjectData {
    ObjectArray { objects: Vec<ObjectData> },
    Unknown { type_id: u64 },
}

impl<'i> Moc<'i> {
    /// Parse moc data from a byte array
    ///
    /// ## Safety
    /// The input data must be a valid moc file
    pub unsafe fn new(data: &'i [u8]) -> Result<Self, L2Error> {
        // Parse parameters and parts
        let mut reader = MocReader { moc: data, ptr: RefCell::new(0) };
        reader.advance(9);
        let params= reader.read_parameters()?;
        Ok(Self { version: 0, params, parts: vec![], canvas_width: 0, canvas_height: 0 })
    }

    /// Get the version of the moc file
    pub fn version(&self) -> u8 {
        self.version
    }

    /// Get the parameter list
    pub fn parameters(&self) -> &[Parameter] {
        &[]
    }

    /// Get the parts list
    pub fn parts(&self) -> &[Part] {
        &self.parts
    }

    /// Get the canvas width
    pub fn canvas_width(&self) -> i32 {
        self.canvas_width
    }

    /// Get the canvas height
    pub fn canvas_height(&self) -> i32 {
        self.canvas_height
    }
}

struct MocReader<'i> {
    moc: &'i [u8],
    ptr: RefCell<usize>,
}

trait MocObject<'i> {
    unsafe fn read_object(reader: &'i MocReader) -> Result<Self, L2Error>
    where
        Self: Sized;
}

impl<'i> MocReader<'i> {
    pub unsafe fn new(moc: &'i [u8]) -> Self {
        Self { moc, ptr: RefCell::new(0) }
    }
    pub unsafe fn rest(&self) -> &[u8] {
        let offset = self.ptr.borrow();
        self.moc.get_unchecked(*offset..)
    }
    pub unsafe fn view(&self, slice: impl SliceIndex<[u8], Output = [u8]>) -> &[u8] {
        self.rest().get_unchecked(slice)
    }
    pub fn advance(&self, n: usize) {
        self.ptr.borrow_mut().add_assign(n)
    }
    pub unsafe fn read_var(&self) -> Result<usize, L2Error> {
        match usize::decode_var(self.rest()) {
            Some((s, delta)) => {
                self.advance(delta);
                Ok(s)
            }
            None => Err(L2Error::Error {}),
        }
    }
    pub unsafe fn read<T: MocObject<'i>>(&'i self) -> Result<T, L2Error> {
        T::read_object(self)
    }

    pub unsafe fn read_str(&'i self) -> Result<&'i str, L2Error> {
        let length = self.read_var()?;
        // tracing::trace!("String Length: {length}");
        let str = std::str::from_utf8_unchecked(self.view(..length));
        self.advance(length);
        Ok(str)
    }
}

impl<'i> MocObject<'i> for ObjectData {
    unsafe fn read_object(reader: &'i MocReader) -> Result<Self, L2Error>
    where
        Self: Sized
    {
        let (type_id, rest) = match u64::decode_var(bytes) {
            Some((s, delta)) => (s, bytes.get_unchecked(delta..)),
            None => Err(crate::errors::L2Error::Error {})?,
        };
        match type_id {
            15 => {
                let (objects, rest) = read_object_array(rest)?;
                Ok(ObjectData::ObjectArray { objects })
            }
            _ => Err(serde_json::Error::custom(format!("Unknown type {}", type_id))),
        }
    }
}

unsafe fn read_object_array(bytes: &[u8]) -> Result<Vec<ObjectData>, L2Error> {
    match u64::decode_var(bytes) {
        Some((s, delta)) => Ok(vec![ObjectData::Unknown { type_id: s as u64 }]),
        None => Err(serde_json::Error::custom("Invalid string length"))?,
    }
}
