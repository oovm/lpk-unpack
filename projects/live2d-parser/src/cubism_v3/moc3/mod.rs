use tracing::warn;

mod params;

pub struct Moc3<'i> {
    raw: &'i [u8],
}

impl<'i> Moc3<'i> {
    pub fn new(moc3: &'i [u8]) -> Moc3<'i> {
        Moc3 { raw: moc3 }
    }
}

#[repr(u8)]
#[derive(Eq, PartialEq, Ord, PartialOrd)]
pub enum Moc3Version {
    V30 = 1,
    V33 = 2,
    V40 = 3,
    V42 = 4,
    V50 = 5,
}

impl<'i> Moc3<'i> {
    /// Should always be "MOC3"
    pub fn magic(&self) -> &str {
        // 0x00000000-0x00000004
        unsafe { std::str::from_utf8_unchecked(&self.raw.get_unchecked(0..4)) }
    }
    /// The version of the Moc3 file
    pub fn version(&self) -> Moc3Version {
        // 0x00000005
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
}

#[derive(Clone, Debug)]
pub struct Parameter<'i> {
    pub id: &'i str,
    pub max_value: f32,
    pub min_value: f32,
    pub default_value: f32,
    pub is_repeat: bool,
    pub decimal_places: u32,
    pub binding_sources_begin: i32,
    pub binding_sources_count: i32,
}

impl<'i> Moc3<'i> {
    pub fn get_parameters(&self) -> Vec<Parameter> {
        // 头区块
        let count_info_table_offset = 64;
        // 参数计数
        let parameters_count_offset = match self.version() {
            Moc3Version::V50 => count_info_table_offset + 256 + 40,
            _ => count_info_table_offset + 128 + 40,
        };
        let parameters_count = unsafe {
            let ptr = self.raw.as_ptr().add(parameters_count_offset) as *const u32;
            u32::from_le(*ptr)
        };
        warn!("parameters_count: {}", parameters_count);
        let mut parameters = Vec::with_capacity(parameters_count as usize);
        // 跳转表
        let offset_table = match self.version() {
            Moc3Version::V50 => count_info_table_offset + 256 + 128,
            _ => count_info_table_offset + 128 + 128,
        };
        unsafe {
            let ids_offset = u32::from_le(*((self.raw.as_ptr().add(offset_table)) as *const u32));
            let max_values_offset = u32::from_le(*((self.raw.as_ptr().add(offset_table + 4)) as *const u32));
            let min_values_offset = u32::from_le(*((self.raw.as_ptr().add(offset_table + 8)) as *const u32));
            let default_values_offset = u32::from_le(*((self.raw.as_ptr().add(offset_table + 12)) as *const u32));
            let is_repeat_offset = u32::from_le(*((self.raw.as_ptr().add(offset_table + 16)) as *const u32));
            let decimal_places_offset = u32::from_le(*((self.raw.as_ptr().add(offset_table + 20)) as *const u32));
            let binding_sources_begin_offset = u32::from_le(*((self.raw.as_ptr().add(offset_table + 24)) as *const u32));
            let binding_sources_count_offset = u32::from_le(*((self.raw.as_ptr().add(offset_table + 28)) as *const u32));

            for i in 0..parameters_count {
                let id_ptr = self.raw.as_ptr().add(ids_offset as usize + (i * 64) as usize);
                let id = std::str::from_utf8_unchecked(std::slice::from_raw_parts(id_ptr, 64)).trim_matches(char::from(0));

                let max_value = *((self.raw.as_ptr().add(max_values_offset as usize + (i * 4) as usize)) as *const f32);
                let min_value = *((self.raw.as_ptr().add(min_values_offset as usize + (i * 4) as usize)) as *const f32);
                let default_value = *((self.raw.as_ptr().add(default_values_offset as usize + (i * 4) as usize)) as *const f32);
                let is_repeat = *((self.raw.as_ptr().add(is_repeat_offset as usize + (i * 4) as usize)) as *const u32) != 0;
                let decimal_places =
                    *((self.raw.as_ptr().add(decimal_places_offset as usize + (i * 4) as usize)) as *const u32);
                let binding_sources_begin =
                    *((self.raw.as_ptr().add(binding_sources_begin_offset as usize + (i * 4) as usize)) as *const i32);
                let binding_sources_count =
                    *((self.raw.as_ptr().add(binding_sources_count_offset as usize + (i * 4) as usize)) as *const i32);

                parameters.push(Parameter {
                    id,
                    max_value,
                    min_value,
                    default_value,
                    is_repeat,
                    decimal_places,
                    binding_sources_begin,
                    binding_sources_count,
                });
            }
        }

        parameters
    }
}
