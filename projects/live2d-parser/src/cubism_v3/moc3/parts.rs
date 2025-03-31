#[repr(C)]
pub struct PartOffset {
    _align: u32,
    name: u32,
    keyform_binding_sources_indices: u32,
    keyform_sources_begin_indices: u32,
    keyform_sources_counts: u32,
    is_visible: u32,
    is_enabled: u32,
    parent_part_indices: u32,
}

pub struct Part<'i> {
    pub name: &'i str,
    pub keyform_binding_sources_indices: i32,
    pub keyform_sources_begin_indices: i32,
    pub keyform_sources_counts: i32,
    pub is_visible: bool,
    pub is_enabled: bool,
    pub parent_part_indices: i32,
}

impl Moc3 {
    pub fn get_parts(&self) -> Parts {
        Parts { moc3: self, index: 0 }
    }
}

pub struct Parts<'i> {
    moc3: &'i Moc3,
    index: u32,
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

impl PartOffset {
    unsafe fn get_unchecked<'i>(&self, moc3: &'i Moc3, index: u32) -> Part<'i> {
        Part {
            name: moc3.read_cstr::<64>(self.name, index),
            keyform_binding_sources_indices: moc3.read(self.keyform_binding_sources_indices, index),
            keyform_sources_begin_indices: moc3.read(self.keyform_sources_begin_indices, index),
            keyform_sources_counts: moc3.read(self.keyform_sources_counts, index),
            is_visible: moc3.read_b32(self.is_visible, index),
            is_enabled: moc3.read_b32(self.is_enabled, index),
            parent_part_indices: moc3.read(self.parent_part_indices, index),
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
