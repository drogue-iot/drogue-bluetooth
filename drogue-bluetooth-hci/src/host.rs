use core::marker::PhantomData;
use crate::vendor::Vendor;
use crate::command::Command;
use crate::packet::Packet;
use core::convert::TryFrom;

pub struct Host<V: Vendor> {
    _vendor: PhantomData<V>,
}

impl<V: Vendor> Host<V>
{
    pub fn new() -> Self {
        Self {
            _vendor: PhantomData::default(),
        }
    }

    pub fn send_command_sync<C: Command>(&mut self, command: C) -> Result<C::ReturnParameters, ()>
        where
            C: Into<V::SupportedCommand>,
    {
        let vc: V::SupportedCommand = command.into();
        Err(())
    }


}
