use super::*;
use tracing::trace;

impl MocObject for ObjectData {
    unsafe fn read_object(r: &MocReader) -> Result<Self, L2Error>
    where
        Self: Sized,
    {
        let type_id = r.read_var()?;
        match type_id {
            15 => Ok(ObjectData::ObjectArray { objects: r.read()? }),
            68 => Ok(ObjectData::RotationDeformer(r.read()?)),
            // _ => Err(L2Error::UnknownType { type_id: type_id as u32 }),
            _ => panic!("unknown type: {type_id}"),
        }
    }
}

impl MocObject for Vec<ObjectData> {
    unsafe fn read_object(r: &MocReader) -> Result<Self, L2Error>
    where
        Self: Sized,
    {
        let count = r.read_var()?;
        let mut objects = Vec::with_capacity(count);
        for _ in 0..count {
            objects.push(r.read()?);
        }
        Ok(objects)
    }
}
impl MocObject for String {
    unsafe fn read_object(r: &MocReader) -> Result<Self, L2Error>
    where
        Self: Sized,
    {
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
