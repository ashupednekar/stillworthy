use endpointsecurity_rs::{EsClient, EsEventData, EsEventType};
use shared::prelude::Result;

pub struct ExtensionMac{
    pub client: EsClient
}

impl ExtensionMac{
    pub fn new() -> Result<Self>{
        let client = EsClient::new()?;
        return Ok(ExtensionMac { client });
    }
}

impl crate::Guard for ExtensionMac{
    fn notify(&self) -> Result<()> {
    loop {
        let msg = self.client.recv_msg()?;
        if let Some(ref data) = msg.event_data {
            match data {
                EsEventData::NotifyExec(proc) => {
                    println!("prof: {:?}", proc);
                },
                _ => {}
            }
        }
    }
    #[allow(unreachable_code)]
    Ok(())
}
}

