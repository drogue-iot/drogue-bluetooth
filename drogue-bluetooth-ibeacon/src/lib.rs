#![cfg_attr(not(test), no_std)]

use drogue_bluetooth_hci::vendor::Vendor;
use drogue_bluetooth_hci::host::Host;
use drogue_bluetooth_hci_core::commands::{SetAdvertisingEnable, SetAdvertisingParameters, StatusReturnParameters};

/*
pub struct IBeacon<V: Vendor> {
    host: Host<V>,
}

impl<V: Vendor> IBeacon<V>
    where
        V::SupportedCommand: From<SetAdvertisingParameters>,
        V::SupportedCommand: From<SetAdvertisingEnable>,
{
    pub fn new(host: Host<V>) -> Self {
        Self {
            host,
        }
    }

    pub fn start(&mut self) -> Result<(),()>{
        let result = self.host.send_command_sync(SetAdvertisingParameters {}).unwrap();
        //let status = result.unwrap();
        let status = result.status;
        //let result = self.host.send_command_sync(SetAdvertisingEnable { enable: true });
        Ok(())
    }
}
 */

pub trait IBeacon {
    fn start(&mut self) -> Result<(),()>;
}

impl<V:Vendor> IBeacon for Host<V>
    where
        V::SupportedCommand: From<SetAdvertisingParameters>,
        V::SupportedCommand: From<SetAdvertisingEnable>,
{
    fn start(&mut self) -> Result<(), ()> {
        let result = self.send_command_sync(SetAdvertisingParameters{}).unwrap();
        Ok(())
    }
}




#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
