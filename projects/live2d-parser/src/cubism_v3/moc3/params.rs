use crate::cubism_v3::moc3::Moc3;
use std::ops::{AddAssign, SubAssign};

pub struct Moc3Parameters<'i> {
    moc3: &'i Moc3,
    table: cOffsetTable,
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

#[repr(C)]
struct cOffsetTable {
    name: u32,
    max_value: u32,
    min_value: u32,
    default_value: u32,
    is_repeat: u32,
    decimal_places: u32,
    binding_sources_begin: u32,
    binding_sources_count: u32,
}

#[repr(C)]
struct cParameter {
    name: [u64; 8],
    max_value: f32,
    min_value: f32,
    default_value: f32,
    is_repeat: u32,
    decimal_places: u32,
    binding_sources_begin: i32,
    binding_sources_count: i32,
}

impl Moc3 {
    pub fn get_parameters(&self) -> Moc3Parameters {
        Moc3Parameters { moc3: self, index: 0, table: cOffsetTable {
            name: 0,
            max_value: 0,
            min_value: 0,
            default_value: 0,
            is_repeat: 0,
            decimal_places: 0,
            binding_sources_begin: 0,
            binding_sources_count: 0,
        } }
    }
    pub fn get_parameter(&self, index: u32) -> Option<Parameter> {
        self.get_parameters().get_parameter(index)
    }
}

impl<'i> Moc3Parameters<'i> {
    pub unsafe fn get_parameter_unchecked(&self, index: u32) -> Parameter<'i> {
        Parameter {
            name: "",
            max_value: 0.0,
            min_value: 0.0,
            default_value: 0.0,
            is_repeat: false,
            decimal_places: 0,
            binding_sources_begin: 0,
            binding_sources_count: 0,
        }
    }
    pub fn get_parameter(&self, index: u32) -> Option<Parameter<'i>> {
        if index >= self.moc3.counter.parameters {
            return None;
        }
        Some(unsafe { self.get_parameter_unchecked(index) })
    }
}

impl<'i> Iterator for Moc3Parameters<'i> {
    type Item = Parameter<'i>;

    fn next(&mut self) -> Option<Self::Item> {
        let this = self.get_parameter(self.index)?;
        self.index.add_assign(1);
        Some(this)
    }
}

impl<'i> DoubleEndedIterator for Moc3Parameters<'i> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.index == 0 {
            return None;
        }
        self.index.sub_assign(1);
        self.get_parameter(self.index)
    }
}
