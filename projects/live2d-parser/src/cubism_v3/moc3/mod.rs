mod element_count;
mod params;

use self::params::ParametersOffset;
pub use self::{element_count::ElementCountTable, params::Parameter};
use serde::de::Error;
use std::{
    ffi::CStr,
    fmt::{Debug, Formatter},
    ops::{AddAssign, SubAssign},
};

#[derive(Clone)]
pub struct Moc3 {
    /// A memory buffer of live-2d data
    m: Vec<u8>,
    /// The element count table of live-2d data
    counter: ElementCountTable,
    parameters: ParametersOffset,
}
impl Debug for Moc3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Moc3").field("bytes", &self.m.len()).finish()
    }
}

#[derive(Eq, PartialEq, Ord, PartialOrd)]
pub enum MocVersion {
    V30,
    V33,
    V40,
    V42,
    V50,
}

impl Moc3 {
    pub fn new(moc3: Vec<u8>) -> Result<Moc3, serde_json::Error> {
        Ok(Moc3 { counter: c_read_ptr32(&moc3, 0x40)?, parameters: c_read(&moc3, 0x104)?, m: moc3 })
    }
    /// Should always be "MOC3"
    pub fn magic_head(&self) -> &str {
        // 0x00000000-0x00000004
        unsafe { std::str::from_utf8_unchecked(&self.m.get_unchecked(0..4)) }
    }
    /// The version of the Moc3 file
    pub fn version(&self) -> MocVersion {
        // 0x00000005
        unsafe {
            match self.m.get_unchecked(4) {
                1 => MocVersion::V30,
                2 => MocVersion::V33,
                3 => MocVersion::V40,
                4 => MocVersion::V42,
                5 => MocVersion::V50,
                _ => panic!("Unknown Moc3 version"),
            }
        }
    }
    pub fn element_count(&self) -> ElementCountTable {
        self.counter
    }
}
impl Moc3 {
    unsafe fn read<T>(&self, address: u32, index: u32) -> T {
        let base = address as usize;
        let size_of = size_of::<T>();
        let start = base + index as usize * size_of;
        std::ptr::read(self.m.as_ptr().add(start) as *const T)
    }
    unsafe fn read_b32(&self, address: u32, index: u32) -> bool {
        self.read::<u32>(address, index) != 0
    }
    unsafe fn read_cstr<const N: u32>(&self, address: u32, index: u32) -> &str {
        let base = address;
        let start = base + index * N;
        let name_ptr = self.m.as_ptr().add(start as usize) as *const i8;
        CStr::from_ptr(name_ptr).to_str().unwrap()
    }
}
fn c_read_ptr32<T>(moc3: &[u8], address: usize) -> Result<T, serde_json::Error> {
    if moc3.len() < address + size_of::<u32>() {
        return Err(serde_json::Error::custom(format!("Missing `{}` pointer", std::any::type_name::<T>())));
    }
    let ptr: usize = unsafe { std::ptr::read(moc3.as_ptr().add(address) as *const u32) as usize };
    c_read(moc3, ptr)
}

fn c_read<T>(moc3: &[u8], address: usize) -> Result<T, serde_json::Error> {
    tracing::debug!("Moc[{}..{}] as {}", address, address + size_of::<T>(), std::any::type_name::<T>());
    if moc3.len() < address + size_of::<T>() {
        return Err(serde_json::Error::custom(format!("Invalid `{}` pointer", std::any::type_name::<T>())));
    }
    unsafe { Ok(std::ptr::read(moc3.as_ptr().add(address) as *const T)) }
}
