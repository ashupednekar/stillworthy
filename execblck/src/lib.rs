use shared::prelude::Result;

#[cfg(feature = "mac")]
pub mod mac_ext;

#[cfg(feature = "ebpf")]
pub mod ebpf;

pub struct Extention;

impl Extention {
    #[cfg(feature = "mac")]
    pub fn new() -> Result<impl Guard> {
        mac_ext::ExtensionMac::new()
    }

    #[cfg(feature = "ebpf")]
    pub fn new() -> Result<impl Guard> {
        ebpf::ExtensionEbpf::new()
    }
}

pub trait Guard {
    fn notify(&self) -> Result<()>;
}
