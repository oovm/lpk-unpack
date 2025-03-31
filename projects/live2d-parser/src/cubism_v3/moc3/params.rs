use super::*;

#[derive(Copy, Clone, Debug)]
pub(crate) struct ParametersOffsets {
    r#type: u32,
    // 0x108
    name: u32,
    max_value: u32,
    min_value: u32,
    default_value: u32,
    is_repeat: u32,
    decimal_places: u32,
    binding_sources_begin: u32,
    binding_sources_count: u32,
    // fake fields
    key_source_begin_indices: u32,
    key_source_counts: u32,
    blend_shape_parameter_binding_sources_begin_indices: u32,
    blend_shape_parameter_binding_sources_counts: u32,
}

impl ParametersOffsets {
    pub unsafe fn read(moc3: *const u8) -> Self {
        Self {
            r#type: std::ptr::read(moc3.add(0x108) as *const u32),
            name: 0,
            max_value: 0,
            min_value: 0,
            default_value: 0,
            is_repeat: 0,
            decimal_places: 0,
            binding_sources_begin: 0,
            binding_sources_count: 0,
            key_source_begin_indices: 0,
            key_source_counts: 0,
            blend_shape_parameter_binding_sources_begin_indices: 0,
            blend_shape_parameter_binding_sources_counts: 0,
        }
    }
}

/// parameter extension in version 4.0
#[repr(C)]
#[derive(Copy, Clone, Debug)]
struct ParametersE40 {
    key_source_begin_indices: u32,
    key_source_counts: u32,
}
/// extension in version 4.2
#[repr(C)]
#[derive(Copy, Clone, Debug)]
struct ParametersE42 {
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
