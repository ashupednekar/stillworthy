use endpointsecurity_rs::{EsClient, EsEventData, EsEventType};
use shared::prelude::Result;


//pub struct ExecNotifier{
//    pub client: EsClient
//}


pub fn notify_exec() -> Result<()>{
    let client = EsClient::new()?;
    loop {
        let msg = client.recv_msg()?;
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
