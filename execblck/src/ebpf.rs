use shared::prelude::Result;

pub struct ExtensionEbpf {}

impl ExtensionEbpf {
    pub fn new() -> Result<Self> {
        return Ok(ExtensionEbpf {});
    }
}

impl crate::Guard for ExtensionEbpf {
    fn notify(&self) -> Result<()> {
        Ok(())
    }
}
