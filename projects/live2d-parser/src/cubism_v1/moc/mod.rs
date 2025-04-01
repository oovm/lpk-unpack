mod affines;
mod deformers;
mod objects;
mod params;
mod parts;
mod pivots;

use self::parts::Part;
use crate::{
    cubism_v1::moc::{
        affines::Affine,
        deformers::{CurvedSurfaceDeformer, RotationDeformer},
        params::Parameter,
        pivots::{Pivot, PivotManager},
    },
    helpers::MocVersion,
    L2Error,
};
use integer_encoding::VarInt;
use std::{cell::RefCell, ops::AddAssign, slice::SliceIndex};
use tracing::debug;

pub struct Moc {
    /// The version of the moc file
    pub version: MocVersion,
    /// Parameter list
    pub parameters: Vec<Parameter>,
    /// Parts list
    pub parts: Vec<Part>,
    /// Canvas width
    pub canvas_width: i32,
    /// Canvas height
    pub canvas_height: i32,
}

#[derive(Debug)]
pub enum ObjectData {
    Null,
    ObjectArray(Vec<ObjectData>),
    RotationDeformer(RotationDeformer),
    CurvedSurfaceDeformer(CurvedSurfaceDeformer),
    Pivot(Pivot),
    PivotManager(PivotManager),
    Affine(Affine),
    Unknown50,
    Unknown51,
    Unknown60,
    Unknown134,
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
            // magic head
            reader.advance(3);
            // version
            reader.advance(1);
            // unknown
            reader.advance(5);
        }
        else {
            return Err(L2Error::UnknownError {});
        }
        let version = reader.read()?;
        let parameters = reader.read()?;
        let parts: ObjectData = reader.read()?;
        let canvas_width = reader.read()?;
        let canvas_height = reader.read()?;
        Ok(Self { version, parameters, parts: parts.as_parts(), canvas_width, canvas_height })
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
    pub unsafe fn version(&self) -> u8 {
        *self.moc.get_unchecked(3)
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
    #[track_caller]
    pub unsafe fn read<T: MocObject>(&self) -> Result<T, L2Error> {
        T::read_object(self)
    }
}

impl MocObject for MocVersion {
    unsafe fn read_object(reader: &MocReader) -> Result<Self, L2Error>
    where
        Self: Sized,
    {
        let v = match reader.moc.get_unchecked(3) {
            6 => MocVersion::V2_6_INTIAL,
            7 => MocVersion::V2_7_OPACITY,
            8 => MocVersion::V2_8_TEX_OPTION,
            9 => MocVersion::V2_9_AVATAR_PARTS,
            10 => MocVersion::V2_10_SDK2,
            11 => MocVersion::V2_11_SDK2_1,
            _ => Err(L2Error::UnknownError {})?,
        };
        Ok(v)
    }
}
