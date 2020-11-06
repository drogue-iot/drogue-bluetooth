use crate::types::advertising_data::AdvertisingData;
use heapless::{
    ArrayLength,
    Vec,
    consts::*,
};
use drogue_bluetooth_hci::ser::Serializable;
use drogue_bluetooth_hci::hci_commands;
use drogue_bluetooth_hci::command::ReturnParameters;
use crate::types::status::Status;

hci_commands!{
    0x06 TestingCommand => {

    },
    0x08 LEControllerCommand => {
        0x006 SetAdvertisingParameters => StatusReturnParameters,
        0x008 SetAdvertisingData       => StatusReturnParameters,
        0x009 SetScanResponseData      => StatusReturnParameters,
        0x00A SetAdvertisingEnable     => StatusReturnParameters,
    }
}

#[derive(HCIDeserializable)]
pub struct StatusReturnParameters {
    pub status: Status,
}

impl ReturnParameters for StatusReturnParameters {

}


#[derive(HCISerializable)]
pub struct SetAdvertisingParameters {

}

pub struct SetScanResponseData {
    data: Vec<AdvertisingData,U16>,
}

pub struct SetAdvertisingData {
    data: Vec<AdvertisingData,U16>,
}

#[derive(HCISerializable)]
pub struct SetAdvertisingEnable {
    pub enable: bool,
}

impl Serializable for SetAdvertisingData {
    fn serialize<N: ArrayLength<u8>>(&self, o: &mut Vec<u8, N>) -> Result<(), ()> {
        // reserve space for length
        let placeholder = o.len();
        o.push( 0x00).map_err(|_|())?;
        self.data.serialize(o)?;
        let len = o.len() - (placeholder+1);
        o[placeholder] = len as u8;
        o.resize( 32, 0).map_err(|_|())?;
        Ok(())
    }
}

impl Serializable for SetScanResponseData {
    fn serialize<N: ArrayLength<u8>>(&self, o: &mut Vec<u8, N>) -> Result<(), ()> {
        // reserve space for length
        let placeholder = o.len();
        o.push( 0x00).map_err(|_|())?;
        self.data.serialize(o)?;
        let len = o.len() - (placeholder+1);
        o[placeholder] = len as u8;
        o.resize( 32, 0).map_err(|_|())?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use drogue_bluetooth_hci::command::Command;
    use drogue_bluetooth_hci::ser::Serializable;
    use crate::commands::{SetAdvertisingEnable, SetAdvertisingData};
    use heapless::{
        String,
        Vec,
        consts::*,
    };
    use crate::types::advertising_data::{AdvertisingData, AdvertisingFlags, Discoverability, Capability};
    use nom::AsBytes;

    #[test]
    fn test_structures() {
        let command = SetAdvertisingEnable{ enable: true };
        assert_eq!(0x200Au16, SetAdvertisingEnable::OPCODE.into());
        assert_eq!(0x200Au16, command.opcode().into());
    }

    #[test]
    fn test_serialize() {
        let command = SetAdvertisingEnable{ enable: true };
        let mut buf: Vec<u8, U16> = Vec::new();
        command.serialize(&mut buf).unwrap();
        assert_eq!(1, buf.len());
        assert_eq!(0x01, buf[0]);
    }

    #[test]
    fn test_set_advertising_data() {
        let mut data = Vec::new();
        data.push(AdvertisingData::Flags(AdvertisingFlags {
            discoverability: Discoverability::Limited,
            capability: Capability::LeOnly,
        })).unwrap();
        data.push(AdvertisingData::CompleteLocalName(String::from("drogue-iot"))).unwrap();
        let command = SetAdvertisingData {
            data,
        };

        let mut buf = Vec::<u8, U32>::new();
        command.serialize( &mut buf ).unwrap();
        println!("{:?}", &buf.as_bytes());
        assert_eq!( 32, buf.len() );

        // length of advertising data
        assert_eq!( 15, buf[0]);
        // length of first chunk
        assert_eq!( 2, buf[1]);
        // <<flags>>
        assert_eq!( 0x02, buf[2]);
        // <<flags -- limited discoverability, le-only>>
        assert_eq!( 5, buf[3] );

        // length of second chunk
        assert_eq!(11, buf[4]);
        // <<complete local name>>
        assert_eq!(0x09, buf[5]);
        // <<complete local name -- "drogue-iot" (10 bytes)
        assert_eq!( "drogue-iot".as_bytes(), &buf[6..16]);

        // padded with zeros
        for i in 16..31 {
            assert_eq!(0, buf[i]);
        }
    }
}