mod element_count;
mod params;

pub use self::element_count::ElementCountTable;

#[derive(Clone)]
pub struct Moc3 {
    raw: Vec<u8>,
    counter: ElementCountTable,
}

#[repr(u8)]
#[derive(Eq, PartialEq, Ord, PartialOrd)]
pub enum Moc3Version {
    V30 = 1,
    V33 = 2,
    V40 = 3,
    V42 = 4,
    V50 = 5,
}

impl Moc3 {
    pub fn new(moc3: Vec<u8>) -> Result<Moc3, serde_json::Error> {
        let counter = ElementCountTable::read(&moc3)?;
        Ok(Moc3 { raw: moc3, counter })
    }
    /// Should always be "MOC3"
    pub fn magic(&self) -> &str {
        // 0x00000000-0x00000004
        unsafe { std::str::from_utf8_unchecked(&self.raw.get_unchecked(0..4)) }
    }
    /// The version of the Moc3 file
    pub fn version(&self) -> Moc3Version {
        // 0x00000005
        unsafe {
            match self.raw.get_unchecked(4) {
                1 => Moc3Version::V30,
                2 => Moc3Version::V33,
                3 => Moc3Version::V40,
                4 => Moc3Version::V42,
                5 => Moc3Version::V50,
                _ => panic!("Unknown Moc3 version"),
            }
        }
    }
    pub fn element_count(&self) -> ElementCountTable {
        self.counter
    }
}
