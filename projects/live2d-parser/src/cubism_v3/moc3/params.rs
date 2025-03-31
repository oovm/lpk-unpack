use super::*;

#[derive(Copy, Clone, Debug)]
pub(crate) struct ParametersOffsets {
    // since v3.0
    name: u32,
    max_value: u32,
    min_value: u32,
    default_value: u32,
    is_repeat: u32,
    decimal_places: u32,
    binding_sources_begin: u32,
    binding_sources_count: u32,
    // since v4.0
    key_source_begin_indices: u32,
    key_source_counts: u32,
    // since v4.2
    r#type: u32,
    blend_shape_parameter_binding_sources_begin_indices: u32,
    blend_shape_parameter_binding_sources_counts: u32,
}

pub struct Parameters<'i> {
    moc3: &'i Moc3,
    index: u32,
}

#[derive(Clone, Debug)]
pub struct Parameter<'i> {
    pub name: &'i str,
    pub max_value: f32,
    pub min_value: f32,
    pub default_value: f32,
    pub is_repeat: bool,
    pub decimal_places: u32,
    pub binding_sources_begin: i32,
    pub binding_sources_count: i32,
}

impl ParametersOffsets {
    pub unsafe fn read(moc3: *const u8) -> Self {
        Self {
            r#type: std::ptr::read(moc3.add(0x208) as *const u32),
            name: std::ptr::read(moc3.add(0x108) as *const u32),
            max_value: std::ptr::read(moc3.add(0x10C) as *const u32),
            min_value: std::ptr::read(moc3.add(0x110) as *const u32),
            default_value: std::ptr::read(moc3.add(0x114) as *const u32),
            is_repeat: std::ptr::read(moc3.add(0x118) as *const u32),
            decimal_places: std::ptr::read(moc3.add(0x11C) as *const u32),
            binding_sources_begin: std::ptr::read(moc3.add(0x120) as *const u32),
            binding_sources_count: std::ptr::read(moc3.add(0x124) as *const u32),
            key_source_begin_indices: std::ptr::read(moc3.add(0x1DC) as *const u32),
            key_source_counts: std::ptr::read(moc3.add(0x1E0) as *const u32),
            blend_shape_parameter_binding_sources_begin_indices: std::ptr::read(moc3.add(0x20C) as *const u32),
            blend_shape_parameter_binding_sources_counts: std::ptr::read(moc3.add(0x210) as *const u32),
        }
    }
}

impl Moc3 {
    pub fn get_parameters(&self) -> Parameters {
        Parameters { moc3: self, index: 0 }
    }
}

impl<'i> Parameters<'i> {
    pub fn get_parameter(&self, index: u32) -> Option<Parameter> {
        if index >= self.moc3.counter.parameters {
            return None;
        }
        unsafe { Some(self.get_unchecked(index)) }
    }
    pub unsafe fn get_unchecked(&self, index: u32) -> Parameter<'i> {
        self.moc3.parameters.get_unchecked(self.moc3, index)
    }
}

impl ParametersOffsets {
    unsafe fn get_unchecked<'i>(&self, moc3: &'i Moc3, index: u32) -> Parameter<'i> {
        Parameter {
            name: moc3.read_cstr::<64>(self.name, index),
            max_value: moc3.read(self.max_value, index),
            min_value: moc3.read(self.min_value, index),
            default_value: moc3.read(self.default_value, index),
            is_repeat: moc3.read_b32(self.is_repeat, index),
            decimal_places: moc3.read(self.decimal_places, index),
            binding_sources_begin: moc3.read(self.binding_sources_begin, index),
            binding_sources_count: moc3.read(self.binding_sources_count, index),
        }
    }
}

impl<'i> Iterator for Parameters<'i> {
    type Item = Parameter<'i>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.moc3.counter.parameters {
            return None;
        }
        let result = unsafe { self.get_unchecked(self.index) };
        self.index.add_assign(1);
        Some(result)
    }
}
