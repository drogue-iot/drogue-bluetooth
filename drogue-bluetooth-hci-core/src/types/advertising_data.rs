use heapless::{
    String,
    Vec,
    ArrayLength,
    consts::*,
};

use crate::types::company_identifier::CompanyIdentifier;
use drogue_bluetooth_hci::ser::Serializable;

macro_rules! advertising_data_impl {
    ( $($name:ident => $code:literal ( $ty:ty ), )* ) => {
        #[derive(Debug)]
        pub enum AdvertisingData {
            $(
                $name($ty),
            )*
        }

        impl Serializable for AdvertisingData {
            fn serialize<N:ArrayLength<u8>>(&self, o: &mut Vec<u8,N>) -> Result<(),()>{
                // placeholder for length
                let len_pos = o.len();
                o.push(0x00).map_err(|_|())?;
                match self {
                    $(
                        AdvertisingData::$name(value) => {
                            o.push($code).map_err(|_|())?;
                            value.serialize(o)?;
                        },
                    )*
                }
                o[len_pos] = (o.len() - (len_pos+1)) as u8;
                Ok(())
            }
        }
    }
}



#[derive(Debug, Copy, Clone)]
pub struct AdvertisingFlags {
    pub discoverability: Discoverability,
    pub capability: Capability,
}

#[derive(Debug, Copy, Clone)]
pub enum Discoverability {
    None,
    Limited,
    General,
}

impl Default for Discoverability {
    fn default() -> Self {
        Discoverability::None
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Capability {
    LeOnly,
    BrEdrController,
    BrEdrHost,
}

impl Default for Capability {
    fn default() -> Self {
        Capability::LeOnly
    }
}

impl Default for AdvertisingFlags {
    fn default() -> Self {
        Self {
            discoverability: Discoverability::default(),
            capability: Capability::default(),
        }
    }
}

impl Serializable for AdvertisingFlags {
    fn serialize<N: ArrayLength<u8>>(&self, o: &mut Vec<u8, N>) -> Result<(), ()> {
        let mut val = 0;
        match self.discoverability {
            Discoverability::None => { /* nothing */ }
            Discoverability::Limited => { val |= 0b1; }
            Discoverability::General => { val |= 0b10; }
        }
        match self.capability {
            Capability::LeOnly => { val |= 0b100; }
            Capability::BrEdrController => { val |= 0b1000; }
            Capability::BrEdrHost => { val |= 0b10000; }
        }
        o.push(val).map_err(|_| ())?;
        Ok(())
    }
}



#[derive(Debug)]
pub struct ManufacturerSpecificData {
    pub company: CompanyIdentifier,
    pub data: Vec<u8, U29>,
}

impl Serializable for ManufacturerSpecificData {
    fn serialize<N: ArrayLength<u8>>(&self, o: &mut Vec<u8, N>) -> Result<(), ()> {
        self.company.serialize(o)?;
        self.data.serialize(o)
    }
}

advertising_data_impl!(
    Flags => 0x02 (AdvertisingFlags),
    ShortenedLocalName => 0x08 ( String<U27> ),
    CompleteLocalName => 0x09 ( String<U27> ),
    ManufacturerSpecific => 0xFF ( ManufacturerSpecificData ),
);

#[cfg(test)]
mod tests {
    use crate::types::advertising_data::{AdvertisingFlags, AdvertisingData, Discoverability};
    use heapless::{
        String,
        Vec,
        consts::*,
    };
    use drogue_bluetooth_hci::ser::Serializable;

    #[test]
    fn advertising_flags_serialization() {
        let mut flags = AdvertisingFlags::default();
        flags.discoverability = Discoverability::General;

        let mut buf: Vec<u8, U16> = Vec::new();

        flags.serialize(&mut buf).unwrap();

        assert_eq!(1, buf.len());
        assert_eq!(6, buf[0]);
    }

    #[test]
    fn flags_serialization() {
        let mut flags = AdvertisingFlags::default();
        flags.discoverability = Discoverability::General;

        let structure = AdvertisingData::Flags(flags);

        let mut buf: Vec<u8, U16> = Vec::new();
        structure.serialize(&mut buf).unwrap();
        assert_eq!(3, buf.len());

        assert_eq!(2, buf[0]);
        assert_eq!(0x02, buf[1]);
        assert_eq!(6, buf[2]);
    }

    #[test]
    fn complete_local_name_serialization() {
        let name: String<U27> = String::from("drogue-iot");
        let structure = AdvertisingData::CompleteLocalName(name);

        let mut buf: Vec<u8, U16> = Vec::new();
        structure.serialize(&mut buf).unwrap();

        assert_eq!(12, buf.len());
        assert_eq!(11, buf[0]);
        assert_eq!(0x09, buf[1]);
        assert_eq!( *"drogue-iot".as_bytes(), buf[2..]);
    }
}

