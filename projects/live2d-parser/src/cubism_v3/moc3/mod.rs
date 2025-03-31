pub struct Moc3 {
    raw: Vec<u8>,
}

#[repr(u8)]
pub enum Moc3Version {
    V30 = 1,
    V33 = 2,
    V40 = 3,
    V42 = 4,
    V50 = 5,
}

impl Moc3 {
    /// Should always be "MOC3"
    pub fn magic(&self) -> &str {
        // 0-4 byte
        unsafe { std::str::from_utf8_unchecked(&self.raw.get_unchecked(0..4)) }
    }
    /// The version of the Moc3 file
    pub fn version(&self) -> Moc3Version {
        // 5 byte
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
    /// unknown from 6-64
    pub fn unknown_1(&self) -> &[u8] {
        // 6-64 byte
        unsafe { self.raw.get_unchecked(5..64) }
    }
}
