use super::*;

pub(crate) struct PartOffsets {
    name: u32,
    is_visible: u32,
    is_enabled: u32,
    parent_part_indices: u32,
    keyform_binding_sources_indices: u32,
    keyform_sources_begin_indices: u32,
    keyform_sources_counts: u32,
}

pub struct Parts<'i> {
    moc3: &'i Moc3,
    index: u32,
}

#[derive(Debug)]
pub struct Part<'i> {
    pub name: &'i str,
    pub is_visible: bool,
    pub is_enabled: bool,
    pub parent_part_indices: i32,
    pub keyform_sources_counts: i32,
    pub keyform_sources_begin_indices: i32,
    pub keyform_binding_sources_indices: i32,
}

impl Moc3 {
    pub fn parts(&self) -> Parts {
        Parts { moc3: self, index: 0 }
    }
}

impl PartOffsets {
    pub unsafe fn read(moc3: *const u8) -> Self {
        Self {
            name: std::ptr::read(moc3.add(0x4C) as *const u32),
            keyform_binding_sources_indices: std::ptr::read(moc3.add(0x50) as *const u32),
            keyform_sources_begin_indices: std::ptr::read(moc3.add(0x54) as *const u32),
            keyform_sources_counts: std::ptr::read(moc3.add(0x58) as *const u32),
            is_visible: std::ptr::read(moc3.add(0x5C) as *const u32),
            is_enabled: std::ptr::read(moc3.add(0x60) as *const u32),
            parent_part_indices: std::ptr::read(moc3.add(0x64) as *const u32),
        }
    }
}

impl<'i> Parts<'i> {
    pub fn get_part(&self, index: u32) -> Option<Part<'i>> {
        if index >= self.moc3.counter.parts {
            return None;
        }
        unsafe { Some(self.get_unchecked(index)) }
    }

    pub unsafe fn get_unchecked(&self, index: u32) -> Part<'i> {
        self.moc3.parts.get_unchecked(self.moc3, index)
    }
}

impl PartOffsets {
    unsafe fn get_unchecked<'i>(&self, moc3: &'i Moc3, index: u32) -> Part<'i> {
        Part {
            name: moc3.read_cstr::<64>(self.name, index),
            is_visible: moc3.read_b32(self.is_visible, index),
            is_enabled: moc3.read_b32(self.is_enabled, index),
            parent_part_indices: moc3.read(self.parent_part_indices, index),
            keyform_binding_sources_indices: moc3.read(self.keyform_binding_sources_indices, index),
            keyform_sources_begin_indices: moc3.read(self.keyform_sources_begin_indices, index),
            keyform_sources_counts: moc3.read(self.keyform_sources_counts, index),
        }
    }
}

impl<'i> Iterator for Parts<'i> {
    type Item = Part<'i>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.moc3.counter.parts {
            return None;
        }
        let result = unsafe { self.get_unchecked(self.index) };
        self.index += 1;
        Some(result)
    }
}
