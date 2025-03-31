use super::*;

pub(crate) struct ArtMeshOffsets {
    vertex_positions: u32,
    vertex_uvs: u32,
    vertex_indices: u32,
    vertex_counts: u32,
    index_counts: u32,
    texture_index: u32,
    opacity: u32,
    drawable_flags: u32,
}

pub struct ArtMeshes<'i> {
    moc3: &'i Moc3,
    index: u32,
}

#[derive(Debug)]
pub struct ArtMesh<'i> {
    pub vertex_positions: &'i [f32],
    pub vertex_uvs: &'i [f32],
    pub vertex_indices: &'i [u16],
    pub vertex_counts: u32,
    pub index_counts: u32,
    pub texture_index: u32,
    pub opacity: f32,
    pub drawable_flags: u32,
}

impl Moc3 {
    pub fn art_meshes(&self) -> ArtMeshes {
        ArtMeshes { moc3: self, index: 0 }
    }
}

impl ArtMeshOffsets {
    pub unsafe fn read(moc3: *const u8) -> Self {
        Self {
            vertex_positions: std::ptr::read(moc3.add(0x20) as *const u32),
            vertex_uvs: std::ptr::read(moc3.add(0x24) as *const u32),
            vertex_indices: std::ptr::read(moc3.add(0x28) as *const u32),
            vertex_counts: std::ptr::read(moc3.add(0x2C) as *const u32),
            index_counts: std::ptr::read(moc3.add(0x30) as *const u32),
            texture_index: std::ptr::read(moc3.add(0x34) as *const u32),
            opacity: std::ptr::read(moc3.add(0x38) as *const u32),
            drawable_flags: std::ptr::read(moc3.add(0x3C) as *const u32),
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
        self.moc3.art_mesh_offsets.get_unchecked(self.moc3, index)
    }
}

impl ArtMeshOffsets {
    unsafe fn get_unchecked<'i>(&self, moc3: &'i Moc3, index: u32) -> ArtMesh<'i> {
        ArtMesh {
            vertex_positions: moc3.read_slice_f32(self.vertex_positions, index),
            vertex_uvs: moc3.read_slice_f32(self.vertex_uvs, index),
            vertex_indices: moc3.read_slice_u16(self.vertex_indices, index),
            vertex_counts: moc3.read(self.vertex_counts, index),
            index_counts: moc3.read(self.index_counts, index),
            texture_index: moc3.read(self.texture_index, index),
            opacity: moc3.read_f32(self.opacity, index),
            drawable_flags: moc3.read(self.drawable_flags, index),
        }
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