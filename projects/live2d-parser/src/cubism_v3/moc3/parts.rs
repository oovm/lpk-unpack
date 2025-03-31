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
    name: &'i str,
    keyform_binding_sources_indices: i32,
    keyform_sources_begin_indices: i32,
    keyform_sources_counts: i32,
    is_visible: bool,
    is_enabled: bool,
    parent_part_indices: i32,
}
