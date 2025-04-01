mod objects;
mod params;
mod parts;
mod deformers;

use self::parts::Part;
use crate::{cubism_v1::moc::params::Parameter, L2Error};
use integer_encoding::VarInt;
use serde::de::Error;
use std::{cell::RefCell, ops::AddAssign, slice::SliceIndex};
use crate::cubism_v1::moc::deformers::RotationDeformer;

pub struct Moc {
    /// The version of the moc file
    version: u8,
    /// Parameter list
    pub parameter: Vec<Parameter>,
    /// Parts list
    pub parts: Vec<Part>,
    /// Canvas width
    canvas_width: i32,
    /// Canvas height
    canvas_height: i32,
}

#[derive(Debug)]
pub enum ObjectData {
    ObjectArray { objects: Vec<ObjectData> },
    RotationDeformer(RotationDeformer),
    Unknown { type_id: u64 },
}

impl Moc {
    /// Parse moc data from a byte array
    ///
    /// ## Safety
    /// The input data must be a valid moc file
    pub unsafe fn new(data: &[u8]) -> Result<Moc, L2Error> {
        // Parse parameters and parts
        let reader = MocReader { moc: data, ptr: RefCell::new(0) };
        reader.advance(9);
        Ok(Self { version: 0, parameter: reader.read()?, parts: reader.read()?, canvas_width: 0, canvas_height: 0 })
    }

    /// Get the version of the moc file
    pub fn version(&self) -> u8 {
        self.version
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

trait MocObject {
    unsafe fn read_object(reader: &MocReader) -> Result<Self, L2Error>
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
    pub unsafe fn read<T: MocObject>(&self) -> Result<T, L2Error> {
        T::read_object(self)
    }
}
