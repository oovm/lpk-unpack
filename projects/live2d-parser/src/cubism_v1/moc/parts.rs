use serde::de::Error;

#[derive(Debug)]
pub struct Part {
    /// Part name
    pub name: String,
    /// Part type
    pub part_type: PartType,
}

#[derive(Debug)]
pub enum PartType {
    /// Normal part
    Normal,
    /// Mesh part
    Mesh,
}

impl Part {
    /// Parse part from moc data
    ///
    /// ## Safety
    /// The input data must be a valid moc file
    pub unsafe fn parse(data: &[u8], offset: usize) -> Result<Self, serde_json::Error> {
        let mut current_offset = offset;

        // Read name length
        let name_len = std::ptr::read(data.as_ptr().add(current_offset) as *const u32);
        current_offset += 4;

        // Read name
        let name = String::from_raw_parts(
            data.as_ptr().add(current_offset) as *mut u8,
            name_len as usize,
            name_len as usize,
        );
        current_offset += name_len as usize;

        // Read part type
        let part_type = match std::ptr::read(data.as_ptr().add(current_offset) as *const u32) {
            0 => PartType::Normal,
            1 => PartType::Mesh,
            _ => return Err(serde_json::Error::custom("Invalid part type")),
        };

        Ok(Self { name, part_type })
    }
}