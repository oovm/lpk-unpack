mod deformers;
mod objects;
mod params;
mod parts;
mod pivots;

use self::parts::Part;
use crate::{
    cubism_v1::moc::{deformers::RotationDeformer, params::Parameter, pivots::PivotManager},
    L2Error,
};
use integer_encoding::VarInt;
use std::{cell::RefCell, ops::AddAssign, slice::SliceIndex};
use tracing::debug;

pub struct Moc {
    /// The version of the moc file
    pub version: u8,
    /// Parameter list
    pub parameter: Vec<Parameter>,
    /// Parts list
    pub parts: Vec<Part>,
    /// Canvas width
    pub canvas_width: i32,
    /// Canvas height
    pub canvas_height: i32,
}

#[derive(Debug)]
pub enum ObjectData {
    ObjectArray(Vec<ObjectData>),
    RotationDeformer(RotationDeformer),
    PivotManager(PivotManager),
    Unknown { type_id: u64 },
}

impl Moc {
    /// Parse moc data from a byte array
    ///
    /// ## Safety
    /// The input data must be a valid moc file
    pub unsafe fn new(data: &[u8]) -> Result<Moc, L2Error> {
        let reader = MocReader { moc: data, ptr: RefCell::new(0) };
        if reader.moc.get_unchecked(..3) == b"moc" {
            reader.advance(3);
        }
        else {
            return Err(L2Error::UnknownError {});
        }
        let version = reader.read()?;
        reader.advance(5);
        Ok(Self { version, parameter: reader.read()?, parts: reader.read()?, canvas_width: 0, canvas_height: 0 })
    }

    /// Get the version of the moc file
    pub fn version(&self) -> u8 {
        self.version
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
            None => Err(L2Error::UnknownError {}),
        }
    }
    pub unsafe fn read<T: MocObject>(&self) -> Result<T, L2Error> {
        T::read_object(self)
    }
}
