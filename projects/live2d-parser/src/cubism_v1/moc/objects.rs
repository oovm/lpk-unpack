use super::*;
use tracing::{trace, warn};

impl MocObject for Vec<ObjectData> {
    unsafe fn read_object(r: &MocReader) -> Result<Self, L2Error>
    where
        Self: Sized,
    {
        let count = r.read_var()?;
        let mut objects = Vec::with_capacity(count);
        trace!("Find objects: {}", count);
        for _ in 0..count {
            objects.push(r.read()?);
        }
        Ok(objects)
    }
}

impl MocObject for ObjectData {
    unsafe fn read_object(r: &MocReader) -> Result<Self, L2Error>
    where
        Self: Sized,
    {
        let type_id = r.read_var()?;
        // trace!("preview: {:?}", r.view(..8));
        let data = match type_id {
            0 => ObjectData::Null,
            // 3 => ObjectData::Byte(r.read()?),
            15 => ObjectData::ObjectArray(r.read()?),
            65 => ObjectData::CurvedSurfaceDeformer(r.read()?),
            66 => ObjectData::PivotManager(r.read()?),
            67 => ObjectData::Pivot(r.read()?),
            68 => ObjectData::RotationDeformer(r.read()?),
            69 => ObjectData::Affine(r.read()?),
            // _ => Err(L2Error::UnknownType { type_id: type_id as u32 })?,
            _ => panic!("unknown type: {type_id}"),
        };
        Ok(data)
    }
}

impl MocObject for String {
    unsafe fn read_object(r: &MocReader) -> Result<Self, L2Error>
    where
        Self: Sized,
    {
        let ty = r.read_var()?;
        if ty != 60 {
            warn!("String Type: {ty}");
        }
        let length = r.read_var()?;
        // tracing::trace!("String Length: {length}");
        let str = String::from_utf8_lossy(r.view(..length));
        r.advance(length);
        Ok(str.to_string())
    }
}
impl<const N: usize> MocObject for [u8; N] {
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

impl MocObject for i32 {
    unsafe fn read_object(r: &MocReader) -> Result<Self, L2Error>
    where
        Self: Sized,
    {
        let float = std::ptr::read(r.rest().as_ptr() as *const i32);
        r.advance(4);
        Ok(float)
    }
}

impl MocObject for Vec<f32> {
    unsafe fn read_object(reader: &MocReader) -> Result<Self, L2Error>
    where
        Self: Sized,
    {
        let count = reader.read_var()?;
        let mut values = Vec::with_capacity(count);
        trace!("Find float values: {}", count);
        for _ in 0..count {
            values.push(reader.read()?);
        }
        Ok(values)
    }
}

impl MocObject for f32 {
    unsafe fn read_object(r: &MocReader) -> Result<Self, L2Error>
    where
        Self: Sized,
    {
        let float = std::ptr::read(r.rest().as_ptr() as *const f32);
        r.advance(4);
        Ok(float)
    }
}

impl MocObject for u8 {
    unsafe fn read_object(r: &MocReader) -> Result<Self, L2Error>
    where
        Self: Sized,
    {
        let float = std::ptr::read(r.rest().as_ptr());
        r.advance(1);
        Ok(float)
    }
}
