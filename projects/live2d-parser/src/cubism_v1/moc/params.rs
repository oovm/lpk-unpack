use serde::de::Error;

#[derive(Debug)]
pub struct Parameter {
    /// Parameter name
    pub name: String,
    /// Default value
    pub default_value: f32,
}

#[derive(Debug)]
pub struct ParamList {
    params: Vec<Parameter>,
}

impl ParamList {
    /// Parse parameter list from moc data
    ///
    /// ## Safety
    /// The input data must be a valid moc file
    pub unsafe fn parse(data: &[u8], offset: usize) -> Result<Self, serde_json::Error> {
        let mut params = Vec::new();
        let mut current_offset = offset;

        // Read parameter count
        let count = std::ptr::read(data.as_ptr().add(current_offset) as *const u32);
        current_offset += 4;

        // Read parameters
        for _ in 0..count {
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

            // Read default value
            let default_value = std::ptr::read(data.as_ptr().add(current_offset) as *const f32);
            current_offset += 4;

            params.push(Parameter {
                name,
                default_value,
            });
        }

        Ok(Self { params })
    }

    /// Get all parameters
    pub fn parameters(&self) -> &[Parameter] {
        &self.params
    }
}