use crate::types::status::Status;
use drogue_bluetooth_hci::command::Opcode;
use drogue_bluetooth_hci::hci_events;


use heapless::{
    Vec,
    consts::*
};

hci_events! {
    0x03 => CommandComplete,
    0x0F => CommandStatus,
}

#[derive(HCIDeserializable)]
pub struct CommandComplete {
    num_hci_packets: u8,
    opcode: Opcode,
    return_parameters: Vec<u8, U32>,
}

#[derive(HCIDeserializable)]
pub struct CommandStatus {
    status: Status,
    num_hci_packets: u8,
}

#[cfg(test)]
mod tests {
    use crate::types::status::Status;
    use drogue_bluetooth_hci::ser::Deserializable;

    #[test]
    fn parse_command_status() {
        let buf = [ 0x03, 4 ];
        let result = crate::events::CommandStatus::parse( &buf ).unwrap().1;
        assert_eq!( Status::HardwareFailure, result.status );
        assert_eq!( 4, result.num_hci_packets );
    }
}