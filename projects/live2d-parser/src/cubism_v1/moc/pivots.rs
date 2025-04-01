use crate::{
    cubism_v1::moc::{MocObject, MocReader, ObjectData},
    L2Error,
};
use tracing::{debug, info, trace, warn};

#[derive(Debug)]
pub struct PivotManager {
    pub items: Vec<Pivot>,
}

#[derive(Debug)]
pub struct Pivot {
    pub id: String,
    pub count: i32,
    pub values: Box<ObjectData>,
}

impl MocObject for PivotManager {
    unsafe fn read_object(reader: &MocReader) -> Result<Self, L2Error>
    where
        Self: Sized,
    {
        let o: ObjectData = reader.read()?;

        Ok(Self { items: o.as_pivots() })
    }
}

impl MocObject for Vec<Pivot> {
    unsafe fn read_object(reader: &MocReader) -> Result<Self, L2Error>
    where
        Self: Sized,
    {
        let count = reader.read_var()?;
        let mut pivots = Vec::with_capacity(count as usize);
        debug!("Find pivots: {}", count);
        for _ in 0..count {
            pivots.push(reader.read()?);
        }
        Ok(pivots)
    }
}

impl MocObject for Pivot {
    unsafe fn read_object(reader: &MocReader) -> Result<Self, L2Error>
    where
        Self: Sized,
    {
        let id = reader.read()?;
        let count = reader.read()?;
        let values = reader.read()?;
        Ok(Self { id, count, values: Box::new(values) })
    }
}
impl ObjectData {
    pub fn as_pivots(self) -> Vec<Pivot> {
        match self {
            ObjectData::Null => Vec::new(),
            ObjectData::ObjectArray(o) => o.into_iter().map(|x| x.as_pivots()).flatten().collect(),
            ObjectData::Pivot(v) => vec![v],
            ObjectData::PivotManager(v) => v.items,
            s => {
                warn!("ObjectData::as_pivots() called on non-pivot object {s:?}");
                vec![]
            }
        }
    }
}


