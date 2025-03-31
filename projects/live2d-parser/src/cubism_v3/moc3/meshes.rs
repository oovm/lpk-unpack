use super::*;

pub(crate) struct ArtMeshOffsets {
    name: u32,
    keyform_binding_sources_indices: u32,
    keyform_sources_begin_indices: u32,
    keyform_sources_counts: u32,
    is_visible: u32,
    is_enabled: u32,
    parent_part_indices: u32,
    parent_deformer_indices: u32,
    texture_indices: u32,
    drawable_flags: u32,
    vertex_counts: u32,
    uv_sources_begin_indices: u32,
    position_index_source_begin_indices: u32,
    position_index_source_counts: u32,
    drawable_mask_sources_begin_indices: u32,
    drawable_mask_source_counts: u32,
    keyform_color_source_begin_indices: u32,
}

#[derive(Debug)]
pub struct DrawableFlags(u32);

impl DrawableFlags {
    pub fn is_visible(&self) -> bool {
        self.0 & 0x01 != 0
    }
    
    pub fn is_enabled(&self) -> bool {
        self.0 & 0x02 != 0
    }
    
    pub fn is_culling(&self) -> bool {
        self.0 & 0x04 != 0
    }
    
    pub fn is_masked(&self) -> bool {
        self.0 & 0x08 != 0
    }
    
    pub fn is_inverted_mask(&self) -> bool {
        self.0 & 0x10 != 0
    }
}

pub struct ArtMeshes<'i> {
    moc3: &'i Moc3,
    index: u32,
}

#[derive(Debug)]
pub struct ArtMesh<'i> {
    pub name: &'i str,
    pub texture_indices: u32,
}

impl Moc3 {
    pub fn art_meshes(&self) -> ArtMeshes {
        ArtMeshes { moc3: self, index: 0 }
    }
}

impl ArtMeshOffsets {
    pub unsafe fn read(moc3: *const u8) -> Self {
        Self {
            name: std::ptr::read(moc3.add(0xC4) as *const u32),
            keyform_binding_sources_indices: std::ptr::read(moc3.add(0xC8) as *const u32),
            keyform_sources_begin_indices: std::ptr::read(moc3.add(0xCC) as *const u32),
            keyform_sources_counts: std::ptr::read(moc3.add(0xD0) as *const u32),
            is_visible: std::ptr::read(moc3.add(0xD4) as *const u32),
            is_enabled: std::ptr::read(moc3.add(0xD8) as *const u32),
            parent_part_indices: std::ptr::read(moc3.add(0xEC) as *const u32),
            parent_deformer_indices: std::ptr::read(moc3.add(0xE0) as *const u32),
            texture_indices: std::ptr::read(moc3.add(0xE4) as *const u32),
            drawable_flags: std::ptr::read(moc3.add(0xE8) as *const u32),
            vertex_counts: std::ptr::read(moc3.add(0xEC) as *const u32),
            uv_sources_begin_indices: std::ptr::read(moc3.add(0xF0) as *const u32),
            position_index_source_begin_indices: std::ptr::read(moc3.add(0xF4) as *const u32),
            position_index_source_counts: std::ptr::read(moc3.add(0xF8) as *const u32),
            drawable_mask_sources_begin_indices: std::ptr::read(moc3.add(0xFC) as *const u32),
            drawable_mask_source_counts: std::ptr::read(moc3.add(0x100) as *const u32),
            keyform_color_source_begin_indices: std::ptr::read(moc3.add(0x1EC) as *const u32),
        }
    }
}

impl<'i> ArtMeshes<'i> {
    pub fn get_art_mesh(&self, index: u32) -> Option<ArtMesh<'i>> {
        if index >= self.moc3.counter.art_meshes {
            return None;
        }
        unsafe { Some(self.get_unchecked(index)) }
    }

    pub unsafe fn get_unchecked(&self, index: u32) -> ArtMesh<'i> {
        self.moc3.meshes.get_unchecked(self.moc3, index)
    }
}

impl ArtMeshOffsets {
    unsafe fn get_unchecked<'i>(&self, moc3: &'i Moc3, index: u32) -> ArtMesh<'i> {
        ArtMesh { name: moc3.read_cstr::<64>(self.name, index), texture_indices: moc3.read(self.texture_indices, index) }
    }
}

impl<'i> Iterator for ArtMeshes<'i> {
    type Item = ArtMesh<'i>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.moc3.counter.art_meshes {
            return None;
        }
        let result = unsafe { self.get_unchecked(self.index) };
        self.index += 1;
        Some(result)
    }
}
